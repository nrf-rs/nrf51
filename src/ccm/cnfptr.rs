#[doc = "Register `CNFPTR` reader"]
pub struct R(crate::R<CNFPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNFPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNFPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNFPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNFPTR` writer"]
pub struct W(crate::W<CNFPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNFPTR_SPEC>;
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
impl From<crate::W<CNFPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNFPTR_SPEC>) -> Self {
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
#[doc = "Pointer to a data structure holding AES key and NONCE vector.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnfptr](index.html) module"]
pub struct CNFPTR_SPEC;
impl crate::RegisterSpec for CNFPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnfptr::R](R) reader structure"]
impl crate::Readable for CNFPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnfptr::W](W) writer structure"]
impl crate::Writable for CNFPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNFPTR to value 0"]
impl crate::Resettable for CNFPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
