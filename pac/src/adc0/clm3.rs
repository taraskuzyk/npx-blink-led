#[doc = "Register `CLM3` reader"]
pub type R = crate::R<Clm3Spec>;
#[doc = "Register `CLM3` writer"]
pub type W = crate::W<Clm3Spec>;
#[doc = "Field `CLM3` reader - Calibration Value"]
pub type Clm3R = crate::FieldReader<u16>;
#[doc = "Field `CLM3` writer - Calibration Value"]
pub type Clm3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    pub fn clm3(&self) -> Clm3R {
        Clm3R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clm3(&mut self) -> Clm3W<Clm3Spec> {
        Clm3W::new(self, 0)
    }
}
#[doc = "ADC Minus-Side General Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clm3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clm3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clm3Spec;
impl crate::RegisterSpec for Clm3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clm3::R`](R) reader structure"]
impl crate::Readable for Clm3Spec {}
#[doc = "`write(|w| ..)` method takes [`clm3::W`](W) writer structure"]
impl crate::Writable for Clm3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLM3 to value 0x0100"]
impl crate::Resettable for Clm3Spec {
    const RESET_VALUE: u32 = 0x0100;
}
