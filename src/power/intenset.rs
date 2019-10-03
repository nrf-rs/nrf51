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
#[doc = "Enable interrupt on POFWARN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFWARN_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED,
    #[doc = "1: Interrupt enabled."]
    ENABLED,
}
impl From<POFWARN_A> for bool {
    #[inline(always)]
    fn from(variant: POFWARN_A) -> Self {
        match variant {
            POFWARN_A::DISABLED => false,
            POFWARN_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `POFWARN`"]
pub type POFWARN_R = crate::R<bool, POFWARN_A>;
impl POFWARN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POFWARN_A {
        match self.bits {
            false => POFWARN_A::DISABLED,
            true => POFWARN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == POFWARN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POFWARN_A::ENABLED
    }
}
#[doc = "Enable interrupt on POFWARN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POFWARN_AW {
    #[doc = "1: Enable interrupt on write."]
    SET,
}
impl From<POFWARN_AW> for bool {
    #[inline(always)]
    fn from(variant: POFWARN_AW) -> Self {
        match variant {
            POFWARN_AW::SET => true,
        }
    }
}
#[doc = "Write proxy for field `POFWARN`"]
pub struct POFWARN_W<'a> {
    w: &'a mut W,
}
impl<'a> POFWARN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POFWARN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(POFWARN_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - Enable interrupt on POFWARN event."]
    #[inline(always)]
    pub fn pofwarn(&self) -> POFWARN_R {
        POFWARN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Enable interrupt on POFWARN event."]
    #[inline(always)]
    pub fn pofwarn(&mut self) -> POFWARN_W {
        POFWARN_W { w: self }
    }
}
