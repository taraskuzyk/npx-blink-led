#[doc = "Register `SLTL` reader"]
pub type R = crate::R<SltlSpec>;
#[doc = "Register `SLTL` writer"]
pub type W = crate::W<SltlSpec>;
#[doc = "Field `SSLT` reader - SSLT\\[7:0\\]"]
pub type SsltR = crate::FieldReader;
#[doc = "Field `SSLT` writer - SSLT\\[7:0\\]"]
pub type SsltW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SSLT\\[7:0\\]"]
    #[inline(always)]
    pub fn sslt(&self) -> SsltR {
        SsltR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - SSLT\\[7:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sslt(&mut self) -> SsltW<SltlSpec> {
        SsltW::new(self, 0)
    }
}
#[doc = "I2C SCL Low Timeout Register Low\n\nYou can [`read`](crate::Reg::read) this register and get [`sltl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sltl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SltlSpec;
impl crate::RegisterSpec for SltlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sltl::R`](R) reader structure"]
impl crate::Readable for SltlSpec {}
#[doc = "`write(|w| ..)` method takes [`sltl::W`](W) writer structure"]
impl crate::Writable for SltlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SLTL to value 0"]
impl crate::Resettable for SltlSpec {
    const RESET_VALUE: u8 = 0;
}
