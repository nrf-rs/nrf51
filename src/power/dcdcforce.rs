#[doc = "Reader of register DCDCFORCE"]
pub type R = crate::R<u32, super::DCDCFORCE>;
#[doc = "Writer for register DCDCFORCE"]
pub type W = crate::W<u32, super::DCDCFORCE>;
#[doc = "Register DCDCFORCE `reset()`'s with value 0"]
impl crate::ResetValue for super::DCDCFORCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DCDC power-up force off.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEOFF_A {
    #[doc = "0: No force."]
    NOFORCE,
    #[doc = "1: Force."]
    FORCE,
}
impl From<FORCEOFF_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEOFF_A) -> Self {
        match variant {
            FORCEOFF_A::NOFORCE => false,
            FORCEOFF_A::FORCE => true,
        }
    }
}
#[doc = "Reader of field `FORCEOFF`"]
pub type FORCEOFF_R = crate::R<bool, FORCEOFF_A>;
impl FORCEOFF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEOFF_A {
        match self.bits {
            false => FORCEOFF_A::NOFORCE,
            true => FORCEOFF_A::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFORCE`"]
    #[inline(always)]
    pub fn is_no_force(&self) -> bool {
        *self == FORCEOFF_A::NOFORCE
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == FORCEOFF_A::FORCE
    }
}
#[doc = "Write proxy for field `FORCEOFF`"]
pub struct FORCEOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEOFF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No force."]
    #[inline(always)]
    pub fn no_force(self) -> &'a mut W {
        self.variant(FORCEOFF_A::NOFORCE)
    }
    #[doc = "Force."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEOFF_A::FORCE)
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
#[doc = "DCDC power-up force on.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCEON_A {
    #[doc = "0: No force."]
    NOFORCE,
    #[doc = "1: Force."]
    FORCE,
}
impl From<FORCEON_A> for bool {
    #[inline(always)]
    fn from(variant: FORCEON_A) -> Self {
        match variant {
            FORCEON_A::NOFORCE => false,
            FORCEON_A::FORCE => true,
        }
    }
}
#[doc = "Reader of field `FORCEON`"]
pub type FORCEON_R = crate::R<bool, FORCEON_A>;
impl FORCEON_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCEON_A {
        match self.bits {
            false => FORCEON_A::NOFORCE,
            true => FORCEON_A::FORCE,
        }
    }
    #[doc = "Checks if the value of the field is `NOFORCE`"]
    #[inline(always)]
    pub fn is_no_force(&self) -> bool {
        *self == FORCEON_A::NOFORCE
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        *self == FORCEON_A::FORCE
    }
}
#[doc = "Write proxy for field `FORCEON`"]
pub struct FORCEON_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCEON_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCEON_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No force."]
    #[inline(always)]
    pub fn no_force(self) -> &'a mut W {
        self.variant(FORCEON_A::NOFORCE)
    }
    #[doc = "Force."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(FORCEON_A::FORCE)
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
    #[doc = "Bit 0 - DCDC power-up force off."]
    #[inline(always)]
    pub fn forceoff(&self) -> FORCEOFF_R {
        FORCEOFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCDC power-up force on."]
    #[inline(always)]
    pub fn forceon(&self) -> FORCEON_R {
        FORCEON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DCDC power-up force off."]
    #[inline(always)]
    pub fn forceoff(&mut self) -> FORCEOFF_W {
        FORCEOFF_W { w: self }
    }
    #[doc = "Bit 1 - DCDC power-up force on."]
    #[inline(always)]
    pub fn forceon(&mut self) -> FORCEON_W {
        FORCEON_W { w: self }
    }
}
