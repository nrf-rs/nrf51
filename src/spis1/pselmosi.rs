#[doc = "Reader of register PSELMOSI"]
pub type R = crate::R<u32, super::PSELMOSI>;
#[doc = "Writer for register PSELMOSI"]
pub type W = crate::W<u32, super::PSELMOSI>;
#[doc = "Register PSELMOSI `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELMOSI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
