#![allow(unused_parens)]

use bindings::Windows::Win32::Foundation::*;
use bindings::Windows::Win32::System::Diagnostics::Debug::*;
use bindings::Windows::Win32::System::LibraryLoader::*;
use bindings::Windows::Win32::System::Memory::*;
use bindings::Windows::Win32::System::ProcessStatus::*;
use bindings::Windows::Win32::System::Threading::*;
use bindings::Windows::Win32::System::WindowsProgramming::*;

use core::ffi::c_void;
use std::{convert::TryInto, ptr::null_mut, str, u32, fmt};

const MAX_PROCESS_LIST_SIZE: usize = 1204;
const INITIAL_PROCESS_LIST_SIZE: usize = 128;

#[derive(Debug)]
struct SafeHandle {
    handle: HANDLE,
}

impl Drop for SafeHandle {
    fn drop(&mut self) {
        if (!self.handle.is_null()) {
            unsafe { CloseHandle(self.handle) };
        }
    }
}

impl SafeHandle {
    fn new(handle: HANDLE) -> SafeHandle {
        SafeHandle { handle: handle }
    }

    fn is_valid(&self) -> bool {
        !self.handle.is_null()
    }
}

#[derive(Debug)]
pub struct Process {
    pub id: u32,
    pub name: String,
}

impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{number:>width$}] {}",
            self.name,
            number = self.id,
            width = 5
        )
    }
}

impl Process {
    pub fn new(id: u32) -> Process {
        let process_handle: SafeHandle = SafeHandle::new(unsafe {
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, id)
        });
        let mut process_name: String = String::from("<unknown>");

        if (process_handle.is_valid()) {
            let mut base_module: HINSTANCE = HINSTANCE::default();
            let mut module_count: u32 = 0;
            let result = unsafe {
                K32EnumProcessModules(
                    process_handle.handle,
                    &mut base_module,
                    std::mem::size_of::<HINSTANCE>().try_into().unwrap(),
                    &mut module_count,
                )
                .into()
            };

            if (result) {
                let mut name_vec = vec![0u8; 100];
                let name = PSTR {
                    0: name_vec.as_mut_ptr(),
                };
                let length =
                    unsafe { K32GetModuleBaseNameA(process_handle.handle, base_module, name, 100) };

                if (length != 0) {
                    name_vec.resize(length.try_into().unwrap(), 0);
                    process_name = String::from(str::from_utf8(&name_vec).unwrap());
                }
            }
        }

        Process {
            id: id,
            name: process_name,
        }
    }

    pub fn list_current_processes() -> std::result::Result<Vec<Process>, String> {
        let mut process_count: u32 = 0;
        let mut processes_ids: Vec<u32> = Vec::new();
        processes_ids.resize(INITIAL_PROCESS_LIST_SIZE as usize, 0);

        loop {
            let size: u32 = (processes_ids.len() * std::mem::size_of::<i32>())
                .try_into()
                .unwrap();

            let result: bool = unsafe {
                K32EnumProcesses(processes_ids.as_mut_ptr(), size, &mut process_count).into()
            };

            if (!result) {
                return Err(String::from("Failed to enum processes"));
            }

            if (( (process_count as usize) / std::mem::size_of::<u32>()  == processes_ids.len())
                && processes_ids.len() < MAX_PROCESS_LIST_SIZE)
            {
                processes_ids.resize(processes_ids.len() * 2, 0);
            } else {
                break;
            }
        }

        let mut procecess: Vec<Process> = Vec::with_capacity(process_count as usize);
        processes_ids.iter().for_each(|item| {
            if (*item != 0) {
                procecess.push(Process::new(*item));
            }
        });

        return Ok(procecess);
    }

    pub fn inject_dll(&self, dll_path: &str) -> std::result::Result<(), String> {
        let process_handle: SafeHandle = SafeHandle::new(unsafe {
            OpenProcess(PROCESS_ALL_ACCESS, false, self.id)
        });

        if (!process_handle.is_valid()) {
            return Err(String::from("Unable to open process"));
        }

        let dll_path_address = unsafe {
            VirtualAllocEx(
                process_handle.handle,
                std::ptr::null_mut(),
                dll_path.len() + 1,
                MEM_COMMIT,
                PAGE_READWRITE,
            )
        };

        if (dll_path_address == null_mut()) {
            return Err(String::from("Unable to allocate memory"));
        }

        let mut bytes_writen: usize = 0;
        let result: bool = unsafe {
            WriteProcessMemory(
                process_handle.handle,
                dll_path_address,
                dll_path.as_ptr() as *const c_void,
                dll_path.len() + 1,
                &mut bytes_writen,
            )
            .into()
        };

        if (!result) {
            return Err(String::from("Unable to copy memory"));
        }

        let kernel32_module = unsafe { GetModuleHandleA("Kernel32.dll") };

        if (kernel32_module == HINSTANCE::default()) {
            return Err(String::from("Unable to get kernel32 handle"));
        }

        let proc_address = unsafe { GetProcAddress(kernel32_module, "LoadLibraryA") };

        if (proc_address.is_none()) {
            return Err(String::from("Unable to get kernel32 address"));
        }

        let thread_handle = SafeHandle::new(unsafe {
            CreateRemoteThread(
                process_handle.handle,
                std::ptr::null_mut(),
                0,
                Some(std::mem::transmute(proc_address)),
                dll_path_address,
                0,
                std::ptr::null_mut(),
            )
        });

        println!("th {:?} {}", thread_handle, dll_path);
        if (!thread_handle.is_valid()) {
            return Err(String::from("Unable to launch remote thread"));
        }

        let wait = unsafe { WaitForSingleObject(thread_handle.handle, INFINITE) };

        if (wait != WAIT_OBJECT_0) {
            return Err(String::from("Wait failed"));
        }

        Ok(())
    }
}
