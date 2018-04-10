flags! {
    #[repr(u32)]
    pub enum CreateDeviceFlags {
        /// Use this flag if your application will only call methods of
        /// Direct3D 11 interfaces from a single thread. By default, the
        /// ID3D11Device object is thread-safe. By using this flag, you
        /// can increase performance. However, if you use this flag and
        /// your application calls methods of Direct3D 11 interfaces from
        /// multiple threads, undefined behavior might result.
        ///
        /// ***Note*** This flag is not currently supported by the crate
        SINGLETHREADED = 0x1,

        /// Creates a device that supports the [debug layer][1].
        ///
        /// To use this flag, you must have D3D11*SDKLayers.dll installed;
        /// otherwise, device creation fails. To get D3D11_1SDKLayers.dll,
        /// install the SDK for Windows 8. Or on windows 10, install the
        /// `Graphics Tools` optional feature from Settings -> Apps ->
        /// Manage Optional Features
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff476881(v=vs.85).aspx#Debug
        DEBUG = 0x2,

        /// ***Note***  This flag is not supported in Direct3D 11.
        SWITCH_TO_REF = 0x4,

        /// Prevents multiple threads from being created. When this flag is
        /// used with a [Windows Advanced Rasterization Platform (WARP)][1]
        /// device, no additional threads will be created by WARP and all
        /// rasterization will occur on the calling thread. This flag is
        /// not recommended for general use.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/gg615082(v=vs.85).aspx
        PREVENT_INTERNAL_THREADING_OPTIMIZATIONS = 0x8,

        /// Creates a device that supports BGRA formats
        /// ([`B8G8R8A8Unorm`][1] and [`B8G8R8A8UnormSrgb`][2]).
        /// All 10level9 and higher hardware with WDDM 1.1+ drivers
        /// support BGRA formats.
        ///
        /// ***Note*** Required for Direct2D interoperability with Direct3D resources.
        ///
        /// [1]: enum.Format.html#variant.B8G8R8A8Unorm
        /// [2]: enum.Format.html#variant.B8G8R8A8UnormSrgb
        BGRA_SUPPORT = 0x20,

        /// Causes the device and driver to keep information that you can
        /// use for shader debugging. The exact impact from this flag will
        /// vary from driver to driver.
        ///
        /// To use this flag, you must have the debug layers installed.
        /// See [`DEBUG`](#associatedconstant.DEBUG)
        ///
        /// If you use this flag and the current driver does not support
        /// shader debugging, device creation fails. Shader debugging
        /// requires a driver that is implemented to the WDDM for Windows
        /// 8 (WDDM 1.2).
        ///
        /// **Direct3D 11:** This value is not supported until Direct3D 11.1.
        DEBUGGABLE = 0x40,

        /// Causes the Direct3D runtime to ignore registry settings that
        /// turn on the debug layer. You can turn on the debug layer by
        /// using the DirectX Control Panel that was included as part of
        /// the DirectX SDK. We shipped the last version of the DirectX
        /// SDK in June 2010; you can download it from the Microsoft
        /// Download Center. You can set this flag in your app, typically
        /// in release builds only, to prevent end users from using the
        /// DirectX Control Panel to monitor how the app uses Direct3D.
        ///
        /// **Note** You can also set this flag in your app to prevent
        /// Direct3D debugging tools, such as Visual Studio Ultimate 2012,
        /// from hooking your app.
        ///
        /// **Windows 8.1:** This flag doesn't prevent Visual Studio 2013
        /// and later running on Windows 8.1 and later from hooking your
        /// app; instead use ID3D11DeviceContext2::IsAnnotationEnabled.
        /// This flag still prevents Visual Studio 2013 and later running
        /// on Windows 8 and earlier from hooking your app.
        ///
        /// **Direct3D 11:** This value is not supported until Direct3D 11.1.
        PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY = 0x80,

        /// Use this flag if the device will produce GPU workloads that
        /// take more than two seconds to complete, and you want the
        /// operating system to allow them to successfully finish. If this
        /// flag is not set, the operating system performs timeout
        /// detection and recovery when it detects a GPU packet that took
        /// more than two seconds to execute. If this flag is set, the
        /// operating system allows such a long running packet to execute
        /// without resetting the GPU. We recommend not to set this flag
        /// if your device needs to be highly responsive so that the
        /// operating system can detect and recover from GPU timeouts. We
        /// recommend to set this flag if your device needs to perform time
        /// consuming background tasks such as compute, image recognition,
        /// and video encoding to allow such tasks to successfully finish.
        ///
        /// **Direct3D 11:** This value is not supported until Direct3D 11.1.
        DISABLE_GPU_TIMEOUT = 0x100,

        /// Forces the creation of the Direct3D device to fail if the display
        /// driver is not implemented to the WDDM for Windows 8 (WDDM 1.2).
        /// When the display driver is not implemented to WDDM 1.2, only a
        /// Direct3D device that is created with feature level 9.1, 9.2, or
        /// 9.3 supports video; therefore, if this flag is set, the runtime
        /// creates the Direct3D device only for [feature level][1] 9.1, 9.2,
        /// or 9.3. We recommend not to specify this flag for applications that
        /// want to favor Direct3D capability over video. If feature level 10
        /// and higher is available, the runtime will use that feature level
        /// regardless of video support.
        ///
        /// If this flag is set, device creation on the Basic Render Device
        /// (BRD) will succeed regardless of the BRD's missing support for
        /// video decode. This is because the Media Foundation video stack
        /// operates in software mode on BRD. In this situation, if you force
        /// the video stack to create the Direct3D device twice (create the
        /// device once with this flag, next discover BRD, then again create
        /// the device without the flag), you actually degrade performance.
        ///
        /// If you attempt to create a Direct3D device with driver type
        /// [`Null`][2], [`Reference`][3], or [`Software`][4], device creation
        /// fails at any [feature level][1] because none of the associated
        /// drivers provide video capability. If you attempt to create a
        /// Direct3D device with driver type [`Warp`][5], device creation
        /// succeeds to allow software fallback for video.
        ///
        /// **Direct3D 11:** This value is not supported until Direct3D 11.1.
        ///
        /// [1]: https://msdn.microsoft.com/en-us/library/windows/desktop/ff476876(v=vs.85).aspx#Overview
        /// [2]: enum.DriverType.html#variant.Null
        /// [3]: enum.DriverType.html#variant.Reference
        /// [4]: enum.DriverType.html#variant.Software
        /// [5]: enum.DriverType.html#variant.Warp
        VIDEO_SUPPORT = 0x800,
    }
}
