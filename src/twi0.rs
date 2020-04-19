#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start 2-Wire master receive sequence."]
    pub tasks_startrx: TASKS_STARTRX,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - Start 2-Wire master transmit sequence."]
    pub tasks_starttx: TASKS_STARTTX,
    _reserved2: [u8; 8usize],
    #[doc = "0x14 - Stop 2-Wire transaction."]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 4usize],
    #[doc = "0x1c - Suspend 2-Wire transaction."]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume 2-Wire transaction."]
    pub tasks_resume: TASKS_RESUME,
    _reserved5: [u8; 224usize],
    #[doc = "0x104 - Two-wire stopped."]
    pub events_stopped: EVENTS_STOPPED,
    #[doc = "0x108 - Two-wire ready to deliver new RXD byte received."]
    pub events_rxdready: EVENTS_RXDREADY,
    _reserved7: [u8; 16usize],
    #[doc = "0x11c - Two-wire finished sending last TXD byte."]
    pub events_txdsent: EVENTS_TXDSENT,
    _reserved8: [u8; 4usize],
    #[doc = "0x124 - Two-wire error detected."]
    pub events_error: EVENTS_ERROR,
    _reserved9: [u8; 16usize],
    #[doc = "0x138 - Two-wire byte boundary."]
    pub events_bb: EVENTS_BB,
    _reserved10: [u8; 12usize],
    #[doc = "0x148 - Two-wire suspended."]
    pub events_suspended: EVENTS_SUSPENDED,
    _reserved11: [u8; 180usize],
    #[doc = "0x200 - Shortcuts for TWI."]
    pub shorts: SHORTS,
    _reserved12: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved14: [u8; 440usize],
    #[doc = "0x4c4 - Two-wire error source. Write error field to 1 to clear error."]
    pub errorsrc: ERRORSRC,
    _reserved15: [u8; 56usize],
    #[doc = "0x500 - Enable two-wire master."]
    pub enable: ENABLE,
    _reserved16: [u8; 4usize],
    #[doc = "0x508 - Pin select for SCL."]
    pub pselscl: PSELSCL,
    #[doc = "0x50c - Pin select for SDA."]
    pub pselsda: PSELSDA,
    _reserved18: [u8; 8usize],
    #[doc = "0x518 - RX data register."]
    pub rxd: RXD,
    #[doc = "0x51c - TX data register."]
    pub txd: TXD,
    _reserved20: [u8; 4usize],
    #[doc = "0x524 - Two-wire frequency."]
    pub frequency: FREQUENCY,
    _reserved21: [u8; 96usize],
    #[doc = "0x588 - Address used in the two-wire transfer."]
    pub address: ADDRESS,
    _reserved22: [u8; 2672usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start 2-Wire master receive sequence.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_startrx](tasks_startrx) module"]
