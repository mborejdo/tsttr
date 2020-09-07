use std::ptr;
use winapi::shared::windef::HWND;

#[derive(Clone, Copy, Debug)]
pub struct Window(pub HWND);

unsafe impl Send for Window {}

impl Window {

}

impl Default for Window {
    fn default() -> Self {
        Window(ptr::null_mut())
    }
}

impl PartialEq for Window {
    fn eq(&self, other: &Window) -> bool {
        self.0 == other.0
    }
}
