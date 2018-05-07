use bare_metal::Nr;
#[cfg(feature = "rt")]
extern "C" {
    fn DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn DH_TRAMPOLINE() {
    DEFAULT_HANDLER();
}
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak POWER_CLOCK\nPOWER_CLOCK = DH_TRAMPOLINE\n.weak RADIO\nRADIO = DH_TRAMPOLINE\n.weak UART0\nUART0 = DH_TRAMPOLINE\n.weak SPI0_TWI0\nSPI0_TWI0 = DH_TRAMPOLINE\n.weak SPI1_TWI1\nSPI1_TWI1 = DH_TRAMPOLINE\n.weak GPIOTE\nGPIOTE = DH_TRAMPOLINE\n.weak ADC\nADC = DH_TRAMPOLINE\n.weak TIMER0\nTIMER0 = DH_TRAMPOLINE\n.weak TIMER1\nTIMER1 = DH_TRAMPOLINE\n.weak TIMER2\nTIMER2 = DH_TRAMPOLINE\n.weak RTC0\nRTC0 = DH_TRAMPOLINE\n.weak TEMP\nTEMP = DH_TRAMPOLINE\n.weak RNG\nRNG = DH_TRAMPOLINE\n.weak ECB\nECB = DH_TRAMPOLINE\n.weak CCM_AAR\nCCM_AAR = DH_TRAMPOLINE\n.weak WDT\nWDT = DH_TRAMPOLINE\n.weak RTC1\nRTC1 = DH_TRAMPOLINE\n.weak QDEC\nQDEC = DH_TRAMPOLINE\n.weak LPCOMP\nLPCOMP = DH_TRAMPOLINE\n.weak SWI0\nSWI0 = DH_TRAMPOLINE\n.weak SWI1\nSWI1 = DH_TRAMPOLINE\n.weak SWI2\nSWI2 = DH_TRAMPOLINE\n.weak SWI3\nSWI3 = DH_TRAMPOLINE\n.weak SWI4\nSWI4 = DH_TRAMPOLINE\n.weak SWI5\nSWI5 = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "C" {
    fn POWER_CLOCK();
    fn RADIO();
    fn UART0();
    fn SPI0_TWI0();
    fn SPI1_TWI1();
    fn GPIOTE();
    fn ADC();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn RTC0();
    fn TEMP();
    fn RNG();
    fn ECB();
    fn CCM_AAR();
    fn WDT();
    fn RTC1();
    fn QDEC();
    fn LPCOMP();
    fn SWI0();
    fn SWI1();
    fn SWI2();
    fn SWI3();
    fn SWI4();
    fn SWI5();
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 26] = [
    Some(POWER_CLOCK),
    Some(RADIO),
    Some(UART0),
    Some(SPI0_TWI0),
    Some(SPI1_TWI1),
    None,
    Some(GPIOTE),
    Some(ADC),
    Some(TIMER0),
    Some(TIMER1),
    Some(TIMER2),
    Some(RTC0),
    Some(TEMP),
    Some(RNG),
    Some(ECB),
    Some(CCM_AAR),
    Some(WDT),
    Some(RTC1),
    Some(QDEC),
    Some(LPCOMP),
    Some(SWI0),
    Some(SWI1),
    Some(SWI2),
    Some(SWI3),
    Some(SWI4),
    Some(SWI5),
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - POWER_CLOCK"]
    POWER_CLOCK,
    #[doc = "1 - RADIO"]
    RADIO,
    #[doc = "2 - UART0"]
    UART0,
    #[doc = "3 - SPI0_TWI0"]
    SPI0_TWI0,
    #[doc = "4 - SPI1_TWI1"]
    SPI1_TWI1,
    #[doc = "6 - GPIOTE"]
    GPIOTE,
    #[doc = "7 - ADC"]
    ADC,
    #[doc = "8 - TIMER0"]
    TIMER0,
    #[doc = "9 - TIMER1"]
    TIMER1,
    #[doc = "10 - TIMER2"]
    TIMER2,
    #[doc = "11 - RTC0"]
    RTC0,
    #[doc = "12 - TEMP"]
    TEMP,
    #[doc = "13 - RNG"]
    RNG,
    #[doc = "14 - ECB"]
    ECB,
    #[doc = "15 - CCM_AAR"]
    CCM_AAR,
    #[doc = "16 - WDT"]
    WDT,
    #[doc = "17 - RTC1"]
    RTC1,
    #[doc = "18 - QDEC"]
    QDEC,
    #[doc = "19 - LPCOMP"]
    LPCOMP,
    #[doc = "20 - SWI0"]
    SWI0,
    #[doc = "21 - SWI1"]
    SWI1,
    #[doc = "22 - SWI2"]
    SWI2,
    #[doc = "23 - SWI3"]
    SWI3,
    #[doc = "24 - SWI4"]
    SWI4,
    #[doc = "25 - SWI5"]
    SWI5,
}
unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::POWER_CLOCK => 0,
            Interrupt::RADIO => 1,
            Interrupt::UART0 => 2,
            Interrupt::SPI0_TWI0 => 3,
            Interrupt::SPI1_TWI1 => 4,
            Interrupt::GPIOTE => 6,
            Interrupt::ADC => 7,
            Interrupt::TIMER0 => 8,
            Interrupt::TIMER1 => 9,
            Interrupt::TIMER2 => 10,
            Interrupt::RTC0 => 11,
            Interrupt::TEMP => 12,
            Interrupt::RNG => 13,
            Interrupt::ECB => 14,
            Interrupt::CCM_AAR => 15,
            Interrupt::WDT => 16,
            Interrupt::RTC1 => 17,
            Interrupt::QDEC => 18,
            Interrupt::LPCOMP => 19,
            Interrupt::SWI0 => 20,
            Interrupt::SWI1 => 21,
            Interrupt::SWI2 => 22,
            Interrupt::SWI3 => 23,
            Interrupt::SWI4 => 24,
            Interrupt::SWI5 => 25,
        }
    }
}
use core::convert::TryFrom;
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl TryFrom<u8> for Interrupt {
    type Error = TryFromInterruptError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Interrupt::POWER_CLOCK),
            1 => Ok(Interrupt::RADIO),
            2 => Ok(Interrupt::UART0),
            3 => Ok(Interrupt::SPI0_TWI0),
            4 => Ok(Interrupt::SPI1_TWI1),
            6 => Ok(Interrupt::GPIOTE),
            7 => Ok(Interrupt::ADC),
            8 => Ok(Interrupt::TIMER0),
            9 => Ok(Interrupt::TIMER1),
            10 => Ok(Interrupt::TIMER2),
            11 => Ok(Interrupt::RTC0),
            12 => Ok(Interrupt::TEMP),
            13 => Ok(Interrupt::RNG),
            14 => Ok(Interrupt::ECB),
            15 => Ok(Interrupt::CCM_AAR),
            16 => Ok(Interrupt::WDT),
            17 => Ok(Interrupt::RTC1),
            18 => Ok(Interrupt::QDEC),
            19 => Ok(Interrupt::LPCOMP),
            20 => Ok(Interrupt::SWI0),
            21 => Ok(Interrupt::SWI1),
            22 => Ok(Interrupt::SWI2),
            23 => Ok(Interrupt::SWI3),
            24 => Ok(Interrupt::SWI4),
            25 => Ok(Interrupt::SWI5),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
