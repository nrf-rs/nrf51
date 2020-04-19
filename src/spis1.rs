#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 36usize],
    #[doc = "0x24 - Acquire SPI semaphore."]
    pub tasks_acquire: TASKS_ACQUIRE,
    #[doc = "0x28 - Release SPI semaphore."]
    pub tasks_release: TASKS_RELEASE,
    _reserved2: [u8; 216usize],
    #[doc = "0x104 - Granted transaction completed."]
    pub events_end: EVENTS_END,
    _reserved3: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached"]
    pub events_endrx: EVENTS_ENDRX,
    _reserved4: [u8; 20usize],
    #[doc = "0x128 - Semaphore acquired."]
    pub events_acquired: EVENTS_ACQUIRED,
    _reserved5: [u8; 212usize],
    #[doc = "0x200 - Shortcuts for SPIS."]
    pub shorts: SHORTS,
    _reserved6: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved8: [u8; 244usize],
    #[doc = "0x400 - Semaphore status."]
    pub semstat: SEMSTAT,
    _reserved9: [u8; 60usize],
    #[doc = "0x440 - Status from last transaction."]
    pub status: STATUS,
    _reserved10: [u8; 188usize],
    #[doc = "0x500 - Enable SPIS."]
    pub enable: ENABLE,
    _reserved11: [u8; 4usize],
    #[doc = "0x508 - Pin select for SCK."]
    pub pselsck: PSELSCK,
    #[doc = "0x50c - Pin select for MISO."]
    pub pselmiso: PSELMISO,
    #[doc = "0x510 - Pin select for MOSI."]
    pub pselmosi: PSELMOSI,
    #[doc = "0x514 - Pin select for CSN."]
    pub pselcsn: PSELCSN,
    _reserved15: [u8; 28usize],
    #[doc = "0x534 - RX data pointer."]
    pub rxdptr: RXDPTR,
    #[doc = "0x538 - Maximum number of bytes in the receive buffer."]
    pub maxrx: MAXRX,
    #[doc = "0x53c - Number of bytes received in last granted transaction."]
    pub amountrx: AMOUNTRX,
    _reserved18: [u8; 4usize],
    #[doc = "0x544 - TX data pointer."]
    pub txdptr: TXDPTR,
    #[doc = "0x548 - Maximum number of bytes in the transmit buffer."]
    pub maxtx: MAXTX,
    #[doc = "0x54c - Number of bytes transmitted in last granted transaction."]
    pub amounttx: AMOUNTTX,
    _reserved21: [u8; 4usize],
    #[doc = "0x554 - Configuration register."]
    pub config: CONFIG,
    _reserved22: [u8; 4usize],
    #[doc = "0x55c - Default character."]
    pub def: DEF,
    _reserved23: [u8; 96usize],
    #[doc = "0x5c0 - Over-read character."]
    pub orc: ORC,
    _reserved24: [u8; 2616usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Acquire SPI semaphore.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_acquire](tasks_acquire) module"]
