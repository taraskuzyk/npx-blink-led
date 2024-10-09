#[doc = "Register `WP7816B_T1` reader"]
pub type R = crate::R<Uart2Wp7816bT1Spec>;
#[doc = "Register `WP7816B_T1` writer"]
pub type W = crate::W<Uart2Wp7816bT1Spec>;
#[doc = "Field `BWI_L` reader - Block Wait Time Integer Low (C7816\\[TTYPE\\]
= 1)"]
pub type BwiLR = crate::FieldReader;
#[doc = "Field `BWI_L` writer - Block Wait Time Integer Low (C7816\\[TTYPE\\]
= 1)"]
pub type BwiLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Block Wait Time Integer Low (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    pub fn bwi_l(&self) -> BwiLR {
        BwiLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Block Wait Time Integer Low (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    #[must_use]
    pub fn bwi_l(&mut self) -> BwiLW<Uart2Wp7816bT1Spec> {
        BwiLW::new(self, 0)
    }
}
#[doc = "UART 7816 Wait Parameter Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_wp7816b_t1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_wp7816b_t1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Uart2Wp7816bT1Spec;
impl crate::RegisterSpec for Uart2Wp7816bT1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`uart2_wp7816b_t1::R`](R) reader structure"]
impl crate::Readable for Uart2Wp7816bT1Spec {}
#[doc = "`write(|w| ..)` method takes [`uart2_wp7816b_t1::W`](W) writer structure"]
impl crate::Writable for Uart2Wp7816bT1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WP7816B_T1 to value 0x14"]
impl crate::Resettable for Uart2Wp7816bT1Spec {
    const RESET_VALUE: u8 = 0x14;
}
