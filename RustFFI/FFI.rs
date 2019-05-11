use std::ffi::c_void;
use std::slice::from_raw_parts;

extern "C" fn callback(data: *const c_void, size: usize, user_data: *mut c_void) {
    let user_data: &mut Vec<u8> = unsafe { &mut *(user_data as *mut Vec<u8>) };

    let data_slice = unsafe { from_raw_parts(data as *const u8, size) };
    user_data.extend_from_slice(data_slice);
}

unsafe fn func_from_c_library(
    _callback: extern "C" fn(*const c_void, usize, *mut c_void),
    _user_data: *mut c_void,
) {
    unimplemented!()
}

pub fn write_to_bytes() -> Vec<u8> {
    let mut user_data = Vec::<u8>::new();
    let pointer_to_c_void = &mut user_data as *mut _ as *mut c_void;

    unsafe {
        func_from_c_library(callback, pointer_to_c_void);
    }

    user_data
}
