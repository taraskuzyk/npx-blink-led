#[doc = "Register `WP7816` reader"]
pub type R = crate::R<Wp7816Spec>;
#[doc = "Register `WP7816` writer"]
pub type W = crate::W<Wp7816Spec>;
#[doc = "Field `WTX` reader - Wait Time Multiplier (C7816\\[TTYPE\\]
= 1)"]
pub type WtxR = crate::FieldReader;
#[doc = "Field `WTX` writer - Wait Time Multiplier (C7816\\[TTYPE\\]
= 1)"]
pub type WtxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Wait Time Multiplier (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    pub fn wtx(&self) -> WtxR {
        WtxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait Time Multiplier (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    #[must_use]
    pub fn wtx(&mut self) -> WtxW<Wp7816Spec> {
        WtxW::new(self, 0)
    }
}
#[doc = "UART 7816 Wait Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wp7816::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp7816::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wp7816Spec;
impl crate::RegisterSpec for Wp7816Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wp7816::R`](R) reader structure"]
impl crate::Readable for Wp7816Spec {}
#[doc = "`write(|w| ..)` method takes [`wp7816::W`](W) writer structure"]
impl crate::Writable for Wp7816Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WP7816 to value 0"]
impl crate::Resettable for Wp7816Spec {
    const RESET_VALUE: u8 = 0;
}
