fn main() {
    windows::build!(
        Windows::Win32::Foundation::{HINSTANCE, HANDLE, INVALID_HANDLE_VALUE, BOOL, MAX_PATH},
        Windows::Win32::System::ProcessStatus::{K32EnumProcesses, K32GetModuleFileNameExA, K32EnumProcessModules},
        Windows::Win32::System::Threading::{CreateRemoteThread, GetCurrentProcessId, OpenProcess, PROCESS_ACCESS_RIGHTS},
        Windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH}
    );
}
