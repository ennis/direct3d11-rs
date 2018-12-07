#![cfg(windows)]

#[macro_use]
extern crate derive_com_wrapper;

#[macro_use]
extern crate auto_enum;

pub use device::Device;
pub use device_context::DeviceContext;
pub use error::Error;
pub use texture2d::Texture2D;

pub mod device;
pub mod device_context;
pub mod error;
pub mod enums;
pub mod texture2d;

