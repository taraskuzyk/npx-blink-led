#[doc = "Register `RA` reader"]
pub type R = crate::R<RaSpec>;
#[doc = "Register `RA` writer"]
pub type W = crate::W<RaSpec>;
#[doc = "Field `RAD` reader - Range Slave Address"]
pub type RadR = crate::FieldReader;
#[doc = "Field `RAD` writer - Range Slave Address"]
pub type RadW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Range Slave Address"]
    #[inline(always)]
    pub fn rad(&self) -> RadR {
        RadR::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bits 1:7 - Range Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn rad(&mut self) -> RadW<RaSpec> {
        RadW::new(self, 1)
    }
}
#[doc = "I2C Range Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RaSpec;
impl crate::RegisterSpec for RaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ra::R`](R) reader structure"]
impl crate::Readable for RaSpec {}
#[doc = "`write(|w| ..)` method takes [`ra::W`](W) writer structure"]
impl crate::Writable for RaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RA to value 0"]
impl crate::Resettable for RaSpec {
    const RESET_VALUE: u8 = 0;
}
