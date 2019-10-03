#[doc = "Reader of register MISO"]
pub type R = crate::R<u32, super::MISO>;
#[doc = "Writer for register MISO"]
pub type W = crate::W<u32, super::MISO>;
#[doc = "Register MISO `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::MISO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
