#[doc = "Reader of register CLENR0"]
pub type R = crate::R<u32, super::CLENR0>;
#[doc = "Writer for register CLENR0"]
pub type W = crate::W<u32, super::CLENR0>;
#[doc = "Register CLENR0 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CLENR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
