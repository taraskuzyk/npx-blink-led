#[doc = "Register `LTMR64L` reader"]
pub type R = crate::R<Ltmr64lSpec>;
#[doc = "Field `LTL` reader - Life Timer value"]
pub type LtlR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Life Timer value"]
    #[inline(always)]
    pub fn ltl(&self) -> LtlR {
        LtlR::new(self.bits)
    }
}
#[doc = "PIT Lower Lifetime Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ltmr64l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ltmr64lSpec;
impl crate::RegisterSpec for Ltmr64lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltmr64l::R`](R) reader structure"]
impl crate::Readable for Ltmr64lSpec {}
#[doc = "`reset()` method sets LTMR64L to value 0"]
impl crate::Resettable for Ltmr64lSpec {
    const RESET_VALUE: u32 = 0;
}
