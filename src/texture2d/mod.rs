use crate::device::Device;
use crate::resource::Resource;

use com_wrapper::ComWrapper;
use dxgi::surface::Surface;
use dxgi::swap_chain::BackbufferTexture;
use winapi::shared::dxgi::IDXGISurface;
use winapi::um::d3d11::ID3D11Texture2D;
use winapi::um::d3d11::ID3D11Resource;
use winapi::um::d3d11::ID3D11DeviceChild;
use wio::com::ComPtr;
use crate::device_child::IDeviceChild;

pub mod builder;
pub mod desc;

#[derive(ComWrapper, PartialEq)]
#[com(send, sync, debug)]
#[repr(transparent)]
pub struct Texture2D {
    ptr: ComPtr<ID3D11Texture2D>,
}

impl Texture2D {
    #[inline]
    pub fn create(device: &Device) -> builder::Texture2DBuilder {
        builder::Texture2DBuilder::new(device)
    }

    #[inline]
    pub fn as_dxgi(&self) -> Surface {
        unsafe { Surface::from_ptr(self.ptr.cast::<IDXGISurface>().unwrap()) }
    }

    #[inline]
    pub fn as_resource(&self) -> Resource {
        unsafe { Resource::from_ptr(self.ptr.cast::<ID3D11Resource>().unwrap()) }
    }
}

unsafe impl IDeviceChild for Texture2D {
    unsafe fn raw_device_child(&self) -> &ID3D11DeviceChild {
        &self.ptr
    }
}

unsafe impl BackbufferTexture for Texture2D {}
