#[doc = "Register `COMPID%s` reader"]
pub type R = crate::R<CompidSpec>;
#[doc = "Field `COMPID` reader - Component ID"]
pub type CompidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Component ID"]
    #[inline(always)]
    pub fn compid(&self) -> CompidR {
        CompidR::new(self.bits)
    }
}
#[doc = "Component ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompidSpec;
impl crate::RegisterSpec for CompidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`compid::R`](R) reader structure"]
impl crate::Readable for CompidSpec {}
#[doc = "`reset()` method sets COMPID%s to value 0"]
impl crate::Resettable for CompidSpec {
    const RESET_VALUE: u32 = 0;
}
