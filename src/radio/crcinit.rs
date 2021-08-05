#[doc = "Register `CRCINIT` reader"]
pub struct R(crate::R<CRCINIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCINIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCINIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCINIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRCINIT` writer"]
pub struct W(crate::W<CRCINIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCINIT_SPEC>;
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
impl From<crate::W<CRCINIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCINIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRCINIT` reader - Initial value for CRC calculation. Decision point: START task."]
pub struct CRCINIT_R(crate::FieldReader<u32, u32>);
impl CRCINIT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CRCINIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRCINIT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRCINIT` writer - Initial value for CRC calculation. Decision point: START task."]
pub struct CRCINIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCINIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | (value as u32 & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - Initial value for CRC calculation. Decision point: START task."]
    #[inline(always)]
    pub fn crcinit(&self) -> CRCINIT_R {
        CRCINIT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - Initial value for CRC calculation. Decision point: START task."]
    #[inline(always)]
    pub fn crcinit(&mut self) -> CRCINIT_W {
        CRCINIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC initial value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcinit](index.html) module"]
pub struct CRCINIT_SPEC;
impl crate::RegisterSpec for CRCINIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcinit::R](R) reader structure"]
impl crate::Readable for CRCINIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crcinit::W](W) writer structure"]
impl crate::Writable for CRCINIT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRCINIT to value 0"]
impl crate::Resettable for CRCINIT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
