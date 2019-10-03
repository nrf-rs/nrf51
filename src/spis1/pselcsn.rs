#[doc = "Reader of register PSELCSN"]
pub type R = crate::R<u32, super::PSELCSN>;
#[doc = "Writer for register PSELCSN"]
pub type W = crate::W<u32, super::PSELCSN>;
#[doc = "Register PSELCSN `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELCSN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
