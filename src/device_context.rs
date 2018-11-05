use winapi::um::d3d11::ID3D11DeviceContext;
use wio::com::ComPtr;

#[derive(PartialEq, ComWrapper)]
#[com(send, debug)]
#[repr(transparent)]
pub struct DeviceContext {
    ptr: ComPtr<ID3D11DeviceContext>,
}
