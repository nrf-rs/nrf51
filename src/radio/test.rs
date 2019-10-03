#[doc = "Reader of register TEST"]
pub type R = crate::R<u32, super::TEST>;
#[doc = "Writer for register TEST"]
pub type W = crate::W<u32, super::TEST>;
#[doc = "Register TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Constant carrier. Decision point: TXEN task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CONSTCARRIER_A {
    #[doc = "0: Constant carrier disabled."]
    DISABLED,
    #[doc = "1: Constant carrier enabled."]
    ENABLED,
}
impl From<CONSTCARRIER_A> for bool {
    #[inline(always)]
    fn from(variant: CONSTCARRIER_A) -> Self {
        match variant {
            CONSTCARRIER_A::DISABLED => false,
            CONSTCARRIER_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `CONSTCARRIER`"]
pub type CONSTCARRIER_R = crate::R<bool, CONSTCARRIER_A>;
impl CONSTCARRIER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONSTCARRIER_A {
        match self.bits {
            false => CONSTCARRIER_A::DISABLED,
            true => CONSTCARRIER_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CONSTCARRIER_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CONSTCARRIER_A::ENABLED
    }
}
#[doc = "Write proxy for field `CONSTCARRIER`"]
pub struct CONSTCARRIER_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSTCARRIER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CONSTCARRIER_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Constant carrier disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CONSTCARRIER_A::DISABLED)
    }
    #[doc = "Constant carrier enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CONSTCARRIER_A::ENABLED)
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
#[doc = "PLL lock. Decision point: TXEN or RXEN task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLLOCK_A {
    #[doc = "0: PLL lock disabled."]
    DISABLED,
    #[doc = "1: PLL lock enabled."]
    ENABLED,
}
impl From<PLLLOCK_A> for bool {
    #[inline(always)]
    fn from(variant: PLLLOCK_A) -> Self {
        match variant {
            PLLLOCK_A::DISABLED => false,
            PLLLOCK_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `PLLLOCK`"]
pub type PLLLOCK_R = crate::R<bool, PLLLOCK_A>;
impl PLLLOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLLOCK_A {
        match self.bits {
            false => PLLLOCK_A::DISABLED,
            true => PLLLOCK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLLLOCK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLLLOCK_A::ENABLED
    }
}
#[doc = "Write proxy for field `PLLLOCK`"]
pub struct PLLLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLLOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLLOCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "PLL lock disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLLLOCK_A::DISABLED)
    }
    #[doc = "PLL lock enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLLLOCK_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Constant carrier. Decision point: TXEN task."]
    #[inline(always)]
    pub fn constcarrier(&self) -> CONSTCARRIER_R {
        CONSTCARRIER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PLL lock. Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn plllock(&self) -> PLLLOCK_R {
        PLLLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Constant carrier. Decision point: TXEN task."]
    #[inline(always)]
    pub fn constcarrier(&mut self) -> CONSTCARRIER_W {
        CONSTCARRIER_W { w: self }
    }
    #[doc = "Bit 1 - PLL lock. Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn plllock(&mut self) -> PLLLOCK_W {
        PLLLOCK_W { w: self }
    }
}
