use windows::Win32::{Foundation::{CloseHandle, HANDLE}, Graphics::Direct3D12::{ID3D12Fence, D3D12_FENCE_FLAG_NONE}, System::Threading::{CreateEventW, WaitForSingleObject, INFINITE}};

use super::{cb, device, swapchain::{self}};

static mut GMAIN_FENCE: Option<ID3D12Fence> = None;
static mut GMAIN_FENCE_VALUE: u64 = 0;
static mut GMAIN_FENCE_EVENT: Option<HANDLE> = None;

pub(crate) fn create() {
    unsafe {
        if let Ok(x) = device::GD3D12_DEVICE.as_ref().unwrap().CreateFence::<ID3D12Fence>(0, D3D12_FENCE_FLAG_NONE) {
            // create event after CreateFence succeeds
            GMAIN_FENCE = Some(x);
            GMAIN_FENCE_VALUE = 1;

            if let Ok(x) = CreateEventW(None, false, false, None) {
                GMAIN_FENCE_EVENT = Some(x);
            }

            wait_for_gpu();
        }
    }
}

pub(crate) fn wait_for_gpu() {
    unsafe {
        let main_fence = GMAIN_FENCE.as_ref().unwrap();
        let prev_fence_value = GMAIN_FENCE_VALUE;
        let _ = cb::GMAIN_COMMAND_QUEUE.as_ref().unwrap().Signal(main_fence, prev_fence_value);
        GMAIN_FENCE_VALUE += 1;

        if main_fence.GetCompletedValue() < prev_fence_value {
            let fence_event = GMAIN_FENCE_EVENT.as_ref();
            let _ = main_fence.SetEventOnCompletion(prev_fence_value, fence_event);
            WaitForSingleObject(fence_event, INFINITE);
        }

        // advance frame index
        swapchain::advance_frame_index();
    }
}

pub(crate) fn shutdown() {
    unsafe  {
        wait_for_gpu();
        let _ = CloseHandle(GMAIN_FENCE_EVENT.as_ref());
    }
}