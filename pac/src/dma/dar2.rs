#[doc = "Register `DAR2` reader"]
pub type R = crate::R<Dar2Spec>;
#[doc = "Register `DAR2` writer"]
pub type W = crate::W<Dar2Spec>;
#[doc = "Field `DAR` reader - DAR"]
pub type DarR = crate::FieldReader<u32>;
#[doc = "Field `DAR` writer - DAR"]
pub type DarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DAR"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DAR"]
    #[inline(always)]
    #[must_use]
    pub fn dar(&mut self) -> DarW<Dar2Spec> {
        DarW::new(self, 0)
    }
}
#[doc = "Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dar2Spec;
impl crate::RegisterSpec for Dar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar2::R`](R) reader structure"]
impl crate::Readable for Dar2Spec {}
#[doc = "`write(|w| ..)` method takes [`dar2::W`](W) writer structure"]
impl crate::Writable for Dar2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DAR2 to value 0"]
impl crate::Resettable for Dar2Spec {
    const RESET_VALUE: u32 = 0;
}
