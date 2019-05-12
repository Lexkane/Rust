#![allow(dead_code)]
#[macro_use]
extern crate slog;
extern crate sloggers;
use sloggers::Build;
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use slog::Logger;
use std::str::FromStr;
#[macro_use]
extern crate chan;
extern crate nix;
use std::process::Command;
use std::os::unix::process::CommandExt;
use std::os::unix::process::ExitStatusExt;
use std::thread;
use std::process::Child;
use std::process::exit;
use nix::sys::signal::*;
use chan::Sender;
use chan::Receiver;
extern crate clap;
extern crate humantime;
use std::time::Duration;
use clap::App;
use clap::Arg;
use reqwest::Url;
extern crate reqwest;
fn run_command(cmd: &str, args: Vec<String>) -> std::io::Result<Child> {
    return Command::new(cmd)
        // This is to put in a separate process group, so signals don't propogate
        .before_exec(|| {
            let mut old_signal_set = SigSet::empty();
            let all_signals = SigSet::all();
            assert!(!nix::unistd::setpgid(nix::unistd::Pid::from_raw(0),
                                          nix::unistd::Pid::from_raw(0))
                .is_err());
            assert!(sigprocmask(SigmaskHow::SIG_UNBLOCK,
                                Some(&all_signals),
                                Some(&mut old_signal_set)).is_ok());
Ok(())
        })
        .args(args)
        .spawn();
}
fn forward_signals(
    logger: Logger,
    pid: nix::unistd::Pid,
    sig_rx: Receiver<nix::sys::signal::Signal>,
) {
    /* This now just forward signals */
    loop {
        let signal = sig_rx.recv().unwrap();
        info!(logger, "Forwarding signal: {:?}", signal);
        assert!(nix::sys::signal::kill(pid, signal).is_ok());
    }
}
enum DegistrationResult {
    /* We received another signal, so we're going into forwarding signals mode */
    Interrupted(nix::sys::signal::Signal),
    TimedOut,
    Success,
    Failed,
}
enum WaitGracePeriodResult {
    Interrupted(nix::sys::signal::Signal),
    Success,
}
fn run_deregistration(logger: Logger, config: Config, sender: Sender<DegistrationResult>) {
    info!(logger, "Beginning discovery deregistration");
    let discovery_host = format!(
        "OBFUSCATED",
        region = config.region,
        environment = config.environment
    );
    let path = format!(
        "OBFUSCATED",
        app = config.app,
        instance = config.instance
    );
    let url = &Url::parse(&format!("OBFUSCATED", discovery_host, path)).unwrap();
debug!(
        logger,
        "Discovery deregistration proceeding against: {}",
        url.to_string()
    );
    let params = [("value", "OBFUSCATED")];
    for _ in 0..3 {
        let client = reqwest::Client::builder()
            .timeout(Some(Duration::from_secs(3)))
            .build()
            .unwrap();
        match client.put(url.clone()).form(&params).send() {
            Ok(mut resp) => {
                if resp.status().is_success() {
                    info!(logger, "Deregistration succeeded: {}", resp.text().unwrap());
                    sender.send(DegistrationResult::Success);
                    return;
                } else {
                    error!(logger, "Deregistration failed, retrying: {}", resp.status());
                }
            }
            Err(err) => {
                error!(logger, "Deregistration failed, retrying: {:?}", err);
            }
        }
    }
    sender.send(DegistrationResult::Failed);
    error!(logger, "Discovery deregistration failed");
}
fn deregistration(
    logger: Logger,
    config: Config,
    sig_rx: &Receiver<nix::sys::signal::Signal>,
) -> DegistrationResult {
    let discovery_deregistration_timeout_chan = chan::after(config.deregistration_timeout);
    let (send, recv) = chan::async();
    thread::Builder::new()
        .name("discovery-degistration".to_string())
        .spawn(move || run_deregistration(logger, config, send))
        .unwrap();
chan_select! {
        discovery_deregistration_timeout_chan.recv() => {
        return DegistrationResult::TimedOut
        },
        recv.recv() -> res => match res {
            Some(status) => return status,
            None => return DegistrationResult::Failed
        },
        sig_rx.recv() -> new_signal => {
        return DegistrationResult::Interrupted(new_signal.unwrap())
        }
    }
}
fn wait_grace_period(
    _logger: &Logger,
    config: Config,
    sig_rx: &Receiver<nix::sys::signal::Signal>,
) -> WaitGracePeriodResult {
    let discovery_grace_period_timeout = chan::after(config.discovery_wait);
chan_select! {
        discovery_grace_period_timeout.recv() => {
        return WaitGracePeriodResult::Success
        },
        sig_rx.recv() -> new_signal => {
        return WaitGracePeriodResult::Interrupted(new_signal.unwrap())
        }
    }
}
fn background_watcher(
    logger: Logger,
    config: Config,
    pid: nix::unistd::Pid,
    sig_rx: Receiver<nix::sys::signal::Signal>,
) {
    /*
     * In this state, the loop is just listening, and waiting for a signal.
     * Once we receive a signal, we kick off deregistration in discovery,
     * and we run that with timeout N. Either timeout N must elapse, or
     * the discovery deregistration must finish. Once that happens,
     * we forward the signal that we last received.
     *
     * If at any point, during this we receive another signal,
     * all bets are off, and we immediately start forwarding
     * signals.
     */
    let first_signal = sig_rx.recv().unwrap();
/* Phase 1 */
    info!(logger, "Entering do deregistration phase");
    match deregistration(logger.clone(), config.clone(), &sig_rx) {
        DegistrationResult::Interrupted(new_signal) => {
            warn!(logger, "Discovery deregistration process interrupted");
            assert!(nix::sys::signal::kill(pid, first_signal).is_ok());
            assert!(nix::sys::signal::kill(pid, new_signal).is_ok());
            return forward_signals(logger, pid, sig_rx);
        }
        DegistrationResult::TimedOut => {
            error!(logger, "Discovery deregistration timed out");
        }
        DegistrationResult::Success => info!(logger, "Discovery deregistration completed"),
        DegistrationResult::Failed => error!(
            logger,
            "Discovery deregistration failed, continuing to grace period"
        ),
    }
info!(logger, "Entering waiting for grace period phase");
    /* Phase 2 */
    match wait_grace_period(&logger, config.clone(), &sig_rx) {
        WaitGracePeriodResult::Interrupted(new_signal) => {
            warn!(logger, "Discovery grace period wait interrupted");
            assert!(nix::sys::signal::kill(pid, first_signal).is_ok());
            assert!(nix::sys::signal::kill(pid, new_signal).is_ok());
            return forward_signals(logger, pid, sig_rx);
        }
        WaitGracePeriodResult::Success => {
            info!(logger, "Discovery grace period successfully elapsed");
            assert!(nix::sys::signal::kill(pid, first_signal).is_ok());
            return forward_signals(logger, pid, sig_rx);
        }
    }
}
fn signal_cb(logger: Logger, ss: SigSet, tx: Sender<nix::sys::signal::Signal>) {
    loop {
        match ss.wait() {
            Err(e) => error!(logger, "Failed to read signal: {:?}", e),
            Ok(val) => {
                info!(logger, "Received signal: {:?}", val);
                tx.send(val);
            }
        };
    }
}
// ss is the set of signals which we intend for the signal callback handler to process / listen to
fn run_signal_watcher(logger: Logger, ss: SigSet, tx: Sender<nix::sys::signal::Signal>) {
    // Has to be constructed in the main thread
    thread::Builder::new()
        .name("signal-cb".to_string())
        .spawn(move || signal_cb(logger, ss, tx))
        .unwrap();
}
fn run_signal_processor(
    logger: Logger,
    config: Config,
    pid: i32,
    rx: Receiver<nix::sys::signal::Signal>,
) {
    thread::Builder::new()
        .name("signal-handler".to_string())
        .spawn(move || background_watcher(logger, config, nix::unistd::Pid::from_raw(pid), rx))
        .unwrap();
}
fn run(logger: Logger, config: Config) {
    let mut old_signal_set = SigSet::empty();
    let mut ss = SigSet::empty();
    ss.add(SIGINT);
    ss.add(SIGTERM);
// Block the signals
    assert!(sigprocmask(SigmaskHow::SIG_BLOCK, Some(&ss), Some(&mut old_signal_set)).is_ok());
let (signal_tx, signal_rx) = chan::async();
    run_signal_watcher(logger.clone(), ss.clone(), signal_tx);
let (cmd, args) = config.command.split_first().unwrap();
    let mut cmd_handle = run_command(cmd, args.to_vec()).unwrap();
    run_signal_processor(
        logger.clone(),
        config.clone(),
        cmd_handle.id() as i32,
        signal_rx,
    );
debug!(logger, "Running command");
    let res = cmd_handle.wait();
    // TODO: Fix up the command cleanup code
    info!(logger, "Cmd completed: {:?}", res);
    match res {
        Ok(exit_status) => {
            match exit_status.code() {
                Some(code) => exit(code),
                // This means we've been terminated by a signal
                None => (),
            };
            match exit_status.signal() {
                Some(signal) => exit(signal + 128),
                None => (),
            }
            exit(0);
        }
        Err(_other) => {
            error!(logger, "Failed to run command");
            exit(1);
        }
    }
}
#[derive(Debug, Clone)]
struct Config {
    deregistration_timeout: Duration,
    discovery_wait: Duration,
    region: String,
    instance: String,
    app: String,
    environment: String,
    command: Vec<String>,
}
fn main() {
    let app = App::new("signal-watcher")
        .version("1.0")
        .arg(
            Arg::with_name("deregistration-timeout")
                .long("deregistration-timeout")
                .takes_value(true)
                .value_name("SECONDS")
                .help("How long to wait for discovery deregistration call")
                .default_value("15s")
                .required(true)
                .validator(validate_timeout),
        )
        .arg(
            Arg::with_name("discovery-wait")
                .long("discovery-wait")
                .takes_value(true)
                .value_name("SECONDS")
                .validator(validate_timeout)
                .required(true)
                .help("How long to wait after deregistration before forwarding the signal")
                .default_value("60s"),
        )
        .arg(
            Arg::with_name("region")
                .long("region")
                .takes_value(true)
                .value_name("REGION")
                .help("What EC2 region are we in")
                .env("EC2_REGION")
                .required(true),
        )
        .arg(
            Arg::with_name("environment")
                .long("environment")
                .takes_value(true)
                .value_name("NETFLIX_ENVIRONMENT")
                .help("Which environment are we running in")
                .env("NETFLIX_ENVIRONMENT")
                .required(true),
        )
        .arg(
            Arg::with_name("app")
                .long("app")
                .takes_value(true)
                .value_name("NETFLIX_APP")
                .help("Which is our app name")
                .env("NETFLIX_APP")
                .required(true),
        )
        .arg(
            Arg::with_name("instance-id")
                .long("instance-id")
                .takes_value(true)
                .value_name("INSTANCEID")
                .help("EC2 / Container instance ID")
                .env("EC2_INSTANCE_ID")
                .required(true),
        )
        .arg(
            Arg::with_name("command")
                .last(true)
                .value_name("COMMAND")
                .multiple(true)
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("log-level")
                .long("local-level")
                .takes_value(true)
                .value_name("LOG_LEVEL")
                .env("LOG_LEVEL")
                .validator(validate_loglevel)
                .default_value("info"),
        )
        .get_matches();
let log_level =
        sloggers::types::Severity::from_str(app.value_of("log-level").unwrap()).unwrap();
    let logger = new_logger(log_level);
let config: Config = Config {
        deregistration_timeout: app.value_of("deregistration-timeout")
            .unwrap()
            .parse::<humantime::Duration>()
            .unwrap()
            .into(),
        discovery_wait: app.value_of("discovery-wait")
            .unwrap()
            .parse::<humantime::Duration>()
            .unwrap()
            .into(),
        region: app.value_of("region").unwrap().to_string(),
        environment: app.value_of("environment").unwrap().to_string(),
        instance: app.value_of("instance-id").unwrap().to_string(),
        app: app.value_of("app").unwrap().to_string(),
        command: app.values_of("command")
            .unwrap()
            .map(|s: &str| s.to_string())
            .collect(),
    };
run(logger, config);
}
fn new_logger(severity: sloggers::types::Severity) -> Logger {
    let mut builder = TerminalLoggerBuilder::new();
    builder.level(severity);
    builder.destination(Destination::Stderr);
let logger = builder.build().unwrap();
    return logger;
}
fn validate_timeout(v: String) -> Result<(), String> {
    let num: Duration = match v.parse::<humantime::Duration>() {
        Ok(n) => n.into(),
        Err(e) => return Err(e.to_string()),
    };
    if num < Duration::from_secs(1) {
        return Err(String::from(
            "Timeouts must be greater than or equal to 1 second",
        ));
    }
    if num > Duration::from_secs(300) {
        return Err(String::from("Timeouts must be smaller than 300 seconds"));
    }
    return Ok(());
}
fn validate_loglevel(v: String) -> Result<(), String> {
    match sloggers::types::Severity::from_str(&v) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}