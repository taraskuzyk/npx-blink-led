#[doc = "Register `UIDL` reader"]
pub type R = crate::R<UidlSpec>;
#[doc = "Field `UID` reader - Unique Identification"]
pub type UidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid(&self) -> UidR {
        UidR::new(self.bits)
    }
}
#[doc = "Unique Identification Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`uidl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UidlSpec;
impl crate::RegisterSpec for UidlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uidl::R`](R) reader structure"]
impl crate::Readable for UidlSpec {}
#[doc = "`reset()` method sets UIDL to value 0"]
impl crate::Resettable for UidlSpec {
    const RESET_VALUE: u32 = 0;
}
