#[doc = "Register `SRVCOP` writer"]
pub type W = crate::W<SrvcopSpec>;
#[doc = "Field `SRVCOP` writer - Service COP Register"]
pub type SrvcopW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Service COP Register"]
    #[inline(always)]
    #[must_use]
    pub fn srvcop(&mut self) -> SrvcopW<SrvcopSpec> {
        SrvcopW::new(self, 0)
    }
}
#[doc = "Service COP\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srvcop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrvcopSpec;
impl crate::RegisterSpec for SrvcopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`srvcop::W`](W) writer structure"]
impl crate::Writable for SrvcopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRVCOP to value 0"]
impl crate::Resettable for SrvcopSpec {
    const RESET_VALUE: u32 = 0;
}
