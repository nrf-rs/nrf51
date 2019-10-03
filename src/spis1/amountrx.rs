#[doc = "Reader of register AMOUNTRX"]
pub type R = crate::R<u32, super::AMOUNTRX>;
#[doc = "Reader of field `AMOUNTRX`"]
pub type AMOUNTRX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of bytes received in last granted transaction."]
    #[inline(always)]
    pub fn amountrx(&self) -> AMOUNTRX_R {
        AMOUNTRX_R::new((self.bits & 0xff) as u8)
    }
}
