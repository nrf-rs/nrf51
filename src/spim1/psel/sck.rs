#[doc = "Reader of register SCK"]
pub type R = crate::R<u32, super::SCK>;
#[doc = "Writer for register SCK"]
pub type W = crate::W<u32, super::SCK>;
#[doc = "Register SCK `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::SCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
