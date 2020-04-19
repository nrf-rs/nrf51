#[doc = "Reader of register INTENSET"]
pub type R = crate::R<u32, super::INTENSET>;
#[doc = "Writer for register INTENSET"]
pub type W = crate::W<u32, super::INTENSET>;
#[doc = "Register INTENSET `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENSET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable interrupt on VALRDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALRDY_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<VALRDY_A> for bool {
    #[inline(always)]
    fn from(variant: VALRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `VALRDY`"]
pub type VALRDY_R = crate::R<bool, VALRDY_A>;
impl VALRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VALRDY_A {
        match self.bits {
            false => VALRDY_A::DISABLED,
            true => VALRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VALRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VALRDY_A::ENABLED
    }
}
#[doc = "Enable interrupt on VALRDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VALRDY_AW {
    #[doc = "1: Enable interrupt on write."]
    SET = 1,
}
impl From<VALRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: VALRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `VALRDY`"]
pub struct VALRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> VALRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VALRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(VALRDY_AW::SET)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt on VALRDY event."]
    #[inline(always)]
    pub fn valrdy(&self) -> VALRDY_R {
        VALRDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt on VALRDY event."]
    #[inline(always)]
    pub fn valrdy(&mut self) -> VALRDY_W {
        VALRDY_W { w: self }
    }
}
