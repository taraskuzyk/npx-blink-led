#[doc = "Register `ADDINFO` reader"]
pub type R = crate::R<AddinfoSpec>;
#[doc = "Field `IEHOST` reader - This bit is set if host mode is enabled."]
pub type IehostR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit is set if host mode is enabled."]
    #[inline(always)]
    pub fn iehost(&self) -> IehostR {
        IehostR::new((self.bits & 1) != 0)
    }
}
#[doc = "Peripheral Additional Info register\n\nYou can [`read`](crate::Reg::read) this register and get [`addinfo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddinfoSpec;
impl crate::RegisterSpec for AddinfoSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`addinfo::R`](R) reader structure"]
impl crate::Readable for AddinfoSpec {}
#[doc = "`reset()` method sets ADDINFO to value 0x01"]
impl crate::Resettable for AddinfoSpec {
    const RESET_VALUE: u8 = 0x01;
}
