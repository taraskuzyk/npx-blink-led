#[doc = "Register `BACKKEY2` reader"]
pub type R = crate::R<Backkey2Spec>;
#[doc = "Field `KEY` reader - Backdoor Comparison Key."]
pub type KeyR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(self.bits)
    }
}
#[doc = "Backdoor Comparison Key 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`backkey2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Backkey2Spec;
impl crate::RegisterSpec for Backkey2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`backkey2::R`](R) reader structure"]
impl crate::Readable for Backkey2Spec {}
#[doc = "`reset()` method sets BACKKEY2 to value 0xff"]
impl crate::Resettable for Backkey2Spec {
    const RESET_VALUE: u8 = 0xff;
}
