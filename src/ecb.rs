#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered."]
    pub tasks_startecb: TASKS_STARTECB,
    #[doc = "0x04 - Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event."]
    pub tasks_stopecb: TASKS_STOPECB,
    _reserved2: [u8; 248usize],
    #[doc = "0x100 - ECB block encrypt complete."]
    pub events_endecb: EVENTS_ENDECB,
    #[doc = "0x104 - ECB block encrypt aborted due to a STOPECB task or due to an error."]
    pub events_errorecb: EVENTS_ERRORECB,
    _reserved4: [u8; 508usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved6: [u8; 504usize],
    #[doc = "0x504 - ECB block encrypt memory pointer."]
    pub ecbdataptr: ECBDATAPTR,
    _reserved7: [u8; 2804usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_startecb](tasks_startecb) module"]
pub type TASKS_STARTECB = crate::Reg<u32, _TASKS_STARTECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTECB;
#[doc = "`write(|w| ..)` method takes [tasks_startecb::W](tasks_startecb::W) writer structure"]
impl crate::Writable for TASKS_STARTECB {}
#[doc = "Start ECB block encrypt. If a crypto operation is running, this will not initiate a new encryption and the ERRORECB event will be triggered."]
pub mod tasks_startecb;
#[doc = "Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stopecb](tasks_stopecb) module"]
pub type TASKS_STOPECB = crate::Reg<u32, _TASKS_STOPECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOPECB;
#[doc = "`write(|w| ..)` method takes [tasks_stopecb::W](tasks_stopecb::W) writer structure"]
impl crate::Writable for TASKS_STOPECB {}
#[doc = "Stop current ECB encryption. If a crypto operation is running, this will will trigger the ERRORECB event."]
pub mod tasks_stopecb;
#[doc = "ECB block encrypt complete.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endecb](events_endecb) module"]
pub type EVENTS_ENDECB = crate::Reg<u32, _EVENTS_ENDECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDECB;
#[doc = "`read()` method returns [events_endecb::R](events_endecb::R) reader structure"]
impl crate::Readable for EVENTS_ENDECB {}
#[doc = "`write(|w| ..)` method takes [events_endecb::W](events_endecb::W) writer structure"]
impl crate::Writable for EVENTS_ENDECB {}
#[doc = "ECB block encrypt complete."]
pub mod events_endecb;
#[doc = "ECB block encrypt aborted due to a STOPECB task or due to an error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_errorecb](events_errorecb) module"]
pub type EVENTS_ERRORECB = crate::Reg<u32, _EVENTS_ERRORECB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERRORECB;
#[doc = "`read()` method returns [events_errorecb::R](events_errorecb::R) reader structure"]
impl crate::Readable for EVENTS_ERRORECB {}
#[doc = "`write(|w| ..)` method takes [events_errorecb::W](events_errorecb::W) writer structure"]
impl crate::Writable for EVENTS_ERRORECB {}
#[doc = "ECB block encrypt aborted due to a STOPECB task or due to an error."]
pub mod events_errorecb;
#[doc = "Interrupt enable set register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](intenset) module"]
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
#[doc = "Interrupt enable clear register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](intenclr) module"]
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
#[doc = "ECB block encrypt memory pointer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecbdataptr](ecbdataptr) module"]
pub type ECBDATAPTR = crate::Reg<u32, _ECBDATAPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECBDATAPTR;
#[doc = "`read()` method returns [ecbdataptr::R](ecbdataptr::R) reader structure"]
impl crate::Readable for ECBDATAPTR {}
#[doc = "`write(|w| ..)` method takes [ecbdataptr::W](ecbdataptr::W) writer structure"]
impl crate::Writable for ECBDATAPTR {}
#[doc = "ECB block encrypt memory pointer."]
pub mod ecbdataptr;
#[doc = "Peripheral power control.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [power](power) module"]
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
