#[doc = "Register `INTENSET` reader"]
pub struct R(crate::R<INTENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENSET` writer"]
pub struct W(crate::W<INTENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENSET_SPEC>;
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
impl From<crate::W<INTENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable interrupt on ENDKSGEN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDKSGEN_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ENDKSGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ENDKSGEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDKSGEN` reader - Enable interrupt on ENDKSGEN event."]
pub struct ENDKSGEN_R(crate::FieldReader<bool, ENDKSGEN_A>);
impl ENDKSGEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDKSGEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDKSGEN_A {
        match self.bits {
            false => ENDKSGEN_A::DISABLED,
            true => ENDKSGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDKSGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDKSGEN_A::ENABLED
    }
}
impl core::ops::Deref for ENDKSGEN_R {
    type Target = crate::FieldReader<bool, ENDKSGEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable interrupt on ENDKSGEN event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDKSGEN_AW {
    #[doc = "1: Enable interrupt on write."]
    SET = 1,
}
impl From<ENDKSGEN_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDKSGEN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDKSGEN` writer - Enable interrupt on ENDKSGEN event."]
pub struct ENDKSGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDKSGEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDKSGEN_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDKSGEN_AW::SET)
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
#[doc = "Enable interrupt on ENDCRYPT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDCRYPT_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ENDCRYPT_A> for bool {
    #[inline(always)]
    fn from(variant: ENDCRYPT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDCRYPT` reader - Enable interrupt on ENDCRYPT event."]
pub struct ENDCRYPT_R(crate::FieldReader<bool, ENDCRYPT_A>);
impl ENDCRYPT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENDCRYPT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDCRYPT_A {
        match self.bits {
            false => ENDCRYPT_A::DISABLED,
            true => ENDCRYPT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ENDCRYPT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ENDCRYPT_A::ENABLED
    }
}
impl core::ops::Deref for ENDCRYPT_R {
    type Target = crate::FieldReader<bool, ENDCRYPT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable interrupt on ENDCRYPT event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENDCRYPT_AW {
    #[doc = "1: Enable interrupt on write."]
    SET = 1,
}
impl From<ENDCRYPT_AW> for bool {
    #[inline(always)]
    fn from(variant: ENDCRYPT_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENDCRYPT` writer - Enable interrupt on ENDCRYPT event."]
pub struct ENDCRYPT_W<'a> {
    w: &'a mut W,
}
impl<'a> ENDCRYPT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENDCRYPT_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ENDCRYPT_AW::SET)
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
#[doc = "Enable interrupt on ERROR event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_A {
    #[doc = "0: Interrupt disabled."]
    DISABLED = 0,
    #[doc = "1: Interrupt enabled."]
    ENABLED = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Enable interrupt on ERROR event."]
pub struct ERROR_R(crate::FieldReader<bool, ERROR_A>);
impl ERROR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERROR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::DISABLED,
            true => ERROR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ERROR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == ERROR_A::ENABLED
    }
}
impl core::ops::Deref for ERROR_R {
    type Target = crate::FieldReader<bool, ERROR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable interrupt on ERROR event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERROR_AW {
    #[doc = "1: Enable interrupt on write."]
    SET = 1,
}
impl From<ERROR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERROR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Enable interrupt on ERROR event."]
pub struct ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERROR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERROR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable interrupt on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(ERROR_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable interrupt on ENDKSGEN event."]
    #[inline(always)]
    pub fn endksgen(&self) -> ENDKSGEN_R {
        ENDKSGEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable interrupt on ENDCRYPT event."]
    #[inline(always)]
    pub fn endcrypt(&self) -> ENDCRYPT_R {
        ENDCRYPT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable interrupt on ERROR event."]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable interrupt on ENDKSGEN event."]
    #[inline(always)]
    pub fn endksgen(&mut self) -> ENDKSGEN_W {
        ENDKSGEN_W { w: self }
    }
    #[doc = "Bit 1 - Enable interrupt on ENDCRYPT event."]
    #[inline(always)]
    pub fn endcrypt(&mut self) -> ENDCRYPT_W {
        ENDCRYPT_W { w: self }
    }
    #[doc = "Bit 2 - Enable interrupt on ERROR event."]
    #[inline(always)]
    pub fn error(&mut self) -> ERROR_W {
        ERROR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable set register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenset](index.html) module"]
pub struct INTENSET_SPEC;
impl crate::RegisterSpec for INTENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenset::R](R) reader structure"]
impl crate::Readable for INTENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenset::W](W) writer structure"]
impl crate::Writable for INTENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for INTENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
