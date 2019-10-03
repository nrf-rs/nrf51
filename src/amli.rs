#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 3584usize],
    #[doc = "0xe00 - RAM configurable priority configuration structure."]
    pub rampri: RAMPRI,
}
#[doc = r"Register block"]
#[repr(C)]
pub struct RAMPRI {
    #[doc = "0x00 - Configurable priority configuration register for CPU0."]
    pub cpu0: self::rampri::CPU0,
    #[doc = "0x04 - Configurable priority configuration register for SPIS1."]
    pub spis1: self::rampri::SPIS1,
    #[doc = "0x08 - Configurable priority configuration register for RADIO."]
    pub radio: self::rampri::RADIO,
    #[doc = "0x0c - Configurable priority configuration register for ECB."]
    pub ecb: self::rampri::ECB,
    #[doc = "0x10 - Configurable priority configuration register for CCM."]
    pub ccm: self::rampri::CCM,
    #[doc = "0x14 - Configurable priority configuration register for AAR."]
    pub aar: self::rampri::AAR,
}
#[doc = r"Register block"]
#[doc = "RAM configurable priority configuration structure."]
pub mod rampri;
