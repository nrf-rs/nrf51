#[doc = "Reader of register RESULT"]
pub type R = crate::R<u32, super::RESULT>;
#[doc = "Result of last compare. Decision point SAMPLE task.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESULT_A {
    #[doc = "0: Input voltage is bellow the reference threshold."]
    BELLOW,
    #[doc = "1: Input voltage is above the reference threshold."]
    ABOVE,
}
impl From<RESULT_A> for bool {
    #[inline(always)]
    fn from(variant: RESULT_A) -> Self {
        match variant {
            RESULT_A::BELLOW => false,
            RESULT_A::ABOVE => true,
        }
    }
}
#[doc = "Reader of field `RESULT`"]
pub type RESULT_R = crate::R<bool, RESULT_A>;
impl RESULT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESULT_A {
        match self.bits {
            false => RESULT_A::BELLOW,
            true => RESULT_A::ABOVE,
        }
    }
    #[doc = "Checks if the value of the field is `BELLOW`"]
    #[inline(always)]
    pub fn is_bellow(&self) -> bool {
        *self == RESULT_A::BELLOW
    }
    #[doc = "Checks if the value of the field is `ABOVE`"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == RESULT_A::ABOVE
    }
}
impl R {
    #[doc = "Bit 0 - Result of last compare. Decision point SAMPLE task."]
    #[inline(always)]
    pub fn result(&self) -> RESULT_R {
        RESULT_R::new((self.bits & 0x01) != 0)
    }
}
