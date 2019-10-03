#[doc = "Reader of register AMOUNTTX"]
pub type R = crate::R<u32, super::AMOUNTTX>;
#[doc = "Reader of field `AMOUNTTX`"]
pub type AMOUNTTX_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Number of bytes transmitted in last granted transaction."]
    #[inline(always)]
    pub fn amounttx(&self) -> AMOUNTTX_R {
        AMOUNTTX_R::new((self.bits & 0xff) as u8)
    }
}
