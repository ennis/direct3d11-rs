#![cfg(windows)]

#[macro_use]
extern crate derive_com_wrapper;

pub use crate::device::Device;
pub use crate::device_context::DeviceContext;
pub use crate::error::Error;
pub use crate::texture2d::Texture2D;

pub mod device;
pub mod device_context;
pub mod error;
pub mod enums;
pub mod texture2d;

