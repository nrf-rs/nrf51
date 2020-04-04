#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start the comparator."]
    pub tasks_start: TASKS_START,
    #[doc = "0x04 - Stop the comparator."]
    pub tasks_stop: TASKS_STOP,
    #[doc = "0x08 - Sample comparator value."]
    pub tasks_sample: TASKS_SAMPLE,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - LPCOMP is ready and output is valid."]
    pub events_ready: EVENTS_READY,
    #[doc = "0x104 - Input voltage crossed the threshold going down."]
    pub events_down: EVENTS_DOWN,
    #[doc = "0x108 - Input voltage crossed the threshold going up."]
    pub events_up: EVENTS_UP,
    #[doc = "0x10c - Input voltage crossed the threshold in any direction."]
    pub events_cross: EVENTS_CROSS,
    _reserved7: [u8; 240usize],
    #[doc = "0x200 - Shortcuts for the LPCOMP."]
    pub shorts: SHORTS,
    _reserved8: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved10: [u8; 244usize],
    #[doc = "0x400 - Result of last compare."]
    pub result: RESULT,
    _reserved11: [u8; 252usize],
    #[doc = "0x500 - Enable the LPCOMP."]
    pub enable: ENABLE,
    #[doc = "0x504 - Input pin select."]
    pub psel: PSEL,
    #[doc = "0x508 - Reference select."]
    pub refsel: REFSEL,
    #[doc = "0x50c - External reference select."]
    pub extrefsel: EXTREFSEL,
    _reserved15: [u8; 16usize],
    #[doc = "0x520 - Analog detect configuration."]
    pub anadetect: ANADETECT,
    _reserved16: [u8; 2776usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start the comparator.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_start](tasks_start) module"]
pub type TASKS_START = crate::Reg<u32, _TASKS_START>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_START;
#[doc = "`write(|w| ..)` method takes [tasks_start::W](tasks_start::W) writer structure"]
impl crate::Writable for TASKS_START {}
#[doc = "Start the comparator."]
pub mod tasks_start;
#[doc = "Stop the comparator.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop the comparator."]
pub mod tasks_stop;
#[doc = "Sample comparator value.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_sample](tasks_sample) module"]
pub type TASKS_SAMPLE = crate::Reg<u32, _TASKS_SAMPLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SAMPLE;
#[doc = "`write(|w| ..)` method takes [tasks_sample::W](tasks_sample::W) writer structure"]
impl crate::Writable for TASKS_SAMPLE {}
#[doc = "Sample comparator value."]
pub mod tasks_sample;
#[doc = "LPCOMP is ready and output is valid.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_ready](events_ready) module"]
pub type EVENTS_READY = crate::Reg<u32, _EVENTS_READY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_READY;
#[doc = "`read()` method returns [events_ready::R](events_ready::R) reader structure"]
impl crate::Readable for EVENTS_READY {}
#[doc = "`write(|w| ..)` method takes [events_ready::W](events_ready::W) writer structure"]
impl crate::Writable for EVENTS_READY {}
#[doc = "LPCOMP is ready and output is valid."]
pub mod events_ready;
#[doc = "Input voltage crossed the threshold going down.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_down](events_down) module"]
pub type EVENTS_DOWN = crate::Reg<u32, _EVENTS_DOWN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_DOWN;
#[doc = "`read()` method returns [events_down::R](events_down::R) reader structure"]
impl crate::Readable for EVENTS_DOWN {}
#[doc = "`write(|w| ..)` method takes [events_down::W](events_down::W) writer structure"]
impl crate::Writable for EVENTS_DOWN {}
#[doc = "Input voltage crossed the threshold going down."]
pub mod events_down;
#[doc = "Input voltage crossed the threshold going up.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_up](events_up) module"]
pub type EVENTS_UP = crate::Reg<u32, _EVENTS_UP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_UP;
#[doc = "`read()` method returns [events_up::R](events_up::R) reader structure"]
impl crate::Readable for EVENTS_UP {}
#[doc = "`write(|w| ..)` method takes [events_up::W](events_up::W) writer structure"]
impl crate::Writable for EVENTS_UP {}
#[doc = "Input voltage crossed the threshold going up."]
pub mod events_up;
#[doc = "Input voltage crossed the threshold in any direction.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_cross](events_cross) module"]
pub type EVENTS_CROSS = crate::Reg<u32, _EVENTS_CROSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_CROSS;
#[doc = "`read()` method returns [events_cross::R](events_cross::R) reader structure"]
impl crate::Readable for EVENTS_CROSS {}
#[doc = "`write(|w| ..)` method takes [events_cross::W](events_cross::W) writer structure"]
impl crate::Writable for EVENTS_CROSS {}
#[doc = "Input voltage crossed the threshold in any direction."]
pub mod events_cross;
#[doc = "Shortcuts for the LPCOMP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts for the LPCOMP."]
pub mod shorts;
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
#[doc = "Result of last compare.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [result](result) module"]
pub type RESULT = crate::Reg<u32, _RESULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESULT;
#[doc = "`read()` method returns [result::R](result::R) reader structure"]
impl crate::Readable for RESULT {}
#[doc = "Result of last compare."]
pub mod result;
#[doc = "Enable the LPCOMP.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable the LPCOMP."]
pub mod enable;
#[doc = "Input pin select.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psel](psel) module"]
pub type PSEL = crate::Reg<u32, _PSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSEL;
#[doc = "`read()` method returns [psel::R](psel::R) reader structure"]
impl crate::Readable for PSEL {}
#[doc = "`write(|w| ..)` method takes [psel::W](psel::W) writer structure"]
impl crate::Writable for PSEL {}
#[doc = "Input pin select."]
pub mod psel;
#[doc = "Reference select.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [refsel](refsel) module"]
pub type REFSEL = crate::Reg<u32, _REFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REFSEL;
#[doc = "`read()` method returns [refsel::R](refsel::R) reader structure"]
impl crate::Readable for REFSEL {}
#[doc = "`write(|w| ..)` method takes [refsel::W](refsel::W) writer structure"]
impl crate::Writable for REFSEL {}
#[doc = "Reference select."]
pub mod refsel;
#[doc = "External reference select.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extrefsel](extrefsel) module"]
pub type EXTREFSEL = crate::Reg<u32, _EXTREFSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXTREFSEL;
#[doc = "`read()` method returns [extrefsel::R](extrefsel::R) reader structure"]
impl crate::Readable for EXTREFSEL {}
#[doc = "`write(|w| ..)` method takes [extrefsel::W](extrefsel::W) writer structure"]
impl crate::Writable for EXTREFSEL {}
#[doc = "External reference select."]
pub mod extrefsel;
#[doc = "Analog detect configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anadetect](anadetect) module"]
pub type ANADETECT = crate::Reg<u32, _ANADETECT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ANADETECT;
#[doc = "`read()` method returns [anadetect::R](anadetect::R) reader structure"]
impl crate::Readable for ANADETECT {}
#[doc = "`write(|w| ..)` method takes [anadetect::W](anadetect::W) writer structure"]
impl crate::Writable for ANADETECT {}
#[doc = "Analog detect configuration."]
pub mod anadetect;
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
