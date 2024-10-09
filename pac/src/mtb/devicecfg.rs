#[doc = "Register `DEVICECFG` reader"]
pub type R = crate::R<DevicecfgSpec>;
#[doc = "Field `DEVICECFG` reader - DEVICECFG"]
pub type DevicecfgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - DEVICECFG"]
    #[inline(always)]
    pub fn devicecfg(&self) -> DevicecfgR {
        DevicecfgR::new(self.bits)
    }
}
#[doc = "Device Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devicecfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevicecfgSpec;
impl crate::RegisterSpec for DevicecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devicecfg::R`](R) reader structure"]
impl crate::Readable for DevicecfgSpec {}
#[doc = "`reset()` method sets DEVICECFG to value 0"]
impl crate::Resettable for DevicecfgSpec {
    const RESET_VALUE: u32 = 0;
}
