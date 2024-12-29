use std::os::raw::c_void;

use windows::Win32::Graphics::{Direct3D::{D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_12_0, D3D_FEATURE_LEVEL_12_1, D3D_FEATURE_LEVEL_12_2}, Direct3D12::{D3D12CreateDevice, D3D12GetDebugInterface, ID3D12Debug, ID3D12Device, ID3D12InfoQueue, D3D12_MESSAGE}, Dxgi::{CreateDXGIFactory2, IDXGIFactory4, DXGI_ADAPTER_FLAG_SOFTWARE, DXGI_CREATE_FACTORY_DEBUG, DXGI_CREATE_FACTORY_FLAGS, DXGI_PRESENT, DXGI_PRESENT_ALLOW_TEARING}};
use windows_core::{Interface, PCSTR};

pub(crate) static mut GDXGI_FACTORY: Option<IDXGIFactory4> = None;
pub(crate) static mut GD3D12_DEVICE: Option<ID3D12Device> = None;
static mut GDEBUG_INFO_QUEUE: Option<ID3D12InfoQueue> = None;

pub(crate) fn create(enable_debug: bool) {
    unsafe {
        let mut dxgi_factory_flag = DXGI_CREATE_FACTORY_FLAGS::default();

        // enable debug layer
        let mut debug_controller:Option<ID3D12Debug> = None;
        if enable_debug {
            if let Ok(()) = D3D12GetDebugInterface(&mut debug_controller) {
                debug_controller.as_ref().unwrap().EnableDebugLayer();
                dxgi_factory_flag = dxgi_factory_flag | DXGI_CREATE_FACTORY_DEBUG;
            }
        }

        // create DXGI factory
        if let Ok(x) = CreateDXGIFactory2::<IDXGIFactory4>(dxgi_factory_flag) {
            GDXGI_FACTORY = Some(x);
        }

        // create D3D device 
        let mut d3d12_device: Option<ID3D12Device> = None;
        if GDXGI_FACTORY.is_some() {
            // try with highest feature level first
            let feature_levels: [D3D_FEATURE_LEVEL; 3] = [D3D_FEATURE_LEVEL_12_2, D3D_FEATURE_LEVEL_12_1, D3D_FEATURE_LEVEL_12_0];
            let feature_levels_names: [&str; 3] = ["12_2", "12_1", "12_0"];
            let mut feature_index = 0;
            let mut adapter_index;

            'FeatureLevelLoop: loop {
                adapter_index = 0;
                loop {
                    if let Ok(x) = GDXGI_FACTORY.as_ref().unwrap().EnumAdapters1(adapter_index) {
                        let adapter_desc = x.GetDesc1().unwrap();
                        if (adapter_desc.Flags & DXGI_ADAPTER_FLAG_SOFTWARE.0 as u32) > 0 {
                            // skip software adapter
                            adapter_index += 1;
                            continue;
                        }

                        // use the first adapter succeeding with the highest feature level
                        if let Ok(_) = D3D12CreateDevice(&x, feature_levels[feature_index], &mut d3d12_device) {
                            println!("Slected adapter for D3D12CreateDevice: {}", String::from_utf16(&adapter_desc.Description).unwrap());
                            println!("Initialized with feature level: {}", feature_levels_names[feature_index]);
                            GD3D12_DEVICE = d3d12_device;
                            break 'FeatureLevelLoop;
                        }
                    } else {
                        // no more devices
                        break;
                    }

                    adapter_index += 1;
                }

                feature_index += 1;
            }
        }

        // cache ID3D12InfoQueue for later use
        if GD3D12_DEVICE.is_some() && debug_controller.is_some() {
            GDEBUG_INFO_QUEUE = Some(GD3D12_DEVICE.as_ref().unwrap().cast().unwrap());
        }
    }
}

// update function
pub(crate) fn update()
{
    unsafe 
    {
        // print out all error messages stored in ID3D12QueueInfo
        if GDEBUG_INFO_QUEUE.is_some()
        {
            let debug_info_queue = GDEBUG_INFO_QUEUE.as_ref().unwrap();
            let error_message_count = debug_info_queue.GetNumStoredMessages();

            // print error message if there is any
            if error_message_count > 0
            {
                for idx in 0..error_message_count
                {
                    // first GetMessage() call to get the error byte length
                    let mut message_byte_length = 0;
                    let _ = debug_info_queue.GetMessage(idx, None, &mut message_byte_length);                    

                    // second GetMessage() to get the error and print it, be sure to allocate D3D12_MESSAGE with enough size!
                    let error_message : Option<*mut D3D12_MESSAGE> = Some(libc::malloc(message_byte_length) as *mut D3D12_MESSAGE);

                    if let Ok(()) = debug_info_queue.GetMessage(idx, error_message, &mut message_byte_length)
                    {
                        let unwrapped_message : *mut D3D12_MESSAGE = error_message.unwrap();
                        println!("{}", PCSTR::from_raw((*unwrapped_message).pDescription).display());
                    }

                    libc::free(error_message.unwrap() as *mut _ as *mut c_void);
                }

                // clear all printed messages
                debug_info_queue.ClearStoredMessages();
            }
        }
    }
}