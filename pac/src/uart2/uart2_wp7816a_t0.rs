#[doc = "Register `WP7816A_T0` reader"]
pub type R = crate::R<Uart2Wp7816aT0Spec>;
#[doc = "Register `WP7816A_T0` writer"]
pub type W = crate::W<Uart2Wp7816aT0Spec>;
#[doc = "Field `WI_H` reader - Wait Time Integer High (C7816\\[TTYPE\\]
= 0)"]
pub type WiHR = crate::FieldReader;
#[doc = "Field `WI_H` writer - Wait Time Integer High (C7816\\[TTYPE\\]
= 0)"]
pub type WiHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Wait Time Integer High (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    pub fn wi_h(&self) -> WiHR {
        WiHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait Time Integer High (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    #[must_use]
    pub fn wi_h(&mut self) -> WiHW<Uart2Wp7816aT0Spec> {
        WiHW::new(self, 0)
    }
}
#[doc = "UART 7816 Wait Parameter Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_wp7816a_t0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_wp7816a_t0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart2Wp7816aT0Spec;
impl crate::RegisterSpec for Uart2Wp7816aT0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uart2_wp7816a_t0::R`](R) reader structure"]
impl crate::Readable for Uart2Wp7816aT0Spec {}
#[doc = "`write(|w| ..)` method takes [`uart2_wp7816a_t0::W`](W) writer structure"]
impl crate::Writable for Uart2Wp7816aT0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WP7816A_T0 to value 0"]
impl crate::Resettable for Uart2Wp7816aT0Spec {
    const RESET_VALUE: u8 = 0;
}
