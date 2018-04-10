extern crate direct3d11;

use direct3d11::device::Device;
use direct3d11::flags::{CreateDeviceFlags, Format};
use direct3d11::texture2d::Texture2D;

static SIMPLE_2X2_IMAGE: &'static [u8] = &[
    0xFF, 0x00, 0x00, 0xFF, 0x00, 0xFF, 0x00, 0xFF,
    0x00, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
];

#[test]
fn create_basic_texture() {
    let (_, device, _) = Device::create()
        .with_flags(CreateDeviceFlags::BGRA_SUPPORT)
        .build()
        .unwrap();

    let _texture = Texture2D::create(&device)
        .with_size(2, 2)
        .with_format(Format::R8G8B8A8Unorm)
        .with_initial_data(SIMPLE_2X2_IMAGE, 8)
        .build()
        .unwrap();
}
