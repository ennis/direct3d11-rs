use com_wrapper::ComWrapper;
use winapi::um::d3d11::ID3D11DeviceContext;
use wio::com::ComPtr;
use crate::resource::Resource;

#[derive(PartialEq, ComWrapper)]
#[com(send, debug)]
#[repr(transparent)]
pub struct DeviceContext {
    ptr: ComPtr<ID3D11DeviceContext>,
}

pub unsafe trait IDeviceContext {
    /// Copies one resource into another.
    // NOTE the order of parameters is reversed (src -> dest seems more idiomatic in rust).
    unsafe fn copy_resource(&self, src: &Resource, dst: &Resource)  {
        self.raw_ctx().CopyResource(dst.get_raw(), src.get_raw());
    }

    unsafe fn raw_ctx(&self) -> &ID3D11DeviceContext;
}

unsafe impl IDeviceContext for DeviceContext {
    unsafe fn raw_ctx(&self) -> &ID3D11DeviceContext {
        &self.ptr
    }
}

