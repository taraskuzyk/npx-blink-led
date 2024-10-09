#[doc = "Register `DEVICEARCH` reader"]
pub type R = crate::R<DevicearchSpec>;
#[doc = "Field `DEVICEARCH` reader - DEVICEARCH"]
pub type DevicearchR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DEVICEARCH"]
    #[inline(always)]
    pub fn devicearch(&self) -> DevicearchR {
        DevicearchR::new(self.bits)
    }
}
#[doc = "Device Architecture Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devicearch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevicearchSpec;
impl crate::RegisterSpec for DevicearchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devicearch::R`](R) reader structure"]
impl crate::Readable for DevicearchSpec {}
#[doc = "`reset()` method sets DEVICEARCH to value 0x4770_0a31"]
impl crate::Resettable for DevicearchSpec {
    const RESET_VALUE: u32 = 0x4770_0a31;
}
