use crate::device::Device;

use com_wrapper::ComWrapper;
use dxgi::surface::Surface;
use dxgi::swap_chain::BackbufferTexture;
use winapi::shared::dxgi::IDXGISurface;
use winapi::um::d3d11::ID3D11Texture2D;
use wio::com::ComPtr;

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
}

unsafe impl BackbufferTexture for Texture2D {}
