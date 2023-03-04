mod hook;

use hook::HookHandler;

use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::HHOOK;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};

use crate::event::{ NativeEventOperation};

use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::Lazy;
use windows::Win32::UI::{HiDpi, WindowsAndMessaging};

const SHOULD_BE_IGNORED_FLAG: usize = 0x1;
const INJECTED_FLAG: usize = 0x2;

static HOOK_HANDLER: Lazy<HookHandler> = Lazy::new(HookHandler::new);

extern "system" fn keyboard_hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match hook::keyboard_hook_proc_inner(&HOOK_HANDLER, n_code, l_param) {
        NativeEventOperation::Block => LRESULT(1),
        NativeEventOperation::Dispatch => unsafe {
            WindowsAndMessaging::CallNextHookEx(HHOOK(0), n_code, w_param, l_param)
        },
    }
}


pub fn install_hook() -> Receiver<String> {
    unsafe {
        // If this is not executed, the GetCursorPos function returns an invalid cursor position.
        HiDpi::SetProcessDpiAwarenessContext(HiDpi::DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE);
    }


    let (tx, rx) = mpsc::channel();
    HOOK_HANDLER.install(tx, keyboard_hook_proc);

    rx
}

pub fn uninstall_hook() {
    HOOK_HANDLER.uninstall();
}