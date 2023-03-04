
use super::{INJECTED_FLAG, SHOULD_BE_IGNORED_FLAG};

use crate::event::{
     NativeEventOperation,
};

use std::mem::MaybeUninit;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Mutex};
use std::thread::{self, JoinHandle};

use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::System::Threading;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use windows::Win32::UI::WindowsAndMessaging;

// For many constants.
use windows::Win32::UI::WindowsAndMessaging::*;

type HookProc = unsafe extern "system" fn(code: i32, WPARAM, LPARAM) -> LRESULT;

#[derive(Debug)]
struct Inner {
    keyboard_hook_handler: HHOOK,
    event_sender: Sender<String>,
    join_handle: JoinHandle<()>,
    thread_id: u32,
}

impl Inner {
    fn spawn_thread(
        tx: Sender<(HHOOK, u32)>,
        keyboard_hook_proc: HookProc,
    ) -> JoinHandle<()> {
        thread::spawn(move || unsafe {
            let keyboard_hook_handler = WindowsAndMessaging::SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(keyboard_hook_proc),
                HINSTANCE(0),
                0,
            )
            .expect("Failed to install keyboard hook.");


            let thread_id = Threading::GetCurrentThreadId();

            tx.send((keyboard_hook_handler, thread_id))
                .unwrap();

            WindowsAndMessaging::GetMessageW(
                &mut MaybeUninit::zeroed().assume_init(),
                HWND(0),
                0,
                0,
            );
        })
    }

    fn new(
        event_sender: Sender<String>,
        keyboard_hook_proc: HookProc,
    ) -> Self {
        let (tx, rx) = mpsc::channel();

        let join_handle = Self::spawn_thread(tx, keyboard_hook_proc);
        let (keyboard_hook_handler, thread_id) = rx.recv().unwrap();

        Inner {
            keyboard_hook_handler,
            event_sender,
            join_handle,
            thread_id,
        }
    }

    fn uninstall(self) {
        unsafe {
            WindowsAndMessaging::UnhookWindowsHookEx(self.keyboard_hook_handler)
                .expect("Failed to uninstall keyboard hook.");

            WindowsAndMessaging::PostThreadMessageW(self.thread_id, WM_QUIT, WPARAM(0), LPARAM(0))
                .unwrap();
        }
        self.join_handle.join().unwrap();
    }
}

#[derive(Debug, Default)]
pub(super) struct HookHandler {
    inner: Mutex<Option<Inner>>,
}

impl HookHandler {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn install(
        &self,
        event_sender: Sender<String>,
        keyboard_hook_proc: HookProc
    ) {
        let mut hook = self.inner.lock().unwrap();
        assert!(hook.is_none(), "Hooks are already installed.");

        *hook = Some(Inner::new(
            event_sender,
            keyboard_hook_proc,
        ));
    }

    pub(super) fn uninstall(&self) {
        self.inner
            .lock()
            .unwrap()
            .take()
            .expect("Hooks are not installed.")
            .uninstall();
    }

    fn send_event(&self, event: String) { //-> NativeEventOperation 
        self.inner
            .lock()
            .unwrap()
            .as_ref()
            .expect("Hooks are not installed.")
            .event_sender
            .send(event);
    }
}

// pub(super) fn create_keyboard_event(hook: &KBDLLHOOKSTRUCT) -> Option<ButtonEvent> {
//     if hook.dwExtraInfo & SHOULD_BE_IGNORED_FLAG != 0 {
//         return None;
//     }
//     let action = if hook.flags.0 >> 7 == 0 {
//         ButtonAction::Press
//     } else {
//         ButtonAction::Release
//     };
//     Some(ButtonEvent {
//         target: vkcode::into_button(VIRTUAL_KEY(hook.vkCode as u16))?,
//         injected: hook.dwExtraInfo & INJECTED_FLAG != 0,
//         action,
//     })
// }

// #[inline]
// fn common_hook_proc_inner(hook_handler: &HookHandler, event: Event) -> NativeEventOperation {
//     if let Event::Button(ButtonEvent { target, action, .. }) = event {
//         match action {
//             ButtonAction::Press => target.assume_pressed(),
//             ButtonAction::Release => target.assume_released(),
//         }
//     }
//     hook_handler.send_event(event)
// }

#[inline]
pub(super) fn keyboard_hook_proc_inner(
    hook_handler: &HookHandler,
    code: i32,
    l_param: LPARAM,
) -> NativeEventOperation {
    if code < 0 {
        return NativeEventOperation::Dispatch;
    }
    let hook_struct = unsafe { *(l_param.0 as *const KBDLLHOOKSTRUCT) };
    let a = hook_struct.vkCode as u8 as char;
    println!("callback code: {},  lparam: {:?},  hookstruckt: {:?}, vccode: {:02X}, c: {}", code, l_param,  hook_struct, hook_struct.vkCode, a);

    hook_handler.send_event(hook_struct.vkCode.to_string());
    // let event = match create_keyboard_event(&hook_struct) {
    //     None => return NativeEventOperation::Dispatch,
    //     Some(event) => event,
    // };

    // let native_operation = common_hook_proc_inner(hook_handler, Event::Button(event));
    // if event.action == ButtonAction::Release {
    //     return NativeEventOperation::Dispatch;
    // }
    // native_operation

    return NativeEventOperation::Dispatch
}


