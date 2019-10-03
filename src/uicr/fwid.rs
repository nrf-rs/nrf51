#[doc = "Reader of register FWID"]
pub type R = crate::R<u32, super::FWID>;
#[doc = "Reader of field `FWID`"]
pub type FWID_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Identification number for the firmware loaded into the chip."]
    #[inline(always)]
    pub fn fwid(&self) -> FWID_R {
        FWID_R::new((self.bits & 0xffff) as u16)
    }
}
