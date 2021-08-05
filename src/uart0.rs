#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Start UART receiver."]
    pub tasks_startrx: crate::Reg<tasks_startrx::TASKS_STARTRX_SPEC>,
    #[doc = "0x04 - Stop UART receiver."]
    pub tasks_stoprx: crate::Reg<tasks_stoprx::TASKS_STOPRX_SPEC>,
    #[doc = "0x08 - Start UART transmitter."]
    pub tasks_starttx: crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>,
    #[doc = "0x0c - Stop UART transmitter."]
    pub tasks_stoptx: crate::Reg<tasks_stoptx::TASKS_STOPTX_SPEC>,
    _reserved4: [u8; 0x0c],
    #[doc = "0x1c - Suspend UART."]
    pub tasks_suspend: crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>,
    _reserved5: [u8; 0xe0],
    #[doc = "0x100 - CTS activated."]
    pub events_cts: crate::Reg<events_cts::EVENTS_CTS_SPEC>,
    #[doc = "0x104 - CTS deactivated."]
    pub events_ncts: crate::Reg<events_ncts::EVENTS_NCTS_SPEC>,
    #[doc = "0x108 - Data received in RXD."]
    pub events_rxdrdy: crate::Reg<events_rxdrdy::EVENTS_RXDRDY_SPEC>,
    _reserved8: [u8; 0x10],
    #[doc = "0x11c - Data sent from TXD."]
    pub events_txdrdy: crate::Reg<events_txdrdy::EVENTS_TXDRDY_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x124 - Error detected."]
    pub events_error: crate::Reg<events_error::EVENTS_ERROR_SPEC>,
    _reserved10: [u8; 0x1c],
    #[doc = "0x144 - Receiver timeout."]
    pub events_rxto: crate::Reg<events_rxto::EVENTS_RXTO_SPEC>,
    _reserved11: [u8; 0xb8],
    #[doc = "0x200 - Shortcuts for UART."]
    pub shorts: crate::Reg<shorts::SHORTS_SPEC>,
    _reserved12: [u8; 0x0100],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: crate::Reg<intenset::INTENSET_SPEC>,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: crate::Reg<intenclr::INTENCLR_SPEC>,
    _reserved14: [u8; 0x0174],
    #[doc = "0x480 - Error source. Write error field to 1 to clear error."]
    pub errorsrc: crate::Reg<errorsrc::ERRORSRC_SPEC>,
    _reserved15: [u8; 0x7c],
    #[doc = "0x500 - Enable UART and acquire IOs."]
    pub enable: crate::Reg<enable::ENABLE_SPEC>,
    _reserved16: [u8; 0x04],
    #[doc = "0x508 - Pin select for RTS."]
    pub pselrts: crate::Reg<pselrts::PSELRTS_SPEC>,
    #[doc = "0x50c - Pin select for TXD."]
    pub pseltxd: crate::Reg<pseltxd::PSELTXD_SPEC>,
    #[doc = "0x510 - Pin select for CTS."]
    pub pselcts: crate::Reg<pselcts::PSELCTS_SPEC>,
    #[doc = "0x514 - Pin select for RXD."]
    pub pselrxd: crate::Reg<pselrxd::PSELRXD_SPEC>,
    #[doc = "0x518 - RXD register. On read action the buffer pointer is displaced. Once read the character is consumed. If read when no character available, the UART will stop working."]
    pub rxd: crate::Reg<rxd::RXD_SPEC>,
    #[doc = "0x51c - TXD register."]
    pub txd: crate::Reg<txd::TXD_SPEC>,
    _reserved22: [u8; 0x04],
    #[doc = "0x524 - UART Baudrate."]
    pub baudrate: crate::Reg<baudrate::BAUDRATE_SPEC>,
    _reserved23: [u8; 0x44],
    #[doc = "0x56c - Configuration of parity and hardware flow control register."]
    pub config: crate::Reg<config::CONFIG_SPEC>,
    _reserved24: [u8; 0x0a8c],
    #[doc = "0xffc - Peripheral power control."]
    pub power: crate::Reg<power::POWER_SPEC>,
}
#[doc = "TASKS_STARTRX register accessor: an alias for `Reg<TASKS_STARTRX_SPEC>`"]
pub type TASKS_STARTRX = crate::Reg<tasks_startrx::TASKS_STARTRX_SPEC>;
#[doc = "Start UART receiver."]
pub mod tasks_startrx;
#[doc = "TASKS_STOPRX register accessor: an alias for `Reg<TASKS_STOPRX_SPEC>`"]
pub type TASKS_STOPRX = crate::Reg<tasks_stoprx::TASKS_STOPRX_SPEC>;
#[doc = "Stop UART receiver."]
pub mod tasks_stoprx;
#[doc = "TASKS_STARTTX register accessor: an alias for `Reg<TASKS_STARTTX_SPEC>`"]
pub type TASKS_STARTTX = crate::Reg<tasks_starttx::TASKS_STARTTX_SPEC>;
#[doc = "Start UART transmitter."]
pub mod tasks_starttx;
#[doc = "TASKS_STOPTX register accessor: an alias for `Reg<TASKS_STOPTX_SPEC>`"]
pub type TASKS_STOPTX = crate::Reg<tasks_stoptx::TASKS_STOPTX_SPEC>;
#[doc = "Stop UART transmitter."]
pub mod tasks_stoptx;
#[doc = "TASKS_SUSPEND register accessor: an alias for `Reg<TASKS_SUSPEND_SPEC>`"]
pub type TASKS_SUSPEND = crate::Reg<tasks_suspend::TASKS_SUSPEND_SPEC>;
#[doc = "Suspend UART."]
pub mod tasks_suspend;
#[doc = "EVENTS_CTS register accessor: an alias for `Reg<EVENTS_CTS_SPEC>`"]
pub type EVENTS_CTS = crate::Reg<events_cts::EVENTS_CTS_SPEC>;
#[doc = "CTS activated."]
pub mod events_cts;
#[doc = "EVENTS_NCTS register accessor: an alias for `Reg<EVENTS_NCTS_SPEC>`"]
pub type EVENTS_NCTS = crate::Reg<events_ncts::EVENTS_NCTS_SPEC>;
#[doc = "CTS deactivated."]
pub mod events_ncts;
#[doc = "EVENTS_RXDRDY register accessor: an alias for `Reg<EVENTS_RXDRDY_SPEC>`"]
pub type EVENTS_RXDRDY = crate::Reg<events_rxdrdy::EVENTS_RXDRDY_SPEC>;
#[doc = "Data received in RXD."]
pub mod events_rxdrdy;
#[doc = "EVENTS_TXDRDY register accessor: an alias for `Reg<EVENTS_TXDRDY_SPEC>`"]
pub type EVENTS_TXDRDY = crate::Reg<events_txdrdy::EVENTS_TXDRDY_SPEC>;
#[doc = "Data sent from TXD."]
pub mod events_txdrdy;
#[doc = "EVENTS_ERROR register accessor: an alias for `Reg<EVENTS_ERROR_SPEC>`"]
pub type EVENTS_ERROR = crate::Reg<events_error::EVENTS_ERROR_SPEC>;
#[doc = "Error detected."]
pub mod events_error;
#[doc = "EVENTS_RXTO register accessor: an alias for `Reg<EVENTS_RXTO_SPEC>`"]
pub type EVENTS_RXTO = crate::Reg<events_rxto::EVENTS_RXTO_SPEC>;
#[doc = "Receiver timeout."]
pub mod events_rxto;
#[doc = "SHORTS register accessor: an alias for `Reg<SHORTS_SPEC>`"]
pub type SHORTS = crate::Reg<shorts::SHORTS_SPEC>;
#[doc = "Shortcuts for UART."]
pub mod shorts;
#[doc = "INTENSET register accessor: an alias for `Reg<INTENSET_SPEC>`"]
pub type INTENSET = crate::Reg<intenset::INTENSET_SPEC>;
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "INTENCLR register accessor: an alias for `Reg<INTENCLR_SPEC>`"]
pub type INTENCLR = crate::Reg<intenclr::INTENCLR_SPEC>;
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "ERRORSRC register accessor: an alias for `Reg<ERRORSRC_SPEC>`"]
pub type ERRORSRC = crate::Reg<errorsrc::ERRORSRC_SPEC>;
#[doc = "Error source. Write error field to 1 to clear error."]
pub mod errorsrc;
#[doc = "ENABLE register accessor: an alias for `Reg<ENABLE_SPEC>`"]
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
#[doc = "Enable UART and acquire IOs."]
pub mod enable;
#[doc = "PSELRTS register accessor: an alias for `Reg<PSELRTS_SPEC>`"]
pub type PSELRTS = crate::Reg<pselrts::PSELRTS_SPEC>;
#[doc = "Pin select for RTS."]
pub mod pselrts;
#[doc = "PSELTXD register accessor: an alias for `Reg<PSELTXD_SPEC>`"]
pub type PSELTXD = crate::Reg<pseltxd::PSELTXD_SPEC>;
#[doc = "Pin select for TXD."]
pub mod pseltxd;
#[doc = "PSELCTS register accessor: an alias for `Reg<PSELCTS_SPEC>`"]
pub type PSELCTS = crate::Reg<pselcts::PSELCTS_SPEC>;
#[doc = "Pin select for CTS."]
pub mod pselcts;
#[doc = "PSELRXD register accessor: an alias for `Reg<PSELRXD_SPEC>`"]
pub type PSELRXD = crate::Reg<pselrxd::PSELRXD_SPEC>;
#[doc = "Pin select for RXD."]
pub mod pselrxd;
#[doc = "RXD register accessor: an alias for `Reg<RXD_SPEC>`"]
pub type RXD = crate::Reg<rxd::RXD_SPEC>;
#[doc = "RXD register. On read action the buffer pointer is displaced. Once read the character is consumed. If read when no character available, the UART will stop working."]
pub mod rxd;
#[doc = "TXD register accessor: an alias for `Reg<TXD_SPEC>`"]
pub type TXD = crate::Reg<txd::TXD_SPEC>;
#[doc = "TXD register."]
pub mod txd;
#[doc = "BAUDRATE register accessor: an alias for `Reg<BAUDRATE_SPEC>`"]
pub type BAUDRATE = crate::Reg<baudrate::BAUDRATE_SPEC>;
#[doc = "UART Baudrate."]
pub mod baudrate;
#[doc = "CONFIG register accessor: an alias for `Reg<CONFIG_SPEC>`"]
pub type CONFIG = crate::Reg<config::CONFIG_SPEC>;
#[doc = "Configuration of parity and hardware flow control register."]
pub mod config;
#[doc = "POWER register accessor: an alias for `Reg<POWER_SPEC>`"]
pub type POWER = crate::Reg<power::POWER_SPEC>;
#[doc = "Peripheral power control."]
pub mod power;
