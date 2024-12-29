mod device;
mod cb;
mod swapchain;
mod fence;
mod rt;

const D3D_DEBUG: bool = true;

pub fn init() {
    device::create(D3D_DEBUG);

    cb::create();

    swapchain::create();

    fence::create();
}