fn main() {
    windows::build!(
        // Common
        Windows::Win32::System::WindowsProgramming::{INFINITE},
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::Foundation::{ BOOL, HANDLE, PSTR, HINSTANCE, INVALID_HANDLE_VALUE, HINSTANCE, FARPROC, MAX_PATH},
        // Thread
        Windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH,LPTHREAD_START_ROUTINE},
        Windows::Win32::System::Threading::{CreateRemoteThread, WAIT_RETURN_CAUSE},
        // Security
        Windows::Win32::Security::SECURITY_ATTRIBUTES,
        // Process
        Windows::Win32::System::Threading::{WaitForSingleObject, GetCurrentProcessId, OpenProcess, PROCESS_ACCESS_RIGHTS},
        Windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
        Windows::Win32::System::ProcessStatus::{K32EnumProcesses, K32GetModuleFileNameExA, K32GetModuleBaseNameA, K32EnumProcessModules},
        // Memory
        Windows::Win32::System::Diagnostics::Debug::{WriteProcessMemory, GetLastError, SetLastError, FormatMessageA, FORMAT_MESSAGE_OPTIONS},
        Windows::Win32::System::Memory::{VirtualAllocEx, VIRTUAL_ALLOCATION_TYPE},
        Windows::Win32::System::Memory::{PAGE_TYPE, LocalFree},

        // I/O
        Windows::Win32::Storage::FileSystem::GetFullPathNameA,
    );
}
