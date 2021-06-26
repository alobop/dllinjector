#![allow(unused_parens)]

mod process;

use crate::process::Process;
use std::env;

fn main() {
    let arg = env::args().nth(1);

    match arg {
        Some(process_name) => {
            let processes = Process::list_current_processes().unwrap();
            let path = "E:\\dllinjector\\injecteddll\\target\\debug\\injecteddll.dll\0";
            
            let found_process = processes.iter().find(|x| {
                return x.name.contains(&process_name)
            });

            match found_process {
                Some(target_process) => {
                    println!("Found {}", target_process);
                
                    match target_process.inject_dll(path){
                        Ok(()) =>println!("Injected DLL"),
                        Err(err) => println!("Failed to inject DLL: {}", err)
                    }
                },
                None => println!("Unable to find process: {}", process_name)
            }
        },
        None => {
            let processes = Process::list_current_processes().unwrap();

            processes.iter().for_each(|x|{
                println!("{}", x);
            })
        }
    }
}
