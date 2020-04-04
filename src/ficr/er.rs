#[doc = "Reader of register ER[%s]"]
pub type R = crate::R<u32, super::ER>;
#[doc = "Writer for register ER[%s]"]
pub type W = crate::W<u32, super::ER>;
#[doc = "Register ER[%s]
`reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::ER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
