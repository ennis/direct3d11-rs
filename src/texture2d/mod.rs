use device::Device;

use dxgi::surface::Surface as DxgiSurface;
use winapi::shared::dxgi::IDXGISurface;
use winapi::um::d3d11::ID3D11Texture2D;
use wio::com::ComPtr;

pub mod builder;
pub mod desc;

pub struct Texture2D {
    ptr: ComPtr<ID3D11Texture2D>,
}

impl Texture2D {
    pub fn create(device: &Device) -> builder::Texture2DBuilder {
        builder::Texture2DBuilder::new(device)
    }

    pub fn as_dxgi(&self) -> DxgiSurface {
        unsafe { DxgiSurface::from_raw(self.ptr.cast::<IDXGISurface>().unwrap().into_raw()) }
    }
}

com_wrapper!(Texture2D: ID3D11Texture2D);
