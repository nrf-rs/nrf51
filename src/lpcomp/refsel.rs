#[doc = "Reader of register REFSEL"]
pub type R = crate::R<u32, super::REFSEL>;
#[doc = "Writer for register REFSEL"]
pub type W = crate::W<u32, super::REFSEL>;
#[doc = "Register REFSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::REFSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reference select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFSEL_A {
    #[doc = "0: Use supply with a 1/8 prescaler as reference."]
    SUPPLYONEEIGHTHPRESCALING,
    #[doc = "1: Use supply with a 2/8 prescaler as reference."]
    SUPPLYTWOEIGHTHSPRESCALING,
    #[doc = "2: Use supply with a 3/8 prescaler as reference."]
    SUPPLYTHREEEIGHTHSPRESCALING,
    #[doc = "3: Use supply with a 4/8 prescaler as reference."]
    SUPPLYFOUREIGHTHSPRESCALING,
    #[doc = "4: Use supply with a 5/8 prescaler as reference."]
    SUPPLYFIVEEIGHTHSPRESCALING,
    #[doc = "5: Use supply with a 6/8 prescaler as reference."]
    SUPPLYSIXEIGHTHSPRESCALING,
    #[doc = "6: Use supply with a 7/8 prescaler as reference."]
    SUPPLYSEVENEIGHTHSPRESCALING,
    #[doc = "7: Use external analog reference as reference."]
    AREF,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        match variant {
            REFSEL_A::SUPPLYONEEIGHTHPRESCALING => 0,
            REFSEL_A::SUPPLYTWOEIGHTHSPRESCALING => 1,
            REFSEL_A::SUPPLYTHREEEIGHTHSPRESCALING => 2,
            REFSEL_A::SUPPLYFOUREIGHTHSPRESCALING => 3,
            REFSEL_A::SUPPLYFIVEEIGHTHSPRESCALING => 4,
            REFSEL_A::SUPPLYSIXEIGHTHSPRESCALING => 5,
            REFSEL_A::SUPPLYSEVENEIGHTHSPRESCALING => 6,
            REFSEL_A::AREF => 7,
        }
    }
}
#[doc = "Reader of field `REFSEL`"]
pub type REFSEL_R = crate::R<u8, REFSEL_A>;
impl REFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFSEL_A {
        match self.bits {
            0 => REFSEL_A::SUPPLYONEEIGHTHPRESCALING,
            1 => REFSEL_A::SUPPLYTWOEIGHTHSPRESCALING,
            2 => REFSEL_A::SUPPLYTHREEEIGHTHSPRESCALING,
            3 => REFSEL_A::SUPPLYFOUREIGHTHSPRESCALING,
            4 => REFSEL_A::SUPPLYFIVEEIGHTHSPRESCALING,
            5 => REFSEL_A::SUPPLYSIXEIGHTHSPRESCALING,
            6 => REFSEL_A::SUPPLYSEVENEIGHTHSPRESCALING,
            7 => REFSEL_A::AREF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SUPPLYONEEIGHTHPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_one_eighth_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLYONEEIGHTHPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYTWOEIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_two_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLYTWOEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYTHREEEIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_three_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLYTHREEEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYFOUREIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_four_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLYFOUREIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYFIVEEIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_five_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLYFIVEEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYSIXEIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_six_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLYSIXEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `SUPPLYSEVENEIGHTHSPRESCALING`"]
    #[inline(always)]
    pub fn is_supply_seven_eighths_prescaling(&self) -> bool {
        *self == REFSEL_A::SUPPLYSEVENEIGHTHSPRESCALING
    }
    #[doc = "Checks if the value of the field is `AREF`"]
    #[inline(always)]
    pub fn is_aref(&self) -> bool {
        *self == REFSEL_A::AREF
    }
}
#[doc = "Write proxy for field `REFSEL`"]
pub struct REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Use supply with a 1/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_one_eighth_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYONEEIGHTHPRESCALING)
    }
    #[doc = "Use supply with a 2/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_two_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYTWOEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 3/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_three_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYTHREEEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 4/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_four_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYFOUREIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 5/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_five_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYFIVEEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 6/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_six_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYSIXEIGHTHSPRESCALING)
    }
    #[doc = "Use supply with a 7/8 prescaler as reference."]
    #[inline(always)]
    pub fn supply_seven_eighths_prescaling(self) -> &'a mut W {
        self.variant(REFSEL_A::SUPPLYSEVENEIGHTHSPRESCALING)
    }
    #[doc = "Use external analog reference as reference."]
    #[inline(always)]
    pub fn aref(self) -> &'a mut W {
        self.variant(REFSEL_A::AREF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference select."]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference select."]
    #[inline(always)]
    pub fn refsel(&mut self) -> REFSEL_W {
        REFSEL_W { w: self }
    }
}
