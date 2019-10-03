#[doc = "Reader of register OVERRIDE3"]
pub type R = crate::R<u32, super::OVERRIDE3>;
#[doc = "Writer for register OVERRIDE3"]
pub type W = crate::W<u32, super::OVERRIDE3>;
#[doc = "Register OVERRIDE3 `reset()`'s with value 0"]
impl crate::ResetValue for super::OVERRIDE3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVERRIDE3`"]
pub type OVERRIDE3_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OVERRIDE3`"]
pub struct OVERRIDE3_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Trim value override 3."]
    #[inline(always)]
    pub fn override3(&self) -> OVERRIDE3_R {
        OVERRIDE3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 3."]
    #[inline(always)]
    pub fn override3(&mut self) -> OVERRIDE3_W {
        OVERRIDE3_W { w: self }
    }
}
