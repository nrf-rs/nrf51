#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start generation of key-stream. This operation will stop by itself when completed."]
    pub tasks_ksgen: TASKS_KSGEN,
    #[doc = "0x04 - Start encrypt/decrypt. This operation will stop by itself when completed."]
    pub tasks_crypt: TASKS_CRYPT,
    #[doc = "0x08 - Stop encrypt/decrypt."]
    pub tasks_stop: TASKS_STOP,
    _reserved3: [u8; 244usize],
    #[doc = "0x100 - Keystream generation completed."]
    pub events_endksgen: EVENTS_ENDKSGEN,
    #[doc = "0x104 - Encrypt/decrypt completed."]
    pub events_endcrypt: EVENTS_ENDCRYPT,
    #[doc = "0x108 - Error happened."]
    pub events_error: EVENTS_ERROR,
    _reserved6: [u8; 244usize],
    #[doc = "0x200 - Shortcuts for the CCM."]
    pub shorts: SHORTS,
    _reserved7: [u8; 256usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved9: [u8; 244usize],
    #[doc = "0x400 - CCM RX MIC check result."]
    pub micstatus: MICSTATUS,
    _reserved10: [u8; 252usize],
    #[doc = "0x500 - CCM enable."]
    pub enable: ENABLE,
    #[doc = "0x504 - Operation mode."]
    pub mode: MODE,
    #[doc = "0x508 - Pointer to a data structure holding AES key and NONCE vector."]
    pub cnfptr: CNFPTR,
    #[doc = "0x50c - Pointer to the input packet."]
    pub inptr: INPTR,
    #[doc = "0x510 - Pointer to the output packet."]
    pub outptr: OUTPTR,
    #[doc = "0x514 - Pointer to a \"scratch\" data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved."]
    pub scratchptr: SCRATCHPTR,
    _reserved16: [u8; 2788usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = "Start generation of key-stream. This operation will stop by itself when completed.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_ksgen](tasks_ksgen) module"]
pub type TASKS_KSGEN = crate::Reg<u32, _TASKS_KSGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_KSGEN;
#[doc = "`write(|w| ..)` method takes [tasks_ksgen::W](tasks_ksgen::W) writer structure"]
impl crate::Writable for TASKS_KSGEN {}
#[doc = "Start generation of key-stream. This operation will stop by itself when completed."]
pub mod tasks_ksgen;
#[doc = "Start encrypt/decrypt. This operation will stop by itself when completed.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_crypt](tasks_crypt) module"]
pub type TASKS_CRYPT = crate::Reg<u32, _TASKS_CRYPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_CRYPT;
#[doc = "`write(|w| ..)` method takes [tasks_crypt::W](tasks_crypt::W) writer structure"]
impl crate::Writable for TASKS_CRYPT {}
#[doc = "Start encrypt/decrypt. This operation will stop by itself when completed."]
pub mod tasks_crypt;
#[doc = "Stop encrypt/decrypt.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tasks_stop](tasks_stop) module"]
pub type TASKS_STOP = crate::Reg<u32, _TASKS_STOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TASKS_STOP;
#[doc = "`write(|w| ..)` method takes [tasks_stop::W](tasks_stop::W) writer structure"]
impl crate::Writable for TASKS_STOP {}
#[doc = "Stop encrypt/decrypt."]
pub mod tasks_stop;
#[doc = "Keystream generation completed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endksgen](events_endksgen) module"]
pub type EVENTS_ENDKSGEN = crate::Reg<u32, _EVENTS_ENDKSGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDKSGEN;
#[doc = "`read()` method returns [events_endksgen::R](events_endksgen::R) reader structure"]
impl crate::Readable for EVENTS_ENDKSGEN {}
#[doc = "`write(|w| ..)` method takes [events_endksgen::W](events_endksgen::W) writer structure"]
impl crate::Writable for EVENTS_ENDKSGEN {}
#[doc = "Keystream generation completed."]
pub mod events_endksgen;
#[doc = "Encrypt/decrypt completed.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_endcrypt](events_endcrypt) module"]
pub type EVENTS_ENDCRYPT = crate::Reg<u32, _EVENTS_ENDCRYPT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ENDCRYPT;
#[doc = "`read()` method returns [events_endcrypt::R](events_endcrypt::R) reader structure"]
impl crate::Readable for EVENTS_ENDCRYPT {}
#[doc = "`write(|w| ..)` method takes [events_endcrypt::W](events_endcrypt::W) writer structure"]
impl crate::Writable for EVENTS_ENDCRYPT {}
#[doc = "Encrypt/decrypt completed."]
pub mod events_endcrypt;
#[doc = "Error happened.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [events_error](events_error) module"]
pub type EVENTS_ERROR = crate::Reg<u32, _EVENTS_ERROR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EVENTS_ERROR;
#[doc = "`read()` method returns [events_error::R](events_error::R) reader structure"]
impl crate::Readable for EVENTS_ERROR {}
#[doc = "`write(|w| ..)` method takes [events_error::W](events_error::W) writer structure"]
impl crate::Writable for EVENTS_ERROR {}
#[doc = "Error happened."]
pub mod events_error;
#[doc = "Shortcuts for the CCM.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [shorts](shorts) module"]
pub type SHORTS = crate::Reg<u32, _SHORTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SHORTS;
#[doc = "`read()` method returns [shorts::R](shorts::R) reader structure"]
impl crate::Readable for SHORTS {}
#[doc = "`write(|w| ..)` method takes [shorts::W](shorts::W) writer structure"]
impl crate::Writable for SHORTS {}
#[doc = "Shortcuts for the CCM."]
pub mod shorts;
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
#[doc = "CCM RX MIC check result.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [micstatus](micstatus) module"]
pub type MICSTATUS = crate::Reg<u32, _MICSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MICSTATUS;
#[doc = "`read()` method returns [micstatus::R](micstatus::R) reader structure"]
impl crate::Readable for MICSTATUS {}
#[doc = "CCM RX MIC check result."]
pub mod micstatus;
#[doc = "CCM enable.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "CCM enable."]
pub mod enable;
#[doc = "Operation mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [mode](mode) module"]
pub type MODE = crate::Reg<u32, _MODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MODE;
#[doc = "`read()` method returns [mode::R](mode::R) reader structure"]
impl crate::Readable for MODE {}
#[doc = "`write(|w| ..)` method takes [mode::W](mode::W) writer structure"]
impl crate::Writable for MODE {}
#[doc = "Operation mode."]
pub mod mode;
#[doc = "Pointer to a data structure holding AES key and NONCE vector.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnfptr](cnfptr) module"]
pub type CNFPTR = crate::Reg<u32, _CNFPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNFPTR;
#[doc = "`read()` method returns [cnfptr::R](cnfptr::R) reader structure"]
impl crate::Readable for CNFPTR {}
#[doc = "`write(|w| ..)` method takes [cnfptr::W](cnfptr::W) writer structure"]
impl crate::Writable for CNFPTR {}
#[doc = "Pointer to a data structure holding AES key and NONCE vector."]
pub mod cnfptr;
#[doc = "Pointer to the input packet.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [inptr](inptr) module"]
pub type INPTR = crate::Reg<u32, _INPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INPTR;
#[doc = "`read()` method returns [inptr::R](inptr::R) reader structure"]
impl crate::Readable for INPTR {}
#[doc = "`write(|w| ..)` method takes [inptr::W](inptr::W) writer structure"]
impl crate::Writable for INPTR {}
#[doc = "Pointer to the input packet."]
pub mod inptr;
#[doc = "Pointer to the output packet.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [outptr](outptr) module"]
pub type OUTPTR = crate::Reg<u32, _OUTPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTPTR;
#[doc = "`read()` method returns [outptr::R](outptr::R) reader structure"]
impl crate::Readable for OUTPTR {}
#[doc = "`write(|w| ..)` method takes [outptr::W](outptr::W) writer structure"]
impl crate::Writable for OUTPTR {}
#[doc = "Pointer to the output packet."]
pub mod outptr;
#[doc = "Pointer to a \"scratch\" data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [scratchptr](scratchptr) module"]
pub type SCRATCHPTR = crate::Reg<u32, _SCRATCHPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCRATCHPTR;
#[doc = "`read()` method returns [scratchptr::R](scratchptr::R) reader structure"]
impl crate::Readable for SCRATCHPTR {}
#[doc = "`write(|w| ..)` method takes [scratchptr::W](scratchptr::W) writer structure"]
impl crate::Writable for SCRATCHPTR {}
#[doc = "Pointer to a \"scratch\" data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved."]
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
