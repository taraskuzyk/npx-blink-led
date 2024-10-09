#[doc = "Register `CLP4` reader"]
pub type R = crate::R<Clp4Spec>;
#[doc = "Register `CLP4` writer"]
pub type W = crate::W<Clp4Spec>;
#[doc = "Field `CLP4` reader - Calibration Value"]
pub type Clp4R = crate::FieldReader<u16>;
#[doc = "Field `CLP4` writer - Calibration Value"]
pub type Clp4W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clp4(&self) -> Clp4R {
        Clp4R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clp4(&mut self) -> Clp4W<Clp4Spec> {
        Clp4W::new(self, 0)
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clp4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clp4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clp4Spec;
impl crate::RegisterSpec for Clp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clp4::R`](R) reader structure"]
impl crate::Readable for Clp4Spec {}
#[doc = "`write(|w| ..)` method takes [`clp4::W`](W) writer structure"]
impl crate::Writable for Clp4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLP4 to value 0x0200"]
impl crate::Resettable for Clp4Spec {
    const RESET_VALUE: u32 = 0x0200;
}
