#[doc = "Reader of register SIZERAMBLOCKS"]
pub type R = crate::R<u32, super::SIZERAMBLOCKS>;
#[doc = "Writer for register SIZERAMBLOCKS"]
pub type W = crate::W<u32, super::SIZERAMBLOCKS>;
#[doc = "Register SIZERAMBLOCKS `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::SIZERAMBLOCKS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
