#[doc = "Register `SHORTS` reader"]
pub struct R(crate::R<SHORTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHORTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHORTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHORTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHORTS` writer"]
pub struct W(crate::W<SHORTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHORTS_SPEC>;
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
impl From<crate::W<SHORTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHORTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Shortcut between BB event and the SUSPEND task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_SUSPEND_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<BB_SUSPEND_A> for bool {
    #[inline(always)]
    fn from(variant: BB_SUSPEND_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB_SUSPEND` reader - Shortcut between BB event and the SUSPEND task."]
pub struct BB_SUSPEND_R(crate::FieldReader<bool, BB_SUSPEND_A>);
impl BB_SUSPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        BB_SUSPEND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_SUSPEND_A {
        match self.bits {
            false => BB_SUSPEND_A::DISABLED,
            true => BB_SUSPEND_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BB_SUSPEND_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BB_SUSPEND_A::ENABLED
    }
}
impl core::ops::Deref for BB_SUSPEND_R {
    type Target = crate::FieldReader<bool, BB_SUSPEND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BB_SUSPEND` writer - Shortcut between BB event and the SUSPEND task."]
pub struct BB_SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_SUSPEND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_SUSPEND_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BB_SUSPEND_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BB_SUSPEND_A::ENABLED)
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
#[doc = "Shortcut between BB event and the STOP task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BB_STOP_A {
    #[doc = "0: Shortcut disabled."]
    DISABLED = 0,
    #[doc = "1: Shortcut enabled."]
    ENABLED = 1,
}
impl From<BB_STOP_A> for bool {
    #[inline(always)]
    fn from(variant: BB_STOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB_STOP` reader - Shortcut between BB event and the STOP task."]
pub struct BB_STOP_R(crate::FieldReader<bool, BB_STOP_A>);
impl BB_STOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        BB_STOP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BB_STOP_A {
        match self.bits {
            false => BB_STOP_A::DISABLED,
            true => BB_STOP_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == BB_STOP_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == BB_STOP_A::ENABLED
    }
}
impl core::ops::Deref for BB_STOP_R {
    type Target = crate::FieldReader<bool, BB_STOP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BB_STOP` writer - Shortcut between BB event and the STOP task."]
pub struct BB_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_STOP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BB_STOP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Shortcut disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BB_STOP_A::DISABLED)
    }
    #[doc = "Shortcut enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BB_STOP_A::ENABLED)
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
    #[doc = "Bit 0 - Shortcut between BB event and the SUSPEND task."]
    #[inline(always)]
    pub fn bb_suspend(&self) -> BB_SUSPEND_R {
        BB_SUSPEND_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Shortcut between BB event and the STOP task."]
    #[inline(always)]
    pub fn bb_stop(&self) -> BB_STOP_R {
        BB_STOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shortcut between BB event and the SUSPEND task."]
    #[inline(always)]
    pub fn bb_suspend(&mut self) -> BB_SUSPEND_W {
        BB_SUSPEND_W { w: self }
    }
    #[doc = "Bit 1 - Shortcut between BB event and the STOP task."]
    #[inline(always)]
    pub fn bb_stop(&mut self) -> BB_STOP_W {
        BB_STOP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shortcuts for TWI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shorts](index.html) module"]
pub struct SHORTS_SPEC;
impl crate::RegisterSpec for SHORTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shorts::R](R) reader structure"]
impl crate::Readable for SHORTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shorts::W](W) writer structure"]
impl crate::Writable for SHORTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for SHORTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
