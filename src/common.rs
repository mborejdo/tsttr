use std::process;
use std::ptr;

use winapi::um::winuser::{MessageBoxW, MB_OK};

use crate::str_to_wide;

pub fn report_and_exit(error_msg: &str) {
    show_msg_box(error_msg);
    process::exit(1);
}

pub fn show_msg_box(message: &str) {
    let mut message = str_to_wide!(message);

    unsafe {
        MessageBoxW(
            ptr::null_mut(),
            message.as_mut_ptr(),
            ptr::null_mut(),
            MB_OK,
        );
    }
}
