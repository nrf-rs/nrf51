#[doc = "Reader of register CRV"]
pub type R = crate::R<u32, super::CRV>;
#[doc = "Writer for register CRV"]
pub type W = crate::W<u32, super::CRV>;
#[doc = "Register CRV `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CRV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
