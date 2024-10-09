#[doc = "Register `LOCKACCESS` reader"]
pub type R = crate::R<LockaccessSpec>;
#[doc = "Field `LOCKACCESS` reader - Hardwired to 0x0000_0000"]
pub type LockaccessR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Hardwired to 0x0000_0000"]
    #[inline(always)]
    pub fn lockaccess(&self) -> LockaccessR {
        LockaccessR::new(self.bits)
    }
}
#[doc = "Lock Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lockaccess::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockaccessSpec;
impl crate::RegisterSpec for LockaccessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockaccess::R`](R) reader structure"]
impl crate::Readable for LockaccessSpec {}
#[doc = "`reset()` method sets LOCKACCESS to value 0"]
impl crate::Resettable for LockaccessSpec {
    const RESET_VALUE: u32 = 0;
}
