#[doc = "Register `SYSTEMOFF` writer"]
pub struct W(crate::W<SYSTEMOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSTEMOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYSTEMOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSTEMOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enter system off mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSTEMOFF_AW {
    #[doc = "1: Enter system off mode."]
    ENTER = 1,
}
impl From<SYSTEMOFF_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSTEMOFF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTEMOFF` writer - Enter system off mode."]
pub struct SYSTEMOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEMOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSTEMOFF_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enter system off mode."]
    #[inline(always)]
    pub fn enter(self) -> &'a mut W {
        self.variant(SYSTEMOFF_AW::ENTER)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Enter system off mode."]
    #[inline(always)]
    pub fn systemoff(&mut self) -> SYSTEMOFF_W {
        SYSTEMOFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System off register.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [systemoff](index.html) module"]
pub struct SYSTEMOFF_SPEC;
impl crate::RegisterSpec for SYSTEMOFF_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [systemoff::W](W) writer structure"]
impl crate::Writable for SYSTEMOFF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSTEMOFF to value 0"]
impl crate::Resettable for SYSTEMOFF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
