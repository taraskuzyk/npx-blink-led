#[doc = "Register `LOCKSTAT` reader"]
pub type R = crate::R<LockstatSpec>;
#[doc = "Field `LOCKSTAT` reader - LOCKSTAT"]
pub type LockstatR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - LOCKSTAT"]
    #[inline(always)]
    pub fn lockstat(&self) -> LockstatR {
        LockstatR::new(self.bits)
    }
}
#[doc = "Lock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lockstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockstatSpec;
impl crate::RegisterSpec for LockstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockstat::R`](R) reader structure"]
impl crate::Readable for LockstatSpec {}
#[doc = "`reset()` method sets LOCKSTAT to value 0"]
impl crate::Resettable for LockstatSpec {
    const RESET_VALUE: u32 = 0;
}
