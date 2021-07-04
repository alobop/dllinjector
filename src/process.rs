use bindings::Windows::Win32::Foundation::*;
use bindings::Windows::Win32::System::Diagnostics::Debug::*;
use bindings::Windows::Win32::System::LibraryLoader::*;
use bindings::Windows::Win32::System::Memory::*;
use bindings::Windows::Win32::System::ProcessStatus::*;
use bindings::Windows::Win32::System::Threading::*;
use bindings::Windows::Win32::System::WindowsProgramming::*;
use winsafe::{SafeHandle, winexec};

use core::ffi::c_void;

use std::{convert::TryInto, fmt, ptr::null_mut, str, u32};

const MAX_PROCESS_LIST_SIZE: usize = 1204;
const INITIAL_PROCESS_LIST_SIZE: usize = 128;

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

impl Process {
    pub fn new(id: u32) -> Process {
        let mut process_name: String = String::from("<unknown>");

        if let Ok(process_handle) = winexec!(SafeHandle::from(OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            id
        ))) {
            let mut base_module: HINSTANCE = HINSTANCE::default();
            let mut module_count: u32 = 0;

            if winexec!(K32EnumProcessModules(
                process_handle.handle,
                &mut base_module,
                std::mem::size_of::<HINSTANCE>().try_into().unwrap(),
                &mut module_count,
            ))
            .is_ok()
            {
                let mut name_vec = vec![0u8; 100];
                let name = PSTR {
                    0: name_vec.as_mut_ptr(),
                };

                if let Ok(length) = winexec!(K32GetModuleBaseNameA(process_handle.handle, base_module, name, 100)) {
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

    pub fn list_current_processes() -> winsafe::Result<Vec<Process>> {
        let mut process_count: u32 = 0;
        let mut processes_ids: Vec<u32> = Vec::new();
        processes_ids.resize(INITIAL_PROCESS_LIST_SIZE as usize, 0);

        loop {
            let size: u32 = (processes_ids.len() * std::mem::size_of::<i32>()).try_into().unwrap();

            winexec!(K32EnumProcesses(processes_ids.as_mut_ptr(), size, &mut process_count))?;

            if (((process_count as usize) / std::mem::size_of::<u32>() == processes_ids.len())
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

    pub fn inject_dll(&self, dll_path: &str) -> winsafe::Result<()> {
        let process_handle = winexec!(SafeHandle::from(OpenProcess(PROCESS_ALL_ACCESS, false, self.id)))?;

        let dll_path_address = winexec!(VirtualAllocEx(
            process_handle.handle,
            std::ptr::null_mut(),
            dll_path.len() + 1,
            MEM_COMMIT,
            PAGE_READWRITE,
        ))?;

        winexec!(WriteProcessMemory(
            process_handle.handle,
            dll_path_address,
            dll_path.as_ptr() as *const c_void,
            dll_path.len() + 1,
            null_mut(),
        ))?;

        let kernel32_module = winexec!(GetModuleHandleA("Kernel32.dll"))?;
        let proc_address = winexec!(GetProcAddress(kernel32_module, "LoadLibraryA"))?;

        let thread_handle = winexec!(SafeHandle::from(CreateRemoteThread(
            process_handle.handle,
            std::ptr::null_mut(),
            0,
            Some(std::mem::transmute(proc_address)),
            dll_path_address,
            0,
            std::ptr::null_mut(),
        )))?;

        winexec!(WaitForSingleObject(thread_handle.handle, INFINITE))?;

        Ok(())
    }
}
