#[doc = "Reader of register CONFIGID"]
pub type R = crate::R<u32, super::CONFIGID>;
#[doc = "Writer for register CONFIGID"]
pub type W = crate::W<u32, super::CONFIGID>;
#[doc = "Register CONFIGID `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIGID {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HWID`"]
pub type HWID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HWID`"]
pub struct HWID_W<'a> {
    w: &'a mut W,
}
impl<'a> HWID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `FWID`"]
pub type FWID_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `FWID`"]
pub struct FWID_W<'a> {
    w: &'a mut W,
}
impl<'a> FWID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Hardware Identification Number."]
    #[inline(always)]
    pub fn hwid(&self) -> HWID_R {
        HWID_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Firmware Identification Number pre-loaded into the flash."]
    #[inline(always)]
    pub fn fwid(&self) -> FWID_R {
        FWID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Hardware Identification Number."]
    #[inline(always)]
    pub fn hwid(&mut self) -> HWID_W {
        HWID_W { w: self }
    }
    #[doc = "Bits 16:31 - Firmware Identification Number pre-loaded into the flash."]
    #[inline(always)]
    pub fn fwid(&mut self) -> FWID_W {
        FWID_W { w: self }
    }
}
