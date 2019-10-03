#[doc = "Reader of register OVERRIDE2"]
pub type R = crate::R<u32, super::OVERRIDE2>;
#[doc = "Writer for register OVERRIDE2"]
pub type W = crate::W<u32, super::OVERRIDE2>;
#[doc = "Register OVERRIDE2 `reset()`'s with value 0"]
impl crate::ResetValue for super::OVERRIDE2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVERRIDE2`"]
pub type OVERRIDE2_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OVERRIDE2`"]
pub struct OVERRIDE2_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Trim value override 2."]
    #[inline(always)]
    pub fn override2(&self) -> OVERRIDE2_R {
        OVERRIDE2_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 2."]
    #[inline(always)]
    pub fn override2(&mut self) -> OVERRIDE2_W {
        OVERRIDE2_W { w: self }
    }
}