pub type TASKS_ACQUIRE = crate::Reg<u32, _TASKS_ACQUIRE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_ACQUIRE;
#[doc = "`write(|w| ..)` method takes [tasks_acquire::W](tasks_acquire::W) writer structure"]
impl crate::Writable for TASKS_ACQUIRE {}
#[doc = "Acquire SPI semaphore."]
pub mod tasks_acquire;
#[doc = "Release SPI semaphore.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tasks_release](tasks_release) module"]
pub type TASKS_RELEASE = crate::Reg<u32, _TASKS_RELEASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_RELEASE;
#[doc = "`write(|w| ..)` method takes [tasks_release::W](tasks_release::W) writer structure"]
impl crate::Writable for TASKS_RELEASE {}
#[doc = "Release SPI semaphore."]
pub mod tasks_release;
#[doc = "Granted transaction completed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_end](events_end) module"]
pub type EVENTS_END = crate::Reg<u32, _EVENTS_END>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_END;
#[doc = "`read()` method returns [events_end::R](events_end::R) reader structure"]
impl crate::Readable for EVENTS_END {}
#[doc = "`write(|w| ..)` method takes [events_end::W](events_end::W) writer structure"]
impl crate::Writable for EVENTS_END {}
#[doc = "Granted transaction completed."]
pub mod events_end;
#[doc = "End of RXD buffer reached\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_endrx](events_endrx) module"]
pub type EVENTS_ENDRX = crate::Reg<u32, _EVENTS_ENDRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDRX;
#[doc = "`read()` method returns [events_endrx::R](events_endrx::R) reader structure"]
impl crate::Readable for EVENTS_ENDRX {}
#[doc = "`write(|w| ..)` method takes [events_endrx::W](events_endrx::W) writer structure"]
impl crate::Writable for EVENTS_ENDRX {}
#[doc = "End of RXD buffer reached"]
pub mod events_endrx;
#[doc = "Semaphore acquired.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_acquired](events_acquired) module"]
pub type EVENTS_ACQUIRED = crate::Reg<u32, _EVENTS_ACQUIRED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ACQUIRED;
#[doc = "`read()` method returns [events_acquired::R](events_acquired::R) reader structure"]
impl crate::Readable for EVENTS_ACQUIRED {}
#[doc = "`write(|w| ..)` method takes [events_acquired::W](events_acquired::W) writer structure"]
impl crate::Writable for EVENTS_ACQUIRED {}
#[doc = "Semaphore acquired."]
pub mod events_acquired;
#[doc = "Shortcuts for SPIS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts for SPIS."]
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
#[doc = "Semaphore status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [semstat](semstat) module"]
pub type SEMSTAT = crate::Reg<u32, _SEMSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEMSTAT;
#[doc = "`read()` method returns [semstat::R](semstat::R) reader structure"]
impl crate::Readable for SEMSTAT {}
#[doc = "Semaphore status."]
pub mod semstat;
#[doc = "Status from last transaction.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status from last transaction."]
pub mod status;
#[doc = "Enable SPIS.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "Enable SPIS."]
pub mod enable;
#[doc = "Pin select for SCK.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselsck](pselsck) module"]
pub type PSELSCK = crate::Reg<u32, _PSELSCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELSCK;
#[doc = "`read()` method returns [pselsck::R](pselsck::R) reader structure"]
impl crate::Readable for PSELSCK {}
#[doc = "`write(|w| ..)` method takes [pselsck::W](pselsck::W) writer structure"]
impl crate::Writable for PSELSCK {}
#[doc = "Pin select for SCK."]
pub mod pselsck;
#[doc = "Pin select for MISO.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselmiso](pselmiso) module"]
pub type PSELMISO = crate::Reg<u32, _PSELMISO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELMISO;
#[doc = "`read()` method returns [pselmiso::R](pselmiso::R) reader structure"]
impl crate::Readable for PSELMISO {}
#[doc = "`write(|w| ..)` method takes [pselmiso::W](pselmiso::W) writer structure"]
impl crate::Writable for PSELMISO {}
#[doc = "Pin select for MISO."]
pub mod pselmiso;
#[doc = "Pin select for MOSI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselmosi](pselmosi) module"]
pub type PSELMOSI = crate::Reg<u32, _PSELMOSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELMOSI;
#[doc = "`read()` method returns [pselmosi::R](pselmosi::R) reader structure"]
impl crate::Readable for PSELMOSI {}
#[doc = "`write(|w| ..)` method takes [pselmosi::W](pselmosi::W) writer structure"]
impl crate::Writable for PSELMOSI {}
#[doc = "Pin select for MOSI."]
pub mod pselmosi;
#[doc = "Pin select for CSN.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pselcsn](pselcsn) module"]
pub type PSELCSN = crate::Reg<u32, _PSELCSN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSELCSN;
#[doc = "`read()` method returns [pselcsn::R](pselcsn::R) reader structure"]
impl crate::Readable for PSELCSN {}
#[doc = "`write(|w| ..)` method takes [pselcsn::W](pselcsn::W) writer structure"]
impl crate::Writable for PSELCSN {}
#[doc = "Pin select for CSN."]
pub mod pselcsn;
#[doc = "RX data pointer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdptr](rxdptr) module"]
pub type RXDPTR = crate::Reg<u32, _RXDPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXDPTR;
#[doc = "`read()` method returns [rxdptr::R](rxdptr::R) reader structure"]
impl crate::Readable for RXDPTR {}
#[doc = "`write(|w| ..)` method takes [rxdptr::W](rxdptr::W) writer structure"]
impl crate::Writable for RXDPTR {}
#[doc = "RX data pointer."]
pub mod rxdptr;
#[doc = "Maximum number of bytes in the receive buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxrx](maxrx) module"]
pub type MAXRX = crate::Reg<u32, _MAXRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXRX;
#[doc = "`read()` method returns [maxrx::R](maxrx::R) reader structure"]
impl crate::Readable for MAXRX {}
#[doc = "`write(|w| ..)` method takes [maxrx::W](maxrx::W) writer structure"]
impl crate::Writable for MAXRX {}
#[doc = "Maximum number of bytes in the receive buffer."]
pub mod maxrx;
#[doc = "Number of bytes received in last granted transaction.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amountrx](amountrx) module"]
pub type AMOUNTRX = crate::Reg<u32, _AMOUNTRX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMOUNTRX;
#[doc = "`read()` method returns [amountrx::R](amountrx::R) reader structure"]
impl crate::Readable for AMOUNTRX {}
#[doc = "Number of bytes received in last granted transaction."]
pub mod amountrx;
#[doc = "TX data pointer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txdptr](txdptr) module"]
pub type TXDPTR = crate::Reg<u32, _TXDPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXDPTR;
#[doc = "`read()` method returns [txdptr::R](txdptr::R) reader structure"]
impl crate::Readable for TXDPTR {}
#[doc = "`write(|w| ..)` method takes [txdptr::W](txdptr::W) writer structure"]
impl crate::Writable for TXDPTR {}
#[doc = "TX data pointer."]
pub mod txdptr;
#[doc = "Maximum number of bytes in the transmit buffer.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maxtx](maxtx) module"]
pub type MAXTX = crate::Reg<u32, _MAXTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXTX;
#[doc = "`read()` method returns [maxtx::R](maxtx::R) reader structure"]
impl crate::Readable for MAXTX {}
#[doc = "`write(|w| ..)` method takes [maxtx::W](maxtx::W) writer structure"]
impl crate::Writable for MAXTX {}
#[doc = "Maximum number of bytes in the transmit buffer."]
pub mod maxtx;
#[doc = "Number of bytes transmitted in last granted transaction.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amounttx](amounttx) module"]
pub type AMOUNTTX = crate::Reg<u32, _AMOUNTTX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AMOUNTTX;
#[doc = "`read()` method returns [amounttx::R](amounttx::R) reader structure"]
impl crate::Readable for AMOUNTTX {}
#[doc = "Number of bytes transmitted in last granted transaction."]
pub mod amounttx;
#[doc = "Configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config](config) module"]
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
#[doc = "Default character.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [def](def) module"]
pub type DEF = crate::Reg<u32, _DEF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEF;
#[doc = "`read()` method returns [def::R](def::R) reader structure"]
impl crate::Readable for DEF {}
#[doc = "`write(|w| ..)` method takes [def::W](def::W) writer structure"]
impl crate::Writable for DEF {}
#[doc = "Default character."]
pub mod def;
#[doc = "Over-read character.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [orc](orc) module"]
pub type ORC = crate::Reg<u32, _ORC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ORC;
#[doc = "`read()` method returns [orc::R](orc::R) reader structure"]
impl crate::Readable for ORC {}
#[doc = "`write(|w| ..)` method takes [orc::W](orc::W) writer structure"]
impl crate::Writable for ORC {}
#[doc = "Over-read character."]
pub mod orc;
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
