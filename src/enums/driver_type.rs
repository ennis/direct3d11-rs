#[auto_enum(u32, checked)]
pub enum DriverType {
    Unknown = 0,
    Hardware = 1,
    Reference = 2,
    Null = 3,
    Software = 4,
    Warp = 5,
}
