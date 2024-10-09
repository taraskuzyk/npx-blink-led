#[doc = "Register `UIDMH` reader"]
pub type R = crate::R<UidmhSpec>;
#[doc = "Field `UID` reader - Unique Identification"]
pub type UidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Unique Identification"]
    #[inline(always)]
    pub fn uid(&self) -> UidR {
        UidR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Unique Identification Register Mid-High\n\nYou can [`read`](crate::Reg::read) this register and get [`uidmh::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UidmhSpec;
impl crate::RegisterSpec for UidmhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uidmh::R`](R) reader structure"]
impl crate::Readable for UidmhSpec {}
#[doc = "`reset()` method sets UIDMH to value 0"]
impl crate::Resettable for UidmhSpec {
    const RESET_VALUE: u32 = 0;
}
