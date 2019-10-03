#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Code memory page size in bytes."]
    pub codepagesize: CODEPAGESIZE,
    #[doc = "0x14 - Code memory size in pages."]
    pub codesize: CODESIZE,
    _reserved2: [u8; 16usize],
    #[doc = "0x28 - Length of code region 0 in bytes."]
    pub clenr0: CLENR0,
    #[doc = "0x2c - Pre-programmed factory code present."]
    pub ppfc: PPFC,
    _reserved4: [u8; 4usize],
    #[doc = "0x34 - Number of individualy controllable RAM blocks."]
    pub numramblock: NUMRAMBLOCK,
    _reserved_5_sizeramblock: [u8; 16usize],
    _reserved6: [u8; 20usize],
    #[doc = "0x5c - Configuration identifier."]
    pub configid: CONFIGID,
    #[doc = "0x60 - Device identifier."]
    pub deviceid: [DEVICEID; 2],
    _reserved8: [u8; 24usize],
    #[doc = "0x80 - Encryption root."]
    pub er: [ER; 4],
    #[doc = "0x90 - Identity root."]
    pub ir: [IR; 4],
    #[doc = "0xa0 - Device address type."]
    pub deviceaddrtype: DEVICEADDRTYPE,
    #[doc = "0xa4 - Device address."]
    pub deviceaddr: [DEVICEADDR; 2],
    #[doc = "0xac - Radio calibration override enable."]
    pub overrideen: OVERRIDEEN,
    #[doc = "0xb0 - Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
    pub nrf_1mbit: [NRF_1MBIT; 5],
    _reserved14: [u8; 40usize],
    #[doc = "0xec - Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
    pub ble_1mbit: [BLE_1MBIT; 5],
}
impl RegisterBlock {
    #[doc = "0x38 - Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
    #[inline(always)]
    pub fn sizeramblock(&self) -> &[SIZERAMBLOCK; 4] {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const [SIZERAMBLOCK; 4]) }
    }
    #[doc = "0x38 - Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
    #[inline(always)]
    pub fn sizeramblock_mut(&self) -> &mut [SIZERAMBLOCK; 4] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut [SIZERAMBLOCK; 4]) }
    }
    #[doc = "0x38 - Size of RAM blocks in bytes."]
    #[inline(always)]
    pub fn sizeramblocks(&self) -> &SIZERAMBLOCKS {
        unsafe { &*(((self as *const Self) as *const u8).add(56usize) as *const SIZERAMBLOCKS) }
    }
    #[doc = "0x38 - Size of RAM blocks in bytes."]
    #[inline(always)]
    pub fn sizeramblocks_mut(&self) -> &mut SIZERAMBLOCKS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(56usize) as *mut SIZERAMBLOCKS) }
    }
}
#[doc = "Code memory page size in bytes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [codepagesize](codepagesize) module"]
pub type CODEPAGESIZE = crate::Reg<u32, _CODEPAGESIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CODEPAGESIZE;
#[doc = "`read()` method returns [codepagesize::R](codepagesize::R) reader structure"]
impl crate::Readable for CODEPAGESIZE {}
#[doc = "`write(|w| ..)` method takes [codepagesize::W](codepagesize::W) writer structure"]
impl crate::Writable for CODEPAGESIZE {}
#[doc = "Code memory page size in bytes."]
pub mod codepagesize;
#[doc = "Code memory size in pages.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [codesize](codesize) module"]
pub type CODESIZE = crate::Reg<u32, _CODESIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CODESIZE;
#[doc = "`read()` method returns [codesize::R](codesize::R) reader structure"]
impl crate::Readable for CODESIZE {}
#[doc = "`write(|w| ..)` method takes [codesize::W](codesize::W) writer structure"]
impl crate::Writable for CODESIZE {}
#[doc = "Code memory size in pages."]
pub mod codesize;
#[doc = "Length of code region 0 in bytes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clenr0](clenr0) module"]
pub type CLENR0 = crate::Reg<u32, _CLENR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLENR0;
#[doc = "`read()` method returns [clenr0::R](clenr0::R) reader structure"]
impl crate::Readable for CLENR0 {}
#[doc = "`write(|w| ..)` method takes [clenr0::W](clenr0::W) writer structure"]
impl crate::Writable for CLENR0 {}
#[doc = "Length of code region 0 in bytes."]
pub mod clenr0;
#[doc = "Pre-programmed factory code present.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ppfc](ppfc) module"]
pub type PPFC = crate::Reg<u32, _PPFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPFC;
#[doc = "`read()` method returns [ppfc::R](ppfc::R) reader structure"]
impl crate::Readable for PPFC {}
#[doc = "`write(|w| ..)` method takes [ppfc::W](ppfc::W) writer structure"]
impl crate::Writable for PPFC {}
#[doc = "Pre-programmed factory code present."]
pub mod ppfc;
#[doc = "Number of individualy controllable RAM blocks.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [numramblock](numramblock) module"]
pub type NUMRAMBLOCK = crate::Reg<u32, _NUMRAMBLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NUMRAMBLOCK;
#[doc = "`read()` method returns [numramblock::R](numramblock::R) reader structure"]
impl crate::Readable for NUMRAMBLOCK {}
#[doc = "`write(|w| ..)` method takes [numramblock::W](numramblock::W) writer structure"]
impl crate::Writable for NUMRAMBLOCK {}
#[doc = "Number of individualy controllable RAM blocks."]
pub mod numramblock;
#[doc = "Size of RAM blocks in bytes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sizeramblocks](sizeramblocks) module"]
pub type SIZERAMBLOCKS = crate::Reg<u32, _SIZERAMBLOCKS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIZERAMBLOCKS;
#[doc = "`read()` method returns [sizeramblocks::R](sizeramblocks::R) reader structure"]
impl crate::Readable for SIZERAMBLOCKS {}
#[doc = "`write(|w| ..)` method takes [sizeramblocks::W](sizeramblocks::W) writer structure"]
impl crate::Writable for SIZERAMBLOCKS {}
#[doc = "Size of RAM blocks in bytes."]
pub mod sizeramblocks;
#[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sizeramblock](sizeramblock) module"]
pub type SIZERAMBLOCK = crate::Reg<u32, _SIZERAMBLOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIZERAMBLOCK;
#[doc = "`read()` method returns [sizeramblock::R](sizeramblock::R) reader structure"]
impl crate::Readable for SIZERAMBLOCK {}
#[doc = "`write(|w| ..)` method takes [sizeramblock::W](sizeramblock::W) writer structure"]
impl crate::Writable for SIZERAMBLOCK {}
#[doc = "Deprecated array of size of RAM block in bytes. This name is kept for backward compatinility purposes. Use SIZERAMBLOCKS instead."]
pub mod sizeramblock;
#[doc = "Configuration identifier.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [configid](configid) module"]
pub type CONFIGID = crate::Reg<u32, _CONFIGID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIGID;
#[doc = "`read()` method returns [configid::R](configid::R) reader structure"]
impl crate::Readable for CONFIGID {}
#[doc = "`write(|w| ..)` method takes [configid::W](configid::W) writer structure"]
impl crate::Writable for CONFIGID {}
#[doc = "Configuration identifier."]
pub mod configid;
#[doc = "Device identifier.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deviceid](deviceid) module"]
pub type DEVICEID = crate::Reg<u32, _DEVICEID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICEID;
#[doc = "`read()` method returns [deviceid::R](deviceid::R) reader structure"]
impl crate::Readable for DEVICEID {}
#[doc = "`write(|w| ..)` method takes [deviceid::W](deviceid::W) writer structure"]
impl crate::Writable for DEVICEID {}
#[doc = "Device identifier."]
pub mod deviceid;
#[doc = "Encryption root.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [er](er) module"]
pub type ER = crate::Reg<u32, _ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ER;
#[doc = "`read()` method returns [er::R](er::R) reader structure"]
impl crate::Readable for ER {}
#[doc = "`write(|w| ..)` method takes [er::W](er::W) writer structure"]
impl crate::Writable for ER {}
#[doc = "Encryption root."]
pub mod er;
#[doc = "Identity root.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ir](ir) module"]
pub type IR = crate::Reg<u32, _IR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IR;
#[doc = "`read()` method returns [ir::R](ir::R) reader structure"]
impl crate::Readable for IR {}
#[doc = "`write(|w| ..)` method takes [ir::W](ir::W) writer structure"]
impl crate::Writable for IR {}
#[doc = "Identity root."]
pub mod ir;
#[doc = "Device address type.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deviceaddrtype](deviceaddrtype) module"]
pub type DEVICEADDRTYPE = crate::Reg<u32, _DEVICEADDRTYPE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICEADDRTYPE;
#[doc = "`read()` method returns [deviceaddrtype::R](deviceaddrtype::R) reader structure"]
impl crate::Readable for DEVICEADDRTYPE {}
#[doc = "`write(|w| ..)` method takes [deviceaddrtype::W](deviceaddrtype::W) writer structure"]
impl crate::Writable for DEVICEADDRTYPE {}
#[doc = "Device address type."]
pub mod deviceaddrtype;
#[doc = "Device address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deviceaddr](deviceaddr) module"]
pub type DEVICEADDR = crate::Reg<u32, _DEVICEADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEVICEADDR;
#[doc = "`read()` method returns [deviceaddr::R](deviceaddr::R) reader structure"]
impl crate::Readable for DEVICEADDR {}
#[doc = "`write(|w| ..)` method takes [deviceaddr::W](deviceaddr::W) writer structure"]
impl crate::Writable for DEVICEADDR {}
#[doc = "Device address."]
pub mod deviceaddr;
#[doc = "Radio calibration override enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [overrideen](overrideen) module"]
pub type OVERRIDEEN = crate::Reg<u32, _OVERRIDEEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OVERRIDEEN;
#[doc = "`read()` method returns [overrideen::R](overrideen::R) reader structure"]
impl crate::Readable for OVERRIDEEN {}
#[doc = "`write(|w| ..)` method takes [overrideen::W](overrideen::W) writer structure"]
impl crate::Writable for OVERRIDEEN {}
#[doc = "Radio calibration override enable."]
pub mod overrideen;
#[doc = "Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nrf_1mbit](nrf_1mbit) module"]
pub type NRF_1MBIT = crate::Reg<u32, _NRF_1MBIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NRF_1MBIT;
#[doc = "`read()` method returns [nrf_1mbit::R](nrf_1mbit::R) reader structure"]
impl crate::Readable for NRF_1MBIT {}
#[doc = "`write(|w| ..)` method takes [nrf_1mbit::W](nrf_1mbit::W) writer structure"]
impl crate::Writable for NRF_1MBIT {}
#[doc = "Override values for the OVERRIDEn registers in RADIO for NRF_1Mbit mode."]
pub mod nrf_1mbit;
#[doc = "Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ble_1mbit](ble_1mbit) module"]
pub type BLE_1MBIT = crate::Reg<u32, _BLE_1MBIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLE_1MBIT;
#[doc = "`read()` method returns [ble_1mbit::R](ble_1mbit::R) reader structure"]
impl crate::Readable for BLE_1MBIT {}
#[doc = "`write(|w| ..)` method takes [ble_1mbit::W](ble_1mbit::W) writer structure"]
impl crate::Writable for BLE_1MBIT {}
#[doc = "Override values for the OVERRIDEn registers in RADIO for BLE_1Mbit mode."]
pub mod ble_1mbit;
