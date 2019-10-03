#[doc = "Reader of register CONFIG[%s]"]
pub type R = crate::R<u32, super::CONFIG>;
#[doc = "Writer for register CONFIG[%s]"]
pub type W = crate::W<u32, super::CONFIG>;
#[doc = "Register CONFIG[%s] `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFIG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Disabled."]
    DISABLED,
    #[doc = "1: Channel configure in event mode."]
    EVENT,
    #[doc = "3: Channel configure in task mode."]
    TASK,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        match variant {
            MODE_A::DISABLED => 0,
            MODE_A::EVENT => 1,
            MODE_A::TASK => 3,
        }
    }
}
#[doc = "Reader of field `MODE`"]
pub type MODE_R = crate::R<u8, MODE_A>;
impl MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MODE_A::DISABLED),
            1 => Val(MODE_A::EVENT),
            3 => Val(MODE_A::TASK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == MODE_A::EVENT
    }
    #[doc = "Checks if the value of the field is `TASK`"]
    #[inline(always)]
    pub fn is_task(&self) -> bool {
        *self == MODE_A::TASK
    }
}
#[doc = "Write proxy for field `MODE`"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MODE_A::DISABLED)
    }
    #[doc = "Channel configure in event mode."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(MODE_A::EVENT)
    }
    #[doc = "Channel configure in task mode."]
    #[inline(always)]
    pub fn task(self) -> &'a mut W {
        self.variant(MODE_A::TASK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `PSEL`"]
pub type PSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PSEL`"]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Effects on output when in Task mode, or events on input that generates an event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POLARITY_A {
    #[doc = "0: No task or event."]
    NONE,
    #[doc = "1: Low to high."]
    LOTOHI,
    #[doc = "2: High to low."]
    HITOLO,
    #[doc = "3: Toggle."]
    TOGGLE,
}
impl From<POLARITY_A> for u8 {
    #[inline(always)]
    fn from(variant: POLARITY_A) -> Self {
        match variant {
            POLARITY_A::NONE => 0,
            POLARITY_A::LOTOHI => 1,
            POLARITY_A::HITOLO => 2,
            POLARITY_A::TOGGLE => 3,
        }
    }
}
#[doc = "Reader of field `POLARITY`"]
pub type POLARITY_R = crate::R<u8, POLARITY_A>;
impl POLARITY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLARITY_A {
        match self.bits {
            0 => POLARITY_A::NONE,
            1 => POLARITY_A::LOTOHI,
            2 => POLARITY_A::HITOLO,
            3 => POLARITY_A::TOGGLE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == POLARITY_A::NONE
    }
    #[doc = "Checks if the value of the field is `LOTOHI`"]
    #[inline(always)]
    pub fn is_lo_to_hi(&self) -> bool {
        *self == POLARITY_A::LOTOHI
    }
    #[doc = "Checks if the value of the field is `HITOLO`"]
    #[inline(always)]
    pub fn is_hi_to_lo(&self) -> bool {
        *self == POLARITY_A::HITOLO
    }
    #[doc = "Checks if the value of the field is `TOGGLE`"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == POLARITY_A::TOGGLE
    }
}
#[doc = "Write proxy for field `POLARITY`"]
pub struct POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLARITY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POLARITY_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No task or event."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(POLARITY_A::NONE)
    }
    #[doc = "Low to high."]
    #[inline(always)]
    pub fn lo_to_hi(self) -> &'a mut W {
        self.variant(POLARITY_A::LOTOHI)
    }
    #[doc = "High to low."]
    #[inline(always)]
    pub fn hi_to_lo(self) -> &'a mut W {
        self.variant(POLARITY_A::HITOLO)
    }
    #[doc = "Toggle."]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut W {
        self.variant(POLARITY_A::TOGGLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Initial value of the output when the GPIOTE channel is configured as a Task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OUTINIT_A {
    #[doc = "0: Initial low output when in task mode."]
    LOW,
    #[doc = "1: Initial high output when in task mode."]
    HIGH,
}
impl From<OUTINIT_A> for bool {
    #[inline(always)]
    fn from(variant: OUTINIT_A) -> Self {
        match variant {
            OUTINIT_A::LOW => false,
            OUTINIT_A::HIGH => true,
        }
    }
}
#[doc = "Reader of field `OUTINIT`"]
pub type OUTINIT_R = crate::R<bool, OUTINIT_A>;
impl OUTINIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTINIT_A {
        match self.bits {
            false => OUTINIT_A::LOW,
            true => OUTINIT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == OUTINIT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == OUTINIT_A::HIGH
    }
}
#[doc = "Write proxy for field `OUTINIT`"]
pub struct OUTINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> OUTINIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OUTINIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Initial low output when in task mode."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(OUTINIT_A::LOW)
    }
    #[doc = "Initial high output when in task mode."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(OUTINIT_A::HIGH)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Pin select."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Effects on output when in Task mode, or events on input that generates an event."]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 20 - Initial value of the output when the GPIOTE channel is configured as a Task."]
    #[inline(always)]
    pub fn outinit(&self) -> OUTINIT_R {
        OUTINIT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bits 8:12 - Pin select."]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Bits 16:17 - Effects on output when in Task mode, or events on input that generates an event."]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W {
        POLARITY_W { w: self }
    }
    #[doc = "Bit 20 - Initial value of the output when the GPIOTE channel is configured as a Task."]
    #[inline(always)]
    pub fn outinit(&mut self) -> OUTINIT_W {
        OUTINIT_W { w: self }
    }
}
