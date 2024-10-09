#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Field `UNALIGN_TRP` reader - Always reads as one, indicates that all unaligned accesses generate a HardFault"]
pub type UnalignTrpR = crate::BitReader;
#[doc = "Field `STKALIGN` reader - Indicates stack alignment on exception entry"]
pub type StkalignR = crate::BitReader;
impl R {
    #[doc = "Bit 3 - Always reads as one, indicates that all unaligned accesses generate a HardFault"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UnalignTrpR {
        UnalignTrpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> StkalignR {
        StkalignR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Configuration and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`reset()` method sets CCR to value 0x0208"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0x0208;
}
