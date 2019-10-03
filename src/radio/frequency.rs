#[doc = "Reader of register FREQUENCY"]
pub type R = crate::R<u32, super::FREQUENCY>;
#[doc = "Writer for register FREQUENCY"]
pub type W = crate::W<u32, super::FREQUENCY>;
#[doc = "Register FREQUENCY `reset()`'s with value 0x02"]
impl crate::ResetValue for super::FREQUENCY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x02
    }
}
#[doc = "Reader of field `FREQUENCY`"]
pub type FREQUENCY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FREQUENCY`"]
pub struct FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> FREQUENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Radio channel frequency offset in MHz: RF Frequency = 2400 + FREQUENCY (MHz). Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn frequency(&self) -> FREQUENCY_R {
        FREQUENCY_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Radio channel frequency offset in MHz: RF Frequency = 2400 + FREQUENCY (MHz). Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn frequency(&mut self) -> FREQUENCY_W {
        FREQUENCY_W { w: self }
    }
}
