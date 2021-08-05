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
#[doc = "Disable interrupt on ENDECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDECB_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ENDECB_A> for bool {
    #[inline(always)]
    fn from(variant: ENDECB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDECB` reader - Disable interrupt on ENDECB event."]
pub struct ENDECB_R(crate::FieldReader<bool, ENDECB_A>);
impl ENDECB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDECB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDECB_A {
        match self.bits {
            false => ENDECB_A::DISABLED,
            true => ENDECB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDECB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDECB_A::ENABLED
    }
}
impl core::ops::Deref for ENDECB_R {
    type Target = crate::FieldReader<bool, ENDECB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on ENDECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDECB_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<ENDECB_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDECB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDECB` writer - Disable interrupt on ENDECB event."]
pub struct ENDECB_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDECB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDECB_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ENDECB_AW::CLEAR)
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
#[doc = "Disable interrupt on ERRORECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORECB_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ERRORECB_A> for bool {
    #[inline(always)]
    fn from(variant: ERRORECB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORECB` reader - Disable interrupt on ERRORECB event."]
pub struct ERRORECB_R(crate::FieldReader<bool, ERRORECB_A>);
impl ERRORECB_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRORECB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRORECB_A {
        match self.bits {
            false => ERRORECB_A::DISABLED,
            true => ERRORECB_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ERRORECB_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ERRORECB_A::ENABLED
    }
}
impl core::ops::Deref for ERRORECB_R {
    type Target = crate::FieldReader<bool, ERRORECB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Disable interrupt on ERRORECB event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRORECB_AW {
    #[doc = "1: Disable interrupt on write."]
    CLEAR = 1,
}
impl From<ERRORECB_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRORECB_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRORECB` writer - Disable interrupt on ERRORECB event."]
pub struct ERRORECB_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRORECB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRORECB_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable interrupt on write."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ERRORECB_AW::CLEAR)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Disable interrupt on ENDECB event."]
    #[inline(always)]
    pub fn endecb(&self) -> ENDECB_R {
        ENDECB_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable interrupt on ERRORECB event."]
    #[inline(always)]
    pub fn errorecb(&self) -> ERRORECB_R {
        ERRORECB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable interrupt on ENDECB event."]
    #[inline(always)]
    pub fn endecb(&mut self) -> ENDECB_W {
        ENDECB_W { w: self }
    }
    #[doc = "Bit 1 - Disable interrupt on ERRORECB event."]
    #[inline(always)]
    pub fn errorecb(&mut self) -> ERRORECB_W {
        ERRORECB_W { w: self }
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
