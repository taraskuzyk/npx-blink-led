#[doc = "Register `DSR0` reader"]
pub type R = crate::R<Dsr0Spec>;
#[doc = "Register `DSR0` writer"]
pub type W = crate::W<Dsr0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DMA_DSR0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsr0Spec;
impl crate::RegisterSpec for Dsr0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dsr0::R`](R) reader structure"]
impl crate::Readable for Dsr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dsr0::W`](W) writer structure"]
impl crate::Writable for Dsr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DSR0 to value 0"]
impl crate::Resettable for Dsr0Spec {
    const RESET_VALUE: u8 = 0;
}
