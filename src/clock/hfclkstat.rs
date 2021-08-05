#[doc = "Register `HFCLKSTAT` reader"]
pub struct R(crate::R<HFCLKSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLKSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCLKSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCLKSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Active clock source for the HF clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: Internal 16MHz RC oscillator running and generating the HFCLK clock."]
    RC = 0,
    #[doc = "1: External 16MHz/32MHz crystal oscillator running and generating the HFCLK clock."]
    XTAL = 1,
}
impl From<SRC_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRC` reader - Active clock source for the HF clock."]
pub struct SRC_R(crate::FieldReader<bool, SRC_A>);
impl SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_A {
        match self.bits {
            false => SRC_A::RC,
            true => SRC_A::XTAL,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        **self == SRC_A::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        **self == SRC_A::XTAL
    }
}
impl core::ops::Deref for SRC_R {
    type Target = crate::FieldReader<bool, SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "State for the HFCLK.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATE_A {
    #[doc = "0: HFCLK clock not running."]
    NOTRUNNING = 0,
    #[doc = "1: HFCLK clock running."]
    RUNNING = 1,
}
impl From<STATE_A> for bool {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATE` reader - State for the HFCLK."]
pub struct STATE_R(crate::FieldReader<bool, STATE_A>);
impl STATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATE_A {
        match self.bits {
            false => STATE_A::NOTRUNNING,
            true => STATE_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        **self == STATE_A::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        **self == STATE_A::RUNNING
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<bool, STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Active clock source for the HF clock."]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - State for the HFCLK."]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
#[doc = "High frequency clock status.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkstat](index.html) module"]
pub struct HFCLKSTAT_SPEC;
impl crate::RegisterSpec for HFCLKSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclkstat::R](R) reader structure"]
impl crate::Readable for HFCLKSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFCLKSTAT to value 0"]
impl crate::Resettable for HFCLKSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
