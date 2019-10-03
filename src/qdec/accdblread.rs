#[doc = "Reader of register ACCDBLREAD"]
pub type R = crate::R<u32, super::ACCDBLREAD>;
#[doc = "Reader of field `ACCDBLREAD`"]
pub type ACCDBLREAD_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Snapshot of accumulated double (error) transitions."]
    #[inline(always)]
    pub fn accdblread(&self) -> ACCDBLREAD_R {
        ACCDBLREAD_R::new((self.bits & 0x0f) as u8)
    }
}
