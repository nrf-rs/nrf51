#[doc = "Reader of register SHORTS"]
pub type R = crate::R<u32, super::SHORTS>;
#[doc = "Writer for register SHORTS"]
pub type W = crate::W<u32, super::SHORTS>;
#[doc = "Register SHORTS `reset()`'s with value 0"]
impl crate::ResetValue for super::SHORTS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Shortcut between REPORTRDY event and READCLRACC task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REPORTRDY_READCLRACC_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<REPORTRDY_READCLRACC_A> for bool {
    #[inline(always)]
    fn from(variant: REPORTRDY_READCLRACC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `REPORTRDY_READCLRACC`"]
pub type REPORTRDY_READCLRACC_R = crate::R<bool, REPORTRDY_READCLRACC_A>;
impl REPORTRDY_READCLRACC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REPORTRDY_READCLRACC_A {
        match self.bits {
            false => REPORTRDY_READCLRACC_A::DISABLED,
            true => REPORTRDY_READCLRACC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REPORTRDY_READCLRACC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REPORTRDY_READCLRACC_A::ENABLED
    }
}
#[doc = "Write proxy for field `REPORTRDY_READCLRACC`"]
pub struct REPORTRDY_READCLRACC_W<'a> {
    w: &'a mut W,
}
impl<'a> REPORTRDY_READCLRACC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REPORTRDY_READCLRACC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REPORTRDY_READCLRACC_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REPORTRDY_READCLRACC_A::ENABLED)
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
#[doc = "Shortcut between SAMPLERDY event and STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMPLERDY_STOP_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<SAMPLERDY_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: SAMPLERDY_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SAMPLERDY_STOP`"]
pub type SAMPLERDY_STOP_R = crate::R<bool, SAMPLERDY_STOP_A>;
impl SAMPLERDY_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLERDY_STOP_A {
        match self.bits {
            false => SAMPLERDY_STOP_A::DISABLED,
            true => SAMPLERDY_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAMPLERDY_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAMPLERDY_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `SAMPLERDY_STOP`"]
pub struct SAMPLERDY_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLERDY_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLERDY_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_STOP_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAMPLERDY_STOP_A::ENABLED)
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
    #[doc = "Bit 0 - Shortcut between REPORTRDY event and READCLRACC task."]
    #[inline(always)]
    pub fn reportrdy_readclracc(&self) -> REPORTRDY_READCLRACC_R {
        REPORTRDY_READCLRACC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between SAMPLERDY event and STOP task."]
    #[inline(always)]
    pub fn samplerdy_stop(&self) -> SAMPLERDY_STOP_R {
        SAMPLERDY_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between REPORTRDY event and READCLRACC task."]
    #[inline(always)]
    pub fn reportrdy_readclracc(&mut self) -> REPORTRDY_READCLRACC_W {
        REPORTRDY_READCLRACC_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between SAMPLERDY event and STOP task."]
    #[inline(always)]
    pub fn samplerdy_stop(&mut self) -> SAMPLERDY_STOP_W {
        SAMPLERDY_STOP_W { w: self }
    }
}
