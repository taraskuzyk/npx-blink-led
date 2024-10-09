#[doc = "Register `BACKKEY7` reader"]
pub type R = crate::R<Backkey7Spec>;
#[doc = "Field `KEY` reader - Backdoor Comparison Key."]
pub type KeyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
#[doc = "Backdoor Comparison Key 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Backkey7Spec;
impl crate::RegisterSpec for Backkey7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`backkey7::R`](R) reader structure"]
impl crate::Readable for Backkey7Spec {}
#[doc = "`reset()` method sets BACKKEY7 to value 0xff"]
impl crate::Resettable for Backkey7Spec {
    const RESET_VALUE: u8 = 0xff;
}
