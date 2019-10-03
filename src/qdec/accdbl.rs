#[doc = "Reader of register ACCDBL"]
pub type R = crate::R<u32, super::ACCDBL>;
#[doc = "Reader of field `ACCDBL`"]
pub type ACCDBL_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Accumulated double (error) transitions."]
    #[inline(always)]
    pub fn accdbl(&self) -> ACCDBL_R {
        ACCDBL_R::new((self.bits & 0x0f) as u8)
    }
}
