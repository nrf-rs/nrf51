#[doc = "Reader of register LFCLKSRC"]
pub type R = crate::R<u32, super::LFCLKSRC>;
#[doc = "Writer for register LFCLKSRC"]
pub type W = crate::W<u32, super::LFCLKSRC>;
#[doc = "Register LFCLKSRC `reset()`'s with value 0"]
impl crate::ResetValue for super::LFCLKSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_A {
    #[doc = "0: Internal 32KiHz RC oscillator."]
    RC = 0,
    #[doc = "1: External 32KiHz crystal."]
    XTAL = 1,
    #[doc = "2: Internal 32KiHz synthesizer from HFCLK system clock."]
    SYNTH = 2,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::RC),
            1 => Val(SRC_A::XTAL),
            2 => Val(SRC_A::SYNTH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == SRC_A::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == SRC_A::XTAL
    }
    #[doc = "Checks if the value of the field is `SYNTH`"]
    #[inline(always)]
    pub fn is_synth(&self) -> bool {
        *self == SRC_A::SYNTH
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal 32KiHz RC oscillator."]
    #[inline(always)]
    pub fn rc(self) -> &'a mut W {
        self.variant(SRC_A::RC)
    }
    #[doc = "External 32KiHz crystal."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(SRC_A::XTAL)
    }
    #[doc = "Internal 32KiHz synthesizer from HFCLK system clock."]
    #[inline(always)]
    pub fn synth(self) -> &'a mut W {
        self.variant(SRC_A::SYNTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source."]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
}
