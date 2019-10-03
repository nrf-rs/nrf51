#[doc = "Reader of register PSELRXD"]
pub type R = crate::R<u32, super::PSELRXD>;
#[doc = "Writer for register PSELRXD"]
pub type W = crate::W<u32, super::PSELRXD>;
#[doc = "Register PSELRXD `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELRXD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
