#[doc = "Reader of register BOOTLOADERADDR"]
pub type R = crate::R<u32, super::BOOTLOADERADDR>;
#[doc = "Writer for register BOOTLOADERADDR"]
pub type W = crate::W<u32, super::BOOTLOADERADDR>;
#[doc = "Register BOOTLOADERADDR `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::BOOTLOADERADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
