#[doc = "Register `RXMATCH` reader"]
pub struct R(crate::R<RXMATCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXMATCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXMATCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXMATCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXMATCH` reader - Logical address in which previous packet was received."]
pub struct RXMATCH_R(crate::FieldReader<u8, u8>);
impl RXMATCH_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXMATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMATCH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:2 - Logical address in which previous packet was received."]
    #[inline(always)]
    pub fn rxmatch(&self) -> RXMATCH_R {
        RXMATCH_R::new((self.bits & 0x07) as u8)
    }
}
#[doc = "Received address.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxmatch](index.html) module"]
pub struct RXMATCH_SPEC;
impl crate::RegisterSpec for RXMATCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxmatch::R](R) reader structure"]
impl crate::Readable for RXMATCH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXMATCH to value 0"]
impl crate::Resettable for RXMATCH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
