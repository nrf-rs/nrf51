#[doc = "Reader of register FREQUENCY"]
pub type R = crate::R<u32, super::FREQUENCY>;
#[doc = "Writer for register FREQUENCY"]
pub type W = crate::W<u32, super::FREQUENCY>;
#[doc = "Register FREQUENCY `reset()`'s with value 0x0400_0000"]
impl crate::ResetValue for super::FREQUENCY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0400_0000
    }
}
#[doc = "SPI data rate.\n\nValue on reset: 67108864"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum FREQUENCY_A {
    #[doc = "33554432: 125kbps."]
    K125 = 33554432,
    #[doc = "67108864: 250kbps."]
    K250 = 67108864,
    #[doc = "134217728: 500kbps."]
    K500 = 134217728,
    #[doc = "268435456: 1Mbps."]
    M1 = 268435456,
    #[doc = "536870912: 2Mbps."]
    M2 = 536870912,
    #[doc = "1073741824: 4Mbps."]
    M4 = 1073741824,
    #[doc = "2147483648: 8Mbps."]
    M8 = 2147483648,
}
impl From<FREQUENCY_A> for u32 {
    #[inline(always)]
    fn from(variant: FREQUENCY_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FREQUENCY`"]
pub type FREQUENCY_R = crate::R<u32, FREQUENCY_A>;
impl FREQUENCY_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, FREQUENCY_A> {
        use crate::Variant::*;
        match self.bits {
            33554432 => Val(FREQUENCY_A::K125),
            67108864 => Val(FREQUENCY_A::K250),
            134217728 => Val(FREQUENCY_A::K500),
            268435456 => Val(FREQUENCY_A::M1),
            536870912 => Val(FREQUENCY_A::M2),
            1073741824 => Val(FREQUENCY_A::M4),
            2147483648 => Val(FREQUENCY_A::M8),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `K125`"]
    #[inline(always)]
    pub fn is_k125(&self) -> bool {
        *self == FREQUENCY_A::K125
    }
    #[doc = "Checks if the value of the field is `K250`"]
    #[inline(always)]
    pub fn is_k250(&self) -> bool {
        *self == FREQUENCY_A::K250
    }
    #[doc = "Checks if the value of the field is `K500`"]
    #[inline(always)]
    pub fn is_k500(&self) -> bool {
        *self == FREQUENCY_A::K500
    }
    #[doc = "Checks if the value of the field is `M1`"]
    #[inline(always)]
    pub fn is_m1(&self) -> bool {
        *self == FREQUENCY_A::M1
    }
    #[doc = "Checks if the value of the field is `M2`"]
    #[inline(always)]
    pub fn is_m2(&self) -> bool {
        *self == FREQUENCY_A::M2
    }
    #[doc = "Checks if the value of the field is `M4`"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        *self == FREQUENCY_A::M4
    }
    #[doc = "Checks if the value of the field is `M8`"]
    #[inline(always)]
    pub fn is_m8(&self) -> bool {
        *self == FREQUENCY_A::M8
    }
}
#[doc = "Write proxy for field `FREQUENCY`"]
pub struct FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQUENCY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FREQUENCY_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "125kbps."]
    #[inline(always)]
    pub fn k125(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K125)
    }
    #[doc = "250kbps."]
    #[inline(always)]
    pub fn k250(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K250)
    }
    #[doc = "500kbps."]
    #[inline(always)]
    pub fn k500(self) -> &'a mut W {
        self.variant(FREQUENCY_A::K500)
    }
    #[doc = "1Mbps."]
    #[inline(always)]
    pub fn m1(self) -> &'a mut W {
        self.variant(FREQUENCY_A::M1)
    }
    #[doc = "2Mbps."]
    #[inline(always)]
    pub fn m2(self) -> &'a mut W {
        self.variant(FREQUENCY_A::M2)
    }
    #[doc = "4Mbps."]
    #[inline(always)]
    pub fn m4(self) -> &'a mut W {
        self.variant(FREQUENCY_A::M4)
    }
    #[doc = "8Mbps."]
    #[inline(always)]
    pub fn m8(self) -> &'a mut W {
        self.variant(FREQUENCY_A::M8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - SPI data rate."]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPI data rate."]
    #[inline(always)]
    pub fn frequency(&mut self) -> FREQUENCY_W {
        FREQUENCY_W { w: self }
    }
}
