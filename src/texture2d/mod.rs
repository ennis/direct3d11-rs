use device::Device;

use dxgi::surface::Surface as DxgiSurface;
use dxgi::swap_chain::BackbufferTexture;
use winapi::ctypes::c_void;
use winapi::shared::dxgi::IDXGISurface;
use winapi::shared::guiddef::GUID;
use winapi::um::d3d11::ID3D11Texture2D;
use winapi::Interface;
use wio::com::ComPtr;

pub mod builder;
pub mod desc;

pub struct Texture2D {
    ptr: ComPtr<ID3D11Texture2D>,
}

impl Texture2D {
    #[inline]
    pub fn create(device: &Device) -> builder::Texture2DBuilder {
        builder::Texture2DBuilder::new(device)
    }

    #[inline]
    pub fn as_dxgi(&self) -> DxgiSurface {
        unsafe { DxgiSurface::from_raw(self.ptr.cast::<IDXGISurface>().unwrap().into_raw()) }
    }

    #[inline]
    pub unsafe fn from_raw(raw: *mut ID3D11Texture2D) -> Self {
        Self {
            ptr: ComPtr::from_raw(raw),
        }
    }
}

unsafe impl BackbufferTexture for Texture2D {
    fn uuidof() -> GUID {
        ID3D11Texture2D::uuidof()
    }

    unsafe fn from_raw(raw: *mut c_void) -> Self {
        Texture2D::from_raw(raw as _)
    }
}

com_wrapper!(Texture2D: ID3D11Texture2D);
