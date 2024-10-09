#[doc = "Register `AP7816A_T0` reader"]
pub type R = crate::R<Ap7816aT0Spec>;
#[doc = "Register `AP7816A_T0` writer"]
pub type W = crate::W<Ap7816aT0Spec>;
#[doc = "Field `ADTI_H` reader - ATR Duration Time Integer High (C7816\\[TTYPE\\]
= 0)"]
pub type AdtiHR = crate::FieldReader;
#[doc = "Field `ADTI_H` writer - ATR Duration Time Integer High (C7816\\[TTYPE\\]
= 0)"]
pub type AdtiHW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ATR Duration Time Integer High (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    pub fn adti_h(&self) -> AdtiHR {
        AdtiHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - ATR Duration Time Integer High (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    #[must_use]
    pub fn adti_h(&mut self) -> AdtiHW<Ap7816aT0Spec> {
        AdtiHW::new(self, 0)
    }
}
#[doc = "UART 7816 ATR Duration Timer Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`ap7816a_t0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ap7816a_t0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ap7816aT0Spec;
impl crate::RegisterSpec for Ap7816aT0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ap7816a_t0::R`](R) reader structure"]
impl crate::Readable for Ap7816aT0Spec {}
#[doc = "`write(|w| ..)` method takes [`ap7816a_t0::W`](W) writer structure"]
impl crate::Writable for Ap7816aT0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AP7816A_T0 to value 0"]
impl crate::Resettable for Ap7816aT0Spec {
    const RESET_VALUE: u8 = 0;
}
