#[doc = "Reader of register INTENCLR"]
pub type R = crate::R<u32, super::INTENCLR>;
#[doc = "Writer for register INTENCLR"]
pub type W = crate::W<u32, super::INTENCLR>;
#[doc = "Register INTENCLR `reset()`'s with value 0"]
impl crate::ResetValue for super::INTENCLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Disable interrupt on SAMPLERDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<SAMPLERDY_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAMPLERDY`"]
pub type SAMPLERDY_R = crate::R<bool, SAMPLERDY_A>;
impl SAMPLERDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLERDY_A {
        match self.bits {
            false => SAMPLERDY_A::DISABLED,
            true => SAMPLERDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAMPLERDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAMPLERDY_A::ENABLED
    }
}
#[doc = "Disable interrupt on SAMPLERDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<SAMPLERDY_AW> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `SAMPLERDY`"]
pub struct SAMPLERDY_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLERDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLERDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(SAMPLERDY_AW::CLEAR)
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
#[doc = "Disable interrupt on REPORTRDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<REPORTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REPORTRDY`"]
pub type REPORTRDY_R = crate::R<bool, REPORTRDY_A>;
impl REPORTRDY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTRDY_A {
        match self.bits {
            false => REPORTRDY_A::DISABLED,
            true => REPORTRDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPORTRDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPORTRDY_A::ENABLED
    }
}
#[doc = "Disable interrupt on REPORTRDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<REPORTRDY_AW> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `REPORTRDY`"]
pub struct REPORTRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> REPORTRDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPORTRDY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(REPORTRDY_AW::CLEAR)
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
#[doc = "Disable interrupt on ACCOF event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCOF_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ACCOF_A> for bool {
    #[inline(always)]
    fn from(variant: ACCOF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ACCOF`"]
pub type ACCOF_R = crate::R<bool, ACCOF_A>;
impl ACCOF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCOF_A {
        match self.bits {
            false => ACCOF_A::DISABLED,
            true => ACCOF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ACCOF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ACCOF_A::ENABLED
    }
}
#[doc = "Disable interrupt on ACCOF event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACCOF_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<ACCOF_AW> for bool {
    #[inline(always)]
    fn from(variant: ACCOF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `ACCOF`"]
pub struct ACCOF_W<'a> {
    w: &'a mut W,
}
impl<'a> ACCOF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACCOF_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ACCOF_AW::CLEAR)
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
    #[doc = "Bit 0 - Disable interrupt on SAMPLERDY event."]
    #[inline(always)]
    pub fn samplerdy(&self) -> SAMPLERDY_R {
        SAMPLERDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on REPORTRDY event."]
    #[inline(always)]
    pub fn reportrdy(&self) -> REPORTRDY_R {
        REPORTRDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable interrupt on ACCOF event."]
    #[inline(always)]
    pub fn accof(&self) -> ACCOF_R {
        ACCOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on SAMPLERDY event."]
    #[inline(always)]
    pub fn samplerdy(&mut self) -> SAMPLERDY_W {
        SAMPLERDY_W { w: self }
    }
    #[doc = "Bit 1 - Disable interrupt on REPORTRDY event."]
    #[inline(always)]
    pub fn reportrdy(&mut self) -> REPORTRDY_W {
        REPORTRDY_W { w: self }
    }
    #[doc = "Bit 2 - Disable interrupt on ACCOF event."]
    #[inline(always)]
    pub fn accof(&mut self) -> ACCOF_W {
        ACCOF_W { w: self }
    }
}
