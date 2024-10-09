#[doc = "Register `TDR` reader"]
pub type R = crate::R<TdrSpec>;
#[doc = "Register `TDR` writer"]
pub type W = crate::W<TdrSpec>;
#[doc = "Field `TDR` reader - Transmit Data Register"]
pub type TdrR = crate::FieldReader<u32>;
#[doc = "Field `TDR` writer - Transmit Data Register"]
pub type TdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Data Register"]
    #[inline(always)]
    pub fn tdr(&self) -> TdrR {
        TdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data Register"]
    #[inline(always)]
    #[must_use]
    pub fn tdr(&mut self) -> TdrW<TdrSpec> {
        TdrW::new(self, 0)
    }
}
#[doc = "SAI Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdr::R`](R) reader structure"]
impl crate::Readable for TdrSpec {}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TdrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TdrSpec {
    const RESET_VALUE: u32 = 0;
}
