#[doc = "Register `IDCOMP` reader"]
pub type R = crate::R<IdcompSpec>;
#[doc = "Field `NID` reader - Ones' complement of PERID\\[ID\\]. bits."]
pub type NidR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Ones' complement of PERID\\[ID\\]. bits."]
    #[inline(always)]
    pub fn nid(&self) -> NidR {
        NidR::new(self.bits & 0x3f)
    }
}
#[doc = "Peripheral ID Complement register\n\nYou can [`read`](crate::Reg::read) this register and get [`idcomp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdcompSpec;
impl crate::RegisterSpec for IdcompSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`idcomp::R`](R) reader structure"]
impl crate::Readable for IdcompSpec {}
#[doc = "`reset()` method sets IDCOMP to value 0xfb"]
impl crate::Resettable for IdcompSpec {
    const RESET_VALUE: u8 = 0xfb;
}
