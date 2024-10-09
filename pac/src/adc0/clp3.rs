#[doc = "Register `CLP3` reader"]
pub type R = crate::R<Clp3Spec>;
#[doc = "Register `CLP3` writer"]
pub type W = crate::W<Clp3Spec>;
#[doc = "Field `CLP3` reader - Calibration Value"]
pub type Clp3R = crate::FieldReader<u16>;
#[doc = "Field `CLP3` writer - Calibration Value"]
pub type Clp3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    pub fn clp3(&self) -> Clp3R {
        Clp3R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clp3(&mut self) -> Clp3W<Clp3Spec> {
        Clp3W::new(self, 0)
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clp3Spec;
impl crate::RegisterSpec for Clp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clp3::R`](R) reader structure"]
impl crate::Readable for Clp3Spec {}
#[doc = "`write(|w| ..)` method takes [`clp3::W`](W) writer structure"]
impl crate::Writable for Clp3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLP3 to value 0x0100"]
impl crate::Resettable for Clp3Spec {
    const RESET_VALUE: u32 = 0x0100;
}
