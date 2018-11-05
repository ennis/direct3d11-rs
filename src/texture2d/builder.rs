use device::Device;
use error::Error;
use enums::{BindFlags, CpuAccessFlags, ResourceMiscFlags, Usage};
use texture2d::Texture2D;

use std::mem;
use std::ptr;

use dxgi::Format;
use winapi::um::d3d11::{D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC};

pub struct Texture2DBuilder<'a, 'b> {
    device: &'a Device,
    desc: D3D11_TEXTURE2D_DESC,
    initial_data: InitialData<'b>,
    unchecked_format: bool,
}

impl<'a, 'b> Texture2DBuilder<'a, 'b> {
    pub fn new(device: &'a Device) -> Self {
        let mut desc: D3D11_TEXTURE2D_DESC = unsafe { mem::zeroed() };

        desc.MipLevels = 1;
        desc.ArraySize = 1;
        desc.SampleDesc.Count = 1;

        Texture2DBuilder {
            device,
            desc,
            initial_data: InitialData::None,
            unchecked_format: false,
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.desc.Width = width;
        self.desc.Height = height;
        self
    }

    pub fn with_mip_levels(mut self, mip_levels: u32) -> Self {
        self.desc.MipLevels = mip_levels;
        self
    }

    pub fn with_array_size(mut self, array_size: u32) -> Self {
        self.desc.ArraySize = array_size;
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.desc.Format = format as u32;
        self
    }

    pub fn with_samples(mut self, count: u32, quality: u32) -> Self {
        self.desc.SampleDesc.Count = count;
        self.desc.SampleDesc.Quality = quality;
        self
    }

    pub fn with_usage(mut self, usage: Usage) -> Self {
        self.desc.Usage = usage as u32;
        self
    }

    pub fn with_bind_flags(mut self, bind_flags: BindFlags) -> Self {
        self.desc.BindFlags = bind_flags.0;
        self
    }

    pub fn with_cpu_access_flags(mut self, cpu_access_flags: CpuAccessFlags) -> Self {
        self.desc.CPUAccessFlags = cpu_access_flags.0;
        self
    }

    pub fn with_misc_flags(mut self, misc_flags: ResourceMiscFlags) -> Self {
        self.desc.MiscFlags = misc_flags.0;
        self
    }

    pub fn with_initial_data(mut self, data: &'b [u8], pitch: u32) -> Self {
        self.initial_data = InitialData::Single(data, pitch);
        self
    }

    pub fn with_initial_data_slice(mut self, data: &'b [(&'b [u8], u32)]) -> Self {
        self.initial_data = InitialData::Slice(data);
        self
    }

    pub fn with_initial_data_vec(mut self, data: Vec<(&'b [u8], u32)>) -> Self {
        self.initial_data = InitialData::Vec(data);
        self
    }

    /// This flag must be used if you are specifying initial data using a
    /// format with `pixel_size() == 0`. It disables the safety check that
    /// initial_data contains enough bytes to fill the texture. Otherwise,
    /// `build()` will panic if `format.pixel_size() == 0`.
    pub unsafe fn with_unchecked_format(mut self) -> Self {
        self.unchecked_format = true;
        self
    }

    pub fn build(self) -> Result<Texture2D, Error> {
        if !self.unchecked_format {
            self.check_format();
        }

        unsafe {
            let mut v_initial_data = vec![];
            let p_initial_data = self.initial_data
                .to_desc(&mut v_initial_data, self.desc.ArraySize);

            let mut ptr = ptr::null_mut();
            let hr = (*self.device.get_raw()).CreateTexture2D(&self.desc, p_initial_data, &mut ptr);

            Error::wrap_if(hr, ptr)
        }
    }

    fn check_format(&self) {
        self.initial_data.check_format(self.desc.Width);
    }
}

enum InitialData<'a> {
    None,
    Single(&'a [u8], u32),
    Slice(&'a [(&'a [u8], u32)]),
    Vec(Vec<(&'a [u8], u32)>),
}

impl<'a> InitialData<'a> {
    fn len(&self) -> usize {
        match *self {
            InitialData::None => 0,
            InitialData::Single(..) => 1,
            InitialData::Slice(items) => items.len(),
            InitialData::Vec(ref items) => items.len(),
        }
    }

    fn to_desc(
        &self,
        data: &mut Vec<D3D11_SUBRESOURCE_DATA>,
        array_size: u32,
    ) -> *const D3D11_SUBRESOURCE_DATA {
        if self.len() == 0 {
            return ptr::null();
        }

        let len = self.len();
        assert!(len == array_size as usize);
        data.clear();
        data.reserve(len);
        unsafe {
            data.set_len(len);

            self.each(|i, bytes, pitch| {
                let desc = &mut data[i];
                desc.pSysMem = bytes.as_ptr() as *const _;
                desc.SysMemPitch = pitch;
                desc.SysMemSlicePitch = bytes.len() as u32;
            });

            data.as_ptr()
        }
    }

    fn each<F>(&self, mut f: F)
    where
        F: FnMut(usize, &'a [u8], u32),
    {
        match *self {
            InitialData::None => (),
            InitialData::Single(bytes, pitch) => f(0, bytes, pitch),
            InitialData::Slice(items) => {
                for (i, &(bytes, pitch)) in items.iter().enumerate() {
                    f(i, bytes, pitch);
                }
            }
            InitialData::Vec(ref items) => {
                for (i, &(bytes, pitch)) in items.iter().enumerate() {
                    f(i, bytes, pitch);
                }
            }
        }
    }

    fn check_format(&self, width: u32) {
        match *self {
            InitialData::None => (),
            InitialData::Single(bytes, pitch) => {
                InitialData::check_format_item(bytes, pitch, width);
            }
            InitialData::Slice(items) => {
                for &(bytes, pitch) in items {
                    InitialData::check_format_item(bytes, pitch, width);
                }
            }
            InitialData::Vec(ref items) => {
                for &(bytes, pitch) in items {
                    InitialData::check_format_item(bytes, pitch, width);
                }
            }
        }
    }

    fn check_format_item(bytes: &[u8], pitch: u32, width: u32) {
        let img_bytes = width as usize * pitch as usize;
        assert!(bytes.len() >= img_bytes);
    }
}
