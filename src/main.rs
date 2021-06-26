#![allow(unused_parens)]

use bindings::Windows::Win32::System::ProcessStatus::*;
use bindings::Windows::Win32::Foundation::*;
use bindings::Windows::Win32::System::Threading::*;
use bindings::Windows::Win32::System::Memory::*;
use bindings::Windows::Win32::System::Diagnostics::Debug::*;
use bindings::Windows::Win32::System::LibraryLoader::*;
use bindings::Windows::Win32::System::WindowsProgramming::*;

use std::{convert::TryInto, u32};
use std::str;
use std::env;
use core::ffi::c_void;

fn enumerate_processes()
{
    let mut processes: Vec<u32> = Vec::new();
    processes.resize(300, 0);
    let mut count: u32 = 0;
    let size: u32 = (processes.len() * std::mem::size_of::<i32>())
        .try_into()
        .unwrap();

    unsafe {
        let _result: BOOL = K32EnumProcesses(processes.as_mut_ptr(), size, &mut count);
    }

    processes.iter().for_each(|item| {

        if (*item != 0)
        {
            let handle: HANDLE;

            unsafe {
                handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, *item);

                let name = if handle.is_null() {
                    String::from("No name")
                } else {
                    let mut name_vec = vec![0u8; 100];
                    let name_ptr = name_vec.as_mut_ptr();
                    let name = PSTR { 0: name_ptr };
                    let _a = K32GetModuleFileNameExA(handle, HINSTANCE::default(), name, 100);
                    CloseHandle(handle);
                    let s = str::from_utf8(&name_vec).unwrap();
                    String::from(s)
                    
                };

                println!("{} {}", item, name);
            }
        }
    });
}

fn inject_dll(id:u32)
{
    let process_handle =  unsafe{
        OpenProcess(PROCESS_ALL_ACCESS, false, id)
    };

    if (process_handle.is_null())
    {
        println!("Unable to open {}", id);
        return
    }

    else
    {
        println!("Opened process");
    }

    let path = "Z:\\Rust\\dllinjector\\injecteddll\\target\\debug\\injecteddll.dll";

    let dll_path_address = unsafe{VirtualAllocEx(process_handle, std::ptr::null_mut(), path.len() + 1, MEM_COMMIT, PAGE_READWRITE)};

    println!("Address {:?}", dll_path_address);

    let mut bytes_writen: usize = 0;
    let result = unsafe{WriteProcessMemory(process_handle, dll_path_address, path.as_ptr() as *const c_void, path.len() + 1, &mut bytes_writen)};

    println!("Wrote into memory {}: {}", bool::from(result), bytes_writen);

    let kernel32_module = unsafe{GetModuleHandleA("Kernel32.dll")};
    let proc_address = unsafe{GetProcAddress(kernel32_module, "LoadLibraryA")};
    let thread_handle = unsafe{CreateRemoteThread(process_handle, std::ptr::null_mut(), 0, Some(std::mem::transmute(proc_address)), dll_path_address, 0, std::ptr::null_mut())};

    println!("Thread handle {:?}", thread_handle);

    unsafe{WaitForSingleObject(thread_handle, INFINITE)};

    println!("DLL loaded");
    unsafe {
        CloseHandle(process_handle);
    }
    
}

fn main() {
    let process_number = env::args().nth(1);

    match process_number {
        Some(pid) => inject_dll(pid.parse().unwrap()),
        None => enumerate_processes()
    }
}
