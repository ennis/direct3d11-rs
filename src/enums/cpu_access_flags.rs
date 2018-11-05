#[enum_flags(u32)]
/// Specifies the types of CPU access allowed for a resource.
///
/// [More Information][1]
///
/// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff476106(v=vs.85).aspx
pub enum CpuAccessFlags {
    /// The resource is to be mappable so that the CPU can change its
    /// contents. Resources created with this flag cannot be set as
    /// outputs of the pipeline and must be created with either dynamic
    /// or staging usage.
    WRITE = 0x10000,

    /// The resource is to be mappable so that the CPU can read its
    /// contents. Resources created with this flag cannot be set as
    /// either inputs or outputs to the pipeline and must be created
    /// with staging usage.
    READ = 0x20000,

    /// Both the READ and WRITE flags set in a convenient constant.
    READWRITE = 0x10000 | 0x20000,
}
