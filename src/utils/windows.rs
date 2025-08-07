use winapi::um::winuser::{EnumWindows, GetWindowThreadProcessId, IsWindowVisible, GetWindowTextLengthW};
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::{BOOL, LPARAM, DWORD, TRUE};

struct EnumWindowsData {
    process_id: DWORD,
    windows: Vec<HWND>,
}

// Callback function for EnumWindows
unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let data = unsafe { &mut *(lparam as *mut EnumWindowsData) };
    let mut window_process_id: DWORD = 0;
    
    // Get the process ID that owns this window
    unsafe { GetWindowThreadProcessId(hwnd, &mut window_process_id) };
    
    // If it matches our target process ID, add it to the list
    if window_process_id == data.process_id {
        data.windows.push(hwnd);
    }
    
    TRUE // Continue enumeration
}

pub fn get_windows_from_process_id(process_id: DWORD) -> Vec<HWND> {
    let mut data = EnumWindowsData {
        process_id,
        windows: Vec::new(),
    };
    
    unsafe {
        EnumWindows(
            Some(enum_windows_proc),
            &mut data as *mut _ as LPARAM,
        );
    }
    
    data.windows
}

pub fn get_main_window_from_process_id(process_id: DWORD) -> Option<HWND> {
    let windows = get_windows_from_process_id(process_id);
    
    for &hwnd in &windows {
        unsafe {
            // Check if window is visible and has a title
            if IsWindowVisible(hwnd) != 0 {
                let title_length = GetWindowTextLengthW(hwnd);
                if title_length > 0 {
                    return Some(hwnd); // Return first visible window with title
                }
            }
        }
    }
    
    // If no main window found, return first window (if any)
    windows.first().copied()
}

// use winapi::um::winuser::GetWindowTextW;
// pub fn get_window_title(hwnd: HWND) -> Option<String> {
//     unsafe {
//         let length = GetWindowTextLengthW(hwnd);
//         if length == 0 {
//             return None;
//         }
        
//         let mut buffer: Vec<u16> = vec![0; (length + 1) as usize];
//         let copied = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);
        
//         if copied > 0 {
//             buffer.truncate(copied as usize);
//             Some(String::from_utf16_lossy(&buffer))
//         } else {
//             None
//         }
//     }
// }

// Add this to your Cargo.toml:
// [dependencies]
// winapi = { version = "0.3", features = ["winuser", "processthreadsapi"] }