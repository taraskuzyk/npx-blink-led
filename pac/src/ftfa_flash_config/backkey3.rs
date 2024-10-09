#[doc = "Register `BACKKEY3` reader"]
pub type R = crate::R<Backkey3Spec>;
#[doc = "Field `KEY` reader - Backdoor Comparison Key."]
pub type KeyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
#[doc = "Backdoor Comparison Key 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Backkey3Spec;
impl crate::RegisterSpec for Backkey3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`backkey3::R`](R) reader structure"]
impl crate::Readable for Backkey3Spec {}
#[doc = "`reset()` method sets BACKKEY3 to value 0xff"]
impl crate::Resettable for Backkey3Spec {
    const RESET_VALUE: u8 = 0xff;
}
