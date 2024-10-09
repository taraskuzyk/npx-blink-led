#[doc = "Register `ML` reader"]
pub type R = crate::R<MlSpec>;
#[doc = "Register `ML` writer"]
pub type W = crate::W<MlSpec>;
#[doc = "Field `Bits` reader - Hardware compare value (low byte)"]
pub type BitsR = crate::FieldReader;
#[doc = "Field `Bits` writer - Hardware compare value (low byte)"]
pub type BitsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Hardware compare value (low byte)"]
    #[inline(always)]
    pub fn bits_(&self) -> BitsR {
        BitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Hardware compare value (low byte)"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BitsW<MlSpec> {
        BitsW::new(self, 0)
    }
}
#[doc = "SPI Match Register low\n\nYou can [`read`](crate::Reg::read) this register and get [`ml::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ml::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MlSpec;
impl crate::RegisterSpec for MlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ml::R`](R) reader structure"]
impl crate::Readable for MlSpec {}
#[doc = "`write(|w| ..)` method takes [`ml::W`](W) writer structure"]
impl crate::Writable for MlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ML to value 0"]
impl crate::Resettable for MlSpec {
    const RESET_VALUE: u8 = 0;
}
