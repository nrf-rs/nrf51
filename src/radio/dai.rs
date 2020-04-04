#[doc = "Reader of register DAI"]
pub type R = crate::R<u32, super::DAI>;
#[doc = "Reader of field `DAI`"]
pub type DAI_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Index (n) of device address (see DAB\\[n\\]
and DAP\\[n\\]) that obtained an address match."]
    #[inline(always)]
    pub fn dai(&self) -> DAI_R {
        DAI_R::new((self.bits & 0x07) as u8)
    }
}
