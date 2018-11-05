use std::fmt;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FeatureLevel(pub u32);
impl FeatureLevel {
    pub const LEVEL_9_1: FeatureLevel = FeatureLevel(0x9100);
    pub const LEVEL_9_2: FeatureLevel = FeatureLevel(0x9200);
    pub const LEVEL_9_3: FeatureLevel = FeatureLevel(0x9300);
    pub const LEVEL_10_0: FeatureLevel = FeatureLevel(0xA000);
    pub const LEVEL_10_1: FeatureLevel = FeatureLevel(0xA100);
    pub const LEVEL_11_0: FeatureLevel = FeatureLevel(0xB000);
    pub const LEVEL_11_1: FeatureLevel = FeatureLevel(0xB100);
    pub const LEVEL_12_0: FeatureLevel = FeatureLevel(0xC000);
    pub const LEVEL_12_1: FeatureLevel = FeatureLevel(0xC100);
    pub fn major(&self) -> u32 {
        (self.0 >> 12) & 0xF
    }
    pub fn minor(&self) -> u32 {
        (self.0 >> 8) & 0xF
    }
}
impl fmt::Debug for FeatureLevel {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "FeatureLevel({}.{})", self.major(), self.minor())
    }
}
