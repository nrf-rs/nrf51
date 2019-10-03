#[doc = "Reader of register RESET"]
pub type R = crate::R<u32, super::RESET>;
#[doc = "Writer for register RESET"]
pub type W = crate::W<u32, super::RESET>;
#[doc = "Register RESET `reset()`'s with value 0"]
impl crate::ResetValue for super::RESET {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable or disable pin reset in debug interface mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_A {
    #[doc = "0: Pin reset in debug interface mode disabled."]
    DISABLED,
    #[doc = "1: Pin reset in debug interface mode enabled."]
    ENABLED,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        match variant {
            RESET_A::DISABLED => false,
            RESET_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `RESET`"]
pub type RESET_R = crate::R<bool, RESET_A>;
impl RESET_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::DISABLED,
            true => RESET_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RESET_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RESET_A::ENABLED
    }
}
#[doc = "Write proxy for field `RESET`"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Pin reset in debug interface mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RESET_A::DISABLED)
    }
    #[doc = "Pin reset in debug interface mode enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RESET_A::ENABLED)
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
    #[doc = "Bit 0 - Enable or disable pin reset in debug interface mode."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable pin reset in debug interface mode."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
}
