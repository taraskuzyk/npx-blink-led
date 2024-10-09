#[doc = "Register `TAGSET` reader"]
pub type R = crate::R<TagsetSpec>;
#[doc = "Field `TAGSET` reader - TAGSET"]
pub type TagsetR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TAGSET"]
    #[inline(always)]
    pub fn tagset(&self) -> TagsetR {
        TagsetR::new(self.bits)
    }
}
#[doc = "Claim TAG Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tagset::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TagsetSpec;
impl crate::RegisterSpec for TagsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tagset::R`](R) reader structure"]
impl crate::Readable for TagsetSpec {}
#[doc = "`reset()` method sets TAGSET to value 0"]
impl crate::Resettable for TagsetSpec {
    const RESET_VALUE: u32 = 0;
}
