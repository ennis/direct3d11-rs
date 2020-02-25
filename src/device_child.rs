use com_wrapper::ComWrapper;
use winapi::um::d3d11::ID3D11DeviceChild;
use wio::com::ComPtr;

pub unsafe trait IDeviceChild {
    unsafe fn raw_device_child(&self) -> &ID3D11DeviceChild;
}

