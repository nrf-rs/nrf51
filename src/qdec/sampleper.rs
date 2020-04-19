#[doc = "Reader of register SAMPLEPER"]
pub type R = crate::R<u32, super::SAMPLEPER>;
#[doc = "Writer for register SAMPLEPER"]
pub type W = crate::W<u32, super::SAMPLEPER>;
#[doc = "Register SAMPLEPER `reset()`'s with value 0"]
impl crate::ResetValue for super::SAMPLEPER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Sample period.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAMPLEPER_A {
    #[doc = "0: 128us sample period."]
    _128US = 0,
    #[doc = "1: 256us sample period."]
    _256US = 1,
    #[doc = "2: 512us sample period."]
    _512US = 2,
    #[doc = "3: 1024us sample period."]
    _1024US = 3,
    #[doc = "4: 2048us sample period."]
    _2048US = 4,
    #[doc = "5: 4096us sample period."]
    _4096US = 5,
    #[doc = "6: 8192us sample period."]
    _8192US = 6,
    #[doc = "7: 16384us sample period."]
    _16384US = 7,
}
impl From<SAMPLEPER_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPLEPER_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAMPLEPER`"]
pub type SAMPLEPER_R = crate::R<u8, SAMPLEPER_A>;
impl SAMPLEPER_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLEPER_A {
        match self.bits {
            0 => SAMPLEPER_A::_128US,
            1 => SAMPLEPER_A::_256US,
            2 => SAMPLEPER_A::_512US,
            3 => SAMPLEPER_A::_1024US,
            4 => SAMPLEPER_A::_2048US,
            5 => SAMPLEPER_A::_4096US,
            6 => SAMPLEPER_A::_8192US,
            7 => SAMPLEPER_A::_16384US,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_128US`"]
    #[inline(always)]
    pub fn is_128us(&self) -> bool {
        *self == SAMPLEPER_A::_128US
    }
    #[doc = "Checks if the value of the field is `_256US`"]
    #[inline(always)]
    pub fn is_256us(&self) -> bool {
        *self == SAMPLEPER_A::_256US
    }
    #[doc = "Checks if the value of the field is `_512US`"]
    #[inline(always)]
    pub fn is_512us(&self) -> bool {
        *self == SAMPLEPER_A::_512US
    }
    #[doc = "Checks if the value of the field is `_1024US`"]
    #[inline(always)]
    pub fn is_1024us(&self) -> bool {
        *self == SAMPLEPER_A::_1024US
    }
    #[doc = "Checks if the value of the field is `_2048US`"]
    #[inline(always)]
    pub fn is_2048us(&self) -> bool {
        *self == SAMPLEPER_A::_2048US
    }
    #[doc = "Checks if the value of the field is `_4096US`"]
    #[inline(always)]
    pub fn is_4096us(&self) -> bool {
        *self == SAMPLEPER_A::_4096US
    }
    #[doc = "Checks if the value of the field is `_8192US`"]
    #[inline(always)]
    pub fn is_8192us(&self) -> bool {
        *self == SAMPLEPER_A::_8192US
    }
    #[doc = "Checks if the value of the field is `_16384US`"]
    #[inline(always)]
    pub fn is_16384us(&self) -> bool {
        *self == SAMPLEPER_A::_16384US
    }
}
#[doc = "Write proxy for field `SAMPLEPER`"]
pub struct SAMPLEPER_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLEPER_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLEPER_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "128us sample period."]
    #[inline(always)]
    pub fn _128us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_128US)
    }
    #[doc = "256us sample period."]
    #[inline(always)]
    pub fn _256us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_256US)
    }
    #[doc = "512us sample period."]
    #[inline(always)]
    pub fn _512us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_512US)
    }
    #[doc = "1024us sample period."]
    #[inline(always)]
    pub fn _1024us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_1024US)
    }
    #[doc = "2048us sample period."]
    #[inline(always)]
    pub fn _2048us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_2048US)
    }
    #[doc = "4096us sample period."]
    #[inline(always)]
    pub fn _4096us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_4096US)
    }
    #[doc = "8192us sample period."]
    #[inline(always)]
    pub fn _8192us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_8192US)
    }
    #[doc = "16384us sample period."]
    #[inline(always)]
    pub fn _16384us(self) -> &'a mut W {
        self.variant(SAMPLEPER_A::_16384US)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Sample period."]
    #[inline(always)]
    pub fn sampleper(&self) -> SAMPLEPER_R {
        SAMPLEPER_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Sample period."]
    #[inline(always)]
    pub fn sampleper(&mut self) -> SAMPLEPER_W {
        SAMPLEPER_W { w: self }
    }
}
