fn main() {
    windows::build!(
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::Foundation::{HINSTANCE, HANDLE, INVALID_HANDLE_VALUE, BOOL, MAX_PATH, PSTR, FARPROC},
        Windows::Win32::System::ProcessStatus::{K32EnumProcesses, K32GetModuleFileNameExA, K32EnumProcessModules},
        Windows::Win32::System::Threading::{CreateRemoteThread, GetCurrentProcessId, OpenProcess, PROCESS_ACCESS_RIGHTS},
        Windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH, DLL_THREAD_ATTACH, DLL_THREAD_DETACH},
        Windows::Win32::System::Diagnostics::Debug::{WriteProcessMemory, GetLastError, SetLastError, FormatMessageA, FORMAT_MESSAGE_OPTIONS},
        Windows::Win32::System::Memory::{VirtualAllocEx, VIRTUAL_ALLOCATION_TYPE},
        Windows::Win32::System::Memory::{PAGE_TYPE, LocalFree}
    );
}
