#[doc = "Register `FOPT` reader"]
pub type R = crate::R<FoptSpec>;
#[doc = "Field `OPT` reader - Nonvolatile Option"]
pub type OptR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Nonvolatile Option"]
    #[inline(always)]
    pub fn opt(&self) -> OptR {
        OptR::new(self.bits)
    }
}
#[doc = "Flash Option Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fopt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FoptSpec;
impl crate::RegisterSpec for FoptSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fopt::R`](R) reader structure"]
impl crate::Readable for FoptSpec {}
#[doc = "`reset()` method sets FOPT to value 0"]
impl crate::Resettable for FoptSpec {
    const RESET_VALUE: u8 = 0;
}
