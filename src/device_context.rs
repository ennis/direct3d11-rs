use winapi::um::d3d11::ID3D11DeviceContext;
use wio::com::ComPtr;

pub struct DeviceContext {
    ptr: ComPtr<ID3D11DeviceContext>,
}

com_wrapper!(DeviceContext: ID3D11DeviceContext);
