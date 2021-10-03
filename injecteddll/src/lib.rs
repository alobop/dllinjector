// lib.rs

use bindings_dll::Windows::Win32::System::Threading::*;
use bindings_dll::Windows::Win32::Foundation::*;
use bindings_dll::Windows::Win32::System::ProcessStatus::{K32EnumProcessModules, K32GetModuleFileNameExA};
use bindings_dll::Windows::Win32::System::SystemServices::*;

use core::ffi::c_void;
use std::convert::TryInto;
use std::mem;
use std::u32;
use std::str;


#[no_mangle]
pub extern "stdcall" fn DllMain(_handle: HINSTANCE, reason: u32, _ptr:*mut c_void) -> bool {

    match reason{
        DLL_PROCESS_ATTACH =>  enumerate_modules(),
        DLL_PROCESS_DETACH => println!("Injected the code pd"),
        DLL_THREAD_ATTACH => println!("Injected the code ta"),
        DLL_THREAD_DETACH => println!("Injected the code td"),
        _ => println!("Unknown reason")
    }
    println!("Reason: {}", reason);
    return true;
}

fn enumerate_modules()
{
    //let h_mods_array_size = 1024;
    let mut h_mods: [HINSTANCE; 1024] = [HINSTANCE::default(); 1024];
    let h_process: HANDLE;
    let mut cb_needed: u32 = 0;

    let mut i = 0;
    
    let id: u32 = unsafe{ GetCurrentProcessId()};
    println!("Current process ID: {}", id);

    h_process = unsafe{ OpenProcess( PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, id ) };
    
    let _name = if h_process.is_null()
    {
        String::from("No process")

    } else{
        String::from("We are good")
    };

    let size_h_mods:  u32 = (h_mods.len() * mem::size_of::<HINSTANCE>())
        .try_into()
        .unwrap();

    let enum_mresult: BOOL = unsafe{ K32EnumProcessModules(h_process, h_mods.as_mut_ptr(), size_h_mods, &mut cb_needed)};

    if enum_mresult.as_bool()
    {
        let size_h_instance:  u32 = (mem::size_of::<HINSTANCE>())
        .try_into()
        .unwrap();

        while i < (cb_needed / size_h_instance)
        {
            let mut name_vec = vec![0u8; 100];
            let name_ptr = name_vec.as_mut_ptr();
            let name = PSTR { 0: name_ptr };
            let module_file_result = unsafe {  K32GetModuleFileNameExA( h_process, h_mods[i as usize], name, MAX_PATH ) } ;

            if module_file_result > 0 as u32
            {
                let s = str::from_utf8(&name_vec).unwrap();
                let o_s = String::from(s);

                println!("{}", o_s);
            }

            i = i + 1;
        }
    }
    
    

}