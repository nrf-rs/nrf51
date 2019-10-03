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
#[doc = "Enable interrupt on END event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED,
    #[doc = "1: Interrupt enabled."]
    ENABLED,
}
impl From<END_A> for bool {
    #[inline(always)]
    fn from(variant: END_A) -> Self {
        match variant {
            END_A::DISABLED => false,
            END_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `END`"]
pub type END_R = crate::R<bool, END_A>;
impl END_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> END_A {
        match self.bits {
            false => END_A::DISABLED,
            true => END_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == END_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == END_A::ENABLED
    }
}
#[doc = "Enable interrupt on END event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum END_AW {
    #[doc = "1: Enable interrupt on write."]
    SET,
}
impl From<END_AW> for bool {
    #[inline(always)]
    fn from(variant: END_AW) -> Self {
        match variant {
            END_AW::SET => true,
        }
    }
}
#[doc = "Write proxy for field `END`"]
pub struct END_W<'a> {
    w: &'a mut W,
}
impl<'a> END_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: END_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(END_AW::SET)
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
    #[doc = "Bit 0 - Enable interrupt on END event."]
    #[inline(always)]
    pub fn end(&self) -> END_R {
        END_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt on END event."]
    #[inline(always)]
    pub fn end(&mut self) -> END_W {
        END_W { w: self }
    }
}
