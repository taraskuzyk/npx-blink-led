#[doc = "Register `ENTRY%s` reader"]
pub type R = crate::R<EntrySpec>;
#[doc = "Field `ENTRY` reader - ENTRY"]
pub type EntryR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ENTRY"]
    #[inline(always)]
    pub fn entry(&self) -> EntryR {
        EntryR::new(self.bits)
    }
}
#[doc = "Entry\n\nYou can [`read`](crate::Reg::read) this register and get [`entry::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EntrySpec;
impl crate::RegisterSpec for EntrySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`entry::R`](R) reader structure"]
impl crate::Readable for EntrySpec {}
#[doc = "`reset()` method sets ENTRY%s to value 0"]
impl crate::Resettable for EntrySpec {
    const RESET_VALUE: u32 = 0;
}
