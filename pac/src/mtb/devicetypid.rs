#[doc = "Register `DEVICETYPID` reader"]
pub type R = crate::R<DevicetypidSpec>;
#[doc = "Field `DEVICETYPID` reader - DEVICETYPID"]
pub type DevicetypidR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DEVICETYPID"]
    #[inline(always)]
    pub fn devicetypid(&self) -> DevicetypidR {
        DevicetypidR::new(self.bits)
    }
}
#[doc = "Device Type Identifier Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devicetypid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevicetypidSpec;
impl crate::RegisterSpec for DevicetypidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devicetypid::R`](R) reader structure"]
impl crate::Readable for DevicetypidSpec {}
#[doc = "`reset()` method sets DEVICETYPID to value 0x31"]
impl crate::Resettable for DevicetypidSpec {
    const RESET_VALUE: u32 = 0x31;
}
