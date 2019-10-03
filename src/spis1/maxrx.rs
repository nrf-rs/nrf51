#[doc = "Reader of register MAXRX"]
pub type R = crate::R<u32, super::MAXRX>;
#[doc = "Writer for register MAXRX"]
pub type W = crate::W<u32, super::MAXRX>;
#[doc = "Register MAXRX `reset()`'s with value 0"]
impl crate::ResetValue for super::MAXRX {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MAXRX`"]
pub type MAXRX_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MAXRX`"]
pub struct MAXRX_W<'a> {
    w: &'a mut W,
}
impl<'a> MAXRX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Maximum number of bytes in the receive buffer."]
    #[inline(always)]
    pub fn maxrx(&self) -> MAXRX_R {
        MAXRX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Maximum number of bytes in the receive buffer."]
    #[inline(always)]
    pub fn maxrx(&mut self) -> MAXRX_W {
        MAXRX_W { w: self }
    }
}
