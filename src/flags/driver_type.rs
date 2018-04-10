enum_! {
    #[repr(u32)]
    pub enum DriverType {
        Unknown = 0,
        Hardware = 1,
        Reference = 2,
        Null = 3,
        Software = 4,
        Warp = 5,
    }
}
