#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Length of code region 0."]
    pub clenr0: CLENR0,
    #[doc = "0x04 - Readback protection configuration."]
    pub rbpconf: RBPCONF,
    #[doc = "0x08 - Reset value for CLOCK XTALFREQ register."]
    pub xtalfreq: XTALFREQ,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Firmware ID."]
    pub fwid: FWID,
    _reserved_4_nrffw: [u8; 60usize],
    #[doc = "0x50 - Reserved for Nordic hardware design."]
    pub nrfhw: [NRFHW; 12],
    #[doc = "0x80 - Reserved for customer."]
    pub customer: [CUSTOMER; 32],
}
impl RegisterBlock {
    #[doc = "0x14 - Reserved for Nordic firmware design."]
    #[inline(always)]
    pub fn nrffw(&self) -> &[NRFFW; 15] {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const [NRFFW; 15]) }
    }
    #[doc = "0x14 - Reserved for Nordic firmware design."]
    #[inline(always)]
    pub fn nrffw_mut(&self) -> &mut [NRFFW; 15] {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut [NRFFW; 15]) }
    }
    #[doc = "0x14 - Bootloader start address."]
    #[inline(always)]
    pub fn bootloaderaddr(&self) -> &BOOTLOADERADDR {
        unsafe { &*(((self as *const Self) as *const u8).add(20usize) as *const BOOTLOADERADDR) }
    }
    #[doc = "0x14 - Bootloader start address."]
    #[inline(always)]
    pub fn bootloaderaddr_mut(&self) -> &mut BOOTLOADERADDR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(20usize) as *mut BOOTLOADERADDR) }
    }
}
#[doc = "Length of code region 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clenr0](clenr0) module"]
pub type CLENR0 = crate::Reg<u32, _CLENR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLENR0;
#[doc = "`read()` method returns [clenr0::R](clenr0::R) reader structure"]
impl crate::Readable for CLENR0 {}
#[doc = "`write(|w| ..)` method takes [clenr0::W](clenr0::W) writer structure"]
impl crate::Writable for CLENR0 {}
#[doc = "Length of code region 0."]
pub mod clenr0;
#[doc = "Readback protection configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rbpconf](rbpconf) module"]
pub type RBPCONF = crate::Reg<u32, _RBPCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RBPCONF;
#[doc = "`read()` method returns [rbpconf::R](rbpconf::R) reader structure"]
impl crate::Readable for RBPCONF {}
#[doc = "`write(|w| ..)` method takes [rbpconf::W](rbpconf::W) writer structure"]
impl crate::Writable for RBPCONF {}
#[doc = "Readback protection configuration."]
pub mod rbpconf;
#[doc = "Reset value for CLOCK XTALFREQ register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xtalfreq](xtalfreq) module"]
pub type XTALFREQ = crate::Reg<u32, _XTALFREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _XTALFREQ;
#[doc = "`read()` method returns [xtalfreq::R](xtalfreq::R) reader structure"]
impl crate::Readable for XTALFREQ {}
#[doc = "`write(|w| ..)` method takes [xtalfreq::W](xtalfreq::W) writer structure"]
impl crate::Writable for XTALFREQ {}
#[doc = "Reset value for CLOCK XTALFREQ register."]
pub mod xtalfreq;
#[doc = "Firmware ID.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fwid](fwid) module"]
pub type FWID = crate::Reg<u32, _FWID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FWID;
#[doc = "`read()` method returns [fwid::R](fwid::R) reader structure"]
impl crate::Readable for FWID {}
#[doc = "Firmware ID."]
pub mod fwid;
#[doc = "Bootloader start address.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootloaderaddr](bootloaderaddr) module"]
pub type BOOTLOADERADDR = crate::Reg<u32, _BOOTLOADERADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOTLOADERADDR;
#[doc = "`read()` method returns [bootloaderaddr::R](bootloaderaddr::R) reader structure"]
impl crate::Readable for BOOTLOADERADDR {}
#[doc = "`write(|w| ..)` method takes [bootloaderaddr::W](bootloaderaddr::W) writer structure"]
impl crate::Writable for BOOTLOADERADDR {}
#[doc = "Bootloader start address."]
pub mod bootloaderaddr;
#[doc = "Reserved for Nordic firmware design.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrffw](nrffw) module"]
pub type NRFFW = crate::Reg<u32, _NRFFW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NRFFW;
#[doc = "`read()` method returns [nrffw::R](nrffw::R) reader structure"]
impl crate::Readable for NRFFW {}
#[doc = "`write(|w| ..)` method takes [nrffw::W](nrffw::W) writer structure"]
impl crate::Writable for NRFFW {}
#[doc = "Reserved for Nordic firmware design."]
pub mod nrffw;
#[doc = "Reserved for Nordic hardware design.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nrfhw](nrfhw) module"]
pub type NRFHW = crate::Reg<u32, _NRFHW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NRFHW;
#[doc = "`read()` method returns [nrfhw::R](nrfhw::R) reader structure"]
impl crate::Readable for NRFHW {}
#[doc = "`write(|w| ..)` method takes [nrfhw::W](nrfhw::W) writer structure"]
impl crate::Writable for NRFHW {}
#[doc = "Reserved for Nordic hardware design."]
pub mod nrfhw;
#[doc = "Reserved for customer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [customer](customer) module"]
pub type CUSTOMER = crate::Reg<u32, _CUSTOMER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CUSTOMER;
#[doc = "`read()` method returns [customer::R](customer::R) reader structure"]
impl crate::Readable for CUSTOMER {}
#[doc = "`write(|w| ..)` method takes [customer::W](customer::W) writer structure"]
impl crate::Writable for CUSTOMER {}
#[doc = "Reserved for customer."]
pub mod customer;
