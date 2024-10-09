#[doc = "Register `PERIPHID%s` reader"]
pub type R = crate::R<PeriphidSpec>;
#[doc = "Field `PERIPHID` reader - PERIPHID"]
pub type PeriphidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - PERIPHID"]
    #[inline(always)]
    pub fn periphid(&self) -> PeriphidR {
        PeriphidR::new(self.bits)
    }
}
#[doc = "Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphidSpec;
impl crate::RegisterSpec for PeriphidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periphid::R`](R) reader structure"]
impl crate::Readable for PeriphidSpec {}
#[doc = "`reset()` method sets PERIPHID%s to value 0"]
impl crate::Resettable for PeriphidSpec {
    const RESET_VALUE: u32 = 0;
}
