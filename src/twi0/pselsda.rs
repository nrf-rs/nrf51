#[doc = "Reader of register PSELSDA"]
pub type R = crate::R<u32, super::PSELSDA>;
#[doc = "Writer for register PSELSDA"]
pub type W = crate::W<u32, super::PSELSDA>;
#[doc = "Register PSELSDA `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELSDA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
