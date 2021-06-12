use bindings::Windows::Win32::System::ProcessStatus::*;
use bindings::Windows::Win32::System::SystemServices::*;
use bindings::Windows::Win32::System::Threading::*;

use std::convert::TryInto;
use std::str;

fn main() {
    let mut processes: Vec<u32> = Vec::new();
    processes.resize(300, 0);
    let mut count: u32 = 0;
    let result_bool;
    let size: u32 = (processes.len() * std::mem::size_of::<i32>())
        .try_into()
        .unwrap();

    unsafe {
        let result: BOOL = K32EnumProcesses(processes.as_mut_ptr(), size, &mut count);
        result_bool = bool::from(result);
    }

    println!("Retrieved processes {}, count {}", result_bool, count / 4);
    println!("{:?}", processes);

    processes.iter().for_each(|item| {
        let handle: HANDLE;

        unsafe {
            handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, *item);

            let name = if handle.is_null() {
                String::from("No name")
            } else {
                let mut name_vec = vec![0u8; 100];
                let name_ptr = name_vec.as_mut_ptr();
                let name = PSTR { 0: name_ptr };
                let a = K32GetModuleFileNameExA(handle, HINSTANCE::default(), name, 100);

                let s = str::from_utf8(&name_vec).unwrap();
                String::from(s)
            };

            println!("{} {}", item, name);
        }
    });
}
