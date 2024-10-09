#[doc = "Register `DSR1` reader"]
pub type R = crate::R<Dsr1Spec>;
#[doc = "Register `DSR1` writer"]
pub type W = crate::W<Dsr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA_DSR1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsr1Spec;
impl crate::RegisterSpec for Dsr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dsr1::R`](R) reader structure"]
impl crate::Readable for Dsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dsr1::W`](W) writer structure"]
impl crate::Writable for Dsr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DSR1 to value 0"]
impl crate::Resettable for Dsr1Spec {
    const RESET_VALUE: u8 = 0;
}
