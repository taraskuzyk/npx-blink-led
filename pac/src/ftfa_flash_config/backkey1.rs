#[doc = "Register `BACKKEY1` reader"]
pub type R = crate::R<Backkey1Spec>;
#[doc = "Field `KEY` reader - Backdoor Comparison Key."]
pub type KeyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
#[doc = "Backdoor Comparison Key 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Backkey1Spec;
impl crate::RegisterSpec for Backkey1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`backkey1::R`](R) reader structure"]
impl crate::Readable for Backkey1Spec {}
#[doc = "`reset()` method sets BACKKEY1 to value 0xff"]
impl crate::Resettable for Backkey1Spec {
    const RESET_VALUE: u8 = 0xff;
}
