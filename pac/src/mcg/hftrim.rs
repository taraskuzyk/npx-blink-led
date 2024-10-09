#[doc = "Register `HFTRIM` reader"]
pub type R = crate::R<HftrimSpec>;
#[doc = "Field `FINE_TRIM` reader - High-frequency IRC Fine Trim"]
pub type FineTrimR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - High-frequency IRC Fine Trim"]
    #[inline(always)]
    pub fn fine_trim(&self) -> FineTrimR {
        FineTrimR::new(self.bits & 0x7f)
    }
}
#[doc = "MCG High-frequency IRC Fine Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hftrim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HftrimSpec;
impl crate::RegisterSpec for HftrimSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hftrim::R`](R) reader structure"]
impl crate::Readable for HftrimSpec {}
#[doc = "`reset()` method sets HFTRIM to value 0"]
impl crate::Resettable for HftrimSpec {
    const RESET_VALUE: u8 = 0;
}
