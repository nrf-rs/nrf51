#[doc = "Register `READY` reader"]
pub struct R(crate::R<READY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<READY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<READY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<READY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "NVMC ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum READY_A {
    #[doc = "0: NVMC is busy (on-going write or erase operation)."]
    BUSY = 0,
    #[doc = "1: NVMC is ready."]
    READY = 1,
}
impl From<READY_A> for bool {
    #[inline(always)]
    fn from(variant: READY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `READY` reader - NVMC ready."]
pub struct READY_R(crate::FieldReader<bool, READY_A>);
impl READY_R {
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> READY_A {
        match self.bits {
            false => READY_A::BUSY,
            true => READY_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        **self == READY_A::BUSY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        **self == READY_A::READY
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, READY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - NVMC ready."]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Ready flag.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ready](index.html) module"]
pub struct READY_SPEC;
impl crate::RegisterSpec for READY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ready::R](R) reader structure"]
impl crate::Readable for READY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets READY to value 0"]
impl crate::Resettable for READY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
