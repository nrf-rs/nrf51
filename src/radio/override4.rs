#[doc = "Reader of register OVERRIDE4"]
pub type R = crate::R<u32, super::OVERRIDE4>;
#[doc = "Writer for register OVERRIDE4"]
pub type W = crate::W<u32, super::OVERRIDE4>;
#[doc = "Register OVERRIDE4 `reset()`'s with value 0"]
impl crate::ResetValue for super::OVERRIDE4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVERRIDE4`"]
pub type OVERRIDE4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OVERRIDE4`"]
pub struct OVERRIDE4_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff_ffff) | ((value as u32) & 0x0fff_ffff);
        self.w
    }
}
#[doc = "Enable or disable override of default trim values.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Override trim values disabled."]
    DISABLED,
    #[doc = "1: Override trim values enabled."]
    ENABLED,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        match variant {
            ENABLE_A::DISABLED => false,
            ENABLE_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `ENABLE`"]
pub type ENABLE_R = crate::R<bool, ENABLE_A>;
impl ENABLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::DISABLED,
            true => ENABLE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE_A::ENABLED
    }
}
#[doc = "Write proxy for field `ENABLE`"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Override trim values disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ENABLE_A::DISABLED)
    }
    #[doc = "Override trim values enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:27 - Trim value override 4."]
    #[inline(always)]
    pub fn override4(&self) -> OVERRIDE4_R {
        OVERRIDE4_R::new((self.bits & 0x0fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Enable or disable override of default trim values."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:27 - Trim value override 4."]
    #[inline(always)]
    pub fn override4(&mut self) -> OVERRIDE4_W {
        OVERRIDE4_W { w: self }
    }
    #[doc = "Bit 31 - Enable or disable override of default trim values."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
}
