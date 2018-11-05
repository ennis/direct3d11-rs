use std::fmt;

use com_wrapper::ComWrapper;
use dxgi::error::Error as DxgiError;
use winapi::shared::winerror::{HRESULT, SUCCEEDED};
use winapi::Interface;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Error {
    Dxgi(DxgiError),
    D3D11(HRESULT),
}

impl Error {
    pub(crate) unsafe fn wrap_if<O, P>(hr: HRESULT, ptr: *mut P) -> Result<O, Error>
    where
        P: Interface,
        O: ComWrapper<Interface = P>,
    {
        if SUCCEEDED(hr) {
            Ok(O::from_raw(ptr))
        } else {
            Err(Error::D3D11(hr))
        }
    }
}

impl From<HRESULT> for Error {
    fn from(hr: HRESULT) -> Error {
        Error::D3D11(hr)
    }
}

impl From<DxgiError> for Error {
    fn from(err: DxgiError) -> Error {
        Error::Dxgi(err)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Dxgi(err) => fmt.debug_tuple("Dxgi").field(&err).finish(),
            Error::D3D11(err) => fmt.debug_tuple("D3D11").field(&DxgiError(err)).finish(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Dxgi(err) => write!(fmt, "{}", err),
            Error::D3D11(err) => write!(fmt, "{}", DxgiError(err)),
        }
    }
}
