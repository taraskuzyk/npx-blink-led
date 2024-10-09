#[doc = "Register `LFTRIM` reader"]
pub type R = crate::R<LftrimSpec>;
#[doc = "Field `LIRC_FTRIM` reader - LIRC8M TRIM"]
pub type LircFtrimR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - LIRC8M TRIM"]
    #[inline(always)]
    pub fn lirc_ftrim(&self) -> LircFtrimR {
        LircFtrimR::new(self.bits & 0x7f)
    }
}
#[doc = "MCG Low-frequency IRC8M Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lftrim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LftrimSpec;
impl crate::RegisterSpec for LftrimSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lftrim::R`](R) reader structure"]
impl crate::Readable for LftrimSpec {}
#[doc = "`reset()` method sets LFTRIM to value 0"]
impl crate::Resettable for LftrimSpec {
    const RESET_VALUE: u8 = 0;
}
