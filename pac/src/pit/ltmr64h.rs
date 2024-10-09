#[doc = "Register `LTMR64H` reader"]
pub type R = crate::R<Ltmr64hSpec>;
#[doc = "Field `LTH` reader - Life Timer value"]
pub type LthR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Life Timer value"]
    #[inline(always)]
    pub fn lth(&self) -> LthR {
        LthR::new(self.bits)
    }
}
#[doc = "PIT Upper Lifetime Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ltmr64h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ltmr64hSpec;
impl crate::RegisterSpec for Ltmr64hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltmr64h::R`](R) reader structure"]
impl crate::Readable for Ltmr64hSpec {}
#[doc = "`reset()` method sets LTMR64H to value 0"]
impl crate::Resettable for Ltmr64hSpec {
    const RESET_VALUE: u32 = 0;
}
