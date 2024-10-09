#[doc = "Register `MODECTRL` reader"]
pub type R = crate::R<ModectrlSpec>;
#[doc = "Field `MODECTRL` reader - MODECTRL"]
pub type ModectrlR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MODECTRL"]
    #[inline(always)]
    pub fn modectrl(&self) -> ModectrlR {
        ModectrlR::new(self.bits)
    }
}
#[doc = "Integration Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`modectrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModectrlSpec;
impl crate::RegisterSpec for ModectrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modectrl::R`](R) reader structure"]
impl crate::Readable for ModectrlSpec {}
#[doc = "`reset()` method sets MODECTRL to value 0"]
impl crate::Resettable for ModectrlSpec {
    const RESET_VALUE: u32 = 0;
}
