// lib.rs
windows::include_bindings!();

use crate::Windows::Win32::Foundation::HINSTANCE;
use crate::Windows::Win32::System::SystemServices::*;
use core::ffi::c_void;

#[no_mangle]
pub extern "stdcall" fn DllMain(_handle: HINSTANCE, reason: u32, _ptr:*mut c_void) -> bool {

    match reason{
        DLL_PROCESS_ATTACH =>  println!("Injected the code pa"),
        DLL_PROCESS_DETACH => println!("Injected the code pd"),
        DLL_THREAD_ATTACH => println!("Injected the code ta"),
        DLL_THREAD_DETACH => println!("Injected the code td"),
        _ => println!("Unknown reason")
    }
    
    return true;
}