#[auto_enum::enum_flags(u32)]
/// Identifies options for resources.
///
/// [More Information][1]
///
/// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff476203(v=vs.85).aspx
pub enum ResourceMiscFlags {
    GENERATE_MIPS = 0x1,
    SHARED = 0x2,
    TEXTURECUBE = 0x4,
    DRAWINDIRECT_ARGS = 0x10,
    BUFFER_ALLOW_RAW_VIEWS = 0x20,
    BUFFER_STRUCTURED = 0x40,
    RESOURCE_CLAMP = 0x80,
    SHARED_KEYEDMUTEX = 0x100,
    GDI_COMPATIBLE = 0x200,
    SHARED_NTHANDLE = 0x800,
    RESTRICTED_CONTENT = 0x1000,
    RESTRICT_SHARED_RESOURCE = 0x2000,
    RESTRICT_SHARED_RESOURCE_DRIVER = 0x4000,
    GUARDED = 0x8000,
    TILE_POOL = 0x20000,
    TILED = 0x40000,
    HW_PROTECTED = 0x80000,
}
