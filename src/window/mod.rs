
extern crate winapi;
extern crate user32;
extern crate kernel32;
extern crate widestring;

mod winwrapper;

use winapi::shared::windef::HWND;
use std::ptr::null_mut;

use winapi::um::winuser::{
    RegisterClassExW,
    WNDCLASSEXW,
    WS_CAPTION,
    WS_MINIMIZEBOX,
    WS_SYSMENU, 
    DefWindowProcW,  
    CreateWindowExW,
    MSG,
    GetMessageW,
    TranslateMessage,
    DispatchMessageW,
    ShowWindow,
    SW_SHOWDEFAULT,
    UpdateWindow 
};


pub struct Window {
    pub window_handle: HWND
}

pub fn get_window(width: i32, height: i32, window_str: &str, class_str: &str) -> Result<Window, std::io::Error> {
    let class_name = winwrapper::LPCWSTR::new(class_str);
    let window_name = winwrapper::LPCWSTR::new(window_str);
    unsafe {
        let instance_handle = kernel32::GetModuleHandleExW(1, null_mut(), null_mut());

         let window_class = WNDCLASSEXW {
            style: 0,
            lpfnWndProc: Some (DefWindowProcW),
            hInstance: instance_handle as winapi::shared::minwindef::HINSTANCE,
            lpszClassName : class_name.as_ptr(),
            cbClsExtra : 0,
            cbWndExtra : 0,
            hIcon: null_mut(),
            hIconSm: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
            cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32
        };
        RegisterClassExW(&window_class);

        let left = 350;
        let right = left + width;
        let top = 100;
        let bottom = top + height;

        let window_handle = CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_CAPTION | WS_MINIMIZEBOX | WS_SYSMENU,
            left,
            top,
            right - left,
            bottom - top,
            null_mut(),
            null_mut(),
            instance_handle as winapi::shared::minwindef::HINSTANCE,
            null_mut()
        );

        if window_handle.is_null() {
            Err ( std::io::Error::last_os_error() )
        }   else {
            ShowWindow(window_handle, SW_SHOWDEFAULT);
            UpdateWindow(window_handle);
            Ok ( Window {window_handle} )
        }
    }
}

pub fn handle_message( window : &mut Window ) -> bool {
    unsafe {
        let mut message : MSG = std::mem::uninitialized();
        if GetMessageW( &mut message as *mut MSG, window.window_handle, 0, 0 ) > 0 {
            TranslateMessage( &message as *const MSG );
            DispatchMessageW( &message as *const MSG );
            true
        } else {
            false
        }
    }
}