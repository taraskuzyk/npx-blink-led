#[doc = "Register `DL` reader"]
pub type R = crate::R<DlSpec>;
#[doc = "Register `DL` writer"]
pub type W = crate::W<DlSpec>;
#[doc = "Field `Bits` reader - Data (low byte)"]
pub type BitsR = crate::FieldReader;
#[doc = "Field `Bits` writer - Data (low byte)"]
pub type BitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data (low byte)"]
    #[inline(always)]
    pub fn bits_(&self) -> BitsR {
        BitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data (low byte)"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BitsW<DlSpec> {
        BitsW::new(self, 0)
    }
}
#[doc = "SPI Data Register low\n\nYou can [`read`](crate::Reg::read) this register and get [`dl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlSpec;
impl crate::RegisterSpec for DlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dl::R`](R) reader structure"]
impl crate::Readable for DlSpec {}
#[doc = "`write(|w| ..)` method takes [`dl::W`](W) writer structure"]
impl crate::Writable for DlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DL to value 0"]
impl crate::Resettable for DlSpec {
    const RESET_VALUE: u8 = 0;
}
