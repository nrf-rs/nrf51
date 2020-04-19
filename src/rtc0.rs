#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start RTC Counter."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop RTC Counter."]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Clear RTC Counter."]
    pub tasks_clear: TASKS_CLEAR,
    #[doc = "0x0c - Set COUNTER to 0xFFFFFFF0."]
    pub tasks_trigovrflw: TASKS_TRIGOVRFLW,
    _reserved4: [u8; 240usize],
    #[doc = "0x100 - Event on COUNTER increment."]
    pub events_tick: EVENTS_TICK,
    #[doc = "0x104 - Event on COUNTER overflow."]
    pub events_ovrflw: EVENTS_OVRFLW,
    _reserved6: [u8; 56usize],
    #[doc = "0x140 - Compare event on CC\\[n\\]
match."]
    pub events_compare: [EVENTS_COMPARE; 4],
    _reserved7: [u8; 436usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 52usize],
    #[doc = "0x340 - Configures event enable routing to PPI for each RTC event."]
    pub evten: EVTEN,
    #[doc = "0x344 - Enable events routing to PPI. The reading of this register gives the value of EVTEN."]
    pub evtenset: EVTENSET,
    #[doc = "0x348 - Disable events routing to PPI. The reading of this register gives the value of EVTEN."]
    pub evtenclr: EVTENCLR,
    _reserved12: [u8; 440usize],
    #[doc = "0x504 - Current COUNTER value."]
    pub counter: COUNTER,
    #[doc = "0x508 - 12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed."]
    pub prescaler: PRESCALER,
    _reserved14: [u8; 52usize],
    #[doc = "0x540 - Capture/compare registers."]
    pub cc: [CC; 4],
    _reserved15: [u8; 2732usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start RTC Counter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start RTC Counter."]
pub mod tasks_start;
#[doc = "Stop RTC Counter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop RTC Counter."]
pub mod tasks_stop;
#[doc = "Clear RTC Counter.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_clear](tasks_clear) module"]
pub type TASKS_CLEAR = crate::Reg<u32, _TASKS_CLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CLEAR;
#[doc = "`write(|w| ..)` method takes [tasks_clear::W](tasks_clear::W) writer structure"]
impl crate::Writable for TASKS_CLEAR {}
#[doc = "Clear RTC Counter."]
pub mod tasks_clear;
#[doc = "Set COUNTER to 0xFFFFFFF0.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_trigovrflw](tasks_trigovrflw) module"]
pub type TASKS_TRIGOVRFLW = crate::Reg<u32, _TASKS_TRIGOVRFLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_TRIGOVRFLW;
#[doc = "`write(|w| ..)` method takes [tasks_trigovrflw::W](tasks_trigovrflw::W) writer structure"]
impl crate::Writable for TASKS_TRIGOVRFLW {}
#[doc = "Set COUNTER to 0xFFFFFFF0."]
pub mod tasks_trigovrflw;
#[doc = "Event on COUNTER increment.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_tick](events_tick) module"]
pub type EVENTS_TICK = crate::Reg<u32, _EVENTS_TICK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TICK;
#[doc = "`read()` method returns [events_tick::R](events_tick::R) reader structure"]
impl crate::Readable for EVENTS_TICK {}
#[doc = "`write(|w| ..)` method takes [events_tick::W](events_tick::W) writer structure"]
impl crate::Writable for EVENTS_TICK {}
#[doc = "Event on COUNTER increment."]
pub mod events_tick;
#[doc = "Event on COUNTER overflow.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ovrflw](events_ovrflw) module"]
pub type EVENTS_OVRFLW = crate::Reg<u32, _EVENTS_OVRFLW>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_OVRFLW;
#[doc = "`read()` method returns [events_ovrflw::R](events_ovrflw::R) reader structure"]
impl crate::Readable for EVENTS_OVRFLW {}
#[doc = "`write(|w| ..)` method takes [events_ovrflw::W](events_ovrflw::W) writer structure"]
impl crate::Writable for EVENTS_OVRFLW {}
#[doc = "Event on COUNTER overflow."]
pub mod events_ovrflw;
#[doc = "Compare event on CC\\[n\\]
match.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_compare](events_compare) module"]
pub type EVENTS_COMPARE = crate::Reg<u32, _EVENTS_COMPARE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_COMPARE;
#[doc = "`read()` method returns [events_compare::R](events_compare::R) reader structure"]
impl crate::Readable for EVENTS_COMPARE {}
#[doc = "`write(|w| ..)` method takes [events_compare::W](events_compare::W) writer structure"]
impl crate::Writable for EVENTS_COMPARE {}
#[doc = "Compare event on CC\\[n\\]
match."]
pub mod events_compare;
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
#[doc = "Configures event enable routing to PPI for each RTC event.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evten](evten) module"]
pub type EVTEN = crate::Reg<u32, _EVTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTEN;
#[doc = "`read()` method returns [evten::R](evten::R) reader structure"]
impl crate::Readable for EVTEN {}
#[doc = "`write(|w| ..)` method takes [evten::W](evten::W) writer structure"]
impl crate::Writable for EVTEN {}
#[doc = "Configures event enable routing to PPI for each RTC event."]
pub mod evten;
#[doc = "Enable events routing to PPI. The reading of this register gives the value of EVTEN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtenset](evtenset) module"]
pub type EVTENSET = crate::Reg<u32, _EVTENSET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTENSET;
#[doc = "`read()` method returns [evtenset::R](evtenset::R) reader structure"]
impl crate::Readable for EVTENSET {}
#[doc = "`write(|w| ..)` method takes [evtenset::W](evtenset::W) writer structure"]
impl crate::Writable for EVTENSET {}
#[doc = "Enable events routing to PPI. The reading of this register gives the value of EVTEN."]
pub mod evtenset;
#[doc = "Disable events routing to PPI. The reading of this register gives the value of EVTEN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evtenclr](evtenclr) module"]
pub type EVTENCLR = crate::Reg<u32, _EVTENCLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVTENCLR;
#[doc = "`read()` method returns [evtenclr::R](evtenclr::R) reader structure"]
impl crate::Readable for EVTENCLR {}
#[doc = "`write(|w| ..)` method takes [evtenclr::W](evtenclr::W) writer structure"]
impl crate::Writable for EVTENCLR {}
#[doc = "Disable events routing to PPI. The reading of this register gives the value of EVTEN."]
pub mod evtenclr;
#[doc = "Current COUNTER value.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [counter](counter) module"]
pub type COUNTER = crate::Reg<u32, _COUNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COUNTER;
#[doc = "`read()` method returns [counter::R](counter::R) reader structure"]
impl crate::Readable for COUNTER {}
#[doc = "Current COUNTER value."]
pub mod counter;
#[doc = "12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prescaler](prescaler) module"]
pub type PRESCALER = crate::Reg<u32, _PRESCALER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRESCALER;
#[doc = "`read()` method returns [prescaler::R](prescaler::R) reader structure"]
impl crate::Readable for PRESCALER {}
#[doc = "`write(|w| ..)` method takes [prescaler::W](prescaler::W) writer structure"]
impl crate::Writable for PRESCALER {}
#[doc = "12-bit prescaler for COUNTER frequency (32768/(PRESCALER+1)). Must be written when RTC is STOPed."]
pub mod prescaler;
#[doc = "Capture/compare registers.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc](cc) module"]
pub type CC = crate::Reg<u32, _CC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CC;
#[doc = "`read()` method returns [cc::R](cc::R) reader structure"]
impl crate::Readable for CC {}
#[doc = "`write(|w| ..)` method takes [cc::W](cc::W) writer structure"]
impl crate::Writable for CC {}
#[doc = "Capture/compare registers."]
pub mod cc;
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
