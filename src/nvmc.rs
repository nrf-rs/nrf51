#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 1024usize],
    #[doc = "0x400 - Ready flag."]
    pub ready: READY,
    _reserved1: [u8; 256usize],
    #[doc = "0x504 - Configuration register."]
    pub config: CONFIG,
    _reserved_2_erasepage: [u8; 4usize],
    #[doc = "0x50c - Register for erasing all non-volatile user memory."]
    pub eraseall: ERASEALL,
    #[doc = "0x510 - Register for erasing a protected non-volatile memory page."]
    pub erasepcr0: ERASEPCR0,
    #[doc = "0x514 - Register for start erasing User Information Congfiguration Registers."]
    pub eraseuicr: ERASEUICR,
}
impl RegisterBlock {
    #[doc = "0x508 - Register for erasing a non-protected non-volatile memory page."]
    #[inline(always)]
    pub fn erasepcr1(&self) -> &ERASEPCR1 {
        unsafe { &*(((self as *const Self) as *const u8).add(1288usize) as *const ERASEPCR1) }
    }
    #[doc = "0x508 - Register for erasing a non-protected non-volatile memory page."]
    #[inline(always)]
    pub fn erasepcr1_mut(&self) -> &mut ERASEPCR1 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1288usize) as *mut ERASEPCR1) }
    }
    #[doc = "0x508 - Register for erasing a non-protected non-volatile memory page."]
    #[inline(always)]
    pub fn erasepage(&self) -> &ERASEPAGE {
        unsafe { &*(((self as *const Self) as *const u8).add(1288usize) as *const ERASEPAGE) }
    }
    #[doc = "0x508 - Register for erasing a non-protected non-volatile memory page."]
    #[inline(always)]
    pub fn erasepage_mut(&self) -> &mut ERASEPAGE {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(1288usize) as *mut ERASEPAGE) }
    }
}
#[doc = "Ready flag.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ready](ready) module"]
pub type READY = crate::Reg<u32, _READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _READY;
#[doc = "`read()` method returns [ready::R](ready::R) reader structure"]
impl crate::Readable for READY {}
#[doc = "Ready flag."]
pub mod ready;
#[doc = "Configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Configuration register."]
pub mod config;
#[doc = "Register for erasing a non-protected non-volatile memory page.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [erasepage](erasepage) module"]
pub type ERASEPAGE = crate::Reg<u32, _ERASEPAGE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPAGE;
#[doc = "`read()` method returns [erasepage::R](erasepage::R) reader structure"]
impl crate::Readable for ERASEPAGE {}
#[doc = "`write(|w| ..)` method takes [erasepage::W](erasepage::W) writer structure"]
impl crate::Writable for ERASEPAGE {}
#[doc = "Register for erasing a non-protected non-volatile memory page."]
pub mod erasepage;
#[doc = "Register for erasing a non-protected non-volatile memory page.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [erasepcr1](erasepcr1) module"]
pub type ERASEPCR1 = crate::Reg<u32, _ERASEPCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPCR1;
#[doc = "`read()` method returns [erasepcr1::R](erasepcr1::R) reader structure"]
impl crate::Readable for ERASEPCR1 {}
#[doc = "`write(|w| ..)` method takes [erasepcr1::W](erasepcr1::W) writer structure"]
impl crate::Writable for ERASEPCR1 {}
#[doc = "Register for erasing a non-protected non-volatile memory page."]
pub mod erasepcr1;
#[doc = "Register for erasing all non-volatile user memory.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eraseall](eraseall) module"]
pub type ERASEALL = crate::Reg<u32, _ERASEALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEALL;
#[doc = "`read()` method returns [eraseall::R](eraseall::R) reader structure"]
impl crate::Readable for ERASEALL {}
#[doc = "`write(|w| ..)` method takes [eraseall::W](eraseall::W) writer structure"]
impl crate::Writable for ERASEALL {}
#[doc = "Register for erasing all non-volatile user memory."]
pub mod eraseall;
#[doc = "Register for erasing a protected non-volatile memory page.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [erasepcr0](erasepcr0) module"]
pub type ERASEPCR0 = crate::Reg<u32, _ERASEPCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEPCR0;
#[doc = "`read()` method returns [erasepcr0::R](erasepcr0::R) reader structure"]
impl crate::Readable for ERASEPCR0 {}
#[doc = "`write(|w| ..)` method takes [erasepcr0::W](erasepcr0::W) writer structure"]
impl crate::Writable for ERASEPCR0 {}
#[doc = "Register for erasing a protected non-volatile memory page."]
pub mod erasepcr0;
#[doc = "Register for start erasing User Information Congfiguration Registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [eraseuicr](eraseuicr) module"]
pub type ERASEUICR = crate::Reg<u32, _ERASEUICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERASEUICR;
#[doc = "`read()` method returns [eraseuicr::R](eraseuicr::R) reader structure"]
impl crate::Readable for ERASEUICR {}
#[doc = "`write(|w| ..)` method takes [eraseuicr::W](eraseuicr::W) writer structure"]
impl crate::Writable for ERASEUICR {}
#[doc = "Register for start erasing User Information Congfiguration Registers."]
pub mod eraseuicr;
