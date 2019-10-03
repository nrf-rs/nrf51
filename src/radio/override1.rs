#[doc = "Reader of register OVERRIDE1"]
pub type R = crate::R<u32, super::OVERRIDE1>;
#[doc = "Writer for register OVERRIDE1"]
pub type W = crate::W<u32, super::OVERRIDE1>;
#[doc = "Register OVERRIDE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::OVERRIDE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVERRIDE1`"]
pub type OVERRIDE1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OVERRIDE1`"]
pub struct OVERRIDE1_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Trim value override 1."]
    #[inline(always)]
    pub fn override1(&self) -> OVERRIDE1_R {
        OVERRIDE1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 1."]
    #[inline(always)]
    pub fn override1(&mut self) -> OVERRIDE1_W {
        OVERRIDE1_W { w: self }
    }
}
