#[doc = "Reader of register PSELRTS"]
pub type R = crate::R<u32, super::PSELRTS>;
#[doc = "Writer for register PSELRTS"]
pub type W = crate::W<u32, super::PSELRTS>;
#[doc = "Register PSELRTS `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELRTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
