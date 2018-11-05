use dxgi::device::Device as DxgiDevice;
use winapi::shared::dxgi::IDXGIDevice;
use winapi::um::d3d11::ID3D11Device;
use wio::com::ComPtr;

pub mod builder;

#[derive(Clone, PartialEq, ComWrapper)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Device {
    ptr: ComPtr<ID3D11Device>,
}

impl Device {
    pub fn create<'a>() -> builder::DeviceBuilder<'a> {
        Default::default()
    }

    pub fn as_dxgi(&self) -> DxgiDevice {
        unsafe { DxgiDevice::from_raw(self.ptr.cast::<IDXGIDevice>().unwrap().into_raw()) }
    }
}
