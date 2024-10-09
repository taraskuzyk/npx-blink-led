#[doc = "Register `UIDML` reader"]
pub type R = crate::R<UidmlSpec>;
#[doc = "Field `UID` reader - Unique Identification"]
pub type UidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid(&self) -> UidR {
        UidR::new(self.bits)
    }
}
#[doc = "Unique Identification Register Mid Low\n\nYou can [`read`](crate::Reg::read) this register and get [`uidml::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UidmlSpec;
impl crate::RegisterSpec for UidmlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uidml::R`](R) reader structure"]
impl crate::Readable for UidmlSpec {}
#[doc = "`reset()` method sets UIDML to value 0"]
impl crate::Resettable for UidmlSpec {
    const RESET_VALUE: u32 = 0;
}
