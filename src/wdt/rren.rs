#[doc = "Reader of register RREN"]
pub type R = crate::R<u32, super::RREN>;
#[doc = "Writer for register RREN"]
pub type W = crate::W<u32, super::RREN>;
#[doc = "Register RREN `reset()`'s with value 0x01"]
impl crate::ResetValue for super::RREN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x01
    }
}
#[doc = "Enable or disable RR\\[0\\]
register.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR0_A {
    #[doc = "0: RR\\[0\\]
register is disabled."]
    DISABLED = 0,
    #[doc = "1: RR\\[0\\]
register is enabled."]
    ENABLED = 1,
}
impl From<RR0_A> for bool {
    #[inline(always)]
    fn from(variant: RR0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR0`"]
pub type RR0_R = crate::R<bool, RR0_A>;
impl RR0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR0_A {
        match self.bits {
            false => RR0_A::DISABLED,
            true => RR0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RR0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RR0_A::ENABLED
    }
}
#[doc = "Write proxy for field `RR0`"]
pub struct RR0_W<'a> {
    w: &'a mut W,
}
impl<'a> RR0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR0_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RR\\[0\\]
register is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR0_A::DISABLED)
    }
    #[doc = "RR\\[0\\]
register is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR0_A::ENABLED)
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
#[doc = "Enable or disable RR\\[1\\]
register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR1_A {
    #[doc = "0: RR\\[1\\]
register is disabled."]
    DISABLED = 0,
    #[doc = "1: RR\\[1\\]
register is enabled."]
    ENABLED = 1,
}
impl From<RR1_A> for bool {
    #[inline(always)]
    fn from(variant: RR1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR1`"]
pub type RR1_R = crate::R<bool, RR1_A>;
impl RR1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR1_A {
        match self.bits {
            false => RR1_A::DISABLED,
            true => RR1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RR1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RR1_A::ENABLED
    }
}
#[doc = "Write proxy for field `RR1`"]
pub struct RR1_W<'a> {
    w: &'a mut W,
}
impl<'a> RR1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR1_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RR\\[1\\]
register is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR1_A::DISABLED)
    }
    #[doc = "RR\\[1\\]
register is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR1_A::ENABLED)
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
#[doc = "Enable or disable RR\\[2\\]
register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR2_A {
    #[doc = "0: RR\\[2\\]
register is disabled."]
    DISABLED = 0,
    #[doc = "1: RR\\[2\\]
register is enabled."]
    ENABLED = 1,
}
impl From<RR2_A> for bool {
    #[inline(always)]
    fn from(variant: RR2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR2`"]
pub type RR2_R = crate::R<bool, RR2_A>;
impl RR2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR2_A {
        match self.bits {
            false => RR2_A::DISABLED,
            true => RR2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RR2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RR2_A::ENABLED
    }
}
#[doc = "Write proxy for field `RR2`"]
pub struct RR2_W<'a> {
    w: &'a mut W,
}
impl<'a> RR2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RR\\[2\\]
register is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR2_A::DISABLED)
    }
    #[doc = "RR\\[2\\]
register is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR2_A::ENABLED)
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
#[doc = "Enable or disable RR\\[3\\]
register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR3_A {
    #[doc = "0: RR\\[3\\]
register is disabled."]
    DISABLED = 0,
    #[doc = "1: RR\\[3\\]
register is enabled."]
    ENABLED = 1,
}
impl From<RR3_A> for bool {
    #[inline(always)]
    fn from(variant: RR3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR3`"]
pub type RR3_R = crate::R<bool, RR3_A>;
impl RR3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR3_A {
        match self.bits {
            false => RR3_A::DISABLED,
            true => RR3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RR3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RR3_A::ENABLED
    }
}
#[doc = "Write proxy for field `RR3`"]
pub struct RR3_W<'a> {
    w: &'a mut W,
}
impl<'a> RR3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RR\\[3\\]
register is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR3_A::DISABLED)
    }
    #[doc = "RR\\[3\\]
register is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR3_A::ENABLED)
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
#[doc = "Enable or disable RR\\[4\\]
register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR4_A {
    #[doc = "0: RR\\[4\\]
register is disabled."]
    DISABLED = 0,
    #[doc = "1: RR\\[4\\]
register is enabled."]
    ENABLED = 1,
}
impl From<RR4_A> for bool {
    #[inline(always)]
    fn from(variant: RR4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR4`"]
pub type RR4_R = crate::R<bool, RR4_A>;
impl RR4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR4_A {
        match self.bits {
            false => RR4_A::DISABLED,
            true => RR4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RR4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RR4_A::ENABLED
    }
}
#[doc = "Write proxy for field `RR4`"]
pub struct RR4_W<'a> {
    w: &'a mut W,
}
impl<'a> RR4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR4_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RR\\[4\\]
register is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR4_A::DISABLED)
    }
    #[doc = "RR\\[4\\]
register is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR4_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Enable or disable RR\\[5\\]
register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR5_A {
    #[doc = "0: RR\\[5\\]
register is disabled."]
    DISABLED = 0,
    #[doc = "1: RR\\[5\\]
register is enabled."]
    ENABLED = 1,
}
impl From<RR5_A> for bool {
    #[inline(always)]
    fn from(variant: RR5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR5`"]
pub type RR5_R = crate::R<bool, RR5_A>;
impl RR5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR5_A {
        match self.bits {
            false => RR5_A::DISABLED,
            true => RR5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RR5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RR5_A::ENABLED
    }
}
#[doc = "Write proxy for field `RR5`"]
pub struct RR5_W<'a> {
    w: &'a mut W,
}
impl<'a> RR5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR5_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RR\\[5\\]
register is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR5_A::DISABLED)
    }
    #[doc = "RR\\[5\\]
