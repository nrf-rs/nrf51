#[doc = "Reader of register RAMONB"]
pub type R = crate::R<u32, super::RAMONB>;
#[doc = "Writer for register RAMONB"]
pub type W = crate::W<u32, super::RAMONB>;
#[doc = "Register RAMONB `reset()`'s with value 0x03"]
impl crate::ResetValue for super::RAMONB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "RAM block 2 behaviour in ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONRAM2_A {
    #[doc = "0: RAM block 2 OFF in ON mode."]
    RAM2OFF = 0,
    #[doc = "1: RAM block 2 ON in ON mode."]
    RAM2ON = 1,
}
impl From<ONRAM2_A> for bool {
    #[inline(always)]
    fn from(variant: ONRAM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONRAM2`"]
pub type ONRAM2_R = crate::R<bool, ONRAM2_A>;
impl ONRAM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONRAM2_A {
        match self.bits {
            false => ONRAM2_A::RAM2OFF,
            true => ONRAM2_A::RAM2ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM2OFF`"]
    #[inline(always)]
    pub fn is_ram2off(&self) -> bool {
        *self == ONRAM2_A::RAM2OFF
    }
    #[doc = "Checks if the value of the field is `RAM2ON`"]
    #[inline(always)]
    pub fn is_ram2on(&self) -> bool {
        *self == ONRAM2_A::RAM2ON
    }
}
#[doc = "Write proxy for field `ONRAM2`"]
pub struct ONRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> ONRAM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONRAM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RAM block 2 OFF in ON mode."]
    #[inline(always)]
    pub fn ram2off(self) -> &'a mut W {
        self.variant(ONRAM2_A::RAM2OFF)
    }
    #[doc = "RAM block 2 ON in ON mode."]
    #[inline(always)]
    pub fn ram2on(self) -> &'a mut W {
        self.variant(ONRAM2_A::RAM2ON)
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
#[doc = "RAM block 3 behaviour in ON mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONRAM3_A {
    #[doc = "0: RAM block 33 OFF in ON mode."]
    RAM3OFF = 0,
    #[doc = "1: RAM block 3 ON in ON mode."]
    RAM3ON = 1,
}
impl From<ONRAM3_A> for bool {
    #[inline(always)]
    fn from(variant: ONRAM3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ONRAM3`"]
pub type ONRAM3_R = crate::R<bool, ONRAM3_A>;
impl ONRAM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONRAM3_A {
        match self.bits {
            false => ONRAM3_A::RAM3OFF,
            true => ONRAM3_A::RAM3ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM3OFF`"]
    #[inline(always)]
    pub fn is_ram3off(&self) -> bool {
        *self == ONRAM3_A::RAM3OFF
    }
    #[doc = "Checks if the value of the field is `RAM3ON`"]
    #[inline(always)]
    pub fn is_ram3on(&self) -> bool {
        *self == ONRAM3_A::RAM3ON
    }
}
#[doc = "Write proxy for field `ONRAM3`"]
pub struct ONRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> ONRAM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONRAM3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RAM block 33 OFF in ON mode."]
    #[inline(always)]
    pub fn ram3off(self) -> &'a mut W {
        self.variant(ONRAM3_A::RAM3OFF)
    }
    #[doc = "RAM block 3 ON in ON mode."]
    #[inline(always)]
    pub fn ram3on(self) -> &'a mut W {
        self.variant(ONRAM3_A::RAM3ON)
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
#[doc = "RAM block 2 behaviour in OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRAM2_A {
    #[doc = "0: RAM block 2 OFF in OFF mode."]
    RAM2OFF = 0,
    #[doc = "1: RAM block 2 ON in OFF mode."]
    RAM2ON = 1,
}
impl From<OFFRAM2_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRAM2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFFRAM2`"]
pub type OFFRAM2_R = crate::R<bool, OFFRAM2_A>;
impl OFFRAM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRAM2_A {
        match self.bits {
            false => OFFRAM2_A::RAM2OFF,
            true => OFFRAM2_A::RAM2ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM2OFF`"]
    #[inline(always)]
    pub fn is_ram2off(&self) -> bool {
        *self == OFFRAM2_A::RAM2OFF
    }
    #[doc = "Checks if the value of the field is `RAM2ON`"]
    #[inline(always)]
    pub fn is_ram2on(&self) -> bool {
        *self == OFFRAM2_A::RAM2ON
    }
}
#[doc = "Write proxy for field `OFFRAM2`"]
pub struct OFFRAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFRAM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFRAM2_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RAM block 2 OFF in OFF mode."]
    #[inline(always)]
    pub fn ram2off(self) -> &'a mut W {
        self.variant(OFFRAM2_A::RAM2OFF)
    }
    #[doc = "RAM block 2 ON in OFF mode."]
    #[inline(always)]
    pub fn ram2on(self) -> &'a mut W {
        self.variant(OFFRAM2_A::RAM2ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "RAM block 3 behaviour in OFF mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OFFRAM3_A {
    #[doc = "0: RAM block 3 OFF in OFF mode."]
    RAM3OFF = 0,
    #[doc = "1: RAM block 3 ON in OFF mode."]
    RAM3ON = 1,
}
impl From<OFFRAM3_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRAM3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `OFFRAM3`"]
pub type OFFRAM3_R = crate::R<bool, OFFRAM3_A>;
impl OFFRAM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRAM3_A {
        match self.bits {
            false => OFFRAM3_A::RAM3OFF,
            true => OFFRAM3_A::RAM3ON,
        }
    }
    #[doc = "Checks if the value of the field is `RAM3OFF`"]
    #[inline(always)]
    pub fn is_ram3off(&self) -> bool {
        *self == OFFRAM3_A::RAM3OFF
    }
    #[doc = "Checks if the value of the field is `RAM3ON`"]
    #[inline(always)]
    pub fn is_ram3on(&self) -> bool {
        *self == OFFRAM3_A::RAM3ON
    }
}
#[doc = "Write proxy for field `OFFRAM3`"]
pub struct OFFRAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFRAM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OFFRAM3_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "RAM block 3 OFF in OFF mode."]
    #[inline(always)]
    pub fn ram3off(self) -> &'a mut W {
        self.variant(OFFRAM3_A::RAM3OFF)
    }
    #[doc = "RAM block 3 ON in OFF mode."]
    #[inline(always)]
    pub fn ram3on(self) -> &'a mut W {
        self.variant(OFFRAM3_A::RAM3ON)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RAM block 2 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram2(&self) -> ONRAM2_R {
        ONRAM2_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RAM block 3 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram3(&self) -> ONRAM3_R {
        ONRAM3_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RAM block 2 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram2(&self) -> OFFRAM2_R {
        OFFRAM2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RAM block 3 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram3(&self) -> OFFRAM3_R {
        OFFRAM3_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAM block 2 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram2(&mut self) -> ONRAM2_W {
        ONRAM2_W { w: self }
    }
    #[doc = "Bit 1 - RAM block 3 behaviour in ON mode."]
    #[inline(always)]
    pub fn onram3(&mut self) -> ONRAM3_W {
        ONRAM3_W { w: self }
    }
    #[doc = "Bit 16 - RAM block 2 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram2(&mut self) -> OFFRAM2_W {
        OFFRAM2_W { w: self }
    }
    #[doc = "Bit 17 - RAM block 3 behaviour in OFF mode."]
    #[inline(always)]
    pub fn offram3(&mut self) -> OFFRAM3_W {
        OFFRAM3_W { w: self }
    }
}
