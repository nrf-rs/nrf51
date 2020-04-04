#[doc = "Reader of register XTALFREQ"]
pub type R = crate::R<u32, super::XTALFREQ>;
#[doc = "Writer for register XTALFREQ"]
pub type W = crate::W<u32, super::XTALFREQ>;
#[doc = "Register XTALFREQ `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::XTALFREQ {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reset value for CLOCK XTALFREQ register.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XTALFREQ_A {
    #[doc = "255: 16MHz Xtal is used."]
    _16MHZ = 255,
    #[doc = "0: 32MHz Xtal is used."]
    _32MHZ = 0,
}
impl From<XTALFREQ_A> for u8 {
    #[inline(always)]
    fn from(variant: XTALFREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `XTALFREQ`"]
pub type XTALFREQ_R = crate::R<u8, XTALFREQ_A>;
impl XTALFREQ_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, XTALFREQ_A> {
        use crate::Variant::*;
        match self.bits {
            255 => Val(XTALFREQ_A::_16MHZ),
            0 => Val(XTALFREQ_A::_32MHZ),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_16MHZ`"]
    #[inline(always)]
    pub fn is_16mhz(&self) -> bool {
        *self == XTALFREQ_A::_16MHZ
    }
    #[doc = "Checks if the value of the field is `_32MHZ`"]
    #[inline(always)]
    pub fn is_32mhz(&self) -> bool {
        *self == XTALFREQ_A::_32MHZ
    }
}
#[doc = "Write proxy for field `XTALFREQ`"]
pub struct XTALFREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALFREQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XTALFREQ_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "16MHz Xtal is used."]
    #[inline(always)]
    pub fn _16mhz(self) -> &'a mut W {
        self.variant(XTALFREQ_A::_16MHZ)
    }
    #[doc = "32MHz Xtal is used."]
    #[inline(always)]
    pub fn _32mhz(self) -> &'a mut W {
        self.variant(XTALFREQ_A::_32MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Reset value for CLOCK XTALFREQ register."]
    #[inline(always)]
    pub fn xtalfreq(&self) -> XTALFREQ_R {
        XTALFREQ_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reset value for CLOCK XTALFREQ register."]
    #[inline(always)]
    pub fn xtalfreq(&mut self) -> XTALFREQ_W {
        XTALFREQ_W { w: self }
    }
}
