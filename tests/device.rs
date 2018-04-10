extern crate direct3d11;

use direct3d11::device::Device;
use direct3d11::flags::{CreateDeviceFlags, DriverType, FeatureLevel};

#[test]
fn create_default() {
    let (feature_level, _dev, _ctx) = Device::create().build().unwrap();
    assert!(feature_level <= FeatureLevel::LEVEL_11_0);
    assert!(feature_level >= FeatureLevel::LEVEL_9_1);
}

#[test]
fn create_debug() {
    let (feature_level, _dev, _ctx) = Device::create()
        .with_driver_type(DriverType::Hardware)
        .with_feature_levels(&[FeatureLevel::LEVEL_11_0])
        .with_flags(CreateDeviceFlags::DEBUG)
        .build()
        .unwrap();
    assert!(feature_level == FeatureLevel::LEVEL_11_0);
}

#[test]
fn create_debuggable() {
    let (feature_level, _dev, _ctx) = Device::create()
        .with_driver_type(DriverType::Warp)
        .with_feature_levels(&[FeatureLevel::LEVEL_11_0])
        .with_flags(CreateDeviceFlags::DEBUGGABLE)
        .build()
        .unwrap();
    assert!(feature_level == FeatureLevel::LEVEL_11_0);
}

#[test]
fn create_hardware_11_0() {
    let (feature_level, _dev, _ctx) = Device::create()
        .with_driver_type(DriverType::Hardware)
        .with_feature_levels(&[FeatureLevel::LEVEL_11_0])
        .build()
        .unwrap();
    assert!(feature_level == FeatureLevel::LEVEL_11_0);
}

#[test]
fn create_hardware_11_1() {
    let (feature_level, _dev, _ctx) = Device::create()
        .with_driver_type(DriverType::Hardware)
        .with_feature_levels(&[FeatureLevel::LEVEL_11_1])
        .build()
        .unwrap();
    assert!(feature_level == FeatureLevel::LEVEL_11_1);
}

#[test]
fn create_reference() {
    let (feature_level, _dev, _ctx) = Device::create()
        .with_driver_type(DriverType::Reference)
        .with_feature_levels(&[FeatureLevel::LEVEL_11_0])
        .build()
        .unwrap();
    assert!(feature_level == FeatureLevel::LEVEL_11_0);
}

#[test]
fn create_warp() {
    let (feature_level, _dev, _ctx) = Device::create()
        .with_driver_type(DriverType::Warp)
        .with_feature_levels(&[FeatureLevel::LEVEL_11_0])
        .build()
        .unwrap();
    assert!(feature_level == FeatureLevel::LEVEL_11_0);
}

#[test]
fn create_bgra() {
    let (feature_level, _dev, _ctx) = Device::create()
        .with_driver_type(DriverType::Hardware)
        .with_feature_levels(&[FeatureLevel::LEVEL_11_0])
        .with_flags(CreateDeviceFlags::BGRA_SUPPORT)
        .build()
        .unwrap();
    assert!(feature_level == FeatureLevel::LEVEL_11_0);
}
