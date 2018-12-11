use crate::device::Device;
use crate::device_context::DeviceContext;
use crate::enums::{CreateDeviceFlags, DriverType, FeatureLevel};
use crate::error::Error;

use std::ptr;

use dxgi::adapter::Adapter;
use winapi::shared::dxgi::IDXGIAdapter;
use winapi::shared::minwindef::HMODULE;
use winapi::um::d3d11::{D3D11CreateDevice, D3D11_SDK_VERSION};
use com_wrapper::ComWrapper;

pub struct DeviceBuilder<'a> {
    adapter: Option<&'a Adapter>,
    driver_type: Option<DriverType>,
    flags: CreateDeviceFlags,
    feature_levels: &'a [FeatureLevel],
    software_module: HMODULE,
}

impl<'a> Default for DeviceBuilder<'a> {
    fn default() -> Self {
        DeviceBuilder {
            adapter: None,
            driver_type: None,
            flags: CreateDeviceFlags::NONE,
            feature_levels: &[],
            software_module: ptr::null_mut(),
        }
    }
}

impl<'a> DeviceBuilder<'a> {
    /// Explicitly set the driver_type. This is not necessary except to trigger
    /// the other sanity checks in the builder methods if you *really* want a
    /// specific DriverType to be passed.
    ///
    /// ### panics
    /// Panics if adapter or software_module has already been specified and the
    /// passed driver_type is not the required value for those configurations.
    pub fn with_driver_type(mut self, driver_type: DriverType) -> Self {
        assert!(
            (driver_type == DriverType::Unknown || self.adapter.is_none())
                && (driver_type == DriverType::Software || self.software_module.is_null()),
            "If an adapter is specified, driver_type must be `Unknown`"
        );
        self.driver_type = Some(driver_type);
        self
    }

    /// Instructs Direct3D to create the device on the specified adapter. This
    /// implicitly sets driver_type to Unknown if it hasn't been set yet.
    ///
    /// ### panics
    /// Panics if driver_type has already been set to a value other than Unknown
    pub fn with_adapter(mut self, adapter: &'a Adapter) -> Self {
        assert!(
            self.driver_type.is_none() || self.driver_type == Some(DriverType::Unknown),
            "Cannot specify Adapter if driver_type is not `Unknown`"
        );
        self.driver_type = Some(DriverType::Unknown);
        self.adapter = Some(adapter);
        self
    }

    /// Pass additional flags to CreateDevice. None are specified by default.
    ///
    /// ### panics
    /// Panics if CreateDeviceFlags::SINGLETHREADED is set. This is not supported
    /// at this time. You can open a pull request if this feature is important to
    /// you.
    pub fn with_flags(mut self, flags: CreateDeviceFlags) -> Self {
        assert!(
            !flags.is_set(CreateDeviceFlags::SINGLETHREADED),
            "SINGLETHREADED is not supported right now. If this usecase is important, please open an issue."
        );
        assert!(flags.validate(), "Invalid flags passed");
        self.flags = flags;
        self
    }

    pub fn with_feature_levels(mut self, levels: &'a [FeatureLevel]) -> Self {
        self.feature_levels = levels;
        self
    }

    pub unsafe fn with_software_module(mut self, software: HMODULE) -> Self {
        assert!(
            self.driver_type.is_none() || self.driver_type == Some(DriverType::Software),
            "Cannot specify software_module if driver_type is not `Software`"
        );
        self.driver_type = Some(DriverType::Software);
        self.software_module = software;
        self
    }

    pub fn build(self) -> Result<(FeatureLevel, Device, DeviceContext), Error> {
        unsafe {
            let mut dev_ptr = ptr::null_mut();
            let mut feature_level = 0;
            let mut devctx_ptr = ptr::null_mut();

            let hr = D3D11CreateDevice(
                // pAdapter:
                self.adapter
                    .map(|a| a.get_raw() as *mut IDXGIAdapter)
                    .unwrap_or(ptr::null_mut()),
                // DriverType:
                self.driver_type.unwrap_or(DriverType::Hardware) as u32,
                // Software:
                self.software_module,
                // Flags:
                self.flags.0,
                // pFeatureLevels:
                if !self.feature_levels.is_empty() {
                    &self.feature_levels[0].0
                } else {
                    ptr::null()
                },
                // FeatureLevels:
                self.feature_levels.len() as u32,
                // SDKVersion
                D3D11_SDK_VERSION,
                // ppDevice:
                &mut dev_ptr,
                // pFeatureLevel:
                &mut feature_level,
                // ppImmediateContext:
                &mut devctx_ptr,
            );

            let dev = Error::wrap_if(hr, dev_ptr)?;
            let ctx = Error::wrap_if(hr, devctx_ptr)?;
            let features = FeatureLevel(feature_level);
            Ok((features, dev, ctx))
        }
    }
}
