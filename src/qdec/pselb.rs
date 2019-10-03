#[doc = "Reader of register PSELB"]
pub type R = crate::R<u32, super::PSELB>;
#[doc = "Writer for register PSELB"]
pub type W = crate::W<u32, super::PSELB>;
#[doc = "Register PSELB `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
