#[doc = "Register `DH` reader"]
pub type R = crate::R<DhSpec>;
#[doc = "Register `DH` writer"]
pub type W = crate::W<DhSpec>;
#[doc = "Field `Bits` reader - Data (high byte)"]
pub type BitsR = crate::FieldReader;
#[doc = "Field `Bits` writer - Data (high byte)"]
pub type BitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data (high byte)"]
    #[inline(always)]
    pub fn bits_(&self) -> BitsR {
        BitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data (high byte)"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BitsW<DhSpec> {
        BitsW::new(self, 0)
    }
}
#[doc = "SPI data register high\n\nYou can [`read`](crate::Reg::read) this register and get [`dh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DhSpec;
impl crate::RegisterSpec for DhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dh::R`](R) reader structure"]
impl crate::Readable for DhSpec {}
#[doc = "`write(|w| ..)` method takes [`dh::W`](W) writer structure"]
impl crate::Writable for DhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DH to value 0"]
impl crate::Resettable for DhSpec {
    const RESET_VALUE: u8 = 0;
}
