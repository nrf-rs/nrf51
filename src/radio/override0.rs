#[doc = "Reader of register OVERRIDE0"]
pub type R = crate::R<u32, super::OVERRIDE0>;
#[doc = "Writer for register OVERRIDE0"]
pub type W = crate::W<u32, super::OVERRIDE0>;
#[doc = "Register OVERRIDE0 `reset()`'s with value 0"]
impl crate::ResetValue for super::OVERRIDE0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OVERRIDE0`"]
pub type OVERRIDE0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `OVERRIDE0`"]
pub struct OVERRIDE0_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERRIDE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Trim value override 0."]
    #[inline(always)]
    pub fn override0(&self) -> OVERRIDE0_R {
        OVERRIDE0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trim value override 0."]
    #[inline(always)]
    pub fn override0(&mut self) -> OVERRIDE0_W {
        OVERRIDE0_W { w: self }
    }
}
