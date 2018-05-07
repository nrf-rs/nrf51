#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 16usize],
    #[doc = "0x10 - Start SPI transaction."]
    pub tasks_start: TASKS_START,
    #[doc = "0x14 - Stop SPI transaction."]
    pub tasks_stop: TASKS_STOP,
    _reserved1: [u8; 4usize],
    #[doc = "0x1c - Suspend SPI transaction."]
    pub tasks_suspend: TASKS_SUSPEND,
    #[doc = "0x20 - Resume SPI transaction."]
    pub tasks_resume: TASKS_RESUME,
    _reserved2: [u8; 224usize],
    #[doc = "0x104 - SPI transaction has stopped."]
    pub events_stopped: EVENTS_STOPPED,
    _reserved3: [u8; 8usize],
    #[doc = "0x110 - End of RXD buffer reached."]
    pub events_endrx: EVENTS_ENDRX,
    _reserved4: [u8; 12usize],
    #[doc = "0x120 - End of TXD buffer reached."]
    pub events_endtx: EVENTS_ENDTX,
    _reserved5: [u8; 40usize],
    #[doc = "0x14c - Transaction started."]
    pub events_started: EVENTS_STARTED,
    _reserved6: [u8; 436usize],
    #[doc = "0x304 - Interrupt enable set register."]
    pub intenset: INTENSET,
    #[doc = "0x308 - Interrupt enable clear register."]
    pub intenclr: INTENCLR,
    _reserved7: [u8; 500usize],
    #[doc = "0x500 - Enable SPIM."]
    pub enable: ENABLE,
    _reserved8: [u8; 4usize],
    #[doc = "0x508 - Pin select configuration."]
    pub psel: PSEL,
    _reserved9: [u8; 16usize],
    #[doc = "0x524 - SPI frequency."]
    pub frequency: FREQUENCY,
    _reserved10: [u8; 12usize],
    #[doc = "0x534 - RXD EasyDMA configuration and status."]
    pub rxd: RXD,
    _reserved11: [u8; 4usize],
    #[doc = "0x544 - TXD EasyDMA configuration and status."]
    pub txd: TXD,
    _reserved12: [u8; 4usize],
    #[doc = "0x554 - Configuration register."]
    pub config: CONFIG,
    _reserved13: [u8; 104usize],
    #[doc = "0x5c0 - Over-read character."]
    pub orc: ORC,
    _reserved14: [u8; 2616usize],
    #[doc = "0xffc - Peripheral power control."]
    pub power: POWER,
}
#[doc = r" Register block"]
#[repr(C)]
pub struct PSEL {
    #[doc = "0x00 - Pin select for SCK."]
    pub sck: self::psel::SCK,
    #[doc = "0x04 - Pin select for MOSI."]
    pub mosi: self::psel::MOSI,
    #[doc = "0x08 - Pin select for MISO."]
    pub miso: self::psel::MISO,
}
#[doc = r" Register block"]
#[doc = "Pin select configuration."]
pub mod psel;
#[doc = r" Register block"]
#[repr(C)]
pub struct RXD {
    #[doc = "0x00 - Data pointer."]
    pub ptr: self::rxd::PTR,
    #[doc = "0x04 - Maximum number of buffer bytes to receive."]
    pub maxcnt: self::rxd::MAXCNT,
    #[doc = "0x08 - Number of bytes received in the last transaction."]
    pub amount: self::rxd::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "RXD EasyDMA configuration and status."]
pub mod rxd;
#[doc = r" Register block"]
#[repr(C)]
pub struct TXD {
    #[doc = "0x00 - Data pointer."]
    pub ptr: self::txd::PTR,
    #[doc = "0x04 - Maximum number of buffer bytes to send."]
    pub maxcnt: self::txd::MAXCNT,
    #[doc = "0x08 - Number of bytes sent in the last transaction."]
    pub amount: self::txd::AMOUNT,
}
#[doc = r" Register block"]
#[doc = "TXD EasyDMA configuration and status."]
pub mod txd;
#[doc = "Start SPI transaction."]
pub struct TASKS_START {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start SPI transaction."]
pub mod tasks_start;
#[doc = "Stop SPI transaction."]
pub struct TASKS_STOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stop SPI transaction."]
pub mod tasks_stop;
#[doc = "Suspend SPI transaction."]
pub struct TASKS_SUSPEND {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Suspend SPI transaction."]
pub mod tasks_suspend;
#[doc = "Resume SPI transaction."]
pub struct TASKS_RESUME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Resume SPI transaction."]
pub mod tasks_resume;
#[doc = "SPI transaction has stopped."]
pub struct EVENTS_STOPPED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI transaction has stopped."]
pub mod events_stopped;
#[doc = "End of RXD buffer reached."]
pub struct EVENTS_ENDRX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of RXD buffer reached."]
pub mod events_endrx;
#[doc = "End of TXD buffer reached."]
pub struct EVENTS_ENDTX {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "End of TXD buffer reached."]
pub mod events_endtx;
#[doc = "Transaction started."]
pub struct EVENTS_STARTED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transaction started."]
pub mod events_started;
#[doc = "Interrupt enable set register."]
pub struct INTENSET {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable set register."]
pub mod intenset;
#[doc = "Interrupt enable clear register."]
pub struct INTENCLR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt enable clear register."]
pub mod intenclr;
#[doc = "Enable SPIM."]
pub struct ENABLE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Enable SPIM."]
pub mod enable;
#[doc = "SPI frequency."]
pub struct FREQUENCY {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "SPI frequency."]
pub mod frequency;
#[doc = "Configuration register."]
pub struct CONFIG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Configuration register."]
pub mod config;
#[doc = "Over-read character."]
pub struct ORC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Over-read character."]
pub mod orc;
#[doc = "Peripheral power control."]
pub struct POWER {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Peripheral power control."]
pub mod power;
