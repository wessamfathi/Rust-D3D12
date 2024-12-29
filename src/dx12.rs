use windows::Win32::Foundation::HWND;

mod device;
mod cb;
mod swapchain;
mod fence;
mod rt;

const D3D_DEBUG: bool = true;

pub fn init(h_wnd: HWND, render_width: i32, render_height: i32) -> bool {
    device::create(D3D_DEBUG);

    cb::create();

    swapchain::create(h_wnd, render_width as u32, render_height as u32);

    fence::create();

    true
}

pub fn destroy() {
    fence::shutdown();
}

pub fn update() {
    device::update();
}

pub fn present() {
    swapchain::present();
}

pub fn wait_for_gpu() {
    fence::wait_for_gpu();
}