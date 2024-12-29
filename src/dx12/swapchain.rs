use std::{mem, os::raw::c_void};

use windows::Win32::{Foundation::HWND, Graphics::{Direct3D12::{ID3D12DescriptorHeap, ID3D12Resource, D3D12_CPU_DESCRIPTOR_HANDLE, D3D12_DESCRIPTOR_HEAP_DESC, D3D12_DESCRIPTOR_HEAP_FLAG_NONE, D3D12_DESCRIPTOR_HEAP_TYPE_RTV}, Dxgi::{Common::{DXGI_FORMAT, DXGI_FORMAT_R8G8B8A8_UNORM, DXGI_SAMPLE_DESC}, IDXGIFactory5, IDXGISwapChain3, DXGI_FEATURE, DXGI_FEATURE_PRESENT_ALLOW_TEARING, DXGI_MWA_NO_ALT_ENTER, DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING, DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_USAGE_RENDER_TARGET_OUTPUT}}};
use windows_core::Interface;

use super::{cb::GMAIN_COMMAND_QUEUE, device::{self, GDXGI_FACTORY}};

const GMAXFRAME: usize = 2;
static GBACK_BUFFER_FORMAT: DXGI_FORMAT = DXGI_FORMAT_R8G8B8A8_UNORM;
pub(crate) static mut GSWAPCHAIN: Option<IDXGISwapChain3> = None;
static mut GSWAPCHAIN_HEAP: Option<ID3D12DescriptorHeap> = None;
static mut GRTV_DESCRIPTOR_SIZE: u32 = 0;
static mut GSWAPCHAIN_RESOURCE: [Option<ID3D12Resource>; GMAXFRAME] = [None, None];
static mut GSUPPORT_SCREEN_TEARING: bool = false;
pub(crate) static mut GCURRENT_FRAME_INDEX: u64 = 0;

pub(crate) fn create(h_wnd: HWND, render_width: u32, render_height: u32) {
    unsafe {
        let d3ddevice = device::GD3D12_DEVICE.as_ref().unwrap();
        // use windows_core::Interface cast() == C++ COM QueryInterface
        let factory: IDXGIFactory5 = device::GDXGI_FACTORY.as_ref().unwrap().cast().unwrap();

        // check tearing support for rendering > monitor refresh rate
        let mut support_tearing = false;
        // ugly hack to convert c void*
        let _ = factory.CheckFeatureSupport(
            DXGI_FEATURE_PRESENT_ALLOW_TEARING,
            &mut support_tearing as *mut _ as *mut c_void,
            mem::size_of::<DXGI_FEATURE>().try_into().unwrap());
            GSUPPORT_SCREEN_TEARING = support_tearing;

        let mut swapchain_flags: u32 = 0;
        if GSUPPORT_SCREEN_TEARING {
            swapchain_flags |= DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING.0 as u32;
        }

        // create swapchain
        let swapchain_desc = DXGI_SWAP_CHAIN_DESC1 {
            BufferCount: GMAXFRAME as u32,
            Width: render_width,
            Height: render_height,
            Format: GBACK_BUFFER_FORMAT,
            BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
            SwapEffect: DXGI_SWAP_EFFECT_FLIP_DISCARD,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0
            },
            Flags: swapchain_flags,
            ..DXGI_SWAP_CHAIN_DESC1::default()
        };

        if let Ok(x) = GDXGI_FACTORY.as_ref().unwrap().CreateSwapChainForHwnd(GMAIN_COMMAND_QUEUE.as_ref().unwrap(), h_wnd, &swapchain_desc, None, None) {
            GSWAPCHAIN = Some(x.cast().unwrap());
        }

        // disable alt-enter behavior by default
        let _ = GDXGI_FACTORY.as_ref().unwrap().MakeWindowAssociation(h_wnd, DXGI_MWA_NO_ALT_ENTER);

        // create swapchain descriptor heap
        let swapchain_descriptor_heap_desc = D3D12_DESCRIPTOR_HEAP_DESC {
            NumDescriptors: GMAXFRAME as u32,
            Type: D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
            Flags: D3D12_DESCRIPTOR_HEAP_FLAG_NONE,
            ..D3D12_DESCRIPTOR_HEAP_DESC::default()
        };

        if let Ok(x) = d3ddevice.CreateDescriptorHeap::<ID3D12DescriptorHeap>(&swapchain_descriptor_heap_desc) {
            GSWAPCHAIN_HEAP = Some(x);
        }
        GRTV_DESCRIPTOR_SIZE = d3ddevice.GetDescriptorHandleIncrementSize(D3D12_DESCRIPTOR_HEAP_TYPE_RTV);

        let mut rtv_handle: D3D12_CPU_DESCRIPTOR_HANDLE = GSWAPCHAIN_HEAP.as_ref().unwrap().GetCPUDescriptorHandleForHeapStart();
        for idx in 0..GMAXFRAME {
            if let Ok(x) = GSWAPCHAIN.as_ref().unwrap().GetBuffer::<ID3D12Resource>(idx as u32) {
                GSWAPCHAIN_RESOURCE[idx] = Some(x);
                d3ddevice.CreateRenderTargetView(GSWAPCHAIN_RESOURCE[idx].as_ref().unwrap(), None, rtv_handle);
                rtv_handle.ptr += GRTV_DESCRIPTOR_SIZE as usize;
            }
        }

        advance_frame_index(); 
    }
}

pub(crate) fn advance_frame_index() {
    unsafe {
        GCURRENT_FRAME_INDEX = GSWAPCHAIN.as_ref().unwrap().GetCurrentBackBufferIndex() as u64;
    }
}