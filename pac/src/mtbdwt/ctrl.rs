#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Field `DWTCFGCTRL` reader - DWT configuration controls"]
pub type DwtcfgctrlR = crate::FieldReader<u32>;
#[doc = "Field `NUMCMP` reader - Number of comparators"]
pub type NumcmpR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:27 - DWT configuration controls"]
    #[inline(always)]
    pub fn dwtcfgctrl(&self) -> DwtcfgctrlR {
        DwtcfgctrlR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - Number of comparators"]
    #[inline(always)]
    pub fn numcmp(&self) -> NumcmpR {
        NumcmpR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "MTB DWT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`reset()` method sets CTRL to value 0x2f00_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x2f00_0000;
}
