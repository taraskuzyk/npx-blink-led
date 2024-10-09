#[doc = "Register `SYSACCESS` reader"]
pub type R = crate::R<SysaccessSpec>;
#[doc = "Field `SYSACCESS` reader - SYSACCESS"]
pub type SysaccessR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SYSACCESS"]
    #[inline(always)]
    pub fn sysaccess(&self) -> SysaccessR {
        SysaccessR::new(self.bits)
    }
}
#[doc = "System Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysaccess::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysaccessSpec;
impl crate::RegisterSpec for SysaccessSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysaccess::R`](R) reader structure"]
impl crate::Readable for SysaccessSpec {}
#[doc = "`reset()` method sets SYSACCESS to value 0x01"]
impl crate::Resettable for SysaccessSpec {
    const RESET_VALUE: u32 = 0x01;
}
