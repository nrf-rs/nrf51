#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Disable interrupt on DATARDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATARDY_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<DATARDY_A> for bool {
    #[inline(always)]
    fn from(variant: DATARDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATARDY` reader - Disable interrupt on DATARDY event."]
pub struct DATARDY_R(crate::FieldReader<bool, DATARDY_A>);
impl DATARDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATARDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATARDY_A {
        match self.bits {
            false => DATARDY_A::DISABLED,
            true => DATARDY_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == DATARDY_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == DATARDY_A::ENABLED
    }
}
impl core::ops::Deref for DATARDY_R {
    type Target = crate::FieldReader<bool, DATARDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on DATARDY event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATARDY_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<DATARDY_AW> for bool {
    #[inline(always)]
    fn from(variant: DATARDY_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATARDY` writer - Disable interrupt on DATARDY event."]
pub struct DATARDY_W<'a> {
    w: &'a mut W,
}
impl<'a> DATARDY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATARDY_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(DATARDY_AW::CLEAR)
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
impl R {
    #[doc = "Bit 0 - Disable interrupt on DATARDY event."]
    #[inline(always)]
    pub fn datardy(&self) -> DATARDY_R {
        DATARDY_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on DATARDY event."]
    #[inline(always)]
    pub fn datardy(&mut self) -> DATARDY_W {
        DATARDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable clear register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
