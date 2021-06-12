fn main() {
    windows::build!(
        // Common
        Windows::Win32::System::SystemServices::{BOOL, HANDLE, PSTR, HINSTANCE, INVALID_HANDLE_VALUE, NULL, HINSTANCE},
        // Thread
        Windows::Win32::System::SystemServices::LPTHREAD_START_ROUTINE,
        Windows::Win32::System::Threading::CreateRemoteThread,
        // Security
        Windows::Win32::System::SystemServices::SECURITY_ATTRIBUTES,
        // Process
        Windows::Win32::System::Threading::{GetCurrentProcessId, OpenProcess, PROCESS_ALL_ACCESS, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        Windows::Win32::System::SystemServices::{GetModuleHandleA, GetProcAddress, FARPROC},
        Windows::Win32::System::ProcessStatus::{K32EnumProcesses, K32GetModuleFileNameExA},
        // Memory
        Windows::Win32::System::Diagnostics::Debug::WriteProcessMemory,
        Windows::Win32::System::Memory::{VirtualAllocEx, MEM_RESERVE, MEM_COMMIT},
        Windows::Win32::System::SystemServices::PAGE_EXECUTE_READWRITE,
        // I/O
        Windows::Win32::Storage::FileSystem::GetFullPathNameA,
    );
}
