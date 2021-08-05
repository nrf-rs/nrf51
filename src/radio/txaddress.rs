#[doc = "Register `TXADDRESS` reader"]
pub struct R(crate::R<TXADDRESS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXADDRESS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXADDRESS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXADDRESS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXADDRESS` writer"]
pub struct W(crate::W<TXADDRESS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXADDRESS_SPEC>;
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
impl From<crate::W<TXADDRESS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXADDRESS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXADDRESS` reader - Logical address to be used when transmitting a packet. Decision point: START task."]
pub struct TXADDRESS_R(crate::FieldReader<u8, u8>);
impl TXADDRESS_R {
    pub(crate) fn new(bits: u8) -> Self {
        TXADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXADDRESS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXADDRESS` writer - Logical address to be used when transmitting a packet. Decision point: START task."]
pub struct TXADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Logical address to be used when transmitting a packet. Decision point: START task."]
    #[inline(always)]
    pub fn txaddress(&self) -> TXADDRESS_R {
        TXADDRESS_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Logical address to be used when transmitting a packet. Decision point: START task."]
    #[inline(always)]
    pub fn txaddress(&mut self) -> TXADDRESS_W {
        TXADDRESS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit address select.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txaddress](index.html) module"]
pub struct TXADDRESS_SPEC;
impl crate::RegisterSpec for TXADDRESS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txaddress::R](R) reader structure"]
impl crate::Readable for TXADDRESS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txaddress::W](W) writer structure"]
impl crate::Writable for TXADDRESS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TXADDRESS to value 0"]
impl crate::Resettable for TXADDRESS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
