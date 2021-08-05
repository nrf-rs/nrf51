#[doc = "Register `CRV` reader"]
pub struct R(crate::R<CRV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRV` writer"]
pub struct W(crate::W<CRV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRV_SPEC>;
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
impl From<crate::W<CRV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRV_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter reload value in number of 32kiHz clock cycles.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crv](index.html) module"]
pub struct CRV_SPEC;
impl crate::RegisterSpec for CRV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crv::R](R) reader structure"]
impl crate::Readable for CRV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crv::W](W) writer structure"]
impl crate::Writable for CRV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRV to value 0xffff_ffff"]
impl crate::Resettable for CRV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
