use windows::Win32::Graphics::Direct3D12::*;

mod device;
mod cb;
mod rt;

const D3D_DEBUG: bool = true;

pub fn init() {
    device::create(D3D_DEBUG);

    cb::create();

    swapchain::create();

    fence::create();
}