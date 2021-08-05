#[doc = "Register `DATAWHITEIV` reader"]
pub struct R(crate::R<DATAWHITEIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAWHITEIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAWHITEIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAWHITEIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAWHITEIV` writer"]
pub struct W(crate::W<DATAWHITEIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAWHITEIV_SPEC>;
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
impl From<crate::W<DATAWHITEIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAWHITEIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAWHITEIV` reader - Data whitening initial value. Bit 0 corresponds to Position 0 of the LSFR, Bit 1 to position 5... Decision point: TXEN or RXEN task."]
pub struct DATAWHITEIV_R(crate::FieldReader<u8, u8>);
impl DATAWHITEIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        DATAWHITEIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAWHITEIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAWHITEIV` writer - Data whitening initial value. Bit 0 corresponds to Position 0 of the LSFR, Bit 1 to position 5... Decision point: TXEN or RXEN task."]
pub struct DATAWHITEIV_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAWHITEIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data whitening initial value.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datawhiteiv](index.html) module"]
pub struct DATAWHITEIV_SPEC;
impl crate::RegisterSpec for DATAWHITEIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datawhiteiv::R](R) reader structure"]
impl crate::Readable for DATAWHITEIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datawhiteiv::W](W) writer structure"]
impl crate::Writable for DATAWHITEIV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATAWHITEIV to value 0x40"]
impl crate::Resettable for DATAWHITEIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
