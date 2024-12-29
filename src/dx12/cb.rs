use windows::Win32::Graphics::Direct3D12::{ID3D12CommandAllocator, ID3D12CommandQueue, ID3D12GraphicsCommandList, D3D12_COMMAND_LIST_TYPE_DIRECT, D3D12_COMMAND_QUEUE_DESC, D3D12_COMMAND_QUEUE_FLAG_NONE};

use super::device;

static mut GMAIN_COMMAND_QUEUE: Option<ID3D12CommandQueue> = None;
static mut GMAIN_COMMAND_ALLOCATOR: Option<ID3D12CommandAllocator> = None;
static mut GMAIN_COMMAND_LIST: Option<ID3D12GraphicsCommandList> = None;

pub(crate) fn create() {
    unsafe {
        let device = device::GD3D12_DEVICE.as_ref().unwrap();

        // create command queue
        let queue_desc = D3D12_COMMAND_QUEUE_DESC {
            Type: D3D12_COMMAND_LIST_TYPE_DIRECT,
            Flags: D3D12_COMMAND_QUEUE_FLAG_NONE,
            ..D3D12_COMMAND_QUEUE_DESC::default()
        };

        if let Ok(x) = device.CreateCommandQueue::<ID3D12CommandQueue>(&queue_desc) {
            GMAIN_COMMAND_QUEUE = Some(x);
        }

        // create command allocator
        if let Ok(x)= device.CreateCommandAllocator::<ID3D12CommandAllocator>(D3D12_COMMAND_LIST_TYPE_DIRECT) {
            GMAIN_COMMAND_ALLOCATOR = Some(x);
        }

        // create command list
        if let Ok(x) = device.CreateCommandList(0, D3D12_COMMAND_LIST_TYPE_DIRECT, GMAIN_COMMAND_ALLOCATOR.as_ref(), None) {
            GMAIN_COMMAND_LIST = Some(x);
            // set the command list as closed
            let _ = GMAIN_COMMAND_LIST.as_ref().unwrap().Close();
        }
    }
}