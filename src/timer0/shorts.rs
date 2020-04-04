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
#[doc = "Shortcut between CC\\[0\\]
event and the CLEAR task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_CLEAR_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<COMPARE0_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE0_CLEAR`"]
pub type COMPARE0_CLEAR_R = crate::R<bool, COMPARE0_CLEAR_A>;
impl COMPARE0_CLEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_CLEAR_A {
        match self.bits {
            false => COMPARE0_CLEAR_A::DISABLED,
            true => COMPARE0_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_CLEAR_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMPARE0_CLEAR`"]
pub struct COMPARE0_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE0_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE0_CLEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_CLEAR_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_CLEAR_A::ENABLED)
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
#[doc = "Shortcut between CC\\[1\\]
event and the CLEAR task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_CLEAR_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<COMPARE1_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE1_CLEAR`"]
pub type COMPARE1_CLEAR_R = crate::R<bool, COMPARE1_CLEAR_A>;
impl COMPARE1_CLEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_CLEAR_A {
        match self.bits {
            false => COMPARE1_CLEAR_A::DISABLED,
            true => COMPARE1_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_CLEAR_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMPARE1_CLEAR`"]
pub struct COMPARE1_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE1_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE1_CLEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_CLEAR_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_CLEAR_A::ENABLED)
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
#[doc = "Shortcut between CC\\[2\\]
event and the CLEAR task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_CLEAR_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<COMPARE2_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE2_CLEAR`"]
pub type COMPARE2_CLEAR_R = crate::R<bool, COMPARE2_CLEAR_A>;
impl COMPARE2_CLEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_CLEAR_A {
        match self.bits {
            false => COMPARE2_CLEAR_A::DISABLED,
            true => COMPARE2_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_CLEAR_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMPARE2_CLEAR`"]
pub struct COMPARE2_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE2_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE2_CLEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_CLEAR_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_CLEAR_A::ENABLED)
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
#[doc = "Shortcut between CC\\[3\\]
event and the CLEAR task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_CLEAR_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<COMPARE3_CLEAR_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_CLEAR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE3_CLEAR`"]
pub type COMPARE3_CLEAR_R = crate::R<bool, COMPARE3_CLEAR_A>;
impl COMPARE3_CLEAR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_CLEAR_A {
        match self.bits {
            false => COMPARE3_CLEAR_A::DISABLED,
            true => COMPARE3_CLEAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_CLEAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_CLEAR_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMPARE3_CLEAR`"]
pub struct COMPARE3_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE3_CLEAR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE3_CLEAR_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_CLEAR_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_CLEAR_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Shortcut between CC\\[0\\]
event and the STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_STOP_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<COMPARE0_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE0_STOP`"]
pub type COMPARE0_STOP_R = crate::R<bool, COMPARE0_STOP_A>;
impl COMPARE0_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_STOP_A {
        match self.bits {
            false => COMPARE0_STOP_A::DISABLED,
            true => COMPARE0_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE0_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE0_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMPARE0_STOP`"]
pub struct COMPARE0_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE0_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE0_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_STOP_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_STOP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Shortcut between CC\\[1\\]
event and the STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_STOP_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<COMPARE1_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE1_STOP`"]
pub type COMPARE1_STOP_R = crate::R<bool, COMPARE1_STOP_A>;
impl COMPARE1_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_STOP_A {
        match self.bits {
            false => COMPARE1_STOP_A::DISABLED,
            true => COMPARE1_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE1_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE1_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMPARE1_STOP`"]
pub struct COMPARE1_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE1_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE1_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_STOP_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_STOP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Shortcut between CC\\[2\\]
event and the STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_STOP_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<COMPARE2_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE2_STOP`"]
pub type COMPARE2_STOP_R = crate::R<bool, COMPARE2_STOP_A>;
impl COMPARE2_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_STOP_A {
        match self.bits {
            false => COMPARE2_STOP_A::DISABLED,
            true => COMPARE2_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE2_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE2_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMPARE2_STOP`"]
