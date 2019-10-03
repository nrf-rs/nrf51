#[doc = "Reader of register RSSISAMPLE"]
pub type R = crate::R<u32, super::RSSISAMPLE>;
#[doc = "Reader of field `RSSISAMPLE`"]
pub type RSSISAMPLE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - RSSI sample result. The result is read as a positive value so that ReceivedSignalStrength = -RSSISAMPLE dBm"]
    #[inline(always)]
    pub fn rssisample(&self) -> RSSISAMPLE_R {
        RSSISAMPLE_R::new((self.bits & 0x7f) as u8)
    }
}
