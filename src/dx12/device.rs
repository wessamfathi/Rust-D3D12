use windows::Win32::Graphics::{Direct3D::{D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_12_0, D3D_FEATURE_LEVEL_12_1, D3D_FEATURE_LEVEL_12_2}, Direct3D12::{D3D12CreateDevice, D3D12GetDebugInterface, ID3D12Debug, ID3D12Device, ID3D12InfoQueue}, Dxgi::{CreateDXGIFactory2, IDXGIFactory4, DXGI_ADAPTER_FLAG_SOFTWARE, DXGI_CREATE_FACTORY_DEBUG, DXGI_CREATE_FACTORY_FLAGS}};
use windows_core::Interface;

static mut GDXGI_FACTORY: Option<IDXGIFactory4> = None;
static mut GD3D12_DEVICE: Option<ID3D12Device> = None;
static mut GDEBUG_INFO_QUEUE: Option<ID3D12InfoQueue> = None;

fn create(enable_debug: bool) {
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