pub struct COMPARE2_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE2_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE2_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_STOP_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_STOP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Shortcut between CC\\[3\\]
event and the STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_STOP_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<COMPARE3_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `COMPARE3_STOP`"]
pub type COMPARE3_STOP_R = crate::R<bool, COMPARE3_STOP_A>;
impl COMPARE3_STOP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_STOP_A {
        match self.bits {
            false => COMPARE3_STOP_A::DISABLED,
            true => COMPARE3_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMPARE3_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMPARE3_STOP_A::ENABLED
    }
}
#[doc = "Write proxy for field `COMPARE3_STOP`"]
pub struct COMPARE3_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE3_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE3_STOP_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_STOP_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_STOP_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Shortcut between CC\\[0\\]
event and the CLEAR task."]
    #[inline(always)]
    pub fn compare0_clear(&self) -> COMPARE0_CLEAR_R {
        COMPARE0_CLEAR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between CC\\[1\\]
event and the CLEAR task."]
    #[inline(always)]
    pub fn compare1_clear(&self) -> COMPARE1_CLEAR_R {
        COMPARE1_CLEAR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Shortcut between CC\\[2\\]
event and the CLEAR task."]
    #[inline(always)]
    pub fn compare2_clear(&self) -> COMPARE2_CLEAR_R {
        COMPARE2_CLEAR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Shortcut between CC\\[3\\]
event and the CLEAR task."]
    #[inline(always)]
    pub fn compare3_clear(&self) -> COMPARE3_CLEAR_R {
        COMPARE3_CLEAR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Shortcut between CC\\[0\\]
event and the STOP task."]
    #[inline(always)]
    pub fn compare0_stop(&self) -> COMPARE0_STOP_R {
        COMPARE0_STOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Shortcut between CC\\[1\\]
event and the STOP task."]
    #[inline(always)]
    pub fn compare1_stop(&self) -> COMPARE1_STOP_R {
        COMPARE1_STOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Shortcut between CC\\[2\\]
event and the STOP task."]
    #[inline(always)]
    pub fn compare2_stop(&self) -> COMPARE2_STOP_R {
        COMPARE2_STOP_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Shortcut between CC\\[3\\]
event and the STOP task."]
    #[inline(always)]
    pub fn compare3_stop(&self) -> COMPARE3_STOP_R {
        COMPARE3_STOP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between CC\\[0\\]
event and the CLEAR task."]
    #[inline(always)]
    pub fn compare0_clear(&mut self) -> COMPARE0_CLEAR_W {
        COMPARE0_CLEAR_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between CC\\[1\\]
event and the CLEAR task."]
    #[inline(always)]
    pub fn compare1_clear(&mut self) -> COMPARE1_CLEAR_W {
        COMPARE1_CLEAR_W { w: self }
    }
    #[doc = "Bit 2 - Shortcut between CC\\[2\\]
event and the CLEAR task."]
    #[inline(always)]
    pub fn compare2_clear(&mut self) -> COMPARE2_CLEAR_W {
        COMPARE2_CLEAR_W { w: self }
    }
    #[doc = "Bit 3 - Shortcut between CC\\[3\\]
event and the CLEAR task."]
    #[inline(always)]
    pub fn compare3_clear(&mut self) -> COMPARE3_CLEAR_W {
        COMPARE3_CLEAR_W { w: self }
    }
    #[doc = "Bit 8 - Shortcut between CC\\[0\\]
event and the STOP task."]
    #[inline(always)]
    pub fn compare0_stop(&mut self) -> COMPARE0_STOP_W {
        COMPARE0_STOP_W { w: self }
    }
    #[doc = "Bit 9 - Shortcut between CC\\[1\\]
event and the STOP task."]
    #[inline(always)]
    pub fn compare1_stop(&mut self) -> COMPARE1_STOP_W {
        COMPARE1_STOP_W { w: self }
    }
    #[doc = "Bit 10 - Shortcut between CC\\[2\\]
event and the STOP task."]
    #[inline(always)]
    pub fn compare2_stop(&mut self) -> COMPARE2_STOP_W {
        COMPARE2_STOP_W { w: self }
    }
    #[doc = "Bit 11 - Shortcut between CC\\[3\\]
event and the STOP task."]
    #[inline(always)]
    pub fn compare3_stop(&mut self) -> COMPARE3_STOP_W {
        COMPARE3_STOP_W { w: self }
    }
}
