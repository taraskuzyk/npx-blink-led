#[doc = "Register `AP7816B_T0` reader"]
pub type R = crate::R<Ap7816bT0Spec>;
#[doc = "Register `AP7816B_T0` writer"]
pub type W = crate::W<Ap7816bT0Spec>;
#[doc = "Field `ADTI_L` reader - ATR Duration Time Integer Low (C7816\\[TTYPE\\]
= 0)"]
pub type AdtiLR = crate::FieldReader;
#[doc = "Field `ADTI_L` writer - ATR Duration Time Integer Low (C7816\\[TTYPE\\]
= 0)"]
pub type AdtiLW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ATR Duration Time Integer Low (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    pub fn adti_l(&self) -> AdtiLR {
        AdtiLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATR Duration Time Integer Low (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    #[must_use]
    pub fn adti_l(&mut self) -> AdtiLW<Ap7816bT0Spec> {
        AdtiLW::new(self, 0)
    }
}
#[doc = "UART 7816 ATR Duration Timer Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`ap7816b_t0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ap7816b_t0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ap7816bT0Spec;
impl crate::RegisterSpec for Ap7816bT0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ap7816b_t0::R`](R) reader structure"]
impl crate::Readable for Ap7816bT0Spec {}
#[doc = "`write(|w| ..)` method takes [`ap7816b_t0::W`](W) writer structure"]
impl crate::Writable for Ap7816bT0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AP7816B_T0 to value 0"]
impl crate::Resettable for Ap7816bT0Spec {
    const RESET_VALUE: u8 = 0;
}
