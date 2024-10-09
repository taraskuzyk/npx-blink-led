#[doc = "Register `A2` reader"]
pub type R = crate::R<A2Spec>;
#[doc = "Register `A2` writer"]
pub type W = crate::W<A2Spec>;
#[doc = "Field `SAD` reader - SMBus Address"]
pub type SadR = crate::FieldReader;
#[doc = "Field `SAD` writer - SMBus Address"]
pub type SadW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - SMBus Address"]
    #[inline(always)]
    pub fn sad(&self) -> SadR {
        SadR::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bits 1:7 - SMBus Address"]
    #[inline(always)]
    #[must_use]
    pub fn sad(&mut self) -> SadW<A2Spec> {
        SadW::new(self, 1)
    }
}
#[doc = "I2C Address Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`a2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A2Spec;
impl crate::RegisterSpec for A2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`a2::R`](R) reader structure"]
impl crate::Readable for A2Spec {}
#[doc = "`write(|w| ..)` method takes [`a2::W`](W) writer structure"]
impl crate::Writable for A2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets A2 to value 0xc2"]
impl crate::Resettable for A2Spec {
    const RESET_VALUE: u8 = 0xc2;
}
