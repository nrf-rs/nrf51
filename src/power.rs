#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 120usize],
    #[doc = "0x78 - Enable constant latency mode."]
    pub tasks_constlat: TASKS_CONSTLAT,
    #[doc = "0x7c - Enable low power mode (variable latency)."]
    pub tasks_lowpwr: TASKS_LOWPWR,
    _reserved2: [u8; 136usize],
    #[doc = "0x108 - Power failure warning."]
    pub events_pofwarn: EVENTS_POFWARN,
    _reserved3: [u8; 504usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved5: [u8; 244usize],
    #[doc = "0x400 - Reset reason."]
    pub resetreas: RESETREAS,
    _reserved6: [u8; 36usize],
    #[doc = "0x428 - Ram status register."]
    pub ramstatus: RAMSTATUS,
    _reserved7: [u8; 212usize],
    #[doc = "0x500 - System off register."]
    pub systemoff: SYSTEMOFF,
    _reserved8: [u8; 12usize],
    #[doc = "0x510 - Power failure configuration."]
    pub pofcon: POFCON,
    _reserved9: [u8; 8usize],
    #[doc = "0x51c - General purpose retention register. This register is a retained register."]
    pub gpregret: GPREGRET,
    _reserved10: [u8; 4usize],
    #[doc = "0x524 - Ram on/off."]
    pub ramon: RAMON,
    _reserved11: [u8; 28usize],
    #[doc = "0x544 - Pin reset functionality configuration register. This register is a retained register."]
    pub reset: RESET,
    _reserved12: [u8; 12usize],
    #[doc = "0x554 - Ram on/off."]
    pub ramonb: RAMONB,
    _reserved13: [u8; 32usize],
    #[doc = "0x578 - DCDC converter enable configuration register."]
    pub dcdcen: DCDCEN,
    _reserved14: [u8; 1164usize],
    #[doc = "0xa08 - DCDC power-up force register."]
    pub dcdcforce: DCDCFORCE,
}
#[doc = "Enable constant latency mode.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_constlat](tasks_constlat) module"]
pub type TASKS_CONSTLAT = crate::Reg<u32, _TASKS_CONSTLAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CONSTLAT;
#[doc = "`write(|w| ..)` method takes [tasks_constlat::W](tasks_constlat::W) writer structure"]
impl crate::Writable for TASKS_CONSTLAT {}
#[doc = "Enable constant latency mode."]
pub mod tasks_constlat;
#[doc = "Enable low power mode (variable latency).\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_lowpwr](tasks_lowpwr) module"]
pub type TASKS_LOWPWR = crate::Reg<u32, _TASKS_LOWPWR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_LOWPWR;
#[doc = "`write(|w| ..)` method takes [tasks_lowpwr::W](tasks_lowpwr::W) writer structure"]
impl crate::Writable for TASKS_LOWPWR {}
#[doc = "Enable low power mode (variable latency)."]
pub mod tasks_lowpwr;
#[doc = "Power failure warning.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_pofwarn](events_pofwarn) module"]
pub type EVENTS_POFWARN = crate::Reg<u32, _EVENTS_POFWARN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_POFWARN;
#[doc = "`read()` method returns [events_pofwarn::R](events_pofwarn::R) reader structure"]
impl crate::Readable for EVENTS_POFWARN {}
#[doc = "`write(|w| ..)` method takes [events_pofwarn::W](events_pofwarn::W) writer structure"]
impl crate::Writable for EVENTS_POFWARN {}
#[doc = "Power failure warning."]
pub mod events_pofwarn;
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
#[doc = "Reset reason.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resetreas](resetreas) module"]
pub type RESETREAS = crate::Reg<u32, _RESETREAS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESETREAS;
#[doc = "`read()` method returns [resetreas::R](resetreas::R) reader structure"]
impl crate::Readable for RESETREAS {}
#[doc = "`write(|w| ..)` method takes [resetreas::W](resetreas::W) writer structure"]
impl crate::Writable for RESETREAS {}
#[doc = "Reset reason."]
pub mod resetreas;
#[doc = "Ram status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ramstatus](ramstatus) module"]
pub type RAMSTATUS = crate::Reg<u32, _RAMSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMSTATUS;
#[doc = "`read()` method returns [ramstatus::R](ramstatus::R) reader structure"]
impl crate::Readable for RAMSTATUS {}
#[doc = "Ram status register."]
pub mod ramstatus;
#[doc = "System off register.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [systemoff](systemoff) module"]
pub type SYSTEMOFF = crate::Reg<u32, _SYSTEMOFF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSTEMOFF;
#[doc = "`write(|w| ..)` method takes [systemoff::W](systemoff::W) writer structure"]
impl crate::Writable for SYSTEMOFF {}
#[doc = "System off register."]
pub mod systemoff;
#[doc = "Power failure configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pofcon](pofcon) module"]
pub type POFCON = crate::Reg<u32, _POFCON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POFCON;
#[doc = "`read()` method returns [pofcon::R](pofcon::R) reader structure"]
impl crate::Readable for POFCON {}
#[doc = "`write(|w| ..)` method takes [pofcon::W](pofcon::W) writer structure"]
impl crate::Writable for POFCON {}
#[doc = "Power failure configuration."]
pub mod pofcon;
#[doc = "General purpose retention register. This register is a retained register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [gpregret](gpregret) module"]
pub type GPREGRET = crate::Reg<u32, _GPREGRET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPREGRET;
#[doc = "`read()` method returns [gpregret::R](gpregret::R) reader structure"]
impl crate::Readable for GPREGRET {}
#[doc = "`write(|w| ..)` method takes [gpregret::W](gpregret::W) writer structure"]
impl crate::Writable for GPREGRET {}
#[doc = "General purpose retention register. This register is a retained register."]
pub mod gpregret;
#[doc = "Ram on/off.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ramon](ramon) module"]
pub type RAMON = crate::Reg<u32, _RAMON>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMON;
#[doc = "`read()` method returns [ramon::R](ramon::R) reader structure"]
impl crate::Readable for RAMON {}
#[doc = "`write(|w| ..)` method takes [ramon::W](ramon::W) writer structure"]
impl crate::Writable for RAMON {}
#[doc = "Ram on/off."]
pub mod ramon;
#[doc = "Pin reset functionality configuration register. This register is a retained register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [reset](reset) module"]
pub type RESET = crate::Reg<u32, _RESET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESET;
#[doc = "`read()` method returns [reset::R](reset::R) reader structure"]
impl crate::Readable for RESET {}
#[doc = "`write(|w| ..)` method takes [reset::W](reset::W) writer structure"]
impl crate::Writable for RESET {}
#[doc = "Pin reset functionality configuration register. This register is a retained register."]
pub mod reset;
#[doc = "Ram on/off.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ramonb](ramonb) module"]
pub type RAMONB = crate::Reg<u32, _RAMONB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RAMONB;
#[doc = "`read()` method returns [ramonb::R](ramonb::R) reader structure"]
impl crate::Readable for RAMONB {}
#[doc = "`write(|w| ..)` method takes [ramonb::W](ramonb::W) writer structure"]
impl crate::Writable for RAMONB {}
#[doc = "Ram on/off."]
pub mod ramonb;
#[doc = "DCDC converter enable configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcen](dcdcen) module"]
pub type DCDCEN = crate::Reg<u32, _DCDCEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCEN;
#[doc = "`read()` method returns [dcdcen::R](dcdcen::R) reader structure"]
impl crate::Readable for DCDCEN {}
#[doc = "`write(|w| ..)` method takes [dcdcen::W](dcdcen::W) writer structure"]
impl crate::Writable for DCDCEN {}
#[doc = "DCDC converter enable configuration register."]
pub mod dcdcen;
#[doc = "DCDC power-up force register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dcdcforce](dcdcforce) module"]
pub type DCDCFORCE = crate::Reg<u32, _DCDCFORCE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCDCFORCE;
#[doc = "`read()` method returns [dcdcforce::R](dcdcforce::R) reader structure"]
impl crate::Readable for DCDCFORCE {}
#[doc = "`write(|w| ..)` method takes [dcdcforce::W](dcdcforce::W) writer structure"]
impl crate::Writable for DCDCFORCE {}
#[doc = "DCDC power-up force register."]
pub mod dcdcforce;
