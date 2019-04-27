#![feature(box_syntax)]

// Framework

trait Bind<S: ?Sized> {
    fn bind(&mut self, service: Box<S>);
    fn get(&self) -> &Box<S>;
    fn get_mut(&mut self) -> &mut Box<S>;
}

macro_rules! binding {
    ($client:ty , $field:ident => $service_trait:ty) => {
        impl Bind<$service_trait> for $client {
            fn bind(&mut self, service: Box<$service_trait>) {
                self.$field = Some(service);
            }
            
            fn get(&self) -> &Box<$service_trait> {
                self.$field.as_ref()
                    .unwrap_or_else(|| panic!(concat!(stringify!($service_trait), " not provided!")))
            }
            
            fn get_mut(&mut self) -> &mut Box<$service_trait> {
                self.$field.as_mut()
                    .unwrap_or_else(|| panic!(concat!(stringify!($service_trait), " not provided!")))
            }
        }
    }
}

macro_rules! bind {
    ( $on:expr, { $($service_trait:ty => $service_impl:expr),* }) => ({
        $(
            <Bind<$service_trait>>::bind($on, $service_impl);
        );*
    });
}

macro_rules! srv {
    ( $bind:expr, mut $t:ty ) => ( <Bind<$t>>::get_mut($bind) );
    ( $bind:expr, $t:ty ) => ( <Bind<$t>>::get($bind) );
}

// Client implementation

#[derive(Default)]
struct Client {
    foo: Option<Box<FooService>>,
    bar: Option<Box<BarService>>,
}

binding!(Client, foo => FooService);
binding!(Client, bar => BarService);

impl Client {
    fn do_the_thing(&mut self) {
        srv!(self, mut FooService).required_foo_method();
        srv!(self, BarService).required_bar_method();
    }
}

// Service interfaces

trait FooService {
    fn required_foo_method(&mut self);
}


trait BarService {
    fn required_bar_method(&self);
}

// Service implementations

#[derive(Default)]
struct ImplFoo { counter: i32 }

struct ImplBar;

impl FooService for ImplFoo {
    fn required_foo_method(&mut self) {
        self.counter += 1;
        println!("Hello from foo! counter={}", self.counter);
    }
}

impl BarService for ImplBar {
    fn required_bar_method(&self) {
        println!("Hello from bar!");
    }
}

// Demonstration

fn main() {
    let mut client = Client::default();
    
    bind!(&mut client, {
        FooService => box ImplFoo::default(),
        BarService => box ImplBar
    });
}