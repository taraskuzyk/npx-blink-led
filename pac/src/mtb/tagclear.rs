#[doc = "Register `TAGCLEAR` reader"]
pub type R = crate::R<TagclearSpec>;
#[doc = "Field `TAGCLEAR` reader - TAGCLEAR"]
pub type TagclearR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TAGCLEAR"]
    #[inline(always)]
    pub fn tagclear(&self) -> TagclearR {
        TagclearR::new(self.bits)
    }
}
#[doc = "Claim TAG Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tagclear::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TagclearSpec;
impl crate::RegisterSpec for TagclearSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tagclear::R`](R) reader structure"]
impl crate::Readable for TagclearSpec {}
#[doc = "`reset()` method sets TAGCLEAR to value 0"]
impl crate::Resettable for TagclearSpec {
    const RESET_VALUE: u32 = 0;
}
