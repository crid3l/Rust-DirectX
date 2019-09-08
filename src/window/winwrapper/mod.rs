use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::winerror::HRESULT;

pub struct LPCWSTR {
    wstr: Vec<u16>,
}

impl LPCWSTR {
    pub fn new(string: &str) -> LPCWSTR {
        LPCWSTR {
            wstr: OsStr::new(string)
                .encode_wide()
                .chain(std::iter::once(0))
                .collect(),
        }
    }
    pub fn as_ptr(&self) -> *const u16 {
        self.wstr.as_ptr()
    }
}