// lib.rs

use bindings::Windows::Win32::System::Threading::*;
use bindings::Windows::Win32::Foundation::*;
use bindings::Windows::Win32::System::ProcessStatus::{K32EnumProcessModules, K32GetModuleFileNameExA};
use bindings::Windows::Win32::System::SystemServices::*;

use core::ffi::c_void;
use std::convert::TryInto;
use std::mem;
use std::u32;
use std::str;
use std::fmt;
use winsafe::{winexec, SafeHandle};

#[derive(Debug)]
pub struct Process {
    pub id: u32,
    pub name: String,
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{number:>width$}] {}", self.name, number = self.id, width = 5)
    }
}


#[no_mangle]
pub extern "stdcall" fn DllMain(_handle: HINSTANCE, reason: u32, _ptr:*mut c_void) -> bool {

    match reason{
        DLL_PROCESS_ATTACH =>  {enumerate_modules().unwrap()},
        DLL_PROCESS_DETACH => println!("Injected the code pd"),
        DLL_THREAD_ATTACH => println!("Injected the code ta"),
        DLL_THREAD_DETACH => println!("Injected the code td"),
        _ => println!("Unknown reason")
    }
    println!("Reason: {}", reason);
    return true;
}

pub fn enumerate_modules()-> winsafe::Result<()>
{
    let mut h_mods: [HINSTANCE; 1024] = [HINSTANCE::default(); 1024];
    let mut cb_needed: u32 = 0;

    let mut i = 0;
    
    let id = winexec!(GetCurrentProcessId())?;
    println!("Current process ID: {}", id);
    

    if let Ok(h_process) = winexec!(SafeHandle::from(OpenProcess( PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, id ))){

        let size_h_mods:  u32 = (h_mods.len() * mem::size_of::<HINSTANCE>())
            .try_into()
            .unwrap();


        if winexec!(K32EnumProcessModules(h_process.handle, h_mods.as_mut_ptr(), size_h_mods, &mut cb_needed))
        .is_ok()
        {
            let size_h_instance:  u32 = (mem::size_of::<HINSTANCE>())
            .try_into()
            .unwrap();

            while i < (cb_needed / size_h_instance)
            {
                let mut name_vec = vec![0u8; 100];
                let name_ptr = name_vec.as_mut_ptr();
                let name = PSTR { 0: name_ptr };

                if let Ok(_module_file_result) = winexec!(K32GetModuleFileNameExA( h_process.handle, h_mods[i as usize], name, MAX_PATH ))
                {
                    let s = str::from_utf8(&name_vec).unwrap();
                    let o_s = String::from(s);

                    println!("{}", o_s);
                }
                
                 i = i + 1;
            }
        }
    }

    Ok(())

}