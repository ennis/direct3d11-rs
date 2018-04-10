flags! {
    #[repr(u32)]
    /// Identifies how to bind a resource to the pipeline.
    /// 
    /// [More Information][1]
    /// 
    /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff476085(v=vs.85).aspx
    pub enum BindFlags {
        /// Bind a buffer as a vertex buffer to the input-assembler stage.
        VERTEX_BUFFER     = 0x1,

        /// Bind a buffer as an index buffer to the input-assembler stage.
        INDEX_BUFFER      = 0x2,

        /// Bind a buffer as a constant buffer to a shader stage; this flag
        /// may NOT be combined with any other bind flag.
        CONSTANT_BUFFER   = 0x4,

        /// Bind a buffer or texture to a shader stage; this flag cannot be
        /// used with the [`Map::WriteNoOverwrite`][1] flag.
        /// 
        /// [1]: enum.Map.html#variant.WriteNoOverwrite
        SHADER_RESOURCE   = 0x8,

        /// Bind an output buffer for the stream-output stage.
        STREAM_OUTPUT     = 0x10,

        /// Bind a texture as a render target for the output-merger stage.
        RENDER_TARGET     = 0x20,

        /// Bind a texture as a depth-stencil target for the output-merger stage.
        DEPTH_STENCIL     = 0x40,

        /// Bind an [unordered access][1] resource.
        /// 
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff476335(v=vs.85).aspx
        UNORDERED_ACCESS  = 0x80,

        /// Set this flag to indicate that a [2D texture][1] is used to receive
        /// output from the decoder API. The common way to create resources
        /// for a decoder output is by calling the ID3D11Device::CreateTexture2D
        /// method to create an array of 2D textures. However, you cannot use
        /// texture arrays that are created with this flag in calls to
        /// ID3D11Device::CreateShaderResourceView.
        /// 
        /// **Direct3D 11:** This value is not supported until Direct3D 11.1.
        /// 
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff476906(v=vs.85).aspx
        DECODER           = 0x200,

        /// Set this flag to indicate that a [2D texture][1] is used to receive
        /// input from the video encoder API. The common way to create
        /// resources for a video encoder is by calling the
        /// ID3D11Device::CreateTexture2D method to create an array of 2D
        /// textures. However, you cannot use texture arrays that are created
        /// with this flag in calls to ID3D11Device::CreateShaderResourceView.
        /// 
        /// **Direct3D 11:** This value is not supported until Direct3D 11.1.
        /// 
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff476906(v=vs.85).aspx
        VIDEO_ENCODER     = 0x400,
    }
}
