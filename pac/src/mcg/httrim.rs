#[doc = "Register `HTTRIM` reader"]
pub type R = crate::R<HttrimSpec>;
#[doc = "Field `TEMPCO_TRIM` reader - High-frequency IRC Tempco Trim"]
pub type TempcoTrimR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - High-frequency IRC Tempco Trim"]
    #[inline(always)]
    pub fn tempco_trim(&self) -> TempcoTrimR {
        TempcoTrimR::new(self.bits & 0x1f)
    }
}
#[doc = "MCG High-frequency IRC Tempco (Temperature Coefficient) Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`httrim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HttrimSpec;
impl crate::RegisterSpec for HttrimSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`httrim::R`](R) reader structure"]
impl crate::Readable for HttrimSpec {}
#[doc = "`reset()` method sets HTTRIM to value 0"]
impl crate::Resettable for HttrimSpec {
    const RESET_VALUE: u8 = 0;
}
