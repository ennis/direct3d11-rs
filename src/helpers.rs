pub trait ComWrapper {
    /// The raw interface type from `winapi`
    type Raw;
    /// Gets a raw pointer to the interface. Does not increment the reference count
    unsafe fn get_raw(&self) -> *mut Self::Raw;
    /// Consumes the wrapper without affecting the reference count
    unsafe fn into_raw(self) -> *mut Self::Raw;
    /// Creates a wrapper from the raw pointer. Takes ownership of the pointer for
    /// reference counting purposes.
    unsafe fn from_raw(raw: *mut Self::Raw) -> Self;
}

macro_rules! com_wrapper {
    (@base $wrap:ident : $raw:ty) => {
        impl $crate::helpers::ComWrapper for $wrap {
            type Raw = $raw;
            unsafe fn get_raw(&self) -> *mut Self::Raw {
                $crate::wio::com::ComPtr::as_raw(&self.ptr)
            }
            unsafe fn into_raw(self) -> *mut Self::Raw {
                $crate::wio::com::ComPtr::into_raw(self.ptr)
            }
            unsafe fn from_raw(raw: *mut Self::Raw) -> Self {
                $wrap {
                    ptr: $crate::wio::com::ComPtr::from_raw(raw),
                }
            }
        }
        impl ::std::fmt::Debug for $wrap {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                fmt.debug_tuple(stringify!($wrap))
                    .field(&self.ptr.as_raw())
                    .finish()
            }
        }
    };
    
    ($wrap:ident: $raw:ty) => {
        com_wrapper!($wrap : $raw, send: true, sync: true);
    };
    
    ($wrap:ident: $raw:ty, send: true, sync: true) => {
        com_wrapper!(@base $wrap : $raw);
        unsafe impl Send for $wrap {}
        unsafe impl Sync for $wrap {}
    };

    ($wrap:ident: $raw:ty, send: true, sync: false) => {
        com_wrapper!(@base $wrap : $raw);
        unsafe impl Send for $wrap {}
    };

    ($wrap:ident: $raw:ty, send: false, sync: false) => {
        com_wrapper!(@base $wrap : $raw);
    };
}

macro_rules! enum_ {
    (
        #[repr($inner:ident)]
        $(#[$attr:meta])*
        pub enum $ety:ident {
            $($(#[$vattr:meta])* $name:ident = $value:expr,)*
        }
    ) => {
        #[repr($inner)]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        $(#[$attr])*
        pub enum $ety {
            $($(#[$vattr])* $name = $value,)*
        }
        impl $ety {
            pub fn try_from(value: $inner) -> Option<Self> {
                match value {
                    $($value => Some($ety :: $name),)*
                    _ => None,
                }
            }
        }
    }
}

macro_rules! flags {
    (
        #[repr($inner:ident)]
        $(#[$attr:meta])*
        pub enum $flagty:ident {
            $($(#[$vattr:meta])* $name:ident = $value:expr,)*
        }
    ) => {
        #[repr(C)]
        #[derive(Copy, Clone, PartialEq, Eq, Hash)]
        $(#[$attr])*
        pub struct $flagty(pub $inner);
        impl $flagty {
            pub const NONE : $flagty = $flagty ( 0 );
            $($(#[$vattr])* pub const $name : $flagty = $flagty ( $value );)*
            pub fn is_set(self, flag: Self) -> bool {
                self & flag == flag
            }
            pub fn clear(&mut self, flag: Self) {
                *self &= !flag;
            }
            pub fn validate(self) -> bool {
                const MASK: $inner = $($value)|*;
                self.0 & !MASK == 0
            }
        }
        impl $crate::std::ops::Not for $flagty {
            type Output = Self;
            fn not(self) -> Self {
                $flagty ( !self.0 )
            }
        }
        impl $crate::std::ops::BitAnd for $flagty {
            type Output = Self;
            fn bitand(self, rhs: Self) -> Self {
                $flagty ( self.0 & rhs.0 )
            }
        }
        impl $crate::std::ops::BitAndAssign for $flagty {
            fn bitand_assign(&mut self, rhs: Self) {
                self.0 &= rhs.0;
            }
        }
        impl $crate::std::ops::BitOr for $flagty {
            type Output = Self;
            fn bitor(self, rhs: Self) -> Self {
                $flagty ( self.0 | rhs.0 )
            }
        }
        impl $crate::std::ops::BitOrAssign for $flagty {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
        impl $crate::std::ops::BitXor for $flagty {
            type Output = Self;
            fn bitxor(self, rhs: Self) -> Self {
                $flagty ( self.0 ^ rhs.0 )
            }
        }
        impl $crate::std::ops::BitXorAssign for $flagty {
            fn bitxor_assign(&mut self, rhs: Self) {
                self.0 ^= rhs.0;
            }
        }
        impl $crate::std::fmt::Debug for $flagty {
            fn fmt(&self, fmt: &mut $crate::std::fmt::Formatter) -> $crate::std::fmt::Result {
                fmt.write_str(concat!(stringify!($flagty), "("))?;
                let mut first = true;
                $(if self.is_set($flagty :: $name) {
                    if first {
                        first = false;
                    } else {
                        fmt.write_str(" | ")?;
                    }
                    fmt.write_str(stringify!($name))?;
                })*
                if first {
                    fmt.write_str("NONE")?;
                }
                fmt.write_str(")")?;
                Ok(())
            }
        }
    }
}
