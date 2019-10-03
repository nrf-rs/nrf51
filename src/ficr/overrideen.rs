#[doc = "Reader of register OVERRIDEEN"]
pub type R = crate::R<u32, super::OVERRIDEEN>;
#[doc = "Writer for register OVERRIDEEN"]
pub type W = crate::W<u32, super::OVERRIDEEN>;
#[doc = "Register OVERRIDEEN `reset()`'s with value 0"]
impl crate::ResetValue for super::OVERRIDEEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Override default values for NRF_1Mbit mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NRF_1MBIT_A {
    #[doc = "0: Override the default values for NRF_1Mbit mode."]
    OVERRIDE,
    #[doc = "1: Do not override the default values for NRF_1Mbit mode."]
    NOTOVERRIDE,
}
impl From<NRF_1MBIT_A> for bool {
    #[inline(always)]
    fn from(variant: NRF_1MBIT_A) -> Self {
        match variant {
            NRF_1MBIT_A::OVERRIDE => false,
            NRF_1MBIT_A::NOTOVERRIDE => true,
        }
    }
}
#[doc = "Reader of field `NRF_1MBIT`"]
pub type NRF_1MBIT_R = crate::R<bool, NRF_1MBIT_A>;
impl NRF_1MBIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NRF_1MBIT_A {
        match self.bits {
            false => NRF_1MBIT_A::OVERRIDE,
            true => NRF_1MBIT_A::NOTOVERRIDE,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override_(&self) -> bool {
        *self == NRF_1MBIT_A::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `NOTOVERRIDE`"]
    #[inline(always)]
    pub fn is_not_override(&self) -> bool {
        *self == NRF_1MBIT_A::NOTOVERRIDE
    }
}
#[doc = "Write proxy for field `NRF_1MBIT`"]
pub struct NRF_1MBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> NRF_1MBIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NRF_1MBIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Override the default values for NRF_1Mbit mode."]
    #[inline(always)]
    pub fn override_(self) -> &'a mut W {
        self.variant(NRF_1MBIT_A::OVERRIDE)
    }
    #[doc = "Do not override the default values for NRF_1Mbit mode."]
    #[inline(always)]
    pub fn not_override(self) -> &'a mut W {
        self.variant(NRF_1MBIT_A::NOTOVERRIDE)
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
#[doc = "Override default values for BLE_1Mbit mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BLE_1MBIT_A {
    #[doc = "0: Override the default values for BLE_1Mbit mode."]
    OVERRIDE,
    #[doc = "1: Do not override the default values for BLE_1Mbit mode."]
    NOTOVERRIDE,
}
impl From<BLE_1MBIT_A> for bool {
    #[inline(always)]
    fn from(variant: BLE_1MBIT_A) -> Self {
        match variant {
            BLE_1MBIT_A::OVERRIDE => false,
            BLE_1MBIT_A::NOTOVERRIDE => true,
        }
    }
}
#[doc = "Reader of field `BLE_1MBIT`"]
pub type BLE_1MBIT_R = crate::R<bool, BLE_1MBIT_A>;
impl BLE_1MBIT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLE_1MBIT_A {
        match self.bits {
            false => BLE_1MBIT_A::OVERRIDE,
            true => BLE_1MBIT_A::NOTOVERRIDE,
        }
    }
    #[doc = "Checks if the value of the field is `OVERRIDE`"]
    #[inline(always)]
    pub fn is_override_(&self) -> bool {
        *self == BLE_1MBIT_A::OVERRIDE
    }
    #[doc = "Checks if the value of the field is `NOTOVERRIDE`"]
    #[inline(always)]
    pub fn is_not_override(&self) -> bool {
        *self == BLE_1MBIT_A::NOTOVERRIDE
    }
}
#[doc = "Write proxy for field `BLE_1MBIT`"]
pub struct BLE_1MBIT_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_1MBIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLE_1MBIT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Override the default values for BLE_1Mbit mode."]
    #[inline(always)]
    pub fn override_(self) -> &'a mut W {
        self.variant(BLE_1MBIT_A::OVERRIDE)
    }
    #[doc = "Do not override the default values for BLE_1Mbit mode."]
    #[inline(always)]
    pub fn not_override(self) -> &'a mut W {
        self.variant(BLE_1MBIT_A::NOTOVERRIDE)
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
    #[doc = "Bit 0 - Override default values for NRF_1Mbit mode."]
    #[inline(always)]
    pub fn nrf_1mbit(&self) -> NRF_1MBIT_R {
        NRF_1MBIT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Override default values for BLE_1Mbit mode."]
    #[inline(always)]
    pub fn ble_1mbit(&self) -> BLE_1MBIT_R {
        BLE_1MBIT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override default values for NRF_1Mbit mode."]
    #[inline(always)]
    pub fn nrf_1mbit(&mut self) -> NRF_1MBIT_W {
        NRF_1MBIT_W { w: self }
    }
    #[doc = "Bit 3 - Override default values for BLE_1Mbit mode."]
    #[inline(always)]
    pub fn ble_1mbit(&mut self) -> BLE_1MBIT_W {
        BLE_1MBIT_W { w: self }
    }
}
