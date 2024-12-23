use windows::{Win32::Graphics::Direct3D12::*};

mod init;
mod rt;

pub fn create() {
    init::init();
    rt::create();
}