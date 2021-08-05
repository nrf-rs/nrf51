#[doc = "Register `RESET` reader"]
pub struct R(crate::R<RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESET` writer"]
pub struct W(crate::W<RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESET_SPEC>;
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
impl From<crate::W<RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable or disable pin reset in debug interface mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESET_A {
    #[doc = "0: Pin reset in debug interface mode disabled."]
    DISABLED = 0,
    #[doc = "1: Pin reset in debug interface mode enabled."]
    ENABLED = 1,
}
impl From<RESET_A> for bool {
    #[inline(always)]
    fn from(variant: RESET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET` reader - Enable or disable pin reset in debug interface mode."]
pub struct RESET_R(crate::FieldReader<bool, RESET_A>);
impl RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESET_A {
        match self.bits {
            false => RESET_A::DISABLED,
            true => RESET_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RESET_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RESET_A::ENABLED
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, RESET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - Enable or disable pin reset in debug interface mode."]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RESET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Pin reset in debug interface mode disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RESET_A::DISABLED)
    }
    #[doc = "Pin reset in debug interface mode enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RESET_A::ENABLED)
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
    #[doc = "Bit 0 - Enable or disable pin reset in debug interface mode."]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable pin reset in debug interface mode."]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pin reset functionality configuration register. This register is a retained register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reset](index.html) module"]
pub struct RESET_SPEC;
impl crate::RegisterSpec for RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reset::R](R) reader structure"]
impl crate::Readable for RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reset::W](W) writer structure"]
impl crate::Writable for RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESET to value 0"]
impl crate::Resettable for RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
