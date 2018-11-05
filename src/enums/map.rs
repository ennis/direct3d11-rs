#[auto_enum(u32, checked)]
/// Identifies a resource to be accessed for reading and writing by the
/// CPU. Applications may combine one or more of these flags.
///
/// [More Information][1]
///
/// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff476181(v=vs.85).aspx
pub enum Map {
    /// Resource is mapped for reading. The resource must have been created
    /// with read access.
    Read = 1,

    /// Resource is mapped for writing. The resource must have been created
    /// with write access.
    Write = 2,

    /// Resource is mapped for reading and writing. The resource must have
    /// been created with read and write access.
    ReadWrite = 3,

    /// Resource is mapped for writing; the previous contents of the
    /// resource will be undefined. The resource must have been created
    /// with write access and dynamic usage.
    Discard = 4,

    /// Resource is mapped for writing; the existing contents of the
    /// resource cannot be overwritten (see [Remarks](#remarks)). This flag
    /// is only valid on vertex and index buffers. The resource must have
    /// been created with write access. Cannot be used on a resource
    /// created with the [`CONSTANT_BUFFER`][1] flag.
    ///
    /// [1]: struct.BindFlags.html#associatedconstant.CONSTANT_BUFFER
    WriteNoOverwrite = 5,
}
