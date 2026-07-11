use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicU32, Ordering};

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::{HWND, POINT};
use winapi::um::libloaderapi::GetModuleHandleW;
use winapi::um::winuser::*;

use crate::cpu::Cpu;
use crate::tray::{TrayIcon, TRAY_CALLBACK_MESSAGE};

/// Posted from the CPU sampler thread to the UI thread with a new load value.
const WM_APP_CPU_UPDATE: u32 = WM_APP + 1;

/// Command id for the "Exit" context-menu entry.
const ID_MENU_EXIT: usize = 1001;

/// Id of the registered `TaskbarCreated` broadcast message. Read from the
/// window procedure, so it lives in a static rather than instance state.
static TASKBAR_CREATED: AtomicU32 = AtomicU32::new(0);

/// State owned by the UI thread and reachable from the window procedure via
/// `GWLP_USERDATA`.
struct AppState {
    tray: TrayIcon,
}

pub struct CpuApp;

impl CpuApp {
    pub fn run() {
        unsafe {
            let hinstance = GetModuleHandleW(null_mut());
            let class_name = wide("CpuTrayWindowClass");

            let mut wc: WNDCLASSW = std::mem::zeroed();
            wc.lpfnWndProc = Some(window_proc);
            wc.hInstance = hinstance;
            wc.lpszClassName = class_name.as_ptr();
            RegisterClassW(&wc);

            // Register the broadcast the shell sends to every top-level window
            // when the taskbar (notification area) is (re)created.
            TASKBAR_CREATED.store(
                RegisterWindowMessageW(wide("TaskbarCreated").as_ptr()),
                Ordering::SeqCst,
            );

            // A hidden, top-level tool window. It is never shown, but being a
            // real top-level window is what lets it receive `TaskbarCreated`
            // (message-only windows do not receive broadcasts).
            let hwnd = CreateWindowExW(
                WS_EX_TOOLWINDOW,
                class_name.as_ptr(),
                wide("CpuTray").as_ptr(),
                WS_POPUP,
                0,
                0,
                0,
                0,
                null_mut(),
                null_mut(),
                hinstance,
                null_mut(),
            );

            let mut state = Box::new(AppState {
                tray: TrayIcon::new(hwnd),
            });
            SetWindowLongPtrW(
                hwnd,
                GWLP_USERDATA,
                Box::as_mut(&mut state) as *mut AppState as isize,
            );

            spawn_cpu_thread(hwnd);

            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, null_mut(), 0, 0) > 0 {
                TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }

            // Message loop finished (WM_QUIT). Dropping the state removes the
            // tray icon via TrayIcon::Drop.
            drop(state);
        }
    }
}

fn spawn_cpu_thread(hwnd: HWND) {
    // HWND is a raw pointer and !Send; move it as an integer instead.
    let hwnd = hwnd as usize;
    std::thread::spawn(move || {
        let hwnd = hwnd as HWND;
        loop {
            let cpu_load = Cpu::current_load();
            // PostMessage is thread-safe and queues onto the UI thread. When
            // the window is gone it returns 0, which is our cue to stop.
            let posted = unsafe {
                PostMessageW(hwnd, WM_APP_CPU_UPDATE, cpu_load as WPARAM, 0)
            };
            if posted == 0 {
                break;
            }
        }
    });
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    // Taskbar was recreated (Explorer restart) — re-add our icon.
    let taskbar_created = TASKBAR_CREATED.load(Ordering::SeqCst);
    if taskbar_created != 0 && msg == taskbar_created {
        if let Some(state) = state_from(hwnd) {
            state.tray.restore();
        }
        return 0;
    }

    match msg {
        WM_APP_CPU_UPDATE => {
            if let Some(state) = state_from(hwnd) {
                state.tray.update(wparam as u8);
            }
            0
        }
        TRAY_CALLBACK_MESSAGE => {
            // With NOTIFYICON_VERSION_4 the triggering event is in LOWORD(lParam).
            let event = (lparam & 0xffff) as u32;
            if event == WM_CONTEXTMENU || event == WM_RBUTTONUP {
                show_context_menu(hwnd);
            }
            0
        }
        WM_COMMAND => {
            if (wparam & 0xffff) == ID_MENU_EXIT {
                DestroyWindow(hwnd);
            }
            0
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

/// Borrow the `AppState` stashed in `GWLP_USERDATA`. Returns `None` for the
/// early lifecycle messages that arrive before it is set.
unsafe fn state_from<'a>(hwnd: HWND) -> Option<&'a mut AppState> {
    let ptr = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut AppState;
    ptr.as_mut()
}

unsafe fn show_context_menu(hwnd: HWND) {
    let menu = CreatePopupMenu();
    AppendMenuW(menu, MF_STRING, ID_MENU_EXIT, wide("Exit").as_ptr());

    let mut cursor: POINT = std::mem::zeroed();
    GetCursorPos(&mut cursor);

    // Required so the menu closes when the user clicks elsewhere.
    SetForegroundWindow(hwnd);
    TrackPopupMenu(
        menu,
        TPM_RIGHTBUTTON | TPM_LEFTALIGN | TPM_BOTTOMALIGN,
        cursor.x,
        cursor.y,
        0,
        hwnd,
        null_mut(),
    );
    // Companion to SetForegroundWindow, per the documented workaround.
    PostMessageW(hwnd, WM_NULL, 0, 0);

    DestroyMenu(menu);
}

fn wide(value: &str) -> Vec<u16> {
    OsStr::new(value)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}
