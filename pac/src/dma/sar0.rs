#[doc = "Register `SAR0` reader"]
pub type R = crate::R<Sar0Spec>;
#[doc = "Register `SAR0` writer"]
pub type W = crate::W<Sar0Spec>;
#[doc = "Field `SAR` reader - SAR"]
pub type SarR = crate::FieldReader<u32>;
#[doc = "Field `SAR` writer - SAR"]
pub type SarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SAR"]
    #[inline(always)]
    pub fn sar(&self) -> SarR {
        SarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SAR"]
    #[inline(always)]
    #[must_use]
    pub fn sar(&mut self) -> SarW<Sar0Spec> {
        SarW::new(self, 0)
    }
}
#[doc = "Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sar0Spec;
impl crate::RegisterSpec for Sar0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar0::R`](R) reader structure"]
impl crate::Readable for Sar0Spec {}
#[doc = "`write(|w| ..)` method takes [`sar0::W`](W) writer structure"]
impl crate::Writable for Sar0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR0 to value 0"]
impl crate::Resettable for Sar0Spec {
    const RESET_VALUE: u32 = 0;
}
