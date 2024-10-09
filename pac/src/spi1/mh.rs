#[doc = "Register `MH` reader"]
pub type R = crate::R<MhSpec>;
#[doc = "Register `MH` writer"]
pub type W = crate::W<MhSpec>;
#[doc = "Field `Bits` reader - Hardware compare value (high byte)"]
pub type BitsR = crate::FieldReader;
#[doc = "Field `Bits` writer - Hardware compare value (high byte)"]
pub type BitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Hardware compare value (high byte)"]
    #[inline(always)]
    pub fn bits_(&self) -> BitsR {
        BitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Hardware compare value (high byte)"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BitsW<MhSpec> {
        BitsW::new(self, 0)
    }
}
#[doc = "SPI match register high\n\nYou can [`read`](crate::Reg::read) this register and get [`mh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MhSpec;
impl crate::RegisterSpec for MhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mh::R`](R) reader structure"]
impl crate::Readable for MhSpec {}
#[doc = "`write(|w| ..)` method takes [`mh::W`](W) writer structure"]
impl crate::Writable for MhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MH to value 0"]
impl crate::Resettable for MhSpec {
    const RESET_VALUE: u8 = 0;
}
