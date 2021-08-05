#[doc = "Register `CRCSTATUS` reader"]
pub struct R(crate::R<CRCSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "CRC status of received packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CRCSTATUS_A {
    #[doc = "0: Packet received with CRC error."]
    CRCERROR = 0,
    #[doc = "1: Packet received with CRC ok."]
    CRCOK = 1,
}
impl From<CRCSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: CRCSTATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRCSTATUS` reader - CRC status of received packet."]
pub struct CRCSTATUS_R(crate::FieldReader<bool, CRCSTATUS_A>);
impl CRCSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CRCSTATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCSTATUS_A {
        match self.bits {
            false => CRCSTATUS_A::CRCERROR,
            true => CRCSTATUS_A::CRCOK,
        }
    }
    #[doc = "Checks if the value of the field is `CRCERROR`"]
    #[inline(always)]
    pub fn is_crcerror(&self) -> bool {
        **self == CRCSTATUS_A::CRCERROR
    }
    #[doc = "Checks if the value of the field is `CRCOK`"]
    #[inline(always)]
    pub fn is_crcok(&self) -> bool {
        **self == CRCSTATUS_A::CRCOK
    }
}
impl core::ops::Deref for CRCSTATUS_R {
    type Target = crate::FieldReader<bool, CRCSTATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - CRC status of received packet."]
    #[inline(always)]
    pub fn crcstatus(&self) -> CRCSTATUS_R {
        CRCSTATUS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "CRC status of received packet.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crcstatus](index.html) module"]
pub struct CRCSTATUS_SPEC;
impl crate::RegisterSpec for CRCSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crcstatus::R](R) reader structure"]
impl crate::Readable for CRCSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CRCSTATUS to value 0"]
impl crate::Resettable for CRCSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
