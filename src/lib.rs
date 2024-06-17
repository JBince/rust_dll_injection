use std::error::Error;
use std::ptr::null_mut;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::{ HINSTANCE, DWORD, LPVOID, BOOL };
use winapi::um::processenv::SetStdHandle;
use winapi::um::winbase::STD_OUTPUT_HANDLE;
use winapi::um::winnt::{
    DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH,
    DLL_THREAD_ATTACH,
    DLL_THREAD_DETACH,
    GENERIC_READ,
    GENERIC_WRITE
};
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::handleapi::{ CloseHandle, INVALID_HANDLE_VALUE };
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::consoleapi::AllocConsole;
use winapi::um::fileapi::{ CreateFileA, OPEN_EXISTING };

unsafe extern "system" fn injection(_lp_parameter: *mut c_void) -> u32 {
    match allocate_console() {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {:?}", e);
            println!("Error code: {:?}", GetLastError());
        }
    }
    println!("Hello, from a DLL!");
    0
}


unsafe fn allocate_console() -> Result<*mut c_void, Box<dyn Error>> {
    // Allocate a console
    if AllocConsole() == 0 {
        return Err(Box::new(std::io::Error::last_os_error()));
    } 

    // Redirect STDOUT to the new console
    let stdout_handle = CreateFileA(
        b"CONOUT$\0".as_ptr() as *const i8,
        GENERIC_WRITE | GENERIC_READ,
        0,
        null_mut(),
        OPEN_EXISTING,
        0,
        null_mut()
    );
    if stdout_handle == INVALID_HANDLE_VALUE {
        let error_code = GetLastError();
        return Err(format!("Failed to open CONOUT$ for STDOUT. Error code {}", error_code).into());
    }

    if SetStdHandle(STD_OUTPUT_HANDLE, stdout_handle) == 0 {
        let error_code = GetLastError();
        return Err(format!("Failed to redirect STDOUT. Error code {}", error_code).into());
    }

    // Redirect stderr
    let stderr_handle = CreateFileA(
        b"CONOUT$\0".as_ptr() as *const i8,
        GENERIC_WRITE | GENERIC_READ,
        0,
        null_mut(),
        OPEN_EXISTING,
        0,
        null_mut()
    );
    if stderr_handle == INVALID_HANDLE_VALUE {
        let error_code = GetLastError();
        return Err(format!("Failed to open CONOUT$ for STDERR. Error code {}", error_code).into());
    }
    if SetStdHandle(STD_OUTPUT_HANDLE, stderr_handle) == 0 {
        let error_code = GetLastError();
        return Err(format!("Failed to redirect STDERR. Error code {}", error_code).into());
    }
    Ok(null_mut())
}

#[no_mangle]
pub unsafe extern "stdcall" fn DllMain(
    _hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => unsafe {
            CloseHandle(CreateThread(null_mut(), 0, Some(injection), null_mut(), 0, null_mut()));
        }
        DLL_PROCESS_DETACH => {}
        DLL_THREAD_ATTACH => {}
        DLL_THREAD_DETACH => {}
        _ => {}
    }
    1
}