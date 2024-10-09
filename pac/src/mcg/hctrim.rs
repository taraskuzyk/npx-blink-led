#[doc = "Register `HCTRIM` reader"]
pub type R = crate::R<HctrimSpec>;
#[doc = "Field `COARSE_TRIM` reader - High-frequency IRC Coarse Trim"]
pub type CoarseTrimR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - High-frequency IRC Coarse Trim"]
    #[inline(always)]
    pub fn coarse_trim(&self) -> CoarseTrimR {
        CoarseTrimR::new(self.bits & 0x3f)
    }
}
#[doc = "MCG High-frequency IRC Coarse Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctrim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HctrimSpec;
impl crate::RegisterSpec for HctrimSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`hctrim::R`](R) reader structure"]
impl crate::Readable for HctrimSpec {}
#[doc = "`reset()` method sets HCTRIM to value 0"]
impl crate::Resettable for HctrimSpec {
    const RESET_VALUE: u8 = 0;
}
