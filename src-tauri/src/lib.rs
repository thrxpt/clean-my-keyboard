use std::ptr;
use winapi::shared::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::shared::windef::HHOOK;
use winapi::um::winuser::{
    CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL,
};

static mut KEYBOARD_HOOK: HHOOK = ptr::null_mut();

unsafe extern "system" fn keyboard_hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code >= 0 {
        return 1; 
    }
    CallNextHookEx(KEYBOARD_HOOK, n_code, w_param, l_param)
}

#[tauri::command]
fn lock_keyboard() -> Result<String, String> {
    unsafe {
        if KEYBOARD_HOOK.is_null() {
            KEYBOARD_HOOK = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(keyboard_hook_proc),
                ptr::null_mut(),
                0,
            );

            if KEYBOARD_HOOK.is_null() {
                return Err("Failed to lock keyboard.".into());
            }
        }
    }
    Ok("Keyboard locked".into())
}

#[tauri::command]
fn unlock_keyboard() -> Result<String, String> {
    unsafe {
        if !KEYBOARD_HOOK.is_null() {
            UnhookWindowsHookEx(KEYBOARD_HOOK);
            KEYBOARD_HOOK = ptr::null_mut();
        }
    }
    Ok("Keyboard unlocked".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![lock_keyboard, unlock_keyboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}