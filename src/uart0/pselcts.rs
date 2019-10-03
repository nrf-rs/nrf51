#[doc = "Reader of register PSELCTS"]
pub type R = crate::R<u32, super::PSELCTS>;
#[doc = "Writer for register PSELCTS"]
pub type W = crate::W<u32, super::PSELCTS>;
#[doc = "Register PSELCTS `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::PSELCTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
