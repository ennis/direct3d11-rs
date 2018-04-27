#![cfg(windows)]

extern crate dxgi;
extern crate winapi;
extern crate wio;

pub use device::Device;
pub use device_context::DeviceContext;
pub use error::Error;
pub use texture2d::Texture2D;

#[macro_use]
pub mod helpers;

pub mod device;
pub mod device_context;
pub mod error;
pub mod flags;
pub mod texture2d;

