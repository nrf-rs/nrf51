#[doc = "Reader of register PSELA"]
pub type R = crate::R<u32, super::PSELA>;
#[doc = "Writer for register PSELA"]
pub type W = crate::W<u32, super::PSELA>;
#[doc = "Register PSELA `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
