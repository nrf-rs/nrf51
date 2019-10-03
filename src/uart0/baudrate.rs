#[doc = "Reader of register BAUDRATE"]
pub type R = crate::R<u32, super::BAUDRATE>;
#[doc = "Writer for register BAUDRATE"]
pub type W = crate::W<u32, super::BAUDRATE>;
#[doc = "Register BAUDRATE `reset()`'s with value 0"]
impl crate::ResetValue for super::BAUDRATE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UART baudrate.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BAUDRATE_A {
    #[doc = "323584: 1200 baud."]
    BAUD1200,
    #[doc = "643072: 2400 baud."]
    BAUD2400,
    #[doc = "1290240: 4800 baud."]
    BAUD4800,
    #[doc = "2576384: 9600 baud."]
    BAUD9600,
    #[doc = "3866624: 14400 baud."]
    BAUD14400,
    #[doc = "5152768: 19200 baud."]
    BAUD19200,
    #[doc = "7729152: 28800 baud."]
    BAUD28800,
    #[doc = "10309632: 38400 baud."]
    BAUD38400,
    #[doc = "15462400: 57600 baud."]
    BAUD57600,
    #[doc = "20615168: 76800 baud."]
    BAUD76800,
    #[doc = "30924800: 115200 baud."]
    BAUD115200,
    #[doc = "61845504: 230400 baud."]
    BAUD230400,
    #[doc = "67108864: 250000 baud."]
    BAUD250000,
    #[doc = "123695104: 460800 baud."]
    BAUD460800,
    #[doc = "247386112: 921600 baud."]
    BAUD921600,
    #[doc = "268435456: 1M baud."]
    BAUD1M,
}
impl From<BAUDRATE_A> for u32 {
    #[inline(always)]
    fn from(variant: BAUDRATE_A) -> Self {
        match variant {
            BAUDRATE_A::BAUD1200 => 323584,
            BAUDRATE_A::BAUD2400 => 643072,
            BAUDRATE_A::BAUD4800 => 1290240,
            BAUDRATE_A::BAUD9600 => 2576384,
            BAUDRATE_A::BAUD14400 => 3866624,
            BAUDRATE_A::BAUD19200 => 5152768,
            BAUDRATE_A::BAUD28800 => 7729152,
            BAUDRATE_A::BAUD38400 => 10309632,
            BAUDRATE_A::BAUD57600 => 15462400,
            BAUDRATE_A::BAUD76800 => 20615168,
            BAUDRATE_A::BAUD115200 => 30924800,
            BAUDRATE_A::BAUD230400 => 61845504,
            BAUDRATE_A::BAUD250000 => 67108864,
            BAUDRATE_A::BAUD460800 => 123695104,
            BAUDRATE_A::BAUD921600 => 247386112,
            BAUDRATE_A::BAUD1M => 268435456,
        }
    }
}
#[doc = "Reader of field `BAUDRATE`"]
pub type BAUDRATE_R = crate::R<u32, BAUDRATE_A>;
impl BAUDRATE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, BAUDRATE_A> {
        use crate::Variant::*;
        match self.bits {
            323584 => Val(BAUDRATE_A::BAUD1200),
            643072 => Val(BAUDRATE_A::BAUD2400),
            1290240 => Val(BAUDRATE_A::BAUD4800),
            2576384 => Val(BAUDRATE_A::BAUD9600),
            3866624 => Val(BAUDRATE_A::BAUD14400),
            5152768 => Val(BAUDRATE_A::BAUD19200),
            7729152 => Val(BAUDRATE_A::BAUD28800),
            10309632 => Val(BAUDRATE_A::BAUD38400),
            15462400 => Val(BAUDRATE_A::BAUD57600),
            20615168 => Val(BAUDRATE_A::BAUD76800),
            30924800 => Val(BAUDRATE_A::BAUD115200),
            61845504 => Val(BAUDRATE_A::BAUD230400),
            67108864 => Val(BAUDRATE_A::BAUD250000),
            123695104 => Val(BAUDRATE_A::BAUD460800),
            247386112 => Val(BAUDRATE_A::BAUD921600),
            268435456 => Val(BAUDRATE_A::BAUD1M),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BAUD1200`"]
    #[inline(always)]
    pub fn is_baud1200(&self) -> bool {
        *self == BAUDRATE_A::BAUD1200
    }
    #[doc = "Checks if the value of the field is `BAUD2400`"]
    #[inline(always)]
    pub fn is_baud2400(&self) -> bool {
        *self == BAUDRATE_A::BAUD2400
    }
    #[doc = "Checks if the value of the field is `BAUD4800`"]
    #[inline(always)]
    pub fn is_baud4800(&self) -> bool {
        *self == BAUDRATE_A::BAUD4800
    }
    #[doc = "Checks if the value of the field is `BAUD9600`"]
    #[inline(always)]
    pub fn is_baud9600(&self) -> bool {
        *self == BAUDRATE_A::BAUD9600
    }
    #[doc = "Checks if the value of the field is `BAUD14400`"]
    #[inline(always)]
    pub fn is_baud14400(&self) -> bool {
        *self == BAUDRATE_A::BAUD14400
    }
    #[doc = "Checks if the value of the field is `BAUD19200`"]
    #[inline(always)]
    pub fn is_baud19200(&self) -> bool {
        *self == BAUDRATE_A::BAUD19200
    }
    #[doc = "Checks if the value of the field is `BAUD28800`"]
    #[inline(always)]
    pub fn is_baud28800(&self) -> bool {
        *self == BAUDRATE_A::BAUD28800
    }
    #[doc = "Checks if the value of the field is `BAUD38400`"]
    #[inline(always)]
    pub fn is_baud38400(&self) -> bool {
        *self == BAUDRATE_A::BAUD38400
    }
    #[doc = "Checks if the value of the field is `BAUD57600`"]
    #[inline(always)]
    pub fn is_baud57600(&self) -> bool {
        *self == BAUDRATE_A::BAUD57600
    }
    #[doc = "Checks if the value of the field is `BAUD76800`"]
    #[inline(always)]
    pub fn is_baud76800(&self) -> bool {
        *self == BAUDRATE_A::BAUD76800
    }
    #[doc = "Checks if the value of the field is `BAUD115200`"]
    #[inline(always)]
    pub fn is_baud115200(&self) -> bool {
        *self == BAUDRATE_A::BAUD115200
    }
    #[doc = "Checks if the value of the field is `BAUD230400`"]
    #[inline(always)]
    pub fn is_baud230400(&self) -> bool {
        *self == BAUDRATE_A::BAUD230400
    }
    #[doc = "Checks if the value of the field is `BAUD250000`"]
    #[inline(always)]
    pub fn is_baud250000(&self) -> bool {
        *self == BAUDRATE_A::BAUD250000
    }
    #[doc = "Checks if the value of the field is `BAUD460800`"]
    #[inline(always)]
    pub fn is_baud460800(&self) -> bool {
        *self == BAUDRATE_A::BAUD460800
    }
    #[doc = "Checks if the value of the field is `BAUD921600`"]
    #[inline(always)]
    pub fn is_baud921600(&self) -> bool {
        *self == BAUDRATE_A::BAUD921600
    }
    #[doc = "Checks if the value of the field is `BAUD1M`"]
    #[inline(always)]
    pub fn is_baud1m(&self) -> bool {
        *self == BAUDRATE_A::BAUD1M
    }
}
#[doc = "Write proxy for field `BAUDRATE`"]
pub struct BAUDRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> BAUDRATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BAUDRATE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1200 baud."]
    #[inline(always)]
    pub fn baud1200(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD1200)
    }
    #[doc = "2400 baud."]
    #[inline(always)]
    pub fn baud2400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD2400)
    }
    #[doc = "4800 baud."]
    #[inline(always)]
    pub fn baud4800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD4800)
    }
    #[doc = "9600 baud."]
    #[inline(always)]
    pub fn baud9600(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD9600)
    }
    #[doc = "14400 baud."]
    #[inline(always)]
    pub fn baud14400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD14400)
    }
    #[doc = "19200 baud."]
    #[inline(always)]
    pub fn baud19200(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD19200)
    }
    #[doc = "28800 baud."]
    #[inline(always)]
    pub fn baud28800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD28800)
    }
    #[doc = "38400 baud."]
    #[inline(always)]
    pub fn baud38400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD38400)
    }
    #[doc = "57600 baud."]
    #[inline(always)]
    pub fn baud57600(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD57600)
    }
    #[doc = "76800 baud."]
    #[inline(always)]
    pub fn baud76800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD76800)
    }
    #[doc = "115200 baud."]
    #[inline(always)]
    pub fn baud115200(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD115200)
    }
    #[doc = "230400 baud."]
    #[inline(always)]
    pub fn baud230400(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD230400)
    }
    #[doc = "250000 baud."]
    #[inline(always)]
    pub fn baud250000(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD250000)
    }
    #[doc = "460800 baud."]
    #[inline(always)]
    pub fn baud460800(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD460800)
    }
    #[doc = "921600 baud."]
    #[inline(always)]
    pub fn baud921600(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD921600)
    }
    #[doc = "1M baud."]
    #[inline(always)]
    pub fn baud1m(self) -> &'a mut W {
        self.variant(BAUDRATE_A::BAUD1M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - UART baudrate."]
    #[inline(always)]
    pub fn baudrate(&self) -> BAUDRATE_R {
        BAUDRATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - UART baudrate."]
    #[inline(always)]
    pub fn baudrate(&mut self) -> BAUDRATE_W {
        BAUDRATE_W { w: self }
    }
}
