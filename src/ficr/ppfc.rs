#[doc = "Reader of register PPFC"]
pub type R = crate::R<u32, super::PPFC>;
#[doc = "Writer for register PPFC"]
pub type W = crate::W<u32, super::PPFC>;
#[doc = "Register PPFC `reset()`'s with value 0"]
impl crate::ResetValue for super::PPFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pre-programmed factory code present.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PPFC_A {
    #[doc = "255: Not present."]
    NOTPRESENT,
    #[doc = "0: Present."]
    PRESENT,
}
impl From<PPFC_A> for u8 {
    #[inline(always)]
    fn from(variant: PPFC_A) -> Self {
        match variant {
            PPFC_A::NOTPRESENT => 255,
            PPFC_A::PRESENT => 0,
        }
    }
}
#[doc = "Reader of field `PPFC`"]
pub type PPFC_R = crate::R<u8, PPFC_A>;
impl PPFC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PPFC_A> {
        use crate::Variant::*;
        match self.bits {
            255 => Val(PPFC_A::NOTPRESENT),
            0 => Val(PPFC_A::PRESENT),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NOTPRESENT`"]
    #[inline(always)]
    pub fn is_not_present(&self) -> bool {
        *self == PPFC_A::NOTPRESENT
    }
    #[doc = "Checks if the value of the field is `PRESENT`"]
    #[inline(always)]
    pub fn is_present(&self) -> bool {
        *self == PPFC_A::PRESENT
    }
}
#[doc = "Write proxy for field `PPFC`"]
pub struct PPFC_W<'a> {
    w: &'a mut W,
}
impl<'a> PPFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPFC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Not present."]
    #[inline(always)]
    pub fn not_present(self) -> &'a mut W {
        self.variant(PPFC_A::NOTPRESENT)
    }
    #[doc = "Present."]
    #[inline(always)]
    pub fn present(self) -> &'a mut W {
        self.variant(PPFC_A::PRESENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Pre-programmed factory code present."]
    #[inline(always)]
    pub fn ppfc(&self) -> PPFC_R {
        PPFC_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Pre-programmed factory code present."]
    #[inline(always)]
    pub fn ppfc(&mut self) -> PPFC_W {
        PPFC_W { w: self }
    }
}
