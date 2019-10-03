#[doc = "Reader of register DATAWHITEIV"]
pub type R = crate::R<u32, super::DATAWHITEIV>;
#[doc = "Writer for register DATAWHITEIV"]
pub type W = crate::W<u32, super::DATAWHITEIV>;
#[doc = "Register DATAWHITEIV `reset()`'s with value 0x40"]
impl crate::ResetValue for super::DATAWHITEIV {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x40
    }
}
#[doc = "Reader of field `DATAWHITEIV`"]
pub type DATAWHITEIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATAWHITEIV`"]
pub struct DATAWHITEIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAWHITEIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Data whitening initial value. Bit 0 corresponds to Position 0 of the LSFR, Bit 1 to position 5... Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn datawhiteiv(&self) -> DATAWHITEIV_R {
        DATAWHITEIV_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Data whitening initial value. Bit 0 corresponds to Position 0 of the LSFR, Bit 1 to position 5... Decision point: TXEN or RXEN task."]
    #[inline(always)]
    pub fn datawhiteiv(&mut self) -> DATAWHITEIV_W {
        DATAWHITEIV_W { w: self }
    }
}
