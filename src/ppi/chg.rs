#[doc = "Register `CHG[%s]` reader"]
pub struct R(crate::R<CHG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHG[%s]` writer"]
pub struct W(crate::W<CHG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHG_SPEC>;
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
impl From<crate::W<CHG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Include CH0 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH0_A> for bool {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0` reader - Include CH0 in channel group."]
pub struct CH0_R(crate::FieldReader<bool, CH0_A>);
impl CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            false => CH0_A::EXCLUDED,
            true => CH0_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH0_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH0_A::INCLUDED
    }
}
impl core::ops::Deref for CH0_R {
    type Target = crate::FieldReader<bool, CH0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0` writer - Include CH0 in channel group."]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH0_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH0_A::INCLUDED)
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
#[doc = "Include CH1 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH1_A> for bool {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1` reader - Include CH1 in channel group."]
pub struct CH1_R(crate::FieldReader<bool, CH1_A>);
impl CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            false => CH1_A::EXCLUDED,
            true => CH1_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH1_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH1_A::INCLUDED
    }
}
impl core::ops::Deref for CH1_R {
    type Target = crate::FieldReader<bool, CH1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1` writer - Include CH1 in channel group."]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH1_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH1_A::INCLUDED)
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
#[doc = "Include CH2 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH2_A> for bool {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2` reader - Include CH2 in channel group."]
pub struct CH2_R(crate::FieldReader<bool, CH2_A>);
impl CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            false => CH2_A::EXCLUDED,
            true => CH2_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH2_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH2_A::INCLUDED
    }
}
impl core::ops::Deref for CH2_R {
    type Target = crate::FieldReader<bool, CH2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2` writer - Include CH2 in channel group."]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH2_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH2_A::INCLUDED)
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
#[doc = "Include CH3 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH3_A> for bool {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3` reader - Include CH3 in channel group."]
pub struct CH3_R(crate::FieldReader<bool, CH3_A>);
impl CH3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            false => CH3_A::EXCLUDED,
            true => CH3_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH3_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH3_A::INCLUDED
    }
}
impl core::ops::Deref for CH3_R {
    type Target = crate::FieldReader<bool, CH3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3` writer - Include CH3 in channel group."]
pub struct CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH3_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH3_A::INCLUDED)
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
#[doc = "Include CH4 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH4_A> for bool {
    #[inline(always)]
    fn from(variant: CH4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4` reader - Include CH4 in channel group."]
pub struct CH4_R(crate::FieldReader<bool, CH4_A>);
impl CH4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4_A {
        match self.bits {
            false => CH4_A::EXCLUDED,
            true => CH4_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH4_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH4_A::INCLUDED
    }
}
impl core::ops::Deref for CH4_R {
    type Target = crate::FieldReader<bool, CH4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4` writer - Include CH4 in channel group."]
pub struct CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH4_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH4_A::INCLUDED)
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
#[doc = "Include CH5 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH5_A> for bool {
    #[inline(always)]
    fn from(variant: CH5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5` reader - Include CH5 in channel group."]
pub struct CH5_R(crate::FieldReader<bool, CH5_A>);
impl CH5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5_A {
        match self.bits {
            false => CH5_A::EXCLUDED,
            true => CH5_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH5_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH5_A::INCLUDED
    }
}
impl core::ops::Deref for CH5_R {
    type Target = crate::FieldReader<bool, CH5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5` writer - Include CH5 in channel group."]
pub struct CH5_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH5_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH5_A::INCLUDED)
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
#[doc = "Include CH6 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH6_A> for bool {
    #[inline(always)]
    fn from(variant: CH6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH6` reader - Include CH6 in channel group."]
pub struct CH6_R(crate::FieldReader<bool, CH6_A>);
impl CH6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6_A {
        match self.bits {
            false => CH6_A::EXCLUDED,
            true => CH6_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH6_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH6_A::INCLUDED
    }
}
impl core::ops::Deref for CH6_R {
    type Target = crate::FieldReader<bool, CH6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6` writer - Include CH6 in channel group."]
pub struct CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH6_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH6_A::INCLUDED)
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
#[doc = "Include CH7 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH7_A> for bool {
    #[inline(always)]
    fn from(variant: CH7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH7` reader - Include CH7 in channel group."]
pub struct CH7_R(crate::FieldReader<bool, CH7_A>);
impl CH7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7_A {
        match self.bits {
            false => CH7_A::EXCLUDED,
            true => CH7_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH7_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH7_A::INCLUDED
    }
}
impl core::ops::Deref for CH7_R {
    type Target = crate::FieldReader<bool, CH7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7` writer - Include CH7 in channel group."]
pub struct CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH7_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH7_A::INCLUDED)
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
#[doc = "Include CH8 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH8_A> for bool {
    #[inline(always)]
    fn from(variant: CH8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH8` reader - Include CH8 in channel group."]
pub struct CH8_R(crate::FieldReader<bool, CH8_A>);
impl CH8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH8_A {
        match self.bits {
            false => CH8_A::EXCLUDED,
            true => CH8_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH8_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH8_A::INCLUDED
    }
}
impl core::ops::Deref for CH8_R {
    type Target = crate::FieldReader<bool, CH8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH8` writer - Include CH8 in channel group."]
pub struct CH8_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH8_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH8_A::INCLUDED)
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
#[doc = "Include CH9 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH9_A> for bool {
    #[inline(always)]
    fn from(variant: CH9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH9` reader - Include CH9 in channel group."]
pub struct CH9_R(crate::FieldReader<bool, CH9_A>);
impl CH9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH9_A {
        match self.bits {
            false => CH9_A::EXCLUDED,
            true => CH9_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH9_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH9_A::INCLUDED
    }
}
impl core::ops::Deref for CH9_R {
    type Target = crate::FieldReader<bool, CH9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH9` writer - Include CH9 in channel group."]
pub struct CH9_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH9_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH9_A::INCLUDED)
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
#[doc = "Include CH10 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH10_A> for bool {
    #[inline(always)]
    fn from(variant: CH10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH10` reader - Include CH10 in channel group."]
pub struct CH10_R(crate::FieldReader<bool, CH10_A>);
impl CH10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH10_A {
        match self.bits {
            false => CH10_A::EXCLUDED,
            true => CH10_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH10_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH10_A::INCLUDED
    }
}
impl core::ops::Deref for CH10_R {
    type Target = crate::FieldReader<bool, CH10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH10` writer - Include CH10 in channel group."]
pub struct CH10_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH10_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH10_A::INCLUDED)
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
#[doc = "Include CH11 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH11_A> for bool {
    #[inline(always)]
    fn from(variant: CH11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH11` reader - Include CH11 in channel group."]
pub struct CH11_R(crate::FieldReader<bool, CH11_A>);
impl CH11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH11_A {
        match self.bits {
            false => CH11_A::EXCLUDED,
            true => CH11_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH11_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH11_A::INCLUDED
    }
}
impl core::ops::Deref for CH11_R {
    type Target = crate::FieldReader<bool, CH11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH11` writer - Include CH11 in channel group."]
pub struct CH11_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH11_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH11_A::INCLUDED)
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
#[doc = "Include CH12 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH12_A> for bool {
    #[inline(always)]
    fn from(variant: CH12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH12` reader - Include CH12 in channel group."]
pub struct CH12_R(crate::FieldReader<bool, CH12_A>);
impl CH12_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH12_A {
        match self.bits {
            false => CH12_A::EXCLUDED,
            true => CH12_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH12_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH12_A::INCLUDED
    }
}
impl core::ops::Deref for CH12_R {
    type Target = crate::FieldReader<bool, CH12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH12` writer - Include CH12 in channel group."]
pub struct CH12_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH12_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH12_A::INCLUDED)
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
#[doc = "Include CH13 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH13_A> for bool {
    #[inline(always)]
    fn from(variant: CH13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH13` reader - Include CH13 in channel group."]
pub struct CH13_R(crate::FieldReader<bool, CH13_A>);
impl CH13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH13_A {
        match self.bits {
            false => CH13_A::EXCLUDED,
            true => CH13_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH13_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH13_A::INCLUDED
    }
}
impl core::ops::Deref for CH13_R {
    type Target = crate::FieldReader<bool, CH13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH13` writer - Include CH13 in channel group."]
pub struct CH13_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH13_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH13_A::INCLUDED)
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
#[doc = "Include CH14 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH14_A> for bool {
    #[inline(always)]
    fn from(variant: CH14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH14` reader - Include CH14 in channel group."]
pub struct CH14_R(crate::FieldReader<bool, CH14_A>);
impl CH14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH14_A {
        match self.bits {
            false => CH14_A::EXCLUDED,
            true => CH14_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH14_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH14_A::INCLUDED
    }
}
impl core::ops::Deref for CH14_R {
    type Target = crate::FieldReader<bool, CH14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH14` writer - Include CH14 in channel group."]
pub struct CH14_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH14_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH14_A::INCLUDED)
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
#[doc = "Include CH15 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH15_A> for bool {
    #[inline(always)]
    fn from(variant: CH15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH15` reader - Include CH15 in channel group."]
pub struct CH15_R(crate::FieldReader<bool, CH15_A>);
impl CH15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH15_A {
        match self.bits {
            false => CH15_A::EXCLUDED,
            true => CH15_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH15_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH15_A::INCLUDED
    }
}
impl core::ops::Deref for CH15_R {
    type Target = crate::FieldReader<bool, CH15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH15` writer - Include CH15 in channel group."]
pub struct CH15_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH15_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH15_A::INCLUDED)
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
#[doc = "Include CH20 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH20_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH20_A> for bool {
    #[inline(always)]
    fn from(variant: CH20_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH20` reader - Include CH20 in channel group."]
pub struct CH20_R(crate::FieldReader<bool, CH20_A>);
impl CH20_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH20_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH20_A {
        match self.bits {
            false => CH20_A::EXCLUDED,
            true => CH20_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH20_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH20_A::INCLUDED
    }
}
impl core::ops::Deref for CH20_R {
    type Target = crate::FieldReader<bool, CH20_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH20` writer - Include CH20 in channel group."]
pub struct CH20_W<'a> {
    w: &'a mut W,
}
impl<'a> CH20_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH20_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH20_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH20_A::INCLUDED)
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
#[doc = "Include CH21 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH21_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH21_A> for bool {
    #[inline(always)]
    fn from(variant: CH21_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH21` reader - Include CH21 in channel group."]
pub struct CH21_R(crate::FieldReader<bool, CH21_A>);
impl CH21_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH21_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH21_A {
        match self.bits {
            false => CH21_A::EXCLUDED,
            true => CH21_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH21_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH21_A::INCLUDED
    }
}
impl core::ops::Deref for CH21_R {
    type Target = crate::FieldReader<bool, CH21_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH21` writer - Include CH21 in channel group."]
pub struct CH21_W<'a> {
    w: &'a mut W,
}
impl<'a> CH21_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH21_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH21_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH21_A::INCLUDED)
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
#[doc = "Include CH22 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH22_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH22_A> for bool {
    #[inline(always)]
    fn from(variant: CH22_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH22` reader - Include CH22 in channel group."]
pub struct CH22_R(crate::FieldReader<bool, CH22_A>);
impl CH22_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH22_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH22_A {
        match self.bits {
            false => CH22_A::EXCLUDED,
            true => CH22_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH22_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH22_A::INCLUDED
    }
}
impl core::ops::Deref for CH22_R {
    type Target = crate::FieldReader<bool, CH22_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH22` writer - Include CH22 in channel group."]
pub struct CH22_W<'a> {
    w: &'a mut W,
}
impl<'a> CH22_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH22_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH22_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH22_A::INCLUDED)
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
#[doc = "Include CH23 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH23_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH23_A> for bool {
    #[inline(always)]
    fn from(variant: CH23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH23` reader - Include CH23 in channel group."]
pub struct CH23_R(crate::FieldReader<bool, CH23_A>);
impl CH23_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH23_A {
        match self.bits {
            false => CH23_A::EXCLUDED,
            true => CH23_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH23_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH23_A::INCLUDED
    }
}
impl core::ops::Deref for CH23_R {
    type Target = crate::FieldReader<bool, CH23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH23` writer - Include CH23 in channel group."]
pub struct CH23_W<'a> {
    w: &'a mut W,
}
impl<'a> CH23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH23_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH23_A::INCLUDED)
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
#[doc = "Include CH24 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH24_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH24_A> for bool {
    #[inline(always)]
    fn from(variant: CH24_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH24` reader - Include CH24 in channel group."]
pub struct CH24_R(crate::FieldReader<bool, CH24_A>);
impl CH24_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH24_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH24_A {
        match self.bits {
            false => CH24_A::EXCLUDED,
            true => CH24_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH24_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH24_A::INCLUDED
    }
}
impl core::ops::Deref for CH24_R {
    type Target = crate::FieldReader<bool, CH24_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH24` writer - Include CH24 in channel group."]
pub struct CH24_W<'a> {
    w: &'a mut W,
}
impl<'a> CH24_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH24_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH24_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH24_A::INCLUDED)
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
#[doc = "Include CH25 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH25_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH25_A> for bool {
    #[inline(always)]
    fn from(variant: CH25_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH25` reader - Include CH25 in channel group."]
pub struct CH25_R(crate::FieldReader<bool, CH25_A>);
impl CH25_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH25_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH25_A {
        match self.bits {
            false => CH25_A::EXCLUDED,
            true => CH25_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH25_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH25_A::INCLUDED
    }
}
impl core::ops::Deref for CH25_R {
    type Target = crate::FieldReader<bool, CH25_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH25` writer - Include CH25 in channel group."]
pub struct CH25_W<'a> {
    w: &'a mut W,
}
impl<'a> CH25_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH25_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH25_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH25_A::INCLUDED)
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
#[doc = "Include CH26 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH26_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH26_A> for bool {
    #[inline(always)]
    fn from(variant: CH26_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH26` reader - Include CH26 in channel group."]
pub struct CH26_R(crate::FieldReader<bool, CH26_A>);
impl CH26_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH26_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH26_A {
        match self.bits {
            false => CH26_A::EXCLUDED,
            true => CH26_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH26_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH26_A::INCLUDED
    }
}
impl core::ops::Deref for CH26_R {
    type Target = crate::FieldReader<bool, CH26_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH26` writer - Include CH26 in channel group."]
pub struct CH26_W<'a> {
    w: &'a mut W,
}
impl<'a> CH26_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH26_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH26_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH26_A::INCLUDED)
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
#[doc = "Include CH27 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH27_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH27_A> for bool {
    #[inline(always)]
    fn from(variant: CH27_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH27` reader - Include CH27 in channel group."]
pub struct CH27_R(crate::FieldReader<bool, CH27_A>);
impl CH27_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH27_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH27_A {
        match self.bits {
            false => CH27_A::EXCLUDED,
            true => CH27_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH27_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH27_A::INCLUDED
    }
}
impl core::ops::Deref for CH27_R {
    type Target = crate::FieldReader<bool, CH27_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH27` writer - Include CH27 in channel group."]
pub struct CH27_W<'a> {
    w: &'a mut W,
}
impl<'a> CH27_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH27_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH27_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH27_A::INCLUDED)
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
#[doc = "Include CH28 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH28_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH28_A> for bool {
    #[inline(always)]
    fn from(variant: CH28_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH28` reader - Include CH28 in channel group."]
pub struct CH28_R(crate::FieldReader<bool, CH28_A>);
impl CH28_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH28_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH28_A {
        match self.bits {
            false => CH28_A::EXCLUDED,
            true => CH28_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH28_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH28_A::INCLUDED
    }
}
impl core::ops::Deref for CH28_R {
    type Target = crate::FieldReader<bool, CH28_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH28` writer - Include CH28 in channel group."]
pub struct CH28_W<'a> {
    w: &'a mut W,
}
impl<'a> CH28_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH28_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH28_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH28_A::INCLUDED)
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
#[doc = "Include CH29 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH29_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH29_A> for bool {
    #[inline(always)]
    fn from(variant: CH29_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH29` reader - Include CH29 in channel group."]
pub struct CH29_R(crate::FieldReader<bool, CH29_A>);
impl CH29_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH29_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH29_A {
        match self.bits {
            false => CH29_A::EXCLUDED,
            true => CH29_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH29_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH29_A::INCLUDED
    }
}
impl core::ops::Deref for CH29_R {
    type Target = crate::FieldReader<bool, CH29_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH29` writer - Include CH29 in channel group."]
pub struct CH29_W<'a> {
    w: &'a mut W,
}
impl<'a> CH29_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH29_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH29_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH29_A::INCLUDED)
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
#[doc = "Include CH30 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH30_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH30_A> for bool {
    #[inline(always)]
    fn from(variant: CH30_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH30` reader - Include CH30 in channel group."]
pub struct CH30_R(crate::FieldReader<bool, CH30_A>);
impl CH30_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH30_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH30_A {
        match self.bits {
            false => CH30_A::EXCLUDED,
            true => CH30_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH30_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH30_A::INCLUDED
    }
}
impl core::ops::Deref for CH30_R {
    type Target = crate::FieldReader<bool, CH30_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH30` writer - Include CH30 in channel group."]
pub struct CH30_W<'a> {
    w: &'a mut W,
}
impl<'a> CH30_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH30_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH30_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH30_A::INCLUDED)
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
#[doc = "Include CH31 in channel group.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH31_A {
    #[doc = "0: Channel excluded."]
    EXCLUDED = 0,
    #[doc = "1: Channel included."]
    INCLUDED = 1,
}
impl From<CH31_A> for bool {
    #[inline(always)]
    fn from(variant: CH31_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH31` reader - Include CH31 in channel group."]
pub struct CH31_R(crate::FieldReader<bool, CH31_A>);
impl CH31_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH31_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH31_A {
        match self.bits {
            false => CH31_A::EXCLUDED,
            true => CH31_A::INCLUDED,
        }
    }
    #[doc = "Checks if the value of the field is `EXCLUDED`"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        **self == CH31_A::EXCLUDED
    }
    #[doc = "Checks if the value of the field is `INCLUDED`"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        **self == CH31_A::INCLUDED
    }
}
impl core::ops::Deref for CH31_R {
    type Target = crate::FieldReader<bool, CH31_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH31` writer - Include CH31 in channel group."]
pub struct CH31_W<'a> {
    w: &'a mut W,
}
impl<'a> CH31_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH31_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Channel excluded."]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut W {
        self.variant(CH31_A::EXCLUDED)
    }
    #[doc = "Channel included."]
    #[inline(always)]
    pub fn included(self) -> &'a mut W {
        self.variant(CH31_A::INCLUDED)
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
    #[doc = "Bit 0 - Include CH0 in channel group."]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Include CH1 in channel group."]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Include CH2 in channel group."]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Include CH3 in channel group."]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Include CH4 in channel group."]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Include CH5 in channel group."]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Include CH6 in channel group."]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Include CH7 in channel group."]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Include CH8 in channel group."]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Include CH9 in channel group."]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Include CH10 in channel group."]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Include CH11 in channel group."]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Include CH12 in channel group."]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Include CH13 in channel group."]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Include CH14 in channel group."]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Include CH15 in channel group."]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Include CH20 in channel group."]
    #[inline(always)]
    pub fn ch20(&self) -> CH20_R {
        CH20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Include CH21 in channel group."]
    #[inline(always)]
    pub fn ch21(&self) -> CH21_R {
        CH21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Include CH22 in channel group."]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Include CH23 in channel group."]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Include CH24 in channel group."]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Include CH25 in channel group."]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Include CH26 in channel group."]
    #[inline(always)]
    pub fn ch26(&self) -> CH26_R {
        CH26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Include CH27 in channel group."]
    #[inline(always)]
    pub fn ch27(&self) -> CH27_R {
        CH27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Include CH28 in channel group."]
    #[inline(always)]
    pub fn ch28(&self) -> CH28_R {
        CH28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Include CH29 in channel group."]
    #[inline(always)]
    pub fn ch29(&self) -> CH29_R {
        CH29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Include CH30 in channel group."]
    #[inline(always)]
    pub fn ch30(&self) -> CH30_R {
        CH30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Include CH31 in channel group."]
    #[inline(always)]
    pub fn ch31(&self) -> CH31_R {
        CH31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Include CH0 in channel group."]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bit 1 - Include CH1 in channel group."]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bit 2 - Include CH2 in channel group."]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bit 3 - Include CH3 in channel group."]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W { w: self }
    }
    #[doc = "Bit 4 - Include CH4 in channel group."]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W { w: self }
    }
    #[doc = "Bit 5 - Include CH5 in channel group."]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W { w: self }
    }
    #[doc = "Bit 6 - Include CH6 in channel group."]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W { w: self }
    }
    #[doc = "Bit 7 - Include CH7 in channel group."]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W {
        CH7_W { w: self }
    }
    #[doc = "Bit 8 - Include CH8 in channel group."]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W {
        CH8_W { w: self }
    }
    #[doc = "Bit 9 - Include CH9 in channel group."]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W {
        CH9_W { w: self }
    }
    #[doc = "Bit 10 - Include CH10 in channel group."]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W {
        CH10_W { w: self }
    }
    #[doc = "Bit 11 - Include CH11 in channel group."]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W {
        CH11_W { w: self }
    }
    #[doc = "Bit 12 - Include CH12 in channel group."]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W {
        CH12_W { w: self }
    }
    #[doc = "Bit 13 - Include CH13 in channel group."]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W {
        CH13_W { w: self }
    }
    #[doc = "Bit 14 - Include CH14 in channel group."]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W {
        CH14_W { w: self }
    }
    #[doc = "Bit 15 - Include CH15 in channel group."]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W {
        CH15_W { w: self }
    }
    #[doc = "Bit 20 - Include CH20 in channel group."]
    #[inline(always)]
    pub fn ch20(&mut self) -> CH20_W {
        CH20_W { w: self }
    }
    #[doc = "Bit 21 - Include CH21 in channel group."]
    #[inline(always)]
    pub fn ch21(&mut self) -> CH21_W {
        CH21_W { w: self }
    }
    #[doc = "Bit 22 - Include CH22 in channel group."]
    #[inline(always)]
    pub fn ch22(&mut self) -> CH22_W {
        CH22_W { w: self }
    }
    #[doc = "Bit 23 - Include CH23 in channel group."]
    #[inline(always)]
    pub fn ch23(&mut self) -> CH23_W {
        CH23_W { w: self }
    }
    #[doc = "Bit 24 - Include CH24 in channel group."]
    #[inline(always)]
    pub fn ch24(&mut self) -> CH24_W {
        CH24_W { w: self }
    }
    #[doc = "Bit 25 - Include CH25 in channel group."]
    #[inline(always)]
    pub fn ch25(&mut self) -> CH25_W {
        CH25_W { w: self }
    }
    #[doc = "Bit 26 - Include CH26 in channel group."]
    #[inline(always)]
    pub fn ch26(&mut self) -> CH26_W {
        CH26_W { w: self }
    }
    #[doc = "Bit 27 - Include CH27 in channel group."]
    #[inline(always)]
    pub fn ch27(&mut self) -> CH27_W {
        CH27_W { w: self }
    }
    #[doc = "Bit 28 - Include CH28 in channel group."]
    #[inline(always)]
    pub fn ch28(&mut self) -> CH28_W {
        CH28_W { w: self }
    }
    #[doc = "Bit 29 - Include CH29 in channel group."]
    #[inline(always)]
    pub fn ch29(&mut self) -> CH29_W {
        CH29_W { w: self }
    }
    #[doc = "Bit 30 - Include CH30 in channel group."]
    #[inline(always)]
    pub fn ch30(&mut self) -> CH30_W {
        CH30_W { w: self }
    }
    #[doc = "Bit 31 - Include CH31 in channel group."]
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
#[doc = "Channel group configuration.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chg](index.html) module"]
pub struct CHG_SPEC;
impl crate::RegisterSpec for CHG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chg::R](R) reader structure"]
impl crate::Readable for CHG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chg::W](W) writer structure"]
impl crate::Writable for CHG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHG[%s]
to value 0"]
impl crate::Resettable for CHG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
