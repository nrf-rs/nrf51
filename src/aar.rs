#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start resolving addresses based on IRKs specified in the IRK data structure."]
    pub tasks_start: TASKS_START,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Stop resolving addresses."]
    pub tasks_stop: TASKS_STOP,
    _reserved2: [u8; 244usize],
    #[doc = "0x100 - Address resolution procedure completed."]
    pub events_end: EVENTS_END,
    #[doc = "0x104 - Address resolved."]
    pub events_resolved: EVENTS_RESOLVED,
    #[doc = "0x108 - Address not resolved."]
    pub events_notresolved: EVENTS_NOTRESOLVED,
    _reserved5: [u8; 504usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 244usize],
    #[doc = "0x400 - Resolution status."]
    pub status: STATUS,
    _reserved8: [u8; 252usize],
    #[doc = "0x500 - Enable AAR."]
    pub enable: ENABLE,
    #[doc = "0x504 - Number of Identity root Keys in the IRK data structure."]
    pub nirk: NIRK,
    #[doc = "0x508 - Pointer to the IRK data structure."]
    pub irkptr: IRKPTR,
    _reserved11: [u8; 4usize],
    #[doc = "0x510 - Pointer to the resolvable address (6 bytes)."]
    pub addrptr: ADDRPTR,
    #[doc = "0x514 - Pointer to a \"scratch\" data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved."]
    pub scratchptr: SCRATCHPTR,
    _reserved13: [u8; 2788usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start resolving addresses based on IRKs specified in the IRK data structure.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start resolving addresses based on IRKs specified in the IRK data structure."]
pub mod tasks_start;
#[doc = "Stop resolving addresses.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop resolving addresses."]
pub mod tasks_stop;
#[doc = "Address resolution procedure completed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_end](events_end) module"]
pub type EVENTS_END = crate::Reg<u32, _EVENTS_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_END;
#[doc = "`read()` method returns [events_end::R](events_end::R) reader structure"]
impl crate::Readable for EVENTS_END {}
#[doc = "`write(|w| ..)` method takes [events_end::W](events_end::W) writer structure"]
impl crate::Writable for EVENTS_END {}
#[doc = "Address resolution procedure completed."]
pub mod events_end;
#[doc = "Address resolved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_resolved](events_resolved) module"]
pub type EVENTS_RESOLVED = crate::Reg<u32, _EVENTS_RESOLVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RESOLVED;
#[doc = "`read()` method returns [events_resolved::R](events_resolved::R) reader structure"]
impl crate::Readable for EVENTS_RESOLVED {}
#[doc = "`write(|w| ..)` method takes [events_resolved::W](events_resolved::W) writer structure"]
impl crate::Writable for EVENTS_RESOLVED {}
#[doc = "Address resolved."]
pub mod events_resolved;
#[doc = "Address not resolved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_notresolved](events_notresolved) module"]
pub type EVENTS_NOTRESOLVED = crate::Reg<u32, _EVENTS_NOTRESOLVED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_NOTRESOLVED;
#[doc = "`read()` method returns [events_notresolved::R](events_notresolved::R) reader structure"]
impl crate::Readable for EVENTS_NOTRESOLVED {}
#[doc = "`write(|w| ..)` method takes [events_notresolved::W](events_notresolved::W) writer structure"]
impl crate::Writable for EVENTS_NOTRESOLVED {}
#[doc = "Address not resolved."]
pub mod events_notresolved;
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
#[doc = "Resolution status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Resolution status."]
pub mod status;
#[doc = "Enable AAR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable AAR."]
pub mod enable;
#[doc = "Number of Identity root Keys in the IRK data structure.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [nirk](nirk) module"]
pub type NIRK = crate::Reg<u32, _NIRK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NIRK;
#[doc = "`read()` method returns [nirk::R](nirk::R) reader structure"]
impl crate::Readable for NIRK {}
#[doc = "`write(|w| ..)` method takes [nirk::W](nirk::W) writer structure"]
impl crate::Writable for NIRK {}
#[doc = "Number of Identity root Keys in the IRK data structure."]
pub mod nirk;
#[doc = "Pointer to the IRK data structure.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [irkptr](irkptr) module"]
pub type IRKPTR = crate::Reg<u32, _IRKPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IRKPTR;
#[doc = "`read()` method returns [irkptr::R](irkptr::R) reader structure"]
impl crate::Readable for IRKPTR {}
#[doc = "`write(|w| ..)` method takes [irkptr::W](irkptr::W) writer structure"]
impl crate::Writable for IRKPTR {}
#[doc = "Pointer to the IRK data structure."]
pub mod irkptr;
#[doc = "Pointer to the resolvable address (6 bytes).\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [addrptr](addrptr) module"]
pub type ADDRPTR = crate::Reg<u32, _ADDRPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRPTR;
#[doc = "`read()` method returns [addrptr::R](addrptr::R) reader structure"]
impl crate::Readable for ADDRPTR {}
#[doc = "`write(|w| ..)` method takes [addrptr::W](addrptr::W) writer structure"]
impl crate::Writable for ADDRPTR {}
#[doc = "Pointer to the resolvable address (6 bytes)."]
pub mod addrptr;
#[doc = "Pointer to a \"scratch\" data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scratchptr](scratchptr) module"]
pub type SCRATCHPTR = crate::Reg<u32, _SCRATCHPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCHPTR;
#[doc = "`read()` method returns [scratchptr::R](scratchptr::R) reader structure"]
impl crate::Readable for SCRATCHPTR {}
#[doc = "`write(|w| ..)` method takes [scratchptr::W](scratchptr::W) writer structure"]
impl crate::Writable for SCRATCHPTR {}
#[doc = "Pointer to a \"scratch\" data area used for temporary storage during resolution. A minimum of 3 bytes must be reserved."]
pub mod scratchptr;
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
