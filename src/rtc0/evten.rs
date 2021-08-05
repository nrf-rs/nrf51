#[doc = "Register `EVTEN` reader"]
pub struct R(crate::R<EVTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVTEN` writer"]
pub struct W(crate::W<EVTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVTEN_SPEC>;
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
impl From<crate::W<EVTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "TICK event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TICK_A {
    #[doc = "0: Event disabled."]
    DISABLED = 0,
    #[doc = "1: Event enabled."]
    ENABLED = 1,
}
impl From<TICK_A> for bool {
    #[inline(always)]
    fn from(variant: TICK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TICK` reader - TICK event enable."]
pub struct TICK_R(crate::FieldReader<bool, TICK_A>);
impl TICK_R {
    pub(crate) fn new(bits: bool) -> Self {
        TICK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TICK_A {
        match self.bits {
            false => TICK_A::DISABLED,
            true => TICK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TICK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TICK_A::ENABLED
    }
}
impl core::ops::Deref for TICK_R {
    type Target = crate::FieldReader<bool, TICK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICK` writer - TICK event enable."]
pub struct TICK_W<'a> {
    w: &'a mut W,
}
impl<'a> TICK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TICK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TICK_A::DISABLED)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TICK_A::ENABLED)
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
#[doc = "OVRFLW event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRFLW_A {
    #[doc = "0: Event disabled."]
    DISABLED = 0,
    #[doc = "1: Event enabled."]
    ENABLED = 1,
}
impl From<OVRFLW_A> for bool {
    #[inline(always)]
    fn from(variant: OVRFLW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRFLW` reader - OVRFLW event enable."]
pub struct OVRFLW_R(crate::FieldReader<bool, OVRFLW_A>);
impl OVRFLW_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVRFLW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRFLW_A {
        match self.bits {
            false => OVRFLW_A::DISABLED,
            true => OVRFLW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == OVRFLW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == OVRFLW_A::ENABLED
    }
}
impl core::ops::Deref for OVRFLW_R {
    type Target = crate::FieldReader<bool, OVRFLW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVRFLW` writer - OVRFLW event enable."]
pub struct OVRFLW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRFLW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVRFLW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRFLW_A::DISABLED)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRFLW_A::ENABLED)
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
#[doc = "COMPARE\\[0\\]
event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE0_A {
    #[doc = "0: Event disabled."]
    DISABLED = 0,
    #[doc = "1: Event enabled."]
    ENABLED = 1,
}
impl From<COMPARE0_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE0` reader - COMPARE\\[0\\]
event enable."]
pub struct COMPARE0_R(crate::FieldReader<bool, COMPARE0_A>);
impl COMPARE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE0_A {
        match self.bits {
            false => COMPARE0_A::DISABLED,
            true => COMPARE0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE0_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE0_R {
    type Target = crate::FieldReader<bool, COMPARE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE0` writer - COMPARE\\[0\\]
event enable."]
pub struct COMPARE0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE0_A::DISABLED)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE0_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "COMPARE\\[1\\]
event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE1_A {
    #[doc = "0: Event disabled."]
    DISABLED = 0,
    #[doc = "1: Event enabled."]
    ENABLED = 1,
}
impl From<COMPARE1_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE1` reader - COMPARE\\[1\\]
event enable."]
pub struct COMPARE1_R(crate::FieldReader<bool, COMPARE1_A>);
impl COMPARE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE1_A {
        match self.bits {
            false => COMPARE1_A::DISABLED,
            true => COMPARE1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE1_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE1_R {
    type Target = crate::FieldReader<bool, COMPARE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE1` writer - COMPARE\\[1\\]
event enable."]
pub struct COMPARE1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE1_A::DISABLED)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE1_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "COMPARE\\[2\\]
event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE2_A {
    #[doc = "0: Event disabled."]
    DISABLED = 0,
    #[doc = "1: Event enabled."]
    ENABLED = 1,
}
impl From<COMPARE2_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE2` reader - COMPARE\\[2\\]
event enable."]
pub struct COMPARE2_R(crate::FieldReader<bool, COMPARE2_A>);
impl COMPARE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE2_A {
        match self.bits {
            false => COMPARE2_A::DISABLED,
            true => COMPARE2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE2_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE2_R {
    type Target = crate::FieldReader<bool, COMPARE2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE2` writer - COMPARE\\[2\\]
event enable."]
pub struct COMPARE2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE2_A::DISABLED)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE2_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "COMPARE\\[3\\]
event enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMPARE3_A {
    #[doc = "0: Event disabled."]
    DISABLED = 0,
    #[doc = "1: Event enabled."]
    ENABLED = 1,
}
impl From<COMPARE3_A> for bool {
    #[inline(always)]
    fn from(variant: COMPARE3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMPARE3` reader - COMPARE\\[3\\]
event enable."]
pub struct COMPARE3_R(crate::FieldReader<bool, COMPARE3_A>);
impl COMPARE3_R {
    pub(crate) fn new(bits: bool) -> Self {
        COMPARE3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMPARE3_A {
        match self.bits {
            false => COMPARE3_A::DISABLED,
            true => COMPARE3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == COMPARE3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == COMPARE3_A::ENABLED
    }
}
impl core::ops::Deref for COMPARE3_R {
    type Target = crate::FieldReader<bool, COMPARE3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPARE3` writer - COMPARE\\[3\\]
event enable."]
pub struct COMPARE3_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPARE3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: COMPARE3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Event disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMPARE3_A::DISABLED)
    }
    #[doc = "Event enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMPARE3_A::ENABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TICK event enable."]
    #[inline(always)]
    pub fn tick(&self) -> TICK_R {
        TICK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - OVRFLW event enable."]
    #[inline(always)]
    pub fn ovrflw(&self) -> OVRFLW_R {
        OVRFLW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - COMPARE\\[0\\]
event enable."]
    #[inline(always)]
    pub fn compare0(&self) -> COMPARE0_R {
        COMPARE0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - COMPARE\\[1\\]
event enable."]
    #[inline(always)]
    pub fn compare1(&self) -> COMPARE1_R {
        COMPARE1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - COMPARE\\[2\\]
event enable."]
    #[inline(always)]
    pub fn compare2(&self) -> COMPARE2_R {
        COMPARE2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - COMPARE\\[3\\]
event enable."]
    #[inline(always)]
    pub fn compare3(&self) -> COMPARE3_R {
        COMPARE3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TICK event enable."]
    #[inline(always)]
    pub fn tick(&mut self) -> TICK_W {
        TICK_W { w: self }
    }
    #[doc = "Bit 1 - OVRFLW event enable."]
    #[inline(always)]
    pub fn ovrflw(&mut self) -> OVRFLW_W {
        OVRFLW_W { w: self }
    }
    #[doc = "Bit 16 - COMPARE\\[0\\]
event enable."]
    #[inline(always)]
    pub fn compare0(&mut self) -> COMPARE0_W {
        COMPARE0_W { w: self }
    }
    #[doc = "Bit 17 - COMPARE\\[1\\]
event enable."]
    #[inline(always)]
    pub fn compare1(&mut self) -> COMPARE1_W {
        COMPARE1_W { w: self }
    }
    #[doc = "Bit 18 - COMPARE\\[2\\]
event enable."]
    #[inline(always)]
    pub fn compare2(&mut self) -> COMPARE2_W {
        COMPARE2_W { w: self }
    }
    #[doc = "Bit 19 - COMPARE\\[3\\]
event enable."]
    #[inline(always)]
    pub fn compare3(&mut self) -> COMPARE3_W {
        COMPARE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures event enable routing to PPI for each RTC event.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evten](index.html) module"]
pub struct EVTEN_SPEC;
impl crate::RegisterSpec for EVTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evten::R](R) reader structure"]
impl crate::Readable for EVTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evten::W](W) writer structure"]
impl crate::Writable for EVTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVTEN to value 0"]
impl crate::Resettable for EVTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
