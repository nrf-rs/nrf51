#[doc = "Reader of register NUMRAMBLOCK"]
pub type R = crate::R<u32, super::NUMRAMBLOCK>;
#[doc = "Writer for register NUMRAMBLOCK"]
pub type W = crate::W<u32, super::NUMRAMBLOCK>;
#[doc = "Register NUMRAMBLOCK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::NUMRAMBLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
