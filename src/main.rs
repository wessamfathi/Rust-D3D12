mod dx12;

use std::mem;

use windows::{core::Result, Win32::{Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM}, Graphics::Gdi::{GetSysColorBrush, COLOR_WINDOWFRAME}, System::LibraryLoader::GetModuleHandleW, UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, LoadCursorW, PeekMessageW, PostQuitMessage, RegisterClassExW, ShowWindow, TranslateMessage, CS_HREDRAW, CS_VREDRAW, HICON, IDC_ARROW, MSG, PM_REMOVE, SW_SHOW, WINDOW_EX_STYLE, WM_DESTROY, WM_QUIT, WNDCLASSEXW, WS_MINIMIZEBOX, WS_OVERLAPPED, WS_SYSMENU}}};
use windows_core::PCWSTR;
use windows_sys::w;

static mut RENDER_WIDTH: i32 = 1920;
static mut RENDER_HEIGHT: i32 = 1080;

fn main() -> Result<()> {

    unsafe {
        let app_window = create_window();

        if !dx12::init(app_window, RENDER_WIDTH, RENDER_HEIGHT) {
            return Ok(());
        }

        let _ = ShowWindow(app_window, SW_SHOW);
        let mut msg = MSG::default();

        while msg.message != WM_QUIT {
            if PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            } else {
                dx12::update();

                // present backbuffer and wait for GPU fence (not very efficient)
                dx12::present();
                dx12::wait_for_gpu();
            }
        }
    }

    dx12::destroy();

    Ok(())
}

unsafe fn create_window() -> HWND {
    let app_instance: HINSTANCE = GetModuleHandleW(None).unwrap().into();
    let app_class_name = PCWSTR::from_raw(w!("Rust D3D12"));

    // set up WNDCLASSEXXW struct
    let app_class = WNDCLASSEXW
    {
        cbSize : mem::size_of::<WNDCLASSEXW>().try_into().unwrap(),
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(wnd_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: app_instance,
        hIcon: HICON::default(),
        hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
        hbrBackground: GetSysColorBrush(COLOR_WINDOWFRAME),
        lpszMenuName: PCWSTR::null(),
        lpszClassName: app_class_name,
        hIconSm: HICON::default(),
    };

    RegisterClassExW(&app_class);

    let app_window = CreateWindowExW(WINDOW_EX_STYLE::default(), app_class_name, app_class_name, WS_OVERLAPPED | WS_MINIMIZEBOX | WS_SYSMENU, 0, 0, RENDER_WIDTH, RENDER_HEIGHT, None, None, app_instance, None).unwrap();

    app_window

}

 unsafe extern  "system" fn wnd_proc(h_wnd: HWND, message: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match message {
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT::default()
        }
        _ => return DefWindowProcW(h_wnd, message, w_param, l_param),
    }
 }