#[doc = "Register `CLP2` reader"]
pub type R = crate::R<Clp2Spec>;
#[doc = "Register `CLP2` writer"]
pub type W = crate::W<Clp2Spec>;
#[doc = "Field `CLP2` reader - Calibration Value"]
pub type Clp2R = crate::FieldReader;
#[doc = "Field `CLP2` writer - Calibration Value"]
pub type Clp2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn clp2(&self) -> Clp2R {
        Clp2R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clp2(&mut self) -> Clp2W<Clp2Spec> {
        Clp2W::new(self, 0)
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clp2Spec;
impl crate::RegisterSpec for Clp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clp2::R`](R) reader structure"]
impl crate::Readable for Clp2Spec {}
#[doc = "`write(|w| ..)` method takes [`clp2::W`](W) writer structure"]
impl crate::Writable for Clp2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLP2 to value 0x80"]
impl crate::Resettable for Clp2Spec {
    const RESET_VALUE: u32 = 0x80;
}
