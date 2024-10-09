#[doc = "Register `BACKKEY5` reader"]
pub type R = crate::R<Backkey5Spec>;
#[doc = "Field `KEY` reader - Backdoor Comparison Key."]
pub type KeyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
#[doc = "Backdoor Comparison Key 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Backkey5Spec;
impl crate::RegisterSpec for Backkey5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`backkey5::R`](R) reader structure"]
impl crate::Readable for Backkey5Spec {}
#[doc = "`reset()` method sets BACKKEY5 to value 0xff"]
impl crate::Resettable for Backkey5Spec {
    const RESET_VALUE: u8 = 0xff;
}
