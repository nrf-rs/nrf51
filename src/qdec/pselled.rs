#[doc = "Reader of register PSELLED"]
pub type R = crate::R<u32, super::PSELLED>;
#[doc = "Writer for register PSELLED"]
pub type W = crate::W<u32, super::PSELLED>;
#[doc = "Register PSELLED `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELLED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
