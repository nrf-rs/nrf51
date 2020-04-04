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
#[doc = "Disable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `READY`"]
pub type READY_R = crate::R<bool, READY_A>;
impl READY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::DISABLED,
            true => READY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == READY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == READY_A::ENABLED
    }
}
#[doc = "Disable interrupt on READY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<READY_AW> for bool {
    #[inline(always)]
    fn from(variant: READY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `READY`"]
pub struct READY_W<'a> {
    w: &'a mut W,
}
impl<'a> READY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: READY_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(READY_AW::CLEAR)
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
#[doc = "Disable interrupt on DOWN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWN_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: DOWN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOWN`"]
pub type DOWN_R = crate::R<bool, DOWN_A>;
impl DOWN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOWN_A {
        match self.bits {
            false => DOWN_A::DISABLED,
            true => DOWN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOWN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOWN_A::ENABLED
    }
}
#[doc = "Disable interrupt on DOWN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOWN_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<DOWN_AW> for bool {
    #[inline(always)]
    fn from(variant: DOWN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `DOWN`"]
pub struct DOWN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOWN_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DOWN_AW::CLEAR)
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
#[doc = "Disable interrupt on UP event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UP_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<UP_A> for bool {
    #[inline(always)]
    fn from(variant: UP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UP`"]
pub type UP_R = crate::R<bool, UP_A>;
impl UP_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UP_A {
        match self.bits {
            false => UP_A::DISABLED,
            true => UP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UP_A::ENABLED
    }
}
#[doc = "Disable interrupt on UP event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UP_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<UP_AW> for bool {
    #[inline(always)]
    fn from(variant: UP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `UP`"]
pub struct UP_W<'a> {
    w: &'a mut W,
}
impl<'a> UP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UP_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(UP_AW::CLEAR)
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
#[doc = "Disable interrupt on CROSS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROSS_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<CROSS_A> for bool {
    #[inline(always)]
    fn from(variant: CROSS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CROSS`"]
pub type CROSS_R = crate::R<bool, CROSS_A>;
impl CROSS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CROSS_A {
        match self.bits {
            false => CROSS_A::DISABLED,
            true => CROSS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CROSS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CROSS_A::ENABLED
    }
}
#[doc = "Disable interrupt on CROSS event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CROSS_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<CROSS_AW> for bool {
    #[inline(always)]
    fn from(variant: CROSS_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Write proxy for field `CROSS`"]
pub struct CROSS_W<'a> {
    w: &'a mut W,
}
impl<'a> CROSS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CROSS_AW) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CROSS_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Disable interrupt on READY event."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on DOWN event."]
    #[inline(always)]
    pub fn down(&self) -> DOWN_R {
        DOWN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Disable interrupt on UP event."]
    #[inline(always)]
    pub fn up(&self) -> UP_R {
        UP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Disable interrupt on CROSS event."]
    #[inline(always)]
    pub fn cross(&self) -> CROSS_R {
        CROSS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on READY event."]
    #[inline(always)]
    pub fn ready(&mut self) -> READY_W {
        READY_W { w: self }
    }
    #[doc = "Bit 1 - Disable interrupt on DOWN event."]
    #[inline(always)]
    pub fn down(&mut self) -> DOWN_W {
        DOWN_W { w: self }
    }
    #[doc = "Bit 2 - Disable interrupt on UP event."]
    #[inline(always)]
    pub fn up(&mut self) -> UP_W {
        UP_W { w: self }
    }
    #[doc = "Bit 3 - Disable interrupt on CROSS event."]
    #[inline(always)]
    pub fn cross(&mut self) -> CROSS_W {
        CROSS_W { w: self }
    }
}
