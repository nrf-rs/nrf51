#[doc = "Reader of register HFCLKRUN"]
pub type R = crate::R<u32, super::HFCLKRUN>;
#[doc = "Task HFCLKSTART trigger status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STATUS_A {
    #[doc = "0: Task HFCLKSTART has not been triggered."]
    NOTTRIGGERED = 0,
    #[doc = "1: Task HFCLKSTART has been triggered."]
    TRIGGERED = 1,
}
impl From<STATUS_A> for bool {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STATUS`"]
pub type STATUS_R = crate::R<bool, STATUS_A>;
impl STATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STATUS_A {
        match self.bits {
            false => STATUS_A::NOTTRIGGERED,
            true => STATUS_A::TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `NOTTRIGGERED`"]
    #[inline(always)]
    pub fn is_not_triggered(&self) -> bool {
        *self == STATUS_A::NOTTRIGGERED
    }
    #[doc = "Checks if the value of the field is `TRIGGERED`"]
    #[inline(always)]
    pub fn is_triggered(&self) -> bool {
        *self == STATUS_A::TRIGGERED
    }
}
impl R {
    #[doc = "Bit 0 - Task HFCLKSTART trigger status."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 0x01) != 0)
    }
}