pub type TASKS_STARTRX = crate::Reg<u32, _TASKS_STARTRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTRX;
#[doc = "`write(|w| ..)` method takes [tasks_startrx::W](tasks_startrx::W) writer structure"]
impl crate::Writable for TASKS_STARTRX {}
#[doc = "Start 2-Wire master receive sequence."]
pub mod tasks_startrx;
#[doc = "Start 2-Wire master transmit sequence.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_starttx](tasks_starttx) module"]
pub type TASKS_STARTTX = crate::Reg<u32, _TASKS_STARTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STARTTX;
#[doc = "`write(|w| ..)` method takes [tasks_starttx::W](tasks_starttx::W) writer structure"]
impl crate::Writable for TASKS_STARTTX {}
#[doc = "Start 2-Wire master transmit sequence."]
pub mod tasks_starttx;
#[doc = "Stop 2-Wire transaction.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop 2-Wire transaction."]
pub mod tasks_stop;
#[doc = "Suspend 2-Wire transaction.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_suspend](tasks_suspend) module"]
pub type TASKS_SUSPEND = crate::Reg<u32, _TASKS_SUSPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_SUSPEND;
#[doc = "`write(|w| ..)` method takes [tasks_suspend::W](tasks_suspend::W) writer structure"]
impl crate::Writable for TASKS_SUSPEND {}
#[doc = "Suspend 2-Wire transaction."]
pub mod tasks_suspend;
#[doc = "Resume 2-Wire transaction.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_resume](tasks_resume) module"]
pub type TASKS_RESUME = crate::Reg<u32, _TASKS_RESUME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RESUME;
#[doc = "`write(|w| ..)` method takes [tasks_resume::W](tasks_resume::W) writer structure"]
impl crate::Writable for TASKS_RESUME {}
#[doc = "Resume 2-Wire transaction."]
pub mod tasks_resume;
#[doc = "Two-wire stopped.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_stopped](events_stopped) module"]
pub type EVENTS_STOPPED = crate::Reg<u32, _EVENTS_STOPPED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_STOPPED;
#[doc = "`read()` method returns [events_stopped::R](events_stopped::R) reader structure"]
impl crate::Readable for EVENTS_STOPPED {}
#[doc = "`write(|w| ..)` method takes [events_stopped::W](events_stopped::W) writer structure"]
impl crate::Writable for EVENTS_STOPPED {}
#[doc = "Two-wire stopped."]
pub mod events_stopped;
#[doc = "Two-wire ready to deliver new RXD byte received.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxdready](events_rxdready) module"]
pub type EVENTS_RXDREADY = crate::Reg<u32, _EVENTS_RXDREADY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_RXDREADY;
#[doc = "`read()` method returns [events_rxdready::R](events_rxdready::R) reader structure"]
impl crate::Readable for EVENTS_RXDREADY {}
#[doc = "`write(|w| ..)` method takes [events_rxdready::W](events_rxdready::W) writer structure"]
impl crate::Writable for EVENTS_RXDREADY {}
#[doc = "Two-wire ready to deliver new RXD byte received."]
pub mod events_rxdready;
#[doc = "Two-wire finished sending last TXD byte.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_txdsent](events_txdsent) module"]
pub type EVENTS_TXDSENT = crate::Reg<u32, _EVENTS_TXDSENT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_TXDSENT;
#[doc = "`read()` method returns [events_txdsent::R](events_txdsent::R) reader structure"]
impl crate::Readable for EVENTS_TXDSENT {}
#[doc = "`write(|w| ..)` method takes [events_txdsent::W](events_txdsent::W) writer structure"]
impl crate::Writable for EVENTS_TXDSENT {}
#[doc = "Two-wire finished sending last TXD byte."]
pub mod events_txdsent;
#[doc = "Two-wire error detected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_error](events_error) module"]
pub type EVENTS_ERROR = crate::Reg<u32, _EVENTS_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERROR;
#[doc = "`read()` method returns [events_error::R](events_error::R) reader structure"]
impl crate::Readable for EVENTS_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_error::W](events_error::W) writer structure"]
impl crate::Writable for EVENTS_ERROR {}
#[doc = "Two-wire error detected."]
pub mod events_error;
#[doc = "Two-wire byte boundary.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_bb](events_bb) module"]
pub type EVENTS_BB = crate::Reg<u32, _EVENTS_BB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_BB;
#[doc = "`read()` method returns [events_bb::R](events_bb::R) reader structure"]
impl crate::Readable for EVENTS_BB {}
#[doc = "`write(|w| ..)` method takes [events_bb::W](events_bb::W) writer structure"]
impl crate::Writable for EVENTS_BB {}
#[doc = "Two-wire byte boundary."]
pub mod events_bb;
#[doc = "Two-wire suspended.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_suspended](events_suspended) module"]
pub type EVENTS_SUSPENDED = crate::Reg<u32, _EVENTS_SUSPENDED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_SUSPENDED;
#[doc = "`read()` method returns [events_suspended::R](events_suspended::R) reader structure"]
impl crate::Readable for EVENTS_SUSPENDED {}
#[doc = "`write(|w| ..)` method takes [events_suspended::W](events_suspended::W) writer structure"]
impl crate::Writable for EVENTS_SUSPENDED {}
#[doc = "Two-wire suspended."]
pub mod events_suspended;
#[doc = "Shortcuts for TWI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts for TWI."]
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
#[doc = "Two-wire error source. Write error field to 1 to clear error.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errorsrc](errorsrc) module"]
pub type ERRORSRC = crate::Reg<u32, _ERRORSRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ERRORSRC;
#[doc = "`read()` method returns [errorsrc::R](errorsrc::R) reader structure"]
impl crate::Readable for ERRORSRC {}
#[doc = "`write(|w| ..)` method takes [errorsrc::W](errorsrc::W) writer structure"]
impl crate::Writable for ERRORSRC {}
#[doc = "Two-wire error source. Write error field to 1 to clear error."]
pub mod errorsrc;
#[doc = "Enable two-wire master.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable two-wire master."]
pub mod enable;
#[doc = "Pin select for SCL.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselscl](pselscl) module"]
pub type PSELSCL = crate::Reg<u32, _PSELSCL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELSCL;
#[doc = "`read()` method returns [pselscl::R](pselscl::R) reader structure"]
impl crate::Readable for PSELSCL {}
#[doc = "`write(|w| ..)` method takes [pselscl::W](pselscl::W) writer structure"]
impl crate::Writable for PSELSCL {}
#[doc = "Pin select for SCL."]
pub mod pselscl;
#[doc = "Pin select for SDA.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselsda](pselsda) module"]
pub type PSELSDA = crate::Reg<u32, _PSELSDA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELSDA;
#[doc = "`read()` method returns [pselsda::R](pselsda::R) reader structure"]
impl crate::Readable for PSELSDA {}
#[doc = "`write(|w| ..)` method takes [pselsda::W](pselsda::W) writer structure"]
impl crate::Writable for PSELSDA {}
#[doc = "Pin select for SDA."]
pub mod pselsda;
#[doc = "RX data register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxd](rxd) module"]
pub type RXD = crate::Reg<u32, _RXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXD;
#[doc = "`read()` method returns [rxd::R](rxd::R) reader structure"]
impl crate::Readable for RXD {}
#[doc = "RX data register."]
pub mod rxd;
#[doc = "TX data register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txd](txd) module"]
pub type TXD = crate::Reg<u32, _TXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXD;
#[doc = "`read()` method returns [txd::R](txd::R) reader structure"]
impl crate::Readable for TXD {}
#[doc = "`write(|w| ..)` method takes [txd::W](txd::W) writer structure"]
impl crate::Writable for TXD {}
#[doc = "TX data register."]
pub mod txd;
#[doc = "Two-wire frequency.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frequency](frequency) module"]
pub type FREQUENCY = crate::Reg<u32, _FREQUENCY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREQUENCY;
#[doc = "`read()` method returns [frequency::R](frequency::R) reader structure"]
impl crate::Readable for FREQUENCY {}
#[doc = "`write(|w| ..)` method takes [frequency::W](frequency::W) writer structure"]
impl crate::Writable for FREQUENCY {}
#[doc = "Two-wire frequency."]
pub mod frequency;
#[doc = "Address used in the two-wire transfer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [address](address) module"]
pub type ADDRESS = crate::Reg<u32, _ADDRESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRESS;
#[doc = "`read()` method returns [address::R](address::R) reader structure"]
impl crate::Readable for ADDRESS {}
#[doc = "`write(|w| ..)` method takes [address::W](address::W) writer structure"]
impl crate::Writable for ADDRESS {}
#[doc = "Address used in the two-wire transfer."]
pub mod address;
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
