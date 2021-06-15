fn main() {
    windows::build!(
        // Common
        Windows::Win32::System::WindowsProgramming::{INFINITE},
        Windows::Win32::Foundation::CloseHandle,
        Windows::Win32::Foundation::{ BOOL, HANDLE, PSTR, HINSTANCE, INVALID_HANDLE_VALUE, HINSTANCE},
        // Thread
        Windows::Win32::System::SystemServices::LPTHREAD_START_ROUTINE,
        Windows::Win32::System::Threading::CreateRemoteThread,
        // Security
        Windows::Win32::Security::SECURITY_ATTRIBUTES,
        // Process
        Windows::Win32::System::Threading::{WaitForSingleObject, GetCurrentProcessId, OpenProcess, PROCESS_ACCESS_RIGHTS},
        Windows::Win32::System::LibraryLoader::{GetModuleHandleA, GetProcAddress},
        Windows::Win32::System::ProcessStatus::{K32EnumProcesses, K32GetModuleFileNameExA},
        // Memory
        Windows::Win32::System::Diagnostics::Debug::WriteProcessMemory,
        Windows::Win32::System::Memory::{VirtualAllocEx, VIRTUAL_ALLOCATION_TYPE},
        Windows::Win32::System::Memory::PAGE_TYPE,
        // I/O
        Windows::Win32::Storage::FileSystem::GetFullPathNameA,
    );
}
