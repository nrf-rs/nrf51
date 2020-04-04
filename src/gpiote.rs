#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Tasks asssociated with GPIOTE channels."]
    pub tasks_out: [TASKS_OUT; 4],
    _reserved1: [u8; 240usize],
    #[doc = "0x100 - Tasks asssociated with GPIOTE channels."]
    pub events_in: [EVENTS_IN; 4],
    _reserved2: [u8; 108usize],
    #[doc = "0x17c - Event generated from multiple pins."]
    pub events_port: EVENTS_PORT,
    _reserved3: [u8; 388usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 516usize],
    #[doc = "0x510 - Channel configuration registers."]
    pub config: [CONFIG; 4],
    _reserved6: [u8; 2780usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Tasks asssociated with GPIOTE channels.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_out](tasks_out) module"]
pub type TASKS_OUT = crate::Reg<u32, _TASKS_OUT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_OUT;
#[doc = "`write(|w| ..)` method takes [tasks_out::W](tasks_out::W) writer structure"]
impl crate::Writable for TASKS_OUT {}
#[doc = "Tasks asssociated with GPIOTE channels."]
pub mod tasks_out;
#[doc = "Tasks asssociated with GPIOTE channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_in](events_in) module"]
pub type EVENTS_IN = crate::Reg<u32, _EVENTS_IN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_IN;
#[doc = "`read()` method returns [events_in::R](events_in::R) reader structure"]
impl crate::Readable for EVENTS_IN {}
#[doc = "`write(|w| ..)` method takes [events_in::W](events_in::W) writer structure"]
impl crate::Writable for EVENTS_IN {}
#[doc = "Tasks asssociated with GPIOTE channels."]
pub mod events_in;
#[doc = "Event generated from multiple pins.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_port](events_port) module"]
pub type EVENTS_PORT = crate::Reg<u32, _EVENTS_PORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_PORT;
#[doc = "`read()` method returns [events_port::R](events_port::R) reader structure"]
impl crate::Readable for EVENTS_PORT {}
#[doc = "`write(|w| ..)` method takes [events_port::W](events_port::W) writer structure"]
impl crate::Writable for EVENTS_PORT {}
#[doc = "Event generated from multiple pins."]
pub mod events_port;
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
#[doc = "Channel configuration registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
pub type CONFIG = crate::Reg<u32, _CONFIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONFIG;
#[doc = "`read()` method returns [config::R](config::R) reader structure"]
impl crate::Readable for CONFIG {}
#[doc = "`write(|w| ..)` method takes [config::W](config::W) writer structure"]
impl crate::Writable for CONFIG {}
#[doc = "Channel configuration registers."]
pub mod config;
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
