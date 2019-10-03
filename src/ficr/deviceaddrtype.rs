#[doc = "Reader of register DEVICEADDRTYPE"]
pub type R = crate::R<u32, super::DEVICEADDRTYPE>;
#[doc = "Writer for register DEVICEADDRTYPE"]
pub type W = crate::W<u32, super::DEVICEADDRTYPE>;
#[doc = "Register DEVICEADDRTYPE `reset()`'s with value 0"]
impl crate::ResetValue for super::DEVICEADDRTYPE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Device address type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEVICEADDRTYPE_A {
    #[doc = "0: Public address."]
    PUBLIC,
    #[doc = "1: Random address."]
    RANDOM,
}
impl From<DEVICEADDRTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: DEVICEADDRTYPE_A) -> Self {
        match variant {
            DEVICEADDRTYPE_A::PUBLIC => false,
            DEVICEADDRTYPE_A::RANDOM => true,
        }
    }
}
#[doc = "Reader of field `DEVICEADDRTYPE`"]
pub type DEVICEADDRTYPE_R = crate::R<bool, DEVICEADDRTYPE_A>;
impl DEVICEADDRTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEVICEADDRTYPE_A {
        match self.bits {
            false => DEVICEADDRTYPE_A::PUBLIC,
            true => DEVICEADDRTYPE_A::RANDOM,
        }
    }
    #[doc = "Checks if the value of the field is `PUBLIC`"]
    #[inline(always)]
    pub fn is_public(&self) -> bool {
        *self == DEVICEADDRTYPE_A::PUBLIC
    }
    #[doc = "Checks if the value of the field is `RANDOM`"]
    #[inline(always)]
    pub fn is_random(&self) -> bool {
        *self == DEVICEADDRTYPE_A::RANDOM
    }
}
#[doc = "Write proxy for field `DEVICEADDRTYPE`"]
pub struct DEVICEADDRTYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICEADDRTYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVICEADDRTYPE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Public address."]
    #[inline(always)]
    pub fn public(self) -> &'a mut W {
        self.variant(DEVICEADDRTYPE_A::PUBLIC)
    }
    #[doc = "Random address."]
    #[inline(always)]
    pub fn random(self) -> &'a mut W {
        self.variant(DEVICEADDRTYPE_A::RANDOM)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Device address type."]
    #[inline(always)]
    pub fn deviceaddrtype(&self) -> DEVICEADDRTYPE_R {
        DEVICEADDRTYPE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device address type."]
    #[inline(always)]
    pub fn deviceaddrtype(&mut self) -> DEVICEADDRTYPE_W {
        DEVICEADDRTYPE_W { w: self }
    }
}
