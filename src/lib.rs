#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]
#![doc = "Peripheral access API for NRF51 microcontrollers (generated using svd2rust v0.12.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.12.1/svd2rust/#peripheral-api"]
#![allow(private_no_mangle_statics)]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![feature(const_fn)]
#![feature(try_from)]
#![no_std]
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
#[cfg(feature = "rt")]
pub use cortex_m_rt::{default_handler, exception};
extern crate bare_metal;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r" Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
pub use interrupt::Interrupt;
#[doc(hidden)]
pub mod interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::CPUID;
pub use cortex_m::peripheral::DCB;
pub use cortex_m::peripheral::DWT;
pub use cortex_m::peripheral::MPU;
pub use cortex_m::peripheral::NVIC;
pub use cortex_m::peripheral::SCB;
pub use cortex_m::peripheral::SYST;
#[doc = "Power Control."]
pub struct POWER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for POWER {}
impl POWER {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const power::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for POWER {
    type Target = power::RegisterBlock;
    fn deref(&self) -> &power::RegisterBlock {
        unsafe { &*POWER::ptr() }
    }
}
#[doc = "Power Control."]
pub mod power;
#[doc = "Clock control."]
pub struct CLOCK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CLOCK {}
impl CLOCK {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const clock::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for CLOCK {
    type Target = clock::RegisterBlock;
    fn deref(&self) -> &clock::RegisterBlock {
        unsafe { &*CLOCK::ptr() }
    }
}
#[doc = "Clock control."]
pub mod clock;
#[doc = "AHB Multi-Layer Interface."]
pub struct AMLI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AMLI {}
impl AMLI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const amli::RegisterBlock {
        1073741824 as *const _
    }
}
impl Deref for AMLI {
    type Target = amli::RegisterBlock;
    fn deref(&self) -> &amli::RegisterBlock {
        unsafe { &*AMLI::ptr() }
    }
}
#[doc = "AHB Multi-Layer Interface."]
pub mod amli;
#[doc = "The radio."]
pub struct RADIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RADIO {}
impl RADIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const radio::RegisterBlock {
        1073745920 as *const _
    }
}
impl Deref for RADIO {
    type Target = radio::RegisterBlock;
    fn deref(&self) -> &radio::RegisterBlock {
        unsafe { &*RADIO::ptr() }
    }
}
#[doc = "The radio."]
pub mod radio;
#[doc = "Universal Asynchronous Receiver/Transmitter."]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uart0::RegisterBlock {
        1073750016 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &uart0::RegisterBlock {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter."]
pub mod uart0;
#[doc = "SPI master 0."]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "SPI master 0."]
pub mod spi0;
#[doc = "Two-wire interface master 0."]
pub struct TWI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI0 {}
impl TWI0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twi0::RegisterBlock {
        1073754112 as *const _
    }
}
impl Deref for TWI0 {
    type Target = twi0::RegisterBlock;
    fn deref(&self) -> &twi0::RegisterBlock {
        unsafe { &*TWI0::ptr() }
    }
}
#[doc = "Two-wire interface master 0."]
pub mod twi0;
#[doc = "SPI master 1."]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spi0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPI1 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &spi0::RegisterBlock {
        unsafe { &*SPI1::ptr() }
    }
}
#[doc = "Two-wire interface master 1."]
pub struct TWI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWI1 {}
impl TWI1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const twi0::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for TWI1 {
    type Target = twi0::RegisterBlock;
    fn deref(&self) -> &twi0::RegisterBlock {
        unsafe { &*TWI1::ptr() }
    }
}
#[doc = "SPI slave 1."]
pub struct SPIS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIS1 {}
impl SPIS1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spis1::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPIS1 {
    type Target = spis1::RegisterBlock;
    fn deref(&self) -> &spis1::RegisterBlock {
        unsafe { &*SPIS1::ptr() }
    }
}
#[doc = "SPI slave 1."]
pub mod spis1;
#[doc = "SPI master with easyDMA 1."]
pub struct SPIM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPIM1 {}
impl SPIM1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const spim1::RegisterBlock {
        1073758208 as *const _
    }
}
impl Deref for SPIM1 {
    type Target = spim1::RegisterBlock;
    fn deref(&self) -> &spim1::RegisterBlock {
        unsafe { &*SPIM1::ptr() }
    }
}
#[doc = "SPI master with easyDMA 1."]
pub mod spim1;
#[doc = "GPIO tasks and events."]
pub struct GPIOTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOTE {}
impl GPIOTE {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpiote::RegisterBlock {
        1073766400 as *const _
    }
}
impl Deref for GPIOTE {
    type Target = gpiote::RegisterBlock;
    fn deref(&self) -> &gpiote::RegisterBlock {
        unsafe { &*GPIOTE::ptr() }
    }
}
#[doc = "GPIO tasks and events."]
pub mod gpiote;
#[doc = "Analog to digital converter."]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const adc::RegisterBlock {
        1073770496 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    fn deref(&self) -> &adc::RegisterBlock {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog to digital converter."]
pub mod adc;
#[doc = "Timer 0."]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073774592 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer 0."]
pub mod timer0;
#[doc = "Timer 1."]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073778688 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "Timer 2."]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const timer0::RegisterBlock {
        1073782784 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    fn deref(&self) -> &timer0::RegisterBlock {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "Real time counter 0."]
pub struct RTC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC0 {}
impl RTC0 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0::RegisterBlock {
        1073786880 as *const _
    }
}
impl Deref for RTC0 {
    type Target = rtc0::RegisterBlock;
    fn deref(&self) -> &rtc0::RegisterBlock {
        unsafe { &*RTC0::ptr() }
    }
}
#[doc = "Real time counter 0."]
pub mod rtc0;
#[doc = "Temperature Sensor."]
pub struct TEMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TEMP {}
impl TEMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const temp::RegisterBlock {
        1073790976 as *const _
    }
}
impl Deref for TEMP {
    type Target = temp::RegisterBlock;
    fn deref(&self) -> &temp::RegisterBlock {
        unsafe { &*TEMP::ptr() }
    }
}
#[doc = "Temperature Sensor."]
pub mod temp;
#[doc = "Random Number Generator."]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rng::RegisterBlock {
        1073795072 as *const _
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    fn deref(&self) -> &rng::RegisterBlock {
        unsafe { &*RNG::ptr() }
    }
}
#[doc = "Random Number Generator."]
pub mod rng;
#[doc = "AES ECB Mode Encryption."]
pub struct ECB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ECB {}
impl ECB {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ecb::RegisterBlock {
        1073799168 as *const _
    }
}
impl Deref for ECB {
    type Target = ecb::RegisterBlock;
    fn deref(&self) -> &ecb::RegisterBlock {
        unsafe { &*ECB::ptr() }
    }
}
#[doc = "AES ECB Mode Encryption."]
pub mod ecb;
#[doc = "Accelerated Address Resolver."]
pub struct AAR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AAR {}
impl AAR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const aar::RegisterBlock {
        1073803264 as *const _
    }
}
impl Deref for AAR {
    type Target = aar::RegisterBlock;
    fn deref(&self) -> &aar::RegisterBlock {
        unsafe { &*AAR::ptr() }
    }
}
#[doc = "Accelerated Address Resolver."]
pub mod aar;
#[doc = "AES CCM Mode Encryption."]
pub struct CCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCM {}
impl CCM {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ccm::RegisterBlock {
        1073803264 as *const _
    }
}
impl Deref for CCM {
    type Target = ccm::RegisterBlock;
    fn deref(&self) -> &ccm::RegisterBlock {
        unsafe { &*CCM::ptr() }
    }
}
#[doc = "AES CCM Mode Encryption."]
pub mod ccm;
#[doc = "Watchdog Timer."]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const wdt::RegisterBlock {
        1073807360 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &wdt::RegisterBlock {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer."]
pub mod wdt;
#[doc = "Real time counter 1."]
pub struct RTC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC1 {}
impl RTC1 {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const rtc0::RegisterBlock {
        1073811456 as *const _
    }
}
impl Deref for RTC1 {
    type Target = rtc0::RegisterBlock;
    fn deref(&self) -> &rtc0::RegisterBlock {
        unsafe { &*RTC1::ptr() }
    }
}
#[doc = "Rotary decoder."]
pub struct QDEC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QDEC {}
impl QDEC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const qdec::RegisterBlock {
        1073815552 as *const _
    }
}
impl Deref for QDEC {
    type Target = qdec::RegisterBlock;
    fn deref(&self) -> &qdec::RegisterBlock {
        unsafe { &*QDEC::ptr() }
    }
}
#[doc = "Rotary decoder."]
pub mod qdec;
#[doc = "Low power comparator."]
pub struct LPCOMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPCOMP {}
impl LPCOMP {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const lpcomp::RegisterBlock {
        1073819648 as *const _
    }
}
impl Deref for LPCOMP {
    type Target = lpcomp::RegisterBlock;
    fn deref(&self) -> &lpcomp::RegisterBlock {
        unsafe { &*LPCOMP::ptr() }
    }
}
#[doc = "Low power comparator."]
pub mod lpcomp;
#[doc = "SW Interrupts."]
pub struct SWI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SWI {}
impl SWI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const swi::RegisterBlock {
        1073823744 as *const _
    }
}
impl Deref for SWI {
    type Target = swi::RegisterBlock;
    fn deref(&self) -> &swi::RegisterBlock {
        unsafe { &*SWI::ptr() }
    }
}
#[doc = "SW Interrupts."]
pub mod swi;
#[doc = "Non Volatile Memory Controller."]
pub struct NVMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for NVMC {}
impl NVMC {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const nvmc::RegisterBlock {
        1073864704 as *const _
    }
}
impl Deref for NVMC {
    type Target = nvmc::RegisterBlock;
    fn deref(&self) -> &nvmc::RegisterBlock {
        unsafe { &*NVMC::ptr() }
    }
}
#[doc = "Non Volatile Memory Controller."]
pub mod nvmc;
#[doc = "PPI controller."]
pub struct PPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PPI {}
impl PPI {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ppi::RegisterBlock {
        1073868800 as *const _
    }
}
impl Deref for PPI {
    type Target = ppi::RegisterBlock;
    fn deref(&self) -> &ppi::RegisterBlock {
        unsafe { &*PPI::ptr() }
    }
}
#[doc = "PPI controller."]
pub mod ppi;
#[doc = "Factory Information Configuration."]
pub struct FICR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FICR {}
impl FICR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const ficr::RegisterBlock {
        268435456 as *const _
    }
}
impl Deref for FICR {
    type Target = ficr::RegisterBlock;
    fn deref(&self) -> &ficr::RegisterBlock {
        unsafe { &*FICR::ptr() }
    }
}
#[doc = "Factory Information Configuration."]
pub mod ficr;
#[doc = "User Information Configuration."]
pub struct UICR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UICR {}
impl UICR {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const uicr::RegisterBlock {
        268439552 as *const _
    }
}
impl Deref for UICR {
    type Target = uicr::RegisterBlock;
    fn deref(&self) -> &uicr::RegisterBlock {
        unsafe { &*UICR::ptr() }
    }
}
#[doc = "User Information Configuration."]
pub mod uicr;
#[doc = "General purpose input and output."]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r" Returns a pointer to the register block"]
    pub fn ptr() -> *const gpio::RegisterBlock {
        1342177280 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    fn deref(&self) -> &gpio::RegisterBlock {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General purpose input and output."]
pub mod gpio;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "POWER"]
    pub POWER: POWER,
    #[doc = "CLOCK"]
    pub CLOCK: CLOCK,
    #[doc = "AMLI"]
    pub AMLI: AMLI,
    #[doc = "RADIO"]
    pub RADIO: RADIO,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "TWI0"]
    pub TWI0: TWI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "TWI1"]
    pub TWI1: TWI1,
    #[doc = "SPIS1"]
    pub SPIS1: SPIS1,
    #[doc = "SPIM1"]
    pub SPIM1: SPIM1,
    #[doc = "GPIOTE"]
    pub GPIOTE: GPIOTE,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "RTC0"]
    pub RTC0: RTC0,
    #[doc = "TEMP"]
    pub TEMP: TEMP,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "ECB"]
    pub ECB: ECB,
    #[doc = "AAR"]
    pub AAR: AAR,
    #[doc = "CCM"]
    pub CCM: CCM,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RTC1"]
    pub RTC1: RTC1,
    #[doc = "QDEC"]
    pub QDEC: QDEC,
    #[doc = "LPCOMP"]
    pub LPCOMP: LPCOMP,
    #[doc = "SWI"]
    pub SWI: SWI,
    #[doc = "NVMC"]
    pub NVMC: NVMC,
    #[doc = "PPI"]
    pub PPI: PPI,
    #[doc = "FICR"]
    pub FICR: FICR,
    #[doc = "UICR"]
    pub UICR: UICR,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        debug_assert!(!DEVICE_PERIPHERALS);
        DEVICE_PERIPHERALS = true;
        Peripherals {
            POWER: POWER {
                _marker: PhantomData,
            },
            CLOCK: CLOCK {
                _marker: PhantomData,
            },
            AMLI: AMLI {
                _marker: PhantomData,
            },
            RADIO: RADIO {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            TWI0: TWI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            TWI1: TWI1 {
                _marker: PhantomData,
            },
            SPIS1: SPIS1 {
                _marker: PhantomData,
            },
            SPIM1: SPIM1 {
                _marker: PhantomData,
            },
            GPIOTE: GPIOTE {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            RTC0: RTC0 {
                _marker: PhantomData,
            },
            TEMP: TEMP {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            ECB: ECB {
                _marker: PhantomData,
            },
            AAR: AAR {
                _marker: PhantomData,
            },
            CCM: CCM {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RTC1: RTC1 {
                _marker: PhantomData,
            },
            QDEC: QDEC {
                _marker: PhantomData,
            },
            LPCOMP: LPCOMP {
                _marker: PhantomData,
            },
            SWI: SWI {
                _marker: PhantomData,
            },
            NVMC: NVMC {
                _marker: PhantomData,
            },
            PPI: PPI {
                _marker: PhantomData,
            },
            FICR: FICR {
                _marker: PhantomData,
            },
            UICR: UICR {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
        }
    }
}
