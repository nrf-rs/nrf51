#[doc = "Reader of register TXPOWER"]
pub type R = crate::R<u32, super::TXPOWER>;
#[doc = "Writer for register TXPOWER"]
pub type W = crate::W<u32, super::TXPOWER>;
#[doc = "Register TXPOWER `reset()`'s with value 0"]
impl crate::ResetValue for super::TXPOWER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Radio output power. Decision point: TXEN task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXPOWER_A {
    #[doc = "4: +4dBm."]
    POS4DBM,
    #[doc = "0: 0dBm."]
    _0DBM,
    #[doc = "252: -4dBm."]
    NEG4DBM,
    #[doc = "248: -8dBm."]
    NEG8DBM,
    #[doc = "244: -12dBm."]
    NEG12DBM,
    #[doc = "240: -16dBm."]
    NEG16DBM,
    #[doc = "236: -20dBm."]
    NEG20DBM,
    #[doc = "216: -30dBm."]
    NEG30DBM,
}
impl From<TXPOWER_A> for u8 {
    #[inline(always)]
    fn from(variant: TXPOWER_A) -> Self {
        match variant {
            TXPOWER_A::POS4DBM => 4,
            TXPOWER_A::_0DBM => 0,
            TXPOWER_A::NEG4DBM => 252,
            TXPOWER_A::NEG8DBM => 248,
            TXPOWER_A::NEG12DBM => 244,
            TXPOWER_A::NEG16DBM => 240,
            TXPOWER_A::NEG20DBM => 236,
            TXPOWER_A::NEG30DBM => 216,
        }
    }
}
#[doc = "Reader of field `TXPOWER`"]
pub type TXPOWER_R = crate::R<u8, TXPOWER_A>;
impl TXPOWER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXPOWER_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(TXPOWER_A::POS4DBM),
            0 => Val(TXPOWER_A::_0DBM),
            252 => Val(TXPOWER_A::NEG4DBM),
            248 => Val(TXPOWER_A::NEG8DBM),
            244 => Val(TXPOWER_A::NEG12DBM),
            240 => Val(TXPOWER_A::NEG16DBM),
            236 => Val(TXPOWER_A::NEG20DBM),
            216 => Val(TXPOWER_A::NEG30DBM),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `POS4DBM`"]
    #[inline(always)]
    pub fn is_pos4d_bm(&self) -> bool {
        *self == TXPOWER_A::POS4DBM
    }
    #[doc = "Checks if the value of the field is `_0DBM`"]
    #[inline(always)]
    pub fn is_0d_bm(&self) -> bool {
        *self == TXPOWER_A::_0DBM
    }
    #[doc = "Checks if the value of the field is `NEG4DBM`"]
    #[inline(always)]
    pub fn is_neg4d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG4DBM
    }
    #[doc = "Checks if the value of the field is `NEG8DBM`"]
    #[inline(always)]
    pub fn is_neg8d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG8DBM
    }
    #[doc = "Checks if the value of the field is `NEG12DBM`"]
    #[inline(always)]
    pub fn is_neg12d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG12DBM
    }
    #[doc = "Checks if the value of the field is `NEG16DBM`"]
    #[inline(always)]
    pub fn is_neg16d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG16DBM
    }
    #[doc = "Checks if the value of the field is `NEG20DBM`"]
    #[inline(always)]
    pub fn is_neg20d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG20DBM
    }
    #[doc = "Checks if the value of the field is `NEG30DBM`"]
    #[inline(always)]
    pub fn is_neg30d_bm(&self) -> bool {
        *self == TXPOWER_A::NEG30DBM
    }
}
#[doc = "Write proxy for field `TXPOWER`"]
pub struct TXPOWER_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPOWER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXPOWER_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "+4dBm."]
    #[inline(always)]
    pub fn pos4d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::POS4DBM)
    }
    #[doc = "0dBm."]
    #[inline(always)]
    pub fn _0d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::_0DBM)
    }
    #[doc = "-4dBm."]
    #[inline(always)]
    pub fn neg4d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG4DBM)
    }
    #[doc = "-8dBm."]
    #[inline(always)]
    pub fn neg8d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG8DBM)
    }
    #[doc = "-12dBm."]
    #[inline(always)]
    pub fn neg12d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG12DBM)
    }
    #[doc = "-16dBm."]
    #[inline(always)]
    pub fn neg16d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG16DBM)
    }
    #[doc = "-20dBm."]
    #[inline(always)]
    pub fn neg20d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG20DBM)
    }
    #[doc = "-30dBm."]
    #[inline(always)]
    pub fn neg30d_bm(self) -> &'a mut W {
        self.variant(TXPOWER_A::NEG30DBM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Radio output power. Decision point: TXEN task."]
    #[inline(always)]
    pub fn txpower(&self) -> TXPOWER_R {
        TXPOWER_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Radio output power. Decision point: TXEN task."]
    #[inline(always)]
    pub fn txpower(&mut self) -> TXPOWER_W {
        TXPOWER_W { w: self }
    }
}
