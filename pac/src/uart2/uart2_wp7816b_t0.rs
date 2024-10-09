#[doc = "Register `WP7816B_T0` reader"]
pub type R = crate::R<Uart2Wp7816bT0Spec>;
#[doc = "Register `WP7816B_T0` writer"]
pub type W = crate::W<Uart2Wp7816bT0Spec>;
#[doc = "Field `WI_L` reader - Wait Time Integer Low (C7816\\[TTYPE\\]
= 0)"]
pub type WiLR = crate::FieldReader;
#[doc = "Field `WI_L` writer - Wait Time Integer Low (C7816\\[TTYPE\\]
= 0)"]
pub type WiLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Wait Time Integer Low (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    pub fn wi_l(&self) -> WiLR {
        WiLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait Time Integer Low (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    #[must_use]
    pub fn wi_l(&mut self) -> WiLW<Uart2Wp7816bT0Spec> {
        WiLW::new(self, 0)
    }
}
#[doc = "UART 7816 Wait Parameter Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_wp7816b_t0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_wp7816b_t0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart2Wp7816bT0Spec;
impl crate::RegisterSpec for Uart2Wp7816bT0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uart2_wp7816b_t0::R`](R) reader structure"]
impl crate::Readable for Uart2Wp7816bT0Spec {}
#[doc = "`write(|w| ..)` method takes [`uart2_wp7816b_t0::W`](W) writer structure"]
impl crate::Writable for Uart2Wp7816bT0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WP7816B_T0 to value 0x14"]
impl crate::Resettable for Uart2Wp7816bT0Spec {
    const RESET_VALUE: u8 = 0x14;
}
