#[doc = "Register `BASE` reader"]
pub type R = crate::R<BaseSpec>;
#[doc = "Field `BASEADDR` reader - BASEADDR"]
pub type BaseaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - BASEADDR"]
    #[inline(always)]
    pub fn baseaddr(&self) -> BaseaddrR {
        BaseaddrR::new(self.bits)
    }
}
#[doc = "MTB Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`base::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaseSpec;
impl crate::RegisterSpec for BaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`base::R`](R) reader structure"]
impl crate::Readable for BaseSpec {}
#[doc = "`reset()` method sets BASE to value 0"]
impl crate::Resettable for BaseSpec {
    const RESET_VALUE: u32 = 0;
}
