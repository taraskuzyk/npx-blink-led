#[doc = "Register `DSR2` reader"]
pub type R = crate::R<Dsr2Spec>;
#[doc = "Register `DSR2` writer"]
pub type W = crate::W<Dsr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA_DSR2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsr2Spec;
impl crate::RegisterSpec for Dsr2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dsr2::R`](R) reader structure"]
impl crate::Readable for Dsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dsr2::W`](W) writer structure"]
impl crate::Writable for Dsr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DSR2 to value 0"]
impl crate::Resettable for Dsr2Spec {
    const RESET_VALUE: u8 = 0;
}
