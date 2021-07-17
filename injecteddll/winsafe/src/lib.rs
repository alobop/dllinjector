use bindings_dll::Windows::Win32::Foundation::*;
use bindings_dll::Windows::Win32::System::Diagnostics::Debug::*;
use bindings_dll::Windows::Win32::System::Memory::*;
use std::ffi::CStr;
use std::fmt;
use std::ptr::null_mut;

#[derive(Debug)]
pub struct SafeHandle {
    pub handle: HANDLE,
}

impl Drop for SafeHandle {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            unsafe { CloseHandle(self.handle) };
        }
    }
}

impl SafeHandle {
    pub fn new(handle: HANDLE) -> SafeHandle {
        SafeHandle { handle: handle }
    }

    pub fn is_valid(&self) -> bool {
        !self.handle.is_null()
    }
}

impl From<HANDLE> for SafeHandle {
    fn from(item: HANDLE) -> Self {
        SafeHandle::new(item)
    }
}

#[derive(Debug)]
pub struct WindowsError {
    pub error_message: String,
    pub error_code: u32,
}

impl fmt::Display for WindowsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{:#X}] {}", self.error_code, self.error_message)
    }
}

impl WindowsError {
    pub fn new(error: u32) -> Self {
        let mut str_ptr: usize = 0;
        let ptr_ptr_str: *mut usize = &mut str_ptr;
        let str_size: u32 = unsafe {
            FormatMessageA(
                FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
                null_mut(),
                error,
                0x0,
                std::mem::transmute(ptr_ptr_str),
                0,
                null_mut(),
            )
        };

        let error_message: String = if 0 != str_size {
            let cstr = unsafe { CStr::from_ptr(str_ptr as *const i8) };
            let result = String::from(cstr.to_str().unwrap());
            unsafe { LocalFree(str_ptr as isize) };
            result
        } else {
            String::from("Unknown error code")
        };

        WindowsError {
            error_code: error,
            error_message: error_message,
        }
    }
}

pub type Result<T> = std::result::Result<T, WindowsError>;

pub fn execute<T>(mut fun: impl FnMut() -> T) -> Result<T> {
    unsafe { SetLastError(0) };
    let output: T = fun();
    let error_code: u32 = (unsafe { GetLastError() }).0;
    if 0 == error_code {
        Ok(output)
    } else {
        Err(WindowsError::new(error_code))
    }
}

#[macro_export]
macro_rules! winexec {
    ($input:expr) => {
        winsafe_dll::execute(|| unsafe { $input })
    };
}
