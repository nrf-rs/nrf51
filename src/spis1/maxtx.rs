#[doc = "Reader of register MAXTX"]
pub type R = crate::R<u32, super::MAXTX>;
#[doc = "Writer for register MAXTX"]
pub type W = crate::W<u32, super::MAXTX>;
#[doc = "Register MAXTX `reset()`'s with value 0"]
impl crate::ResetValue for super::MAXTX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXTX`"]
pub type MAXTX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXTX`"]
pub struct MAXTX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum number of bytes in the transmit buffer."]
    #[inline(always)]
    pub fn maxtx(&self) -> MAXTX_R {
        MAXTX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum number of bytes in the transmit buffer."]
    #[inline(always)]
    pub fn maxtx(&mut self) -> MAXTX_W {
        MAXTX_W { w: self }
    }
}
