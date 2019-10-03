#[doc = "Reader of register PCNF0"]
pub type R = crate::R<u32, super::PCNF0>;
#[doc = "Writer for register PCNF0"]
pub type W = crate::W<u32, super::PCNF0>;
#[doc = "Register PCNF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCNF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LFLEN`"]
pub type LFLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LFLEN`"]
pub struct LFLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `S0LEN`"]
pub type S0LEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `S0LEN`"]
pub struct S0LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S0LEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `S1LEN`"]
pub type S1LEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `S1LEN`"]
pub struct S1LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> S1LEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Length of length field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn lflen(&self) -> LFLEN_R {
        LFLEN_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Length of S0 field in number of bytes. Decision point: START task."]
    #[inline(always)]
    pub fn s0len(&self) -> S0LEN_R {
        S0LEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Length of S1 field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn s1len(&self) -> S1LEN_R {
        S1LEN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Length of length field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn lflen(&mut self) -> LFLEN_W {
        LFLEN_W { w: self }
    }
    #[doc = "Bit 8 - Length of S0 field in number of bytes. Decision point: START task."]
    #[inline(always)]
    pub fn s0len(&mut self) -> S0LEN_W {
        S0LEN_W { w: self }
    }
    #[doc = "Bits 16:19 - Length of S1 field in number of bits. Decision point: START task."]
    #[inline(always)]
    pub fn s1len(&mut self) -> S1LEN_W {
        S1LEN_W { w: self }
    }
}
