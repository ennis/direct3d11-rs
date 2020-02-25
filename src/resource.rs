use com_wrapper::ComWrapper;
use winapi::shared::dxgi::{IDXGIDeviceSubObject, IDXGIResource};
use winapi::shared::winerror::SUCCEEDED;
use winapi::um::d3d11::ID3D11Resource;
use wio::com::ComPtr;

#[repr(transparent)]
#[derive(ComWrapper)]
#[com(send, sync, debug)]
pub struct Resource {
    ptr: ComPtr<ID3D11Resource>,
}

pub unsafe trait IResource {
    unsafe fn raw_res(&self) -> &ID3D11Resource;
}

unsafe impl IResource for Resource {
    unsafe fn raw_res(&self) -> &ID3D11Resource {
        &self.ptr
    }
}
