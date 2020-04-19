#[doc = "Reader of register CODEPAGESIZE"]
pub type R = crate::R<u32, super::CODEPAGESIZE>;
#[doc = "Writer for register CODEPAGESIZE"]
pub type W = crate::W<u32, super::CODEPAGESIZE>;
#[doc = "Register CODEPAGESIZE `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::CODEPAGESIZE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
