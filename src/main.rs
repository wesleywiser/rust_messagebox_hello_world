extern crate user32;
extern crate winapi;

use user32::MessageBoxW;
use winapi::winuser::{MB_OK, MB_ICONINFORMATION};

fn main() {
    let lp_text : Vec<u16> = "Hello, world! \u{1F60E}\0".encode_utf16().collect();
    let lp_caption : Vec<u16> = "MessageBox Example\0".encode_utf16().collect();

    unsafe {
        MessageBoxW(
            std::ptr::null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            MB_OK | MB_ICONINFORMATION
        );
    }
}
