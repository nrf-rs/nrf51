#[doc = "Reader of register MOSI"]
pub type R = crate::R<u32, super::MOSI>;
#[doc = "Writer for register MOSI"]
pub type W = crate::W<u32, super::MOSI>;
#[doc = "Register MOSI `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MOSI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
