#[doc = "Register `WN7816` reader"]
pub type R = crate::R<Wn7816Spec>;
#[doc = "Register `WN7816` writer"]
pub type W = crate::W<Wn7816Spec>;
#[doc = "Field `GTN` reader - Guard Band N"]
pub type GtnR = crate::FieldReader;
#[doc = "Field `GTN` writer - Guard Band N"]
pub type GtnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Guard Band N"]
    #[inline(always)]
    pub fn gtn(&self) -> GtnR {
        GtnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Guard Band N"]
    #[inline(always)]
    #[must_use]
    pub fn gtn(&mut self) -> GtnW<Wn7816Spec> {
        GtnW::new(self, 0)
    }
}
#[doc = "UART 7816 Wait N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wn7816::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wn7816::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wn7816Spec;
impl crate::RegisterSpec for Wn7816Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wn7816::R`](R) reader structure"]
impl crate::Readable for Wn7816Spec {}
#[doc = "`write(|w| ..)` method takes [`wn7816::W`](W) writer structure"]
impl crate::Writable for Wn7816Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WN7816 to value 0"]
impl crate::Resettable for Wn7816Spec {
    const RESET_VALUE: u8 = 0;
}
