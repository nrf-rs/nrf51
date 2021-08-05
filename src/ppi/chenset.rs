#[doc = "Register `CHENSET` reader"]
pub struct R(crate::R<CHENSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHENSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHENSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHENSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHENSET` writer"]
pub struct W(crate::W<CHENSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHENSET_SPEC>;
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
impl From<crate::W<CHENSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHENSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable PPI channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Enable PPI channel 0."]
pub struct CH0_R(crate::FieldReader<bool, CH0_A>);
impl CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            false => CH0_A::DISABLED,
            true => CH0_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH0_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH0_A::ENABLED
    }
}
impl core::ops::Deref for CH0_R {
    type Target = crate::FieldReader<bool, CH0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH0_AW> for bool {
    #[inline(always)]
    fn from(variant: CH0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` writer - Enable PPI channel 0."]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH0_AW::SET)
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
#[doc = "Enable PPI channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` reader - Enable PPI channel 1."]
pub struct CH1_R(crate::FieldReader<bool, CH1_A>);
impl CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            false => CH1_A::DISABLED,
            true => CH1_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH1_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH1_A::ENABLED
    }
}
impl core::ops::Deref for CH1_R {
    type Target = crate::FieldReader<bool, CH1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH1_AW> for bool {
    #[inline(always)]
    fn from(variant: CH1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` writer - Enable PPI channel 1."]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH1_AW::SET)
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
#[doc = "Enable PPI channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` reader - Enable PPI channel 2."]
pub struct CH2_R(crate::FieldReader<bool, CH2_A>);
impl CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            false => CH2_A::DISABLED,
            true => CH2_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH2_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH2_A::ENABLED
    }
}
impl core::ops::Deref for CH2_R {
    type Target = crate::FieldReader<bool, CH2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH2_AW> for bool {
    #[inline(always)]
    fn from(variant: CH2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` writer - Enable PPI channel 2."]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH2_AW::SET)
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
#[doc = "Enable PPI channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` reader - Enable PPI channel 3."]
pub struct CH3_R(crate::FieldReader<bool, CH3_A>);
impl CH3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            false => CH3_A::DISABLED,
            true => CH3_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH3_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH3_A::ENABLED
    }
}
impl core::ops::Deref for CH3_R {
    type Target = crate::FieldReader<bool, CH3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH3_AW> for bool {
    #[inline(always)]
    fn from(variant: CH3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` writer - Enable PPI channel 3."]
pub struct CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH3_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Enable PPI channel 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH4_A> for bool {
    #[inline(always)]
    fn from(variant: CH4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` reader - Enable PPI channel 4."]
pub struct CH4_R(crate::FieldReader<bool, CH4_A>);
impl CH4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4_A {
        match self.bits {
            false => CH4_A::DISABLED,
            true => CH4_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH4_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH4_A::ENABLED
    }
}
impl core::ops::Deref for CH4_R {
    type Target = crate::FieldReader<bool, CH4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH4_AW> for bool {
    #[inline(always)]
    fn from(variant: CH4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` writer - Enable PPI channel 4."]
pub struct CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH4_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Enable PPI channel 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH5_A> for bool {
    #[inline(always)]
    fn from(variant: CH5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` reader - Enable PPI channel 5."]
pub struct CH5_R(crate::FieldReader<bool, CH5_A>);
impl CH5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5_A {
        match self.bits {
            false => CH5_A::DISABLED,
            true => CH5_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH5_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH5_A::ENABLED
    }
}
impl core::ops::Deref for CH5_R {
    type Target = crate::FieldReader<bool, CH5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH5_AW> for bool {
    #[inline(always)]
    fn from(variant: CH5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` writer - Enable PPI channel 5."]
pub struct CH5_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH5_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Enable PPI channel 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH6_A> for bool {
    #[inline(always)]
    fn from(variant: CH6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` reader - Enable PPI channel 6."]
pub struct CH6_R(crate::FieldReader<bool, CH6_A>);
impl CH6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6_A {
        match self.bits {
            false => CH6_A::DISABLED,
            true => CH6_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH6_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH6_A::ENABLED
    }
}
impl core::ops::Deref for CH6_R {
    type Target = crate::FieldReader<bool, CH6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH6_AW> for bool {
    #[inline(always)]
    fn from(variant: CH6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` writer - Enable PPI channel 6."]
pub struct CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH6_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Enable PPI channel 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH7_A> for bool {
    #[inline(always)]
    fn from(variant: CH7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` reader - Enable PPI channel 7."]
pub struct CH7_R(crate::FieldReader<bool, CH7_A>);
impl CH7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7_A {
        match self.bits {
            false => CH7_A::DISABLED,
            true => CH7_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH7_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH7_A::ENABLED
    }
}
impl core::ops::Deref for CH7_R {
    type Target = crate::FieldReader<bool, CH7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH7_AW> for bool {
    #[inline(always)]
    fn from(variant: CH7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` writer - Enable PPI channel 7."]
pub struct CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH7_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Enable PPI channel 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH8_A> for bool {
    #[inline(always)]
    fn from(variant: CH8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH8` reader - Enable PPI channel 8."]
pub struct CH8_R(crate::FieldReader<bool, CH8_A>);
impl CH8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH8_A {
        match self.bits {
            false => CH8_A::DISABLED,
            true => CH8_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH8_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH8_A::ENABLED
    }
}
impl core::ops::Deref for CH8_R {
    type Target = crate::FieldReader<bool, CH8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH8_AW> for bool {
    #[inline(always)]
    fn from(variant: CH8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH8` writer - Enable PPI channel 8."]
pub struct CH8_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH8_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH8_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Enable PPI channel 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH9_A> for bool {
    #[inline(always)]
    fn from(variant: CH9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH9` reader - Enable PPI channel 9."]
pub struct CH9_R(crate::FieldReader<bool, CH9_A>);
impl CH9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH9_A {
        match self.bits {
            false => CH9_A::DISABLED,
            true => CH9_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH9_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH9_A::ENABLED
    }
}
impl core::ops::Deref for CH9_R {
    type Target = crate::FieldReader<bool, CH9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH9_AW> for bool {
    #[inline(always)]
    fn from(variant: CH9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH9` writer - Enable PPI channel 9."]
pub struct CH9_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH9_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH9_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Enable PPI channel 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH10_A> for bool {
    #[inline(always)]
    fn from(variant: CH10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH10` reader - Enable PPI channel 10."]
pub struct CH10_R(crate::FieldReader<bool, CH10_A>);
impl CH10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH10_A {
        match self.bits {
            false => CH10_A::DISABLED,
            true => CH10_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH10_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH10_A::ENABLED
    }
}
impl core::ops::Deref for CH10_R {
    type Target = crate::FieldReader<bool, CH10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH10_AW> for bool {
    #[inline(always)]
    fn from(variant: CH10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH10` writer - Enable PPI channel 10."]
pub struct CH10_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH10_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH10_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Enable PPI channel 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH11_A> for bool {
    #[inline(always)]
    fn from(variant: CH11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH11` reader - Enable PPI channel 11."]
pub struct CH11_R(crate::FieldReader<bool, CH11_A>);
impl CH11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH11_A {
        match self.bits {
            false => CH11_A::DISABLED,
            true => CH11_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH11_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH11_A::ENABLED
    }
}
impl core::ops::Deref for CH11_R {
    type Target = crate::FieldReader<bool, CH11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH11_AW> for bool {
    #[inline(always)]
    fn from(variant: CH11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH11` writer - Enable PPI channel 11."]
pub struct CH11_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH11_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH11_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Enable PPI channel 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH12_A> for bool {
    #[inline(always)]
    fn from(variant: CH12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH12` reader - Enable PPI channel 12."]
pub struct CH12_R(crate::FieldReader<bool, CH12_A>);
impl CH12_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH12_A {
        match self.bits {
            false => CH12_A::DISABLED,
            true => CH12_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH12_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH12_A::ENABLED
    }
}
impl core::ops::Deref for CH12_R {
    type Target = crate::FieldReader<bool, CH12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH12_AW> for bool {
    #[inline(always)]
    fn from(variant: CH12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH12` writer - Enable PPI channel 12."]
pub struct CH12_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH12_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH12_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Enable PPI channel 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH13_A> for bool {
    #[inline(always)]
    fn from(variant: CH13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH13` reader - Enable PPI channel 13."]
pub struct CH13_R(crate::FieldReader<bool, CH13_A>);
impl CH13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH13_A {
        match self.bits {
            false => CH13_A::DISABLED,
            true => CH13_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH13_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH13_A::ENABLED
    }
}
impl core::ops::Deref for CH13_R {
    type Target = crate::FieldReader<bool, CH13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH13_AW> for bool {
    #[inline(always)]
    fn from(variant: CH13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH13` writer - Enable PPI channel 13."]
pub struct CH13_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH13_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH13_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Enable PPI channel 14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH14_A> for bool {
    #[inline(always)]
    fn from(variant: CH14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH14` reader - Enable PPI channel 14."]
pub struct CH14_R(crate::FieldReader<bool, CH14_A>);
impl CH14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH14_A {
        match self.bits {
            false => CH14_A::DISABLED,
            true => CH14_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH14_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH14_A::ENABLED
    }
}
impl core::ops::Deref for CH14_R {
    type Target = crate::FieldReader<bool, CH14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH14_AW> for bool {
    #[inline(always)]
    fn from(variant: CH14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH14` writer - Enable PPI channel 14."]
pub struct CH14_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH14_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH14_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Enable PPI channel 15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH15_A> for bool {
    #[inline(always)]
    fn from(variant: CH15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH15` reader - Enable PPI channel 15."]
pub struct CH15_R(crate::FieldReader<bool, CH15_A>);
impl CH15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH15_A {
        match self.bits {
            false => CH15_A::DISABLED,
            true => CH15_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH15_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH15_A::ENABLED
    }
}
impl core::ops::Deref for CH15_R {
    type Target = crate::FieldReader<bool, CH15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH15_AW> for bool {
    #[inline(always)]
    fn from(variant: CH15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH15` writer - Enable PPI channel 15."]
pub struct CH15_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH15_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH15_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Enable PPI channel 20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH20_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH20_A> for bool {
    #[inline(always)]
    fn from(variant: CH20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH20` reader - Enable PPI channel 20."]
pub struct CH20_R(crate::FieldReader<bool, CH20_A>);
impl CH20_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH20_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH20_A {
        match self.bits {
            false => CH20_A::DISABLED,
            true => CH20_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH20_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH20_A::ENABLED
    }
}
impl core::ops::Deref for CH20_R {
    type Target = crate::FieldReader<bool, CH20_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH20_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH20_AW> for bool {
    #[inline(always)]
    fn from(variant: CH20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH20` writer - Enable PPI channel 20."]
pub struct CH20_W<'a> {
    w: &'a mut W,
}
impl<'a> CH20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH20_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH20_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Enable PPI channel 21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH21_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH21_A> for bool {
    #[inline(always)]
    fn from(variant: CH21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH21` reader - Enable PPI channel 21."]
pub struct CH21_R(crate::FieldReader<bool, CH21_A>);
impl CH21_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH21_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH21_A {
        match self.bits {
            false => CH21_A::DISABLED,
            true => CH21_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH21_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH21_A::ENABLED
    }
}
impl core::ops::Deref for CH21_R {
    type Target = crate::FieldReader<bool, CH21_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH21_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH21_AW> for bool {
    #[inline(always)]
    fn from(variant: CH21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH21` writer - Enable PPI channel 21."]
pub struct CH21_W<'a> {
    w: &'a mut W,
}
impl<'a> CH21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH21_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH21_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Enable PPI channel 22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH22_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH22_A> for bool {
    #[inline(always)]
    fn from(variant: CH22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH22` reader - Enable PPI channel 22."]
pub struct CH22_R(crate::FieldReader<bool, CH22_A>);
impl CH22_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH22_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH22_A {
        match self.bits {
            false => CH22_A::DISABLED,
            true => CH22_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH22_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH22_A::ENABLED
    }
}
impl core::ops::Deref for CH22_R {
    type Target = crate::FieldReader<bool, CH22_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH22_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH22_AW> for bool {
    #[inline(always)]
    fn from(variant: CH22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH22` writer - Enable PPI channel 22."]
pub struct CH22_W<'a> {
    w: &'a mut W,
}
impl<'a> CH22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH22_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH22_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Enable PPI channel 23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH23_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH23_A> for bool {
    #[inline(always)]
    fn from(variant: CH23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH23` reader - Enable PPI channel 23."]
pub struct CH23_R(crate::FieldReader<bool, CH23_A>);
impl CH23_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH23_A {
        match self.bits {
            false => CH23_A::DISABLED,
            true => CH23_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH23_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH23_A::ENABLED
    }
}
impl core::ops::Deref for CH23_R {
    type Target = crate::FieldReader<bool, CH23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH23_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH23_AW> for bool {
    #[inline(always)]
    fn from(variant: CH23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH23` writer - Enable PPI channel 23."]
pub struct CH23_W<'a> {
    w: &'a mut W,
}
impl<'a> CH23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH23_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH23_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Enable PPI channel 24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH24_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH24_A> for bool {
    #[inline(always)]
    fn from(variant: CH24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH24` reader - Enable PPI channel 24."]
pub struct CH24_R(crate::FieldReader<bool, CH24_A>);
impl CH24_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH24_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH24_A {
        match self.bits {
            false => CH24_A::DISABLED,
            true => CH24_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH24_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH24_A::ENABLED
    }
}
impl core::ops::Deref for CH24_R {
    type Target = crate::FieldReader<bool, CH24_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH24_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH24_AW> for bool {
    #[inline(always)]
    fn from(variant: CH24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH24` writer - Enable PPI channel 24."]
pub struct CH24_W<'a> {
    w: &'a mut W,
}
impl<'a> CH24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH24_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH24_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Enable PPI channel 25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH25_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH25_A> for bool {
    #[inline(always)]
    fn from(variant: CH25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH25` reader - Enable PPI channel 25."]
pub struct CH25_R(crate::FieldReader<bool, CH25_A>);
impl CH25_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH25_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH25_A {
        match self.bits {
            false => CH25_A::DISABLED,
            true => CH25_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH25_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH25_A::ENABLED
    }
}
impl core::ops::Deref for CH25_R {
    type Target = crate::FieldReader<bool, CH25_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH25_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH25_AW> for bool {
    #[inline(always)]
    fn from(variant: CH25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH25` writer - Enable PPI channel 25."]
pub struct CH25_W<'a> {
    w: &'a mut W,
}
impl<'a> CH25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH25_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH25_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Enable PPI channel 26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH26_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH26_A> for bool {
    #[inline(always)]
    fn from(variant: CH26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH26` reader - Enable PPI channel 26."]
pub struct CH26_R(crate::FieldReader<bool, CH26_A>);
impl CH26_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH26_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH26_A {
        match self.bits {
            false => CH26_A::DISABLED,
            true => CH26_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH26_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH26_A::ENABLED
    }
}
impl core::ops::Deref for CH26_R {
    type Target = crate::FieldReader<bool, CH26_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH26_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH26_AW> for bool {
    #[inline(always)]
    fn from(variant: CH26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH26` writer - Enable PPI channel 26."]
pub struct CH26_W<'a> {
    w: &'a mut W,
}
impl<'a> CH26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH26_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH26_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Enable PPI channel 27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH27_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH27_A> for bool {
    #[inline(always)]
    fn from(variant: CH27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH27` reader - Enable PPI channel 27."]
pub struct CH27_R(crate::FieldReader<bool, CH27_A>);
impl CH27_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH27_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH27_A {
        match self.bits {
            false => CH27_A::DISABLED,
            true => CH27_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH27_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH27_A::ENABLED
    }
}
impl core::ops::Deref for CH27_R {
    type Target = crate::FieldReader<bool, CH27_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH27_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH27_AW> for bool {
    #[inline(always)]
    fn from(variant: CH27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH27` writer - Enable PPI channel 27."]
pub struct CH27_W<'a> {
    w: &'a mut W,
}
impl<'a> CH27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH27_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH27_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Enable PPI channel 28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH28_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH28_A> for bool {
    #[inline(always)]
    fn from(variant: CH28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH28` reader - Enable PPI channel 28."]
pub struct CH28_R(crate::FieldReader<bool, CH28_A>);
impl CH28_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH28_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH28_A {
        match self.bits {
            false => CH28_A::DISABLED,
            true => CH28_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH28_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH28_A::ENABLED
    }
}
impl core::ops::Deref for CH28_R {
    type Target = crate::FieldReader<bool, CH28_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH28_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH28_AW> for bool {
    #[inline(always)]
    fn from(variant: CH28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH28` writer - Enable PPI channel 28."]
pub struct CH28_W<'a> {
    w: &'a mut W,
}
impl<'a> CH28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH28_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH28_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Enable PPI channel 29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH29_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH29_A> for bool {
    #[inline(always)]
    fn from(variant: CH29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH29` reader - Enable PPI channel 29."]
pub struct CH29_R(crate::FieldReader<bool, CH29_A>);
impl CH29_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH29_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH29_A {
        match self.bits {
            false => CH29_A::DISABLED,
            true => CH29_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH29_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH29_A::ENABLED
    }
}
impl core::ops::Deref for CH29_R {
    type Target = crate::FieldReader<bool, CH29_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH29_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH29_AW> for bool {
    #[inline(always)]
    fn from(variant: CH29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH29` writer - Enable PPI channel 29."]
pub struct CH29_W<'a> {
    w: &'a mut W,
}
impl<'a> CH29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH29_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH29_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Enable PPI channel 30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH30_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH30_A> for bool {
    #[inline(always)]
    fn from(variant: CH30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH30` reader - Enable PPI channel 30."]
pub struct CH30_R(crate::FieldReader<bool, CH30_A>);
impl CH30_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH30_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH30_A {
        match self.bits {
            false => CH30_A::DISABLED,
            true => CH30_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH30_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH30_A::ENABLED
    }
}
impl core::ops::Deref for CH30_R {
    type Target = crate::FieldReader<bool, CH30_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH30_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH30_AW> for bool {
    #[inline(always)]
    fn from(variant: CH30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH30` writer - Enable PPI channel 30."]
pub struct CH30_W<'a> {
    w: &'a mut W,
}
impl<'a> CH30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH30_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH30_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Enable PPI channel 31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH31_A {
    #[doc = "0: Channel disabled."]
    DISABLED = 0,
    #[doc = "1: Channel enabled."]
    ENABLED = 1,
}
impl From<CH31_A> for bool {
    #[inline(always)]
    fn from(variant: CH31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH31` reader - Enable PPI channel 31."]
pub struct CH31_R(crate::FieldReader<bool, CH31_A>);
impl CH31_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH31_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH31_A {
        match self.bits {
            false => CH31_A::DISABLED,
            true => CH31_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == CH31_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == CH31_A::ENABLED
    }
}
impl core::ops::Deref for CH31_R {
    type Target = crate::FieldReader<bool, CH31_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Enable PPI channel 31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH31_AW {
    #[doc = "1: Enable channel on write."]
    SET = 1,
}
impl From<CH31_AW> for bool {
    #[inline(always)]
    fn from(variant: CH31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH31` writer - Enable PPI channel 31."]
pub struct CH31_W<'a> {
    w: &'a mut W,
}
impl<'a> CH31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH31_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable channel on write."]
    #[inline(always)]
    pub fn set(self) -> &'a mut W {
        self.variant(CH31_AW::SET)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable PPI channel 0."]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable PPI channel 1."]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable PPI channel 2."]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable PPI channel 3."]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Enable PPI channel 4."]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enable PPI channel 5."]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Enable PPI channel 6."]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable PPI channel 7."]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable PPI channel 8."]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable PPI channel 9."]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Enable PPI channel 10."]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable PPI channel 11."]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable PPI channel 12."]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable PPI channel 13."]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Enable PPI channel 14."]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Enable PPI channel 15."]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Enable PPI channel 20."]
    #[inline(always)]
    pub fn ch20(&self) -> CH20_R {
        CH20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Enable PPI channel 21."]
    #[inline(always)]
    pub fn ch21(&self) -> CH21_R {
        CH21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Enable PPI channel 22."]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Enable PPI channel 23."]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Enable PPI channel 24."]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable PPI channel 25."]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Enable PPI channel 26."]
    #[inline(always)]
    pub fn ch26(&self) -> CH26_R {
        CH26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Enable PPI channel 27."]
    #[inline(always)]
    pub fn ch27(&self) -> CH27_R {
        CH27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Enable PPI channel 28."]
    #[inline(always)]
    pub fn ch28(&self) -> CH28_R {
        CH28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Enable PPI channel 29."]
    #[inline(always)]
    pub fn ch29(&self) -> CH29_R {
        CH29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Enable PPI channel 30."]
    #[inline(always)]
    pub fn ch30(&self) -> CH30_R {
        CH30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable PPI channel 31."]
    #[inline(always)]
    pub fn ch31(&self) -> CH31_R {
        CH31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable PPI channel 0."]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bit 1 - Enable PPI channel 1."]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bit 2 - Enable PPI channel 2."]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bit 3 - Enable PPI channel 3."]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W { w: self }
    }
    #[doc = "Bit 4 - Enable PPI channel 4."]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W { w: self }
    }
    #[doc = "Bit 5 - Enable PPI channel 5."]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W { w: self }
    }
    #[doc = "Bit 6 - Enable PPI channel 6."]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W { w: self }
    }
    #[doc = "Bit 7 - Enable PPI channel 7."]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W {
        CH7_W { w: self }
    }
    #[doc = "Bit 8 - Enable PPI channel 8."]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W {
        CH8_W { w: self }
    }
    #[doc = "Bit 9 - Enable PPI channel 9."]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W {
        CH9_W { w: self }
    }
    #[doc = "Bit 10 - Enable PPI channel 10."]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W {
        CH10_W { w: self }
    }
    #[doc = "Bit 11 - Enable PPI channel 11."]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W {
        CH11_W { w: self }
    }
    #[doc = "Bit 12 - Enable PPI channel 12."]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W {
        CH12_W { w: self }
    }
    #[doc = "Bit 13 - Enable PPI channel 13."]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W {
        CH13_W { w: self }
    }
    #[doc = "Bit 14 - Enable PPI channel 14."]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W {
        CH14_W { w: self }
    }
    #[doc = "Bit 15 - Enable PPI channel 15."]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W {
        CH15_W { w: self }
    }
    #[doc = "Bit 20 - Enable PPI channel 20."]
    #[inline(always)]
    pub fn ch20(&mut self) -> CH20_W {
        CH20_W { w: self }
    }
    #[doc = "Bit 21 - Enable PPI channel 21."]
    #[inline(always)]
    pub fn ch21(&mut self) -> CH21_W {
        CH21_W { w: self }
    }
    #[doc = "Bit 22 - Enable PPI channel 22."]
    #[inline(always)]
    pub fn ch22(&mut self) -> CH22_W {
        CH22_W { w: self }
    }
    #[doc = "Bit 23 - Enable PPI channel 23."]
    #[inline(always)]
    pub fn ch23(&mut self) -> CH23_W {
        CH23_W { w: self }
    }
    #[doc = "Bit 24 - Enable PPI channel 24."]
    #[inline(always)]
    pub fn ch24(&mut self) -> CH24_W {
        CH24_W { w: self }
    }
    #[doc = "Bit 25 - Enable PPI channel 25."]
    #[inline(always)]
    pub fn ch25(&mut self) -> CH25_W {
        CH25_W { w: self }
    }
    #[doc = "Bit 26 - Enable PPI channel 26."]
    #[inline(always)]
    pub fn ch26(&mut self) -> CH26_W {
        CH26_W { w: self }
    }
    #[doc = "Bit 27 - Enable PPI channel 27."]
    #[inline(always)]
    pub fn ch27(&mut self) -> CH27_W {
        CH27_W { w: self }
    }
    #[doc = "Bit 28 - Enable PPI channel 28."]
    #[inline(always)]
    pub fn ch28(&mut self) -> CH28_W {
        CH28_W { w: self }
    }
    #[doc = "Bit 29 - Enable PPI channel 29."]
    #[inline(always)]
    pub fn ch29(&mut self) -> CH29_W {
        CH29_W { w: self }
    }
    #[doc = "Bit 30 - Enable PPI channel 30."]
    #[inline(always)]
    pub fn ch30(&mut self) -> CH30_W {
        CH30_W { w: self }
    }
    #[doc = "Bit 31 - Enable PPI channel 31."]
    #[inline(always)]
    pub fn ch31(&mut self) -> CH31_W {
        CH31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel enable set.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chenset](index.html) module"]
pub struct CHENSET_SPEC;
impl crate::RegisterSpec for CHENSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chenset::R](R) reader structure"]
impl crate::Readable for CHENSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chenset::W](W) writer structure"]
impl crate::Writable for CHENSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHENSET to value 0"]
impl crate::Resettable for CHENSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
