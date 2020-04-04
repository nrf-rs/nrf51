#[doc = "Reader of register SIZERAMBLOCK[%s]"]
pub type R = crate::R<u32, super::SIZERAMBLOCK>;
#[doc = "Writer for register SIZERAMBLOCK[%s]"]
pub type W = crate::W<u32, super::SIZERAMBLOCK>;
#[doc = "Register SIZERAMBLOCK[%s]
`reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::SIZERAMBLOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
