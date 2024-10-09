#[doc = "Register `CLMD` reader"]
pub type R = crate::R<ClmdSpec>;
#[doc = "Register `CLMD` writer"]
pub type W = crate::W<ClmdSpec>;
#[doc = "Field `CLMD` reader - Calibration Value"]
pub type ClmdR = crate::FieldReader;
#[doc = "Field `CLMD` writer - Calibration Value"]
pub type ClmdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clmd(&self) -> ClmdR {
        ClmdR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clmd(&mut self) -> ClmdW<ClmdSpec> {
        ClmdW::new(self, 0)
    }
}
#[doc = "ADC Minus-Side General Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClmdSpec;
impl crate::RegisterSpec for ClmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clmd::R`](R) reader structure"]
impl crate::Readable for ClmdSpec {}
#[doc = "`write(|w| ..)` method takes [`clmd::W`](W) writer structure"]
impl crate::Writable for ClmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLMD to value 0x0a"]
impl crate::Resettable for ClmdSpec {
    const RESET_VALUE: u32 = 0x0a;
}