register is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR5_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Enable or disable RR\\[6\\]
register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR6_A {
    #[doc = "0: RR\\[6\\]
register is disabled."]
    DISABLED = 0,
    #[doc = "1: RR\\[6\\]
register is enabled."]
    ENABLED = 1,
}
impl From<RR6_A> for bool {
    #[inline(always)]
    fn from(variant: RR6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR6`"]
pub type RR6_R = crate::R<bool, RR6_A>;
impl RR6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR6_A {
        match self.bits {
            false => RR6_A::DISABLED,
            true => RR6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RR6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RR6_A::ENABLED
    }
}
#[doc = "Write proxy for field `RR6`"]
pub struct RR6_W<'a> {
    w: &'a mut W,
}
impl<'a> RR6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR6_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RR\\[6\\]
register is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR6_A::DISABLED)
    }
    #[doc = "RR\\[6\\]
register is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR6_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable or disable RR\\[7\\]
register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RR7_A {
    #[doc = "0: RR\\[7\\]
register is disabled."]
    DISABLED = 0,
    #[doc = "1: RR\\[7\\]
register is enabled."]
    ENABLED = 1,
}
impl From<RR7_A> for bool {
    #[inline(always)]
    fn from(variant: RR7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RR7`"]
pub type RR7_R = crate::R<bool, RR7_A>;
impl RR7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RR7_A {
        match self.bits {
            false => RR7_A::DISABLED,
            true => RR7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RR7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RR7_A::ENABLED
    }
}
#[doc = "Write proxy for field `RR7`"]
pub struct RR7_W<'a> {
    w: &'a mut W,
}
impl<'a> RR7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RR7_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RR\\[7\\]
register is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RR7_A::DISABLED)
    }
    #[doc = "RR\\[7\\]
register is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RR7_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable RR\\[0\\]
register."]
    #[inline(always)]
    pub fn rr0(&self) -> RR0_R {
        RR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable or disable RR\\[1\\]
register."]
    #[inline(always)]
    pub fn rr1(&self) -> RR1_R {
        RR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable or disable RR\\[2\\]
register."]
    #[inline(always)]
    pub fn rr2(&self) -> RR2_R {
        RR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable or disable RR\\[3\\]
register."]
    #[inline(always)]
    pub fn rr3(&self) -> RR3_R {
        RR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable or disable RR\\[4\\]
register."]
    #[inline(always)]
    pub fn rr4(&self) -> RR4_R {
        RR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable or disable RR\\[5\\]
register."]
    #[inline(always)]
    pub fn rr5(&self) -> RR5_R {
        RR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable or disable RR\\[6\\]
register."]
    #[inline(always)]
    pub fn rr6(&self) -> RR6_R {
        RR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable or disable RR\\[7\\]
register."]
    #[inline(always)]
    pub fn rr7(&self) -> RR7_R {
        RR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable RR\\[0\\]
register."]
    #[inline(always)]
    pub fn rr0(&mut self) -> RR0_W {
        RR0_W { w: self }
    }
    #[doc = "Bit 1 - Enable or disable RR\\[1\\]
register."]
    #[inline(always)]
    pub fn rr1(&mut self) -> RR1_W {
        RR1_W { w: self }
    }
    #[doc = "Bit 2 - Enable or disable RR\\[2\\]
register."]
    #[inline(always)]
    pub fn rr2(&mut self) -> RR2_W {
        RR2_W { w: self }
    }
    #[doc = "Bit 3 - Enable or disable RR\\[3\\]
register."]
    #[inline(always)]
    pub fn rr3(&mut self) -> RR3_W {
        RR3_W { w: self }
    }
    #[doc = "Bit 4 - Enable or disable RR\\[4\\]
register."]
    #[inline(always)]
    pub fn rr4(&mut self) -> RR4_W {
        RR4_W { w: self }
    }
    #[doc = "Bit 5 - Enable or disable RR\\[5\\]
register."]
    #[inline(always)]
    pub fn rr5(&mut self) -> RR5_W {
        RR5_W { w: self }
    }
    #[doc = "Bit 6 - Enable or disable RR\\[6\\]
register."]
    #[inline(always)]
    pub fn rr6(&mut self) -> RR6_W {
        RR6_W { w: self }
    }
    #[doc = "Bit 7 - Enable or disable RR\\[7\\]
register."]
    #[inline(always)]
    pub fn rr7(&mut self) -> RR7_W {
        RR7_W { w: self }
    }
}
