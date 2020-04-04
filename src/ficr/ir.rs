#[doc = "Reader of register IR[%s]"]
pub type R = crate::R<u32, super::IR>;
#[doc = "Writer for register IR[%s]"]
pub type W = crate::W<u32, super::IR>;
#[doc = "Register IR[%s]
`reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
