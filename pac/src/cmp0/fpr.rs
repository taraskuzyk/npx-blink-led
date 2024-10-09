#[doc = "Register `FPR` reader"]
pub type R = crate::R<FprSpec>;
#[doc = "Register `FPR` writer"]
pub type W = crate::W<FprSpec>;
#[doc = "Field `FILT_PER` reader - Filter Sample Period"]
pub type FiltPerR = crate::FieldReader;
#[doc = "Field `FILT_PER` writer - Filter Sample Period"]
pub type FiltPerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Filter Sample Period"]
    #[inline(always)]
    pub fn filt_per(&self) -> FiltPerR {
        FiltPerR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Filter Sample Period"]
    #[inline(always)]
    #[must_use]
    pub fn filt_per(&mut self) -> FiltPerW<FprSpec> {
        FiltPerW::new(self, 0)
    }
}
#[doc = "CMP Filter Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FprSpec;
impl crate::RegisterSpec for FprSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fpr::R`](R) reader structure"]
impl crate::Readable for FprSpec {}
#[doc = "`write(|w| ..)` method takes [`fpr::W`](W) writer structure"]
impl crate::Writable for FprSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FPR to value 0"]
impl crate::Resettable for FprSpec {
    const RESET_VALUE: u8 = 0;
}
