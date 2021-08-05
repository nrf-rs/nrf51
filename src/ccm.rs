#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start generation of key-stream. This operation will stop by itself when completed."]
    pub tasks_ksgen: crate::Reg<tasks_ksgen::TASKS_KSGEN_SPEC>,
    #[doc = "0x04 - Start encrypt/decrypt. This operation will stop by itself when completed."]
    pub tasks_crypt: crate::Reg<tasks_crypt::TASKS_CRYPT_SPEC>,
    #[doc = "0x08 - Stop encrypt/decrypt."]
    pub tasks_stop: crate::Reg<tasks_stop::TASKS_STOP_SPEC>,
    _reserved3: [u8; 0xf4],
    #[doc = "0x100 - Keystream generation completed."]
    pub events_endksgen: crate::Reg<events_endksgen::EVENTS_ENDKSGEN_SPEC>,
    #[doc = "0x104 - Encrypt/decrypt completed."]
    pub events_endcrypt: crate::Reg<events_endcrypt::EVENTS_ENDCRYPT_SPEC>,
    #[doc = "0x108 - Error happened."]
    pub events_error: crate::Reg<events_error::EVENTS_ERROR_SPEC>,
    _reserved6: [u8; 0xf4],
    #[doc = "0x200 - Shortcuts for the CCM."]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved7: [u8; 0x0100],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved9: [u8; 0xf4],
    #[doc = "0x400 - CCM RX MIC check result."]
    pub micstatus: crate::Reg<micstatus::MICSTATUS_SPEC>,
    _reserved10: [u8; 0xfc],
    #[doc = "0x500 - CCM enable."]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    #[doc = "0x504 - Operation mode."]
    pub mode: crate::Reg<mode::MODE_SPEC>,
    #[doc = "0x508 - Pointer to a data structure holding AES key and NONCE vector."]
    pub cnfptr: crate::Reg<cnfptr::CNFPTR_SPEC>,
    #[doc = "0x50c - Pointer to the input packet."]
    pub inptr: crate::Reg<inptr::INPTR_SPEC>,
    #[doc = "0x510 - Pointer to the output packet."]
    pub outptr: crate::Reg<outptr::OUTPTR_SPEC>,
    #[doc = "0x514 - Pointer to a scratch data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved."]
    pub scratchptr: crate::Reg<scratchptr::SCRATCHPTR_SPEC>,
    _reserved16: [u8; 0x0ae4],
    #[doc = "0xffc - Peripheral power control."]
    pub power: crate::Reg<power::POWER_SPEC>,
}
#[doc = "TASKS_KSGEN register accessor: an alias for `Reg<TASKS_KSGEN_SPEC>`"]
pub type TASKS_KSGEN = crate::Reg<tasks_ksgen::TASKS_KSGEN_SPEC>;
#[doc = "Start generation of key-stream. This operation will stop by itself when completed."]
pub mod tasks_ksgen;
#[doc = "TASKS_CRYPT register accessor: an alias for `Reg<TASKS_CRYPT_SPEC>`"]
pub type TASKS_CRYPT = crate::Reg<tasks_crypt::TASKS_CRYPT_SPEC>;
#[doc = "Start encrypt/decrypt. This operation will stop by itself when completed."]
pub mod tasks_crypt;
#[doc = "TASKS_STOP register accessor: an alias for `Reg<TASKS_STOP_SPEC>`"]
pub type TASKS_STOP = crate::Reg<tasks_stop::TASKS_STOP_SPEC>;
#[doc = "Stop encrypt/decrypt."]
pub mod tasks_stop;
#[doc = "EVENTS_ENDKSGEN register accessor: an alias for `Reg<EVENTS_ENDKSGEN_SPEC>`"]
pub type EVENTS_ENDKSGEN = crate::Reg<events_endksgen::EVENTS_ENDKSGEN_SPEC>;
#[doc = "Keystream generation completed."]
pub mod events_endksgen;
#[doc = "EVENTS_ENDCRYPT register accessor: an alias for `Reg<EVENTS_ENDCRYPT_SPEC>`"]
pub type EVENTS_ENDCRYPT = crate::Reg<events_endcrypt::EVENTS_ENDCRYPT_SPEC>;
#[doc = "Encrypt/decrypt completed."]
pub mod events_endcrypt;
#[doc = "EVENTS_ERROR register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "Error happened."]
pub mod events_error;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts for the CCM."]
pub mod shorts;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "MICSTATUS register accessor: an alias for `Reg<MICSTATUS_SPEC>`"]
pub type MICSTATUS = crate::Reg<micstatus::MICSTATUS_SPEC>;
#[doc = "CCM RX MIC check result."]
pub mod micstatus;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "CCM enable."]
pub mod enable;
#[doc = "MODE register accessor: an alias for `Reg<MODE_SPEC>`"]
pub type MODE = crate::Reg<mode::MODE_SPEC>;
#[doc = "Operation mode."]
pub mod mode;
#[doc = "CNFPTR register accessor: an alias for `Reg<CNFPTR_SPEC>`"]
pub type CNFPTR = crate::Reg<cnfptr::CNFPTR_SPEC>;
#[doc = "Pointer to a data structure holding AES key and NONCE vector."]
pub mod cnfptr;
#[doc = "INPTR register accessor: an alias for `Reg<INPTR_SPEC>`"]
pub type INPTR = crate::Reg<inptr::INPTR_SPEC>;
#[doc = "Pointer to the input packet."]
pub mod inptr;
#[doc = "OUTPTR register accessor: an alias for `Reg<OUTPTR_SPEC>`"]
pub type OUTPTR = crate::Reg<outptr::OUTPTR_SPEC>;
#[doc = "Pointer to the output packet."]
pub mod outptr;
#[doc = "SCRATCHPTR register accessor: an alias for `Reg<SCRATCHPTR_SPEC>`"]
pub type SCRATCHPTR = crate::Reg<scratchptr::SCRATCHPTR_SPEC>;
#[doc = "Pointer to a scratch data area used for temporary storage during resolution. A minimum of 43 bytes must be reserved."]
pub mod scratchptr;
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
