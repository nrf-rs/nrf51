#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the watchdog."]
    pub tasks_start: TASKS_START,
    _reserved1: [u8; 252usize],
    #[doc = "0x100 - Watchdog timeout."]
    pub events_timeout: EVENTS_TIMEOUT,
    _reserved2: [u8; 512usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved4: [u8; 244usize],
    #[doc = "0x400 - Watchdog running status."]
    pub runstatus: RUNSTATUS,
    #[doc = "0x404 - Request status."]
    pub reqstatus: REQSTATUS,
    _reserved6: [u8; 252usize],
    #[doc = "0x504 - Counter reload value in number of 32kiHz clock cycles."]
    pub crv: CRV,
    #[doc = "0x508 - Reload request enable."]
    pub rren: RREN,
    #[doc = "0x50c - Configuration register."]
    pub config: CONFIG,
    _reserved9: [u8; 240usize],
    #[doc = "0x600 - Reload requests registers."]
    pub rr: [RR; 8],
    _reserved10: [u8; 2524usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start the watchdog.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start the watchdog."]
pub mod tasks_start;
#[doc = "Watchdog timeout.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_timeout](events_timeout) module"]
pub type EVENTS_TIMEOUT = crate::Reg<u32, _EVENTS_TIMEOUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TIMEOUT;
#[doc = "`read()` method returns [events_timeout::R](events_timeout::R) reader structure"]
impl crate::Readable for EVENTS_TIMEOUT {}
#[doc = "`write(|w| ..)` method takes [events_timeout::W](events_timeout::W) writer structure"]
impl crate::Writable for EVENTS_TIMEOUT {}
#[doc = "Watchdog timeout."]
pub mod events_timeout;
#[doc = "Interrupt enable set register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenset](intenset) module"]
pub type INTENSET = crate::Reg<u32, _INTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENSET;
#[doc = "`read()` method returns [intenset::R](intenset::R) reader structure"]
impl crate::Readable for INTENSET {}
#[doc = "`write(|w| ..)` method takes [intenset::W](intenset::W) writer structure"]
impl crate::Writable for INTENSET {}
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "Interrupt enable clear register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [intenclr](intenclr) module"]
pub type INTENCLR = crate::Reg<u32, _INTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTENCLR;
#[doc = "`read()` method returns [intenclr::R](intenclr::R) reader structure"]
impl crate::Readable for INTENCLR {}
#[doc = "`write(|w| ..)` method takes [intenclr::W](intenclr::W) writer structure"]
impl crate::Writable for INTENCLR {}
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "Watchdog running status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [runstatus](runstatus) module"]
pub type RUNSTATUS = crate::Reg<u32, _RUNSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RUNSTATUS;
#[doc = "`read()` method returns [runstatus::R](runstatus::R) reader structure"]
impl crate::Readable for RUNSTATUS {}
#[doc = "Watchdog running status."]
pub mod runstatus;
#[doc = "Request status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reqstatus](reqstatus) module"]
pub type REQSTATUS = crate::Reg<u32, _REQSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQSTATUS;
#[doc = "`read()` method returns [reqstatus::R](reqstatus::R) reader structure"]
impl crate::Readable for REQSTATUS {}
#[doc = "Request status."]
pub mod reqstatus;
#[doc = "Counter reload value in number of 32kiHz clock cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crv](crv) module"]
pub type CRV = crate::Reg<u32, _CRV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRV;
#[doc = "`read()` method returns [crv::R](crv::R) reader structure"]
impl crate::Readable for CRV {}
#[doc = "`write(|w| ..)` method takes [crv::W](crv::W) writer structure"]
impl crate::Writable for CRV {}
#[doc = "Counter reload value in number of 32kiHz clock cycles."]
pub mod crv;
#[doc = "Reload request enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rren](rren) module"]
pub type RREN = crate::Reg<u32, _RREN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RREN;
#[doc = "`read()` method returns [rren::R](rren::R) reader structure"]
impl crate::Readable for RREN {}
#[doc = "`write(|w| ..)` method takes [rren::W](rren::W) writer structure"]
impl crate::Writable for RREN {}
#[doc = "Reload request enable."]
pub mod rren;
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
#[doc = "Reload requests registers.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rr](rr) module"]
pub type RR = crate::Reg<u32, _RR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RR;
#[doc = "`write(|w| ..)` method takes [rr::W](rr::W) writer structure"]
impl crate::Writable for RR {}
#[doc = "Reload requests registers."]
pub mod rr;
#[doc = "Peripheral power control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [power](power) module"]
pub type POWER = crate::Reg<u32, _POWER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POWER;
#[doc = "`read()` method returns [power::R](power::R) reader structure"]
impl crate::Readable for POWER {}
#[doc = "`write(|w| ..)` method takes [power::W](power::W) writer structure"]
impl crate::Writable for POWER {}
#[doc = "Peripheral power control."]
pub mod power;
