#[doc = "Register `CVAL%s` reader"]
pub type R = crate::R<CvalSpec>;
#[doc = "Field `TVL` reader - Current Timer Value"]
pub type TvlR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current Timer Value"]
    #[inline(always)]
    pub fn tvl(&self) -> TvlR {
        TvlR::new(self.bits)
    }
}
#[doc = "Current Timer Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cval::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvalSpec;
impl crate::RegisterSpec for CvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cval::R`](R) reader structure"]
impl crate::Readable for CvalSpec {}
#[doc = "`reset()` method sets CVAL%s to value 0"]
impl crate::Resettable for CvalSpec {
    const RESET_VALUE: u32 = 0;
}
