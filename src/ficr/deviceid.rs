#[doc = "Reader of register DEVICEID[%s]"]
pub type R = crate::R<u32, super::DEVICEID>;
#[doc = "Writer for register DEVICEID[%s]"]
pub type W = crate::W<u32, super::DEVICEID>;
#[doc = "Register DEVICEID[%s]
`reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::DEVICEID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
impl R {}
impl W {}
