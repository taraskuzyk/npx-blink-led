#[doc = "Register `PMSTAT` reader"]
pub type R = crate::R<PmstatSpec>;
#[doc = "Field `PMSTAT` reader - Power Mode Status"]
pub type PmstatR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Power Mode Status"]
    #[inline(always)]
    pub fn pmstat(&self) -> PmstatR {
        PmstatR::new(self.bits)
    }
}
#[doc = "Power Mode Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmstatSpec;
impl crate::RegisterSpec for PmstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmstat::R`](R) reader structure"]
impl crate::Readable for PmstatSpec {}
#[doc = "`reset()` method sets PMSTAT to value 0x01"]
impl crate::Resettable for PmstatSpec {
    const RESET_VALUE: u8 = 0x01;
}
