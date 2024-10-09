#[doc = "Register `BACKKEY6` reader"]
pub type R = crate::R<Backkey6Spec>;
#[doc = "Field `KEY` reader - Backdoor Comparison Key."]
pub type KeyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
#[doc = "Backdoor Comparison Key 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Backkey6Spec;
impl crate::RegisterSpec for Backkey6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`backkey6::R`](R) reader structure"]
impl crate::Readable for Backkey6Spec {}
#[doc = "`reset()` method sets BACKKEY6 to value 0xff"]
impl crate::Resettable for Backkey6Spec {
    const RESET_VALUE: u8 = 0xff;
}
