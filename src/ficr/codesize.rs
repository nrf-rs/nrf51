#[doc = "Reader of register CODESIZE"]
pub type R = crate::R<u32, super::CODESIZE>;
#[doc = "Writer for register CODESIZE"]
pub type W = crate::W<u32, super::CODESIZE>;
#[doc = "Register CODESIZE `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CODESIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
