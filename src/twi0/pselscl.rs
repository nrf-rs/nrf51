#[doc = "Reader of register PSELSCL"]
pub type R = crate::R<u32, super::PSELSCL>;
#[doc = "Writer for register PSELSCL"]
pub type W = crate::W<u32, super::PSELSCL>;
#[doc = "Register PSELSCL `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELSCL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
