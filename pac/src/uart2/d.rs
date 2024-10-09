#[doc = "Register `D` reader"]
pub type R = crate::R<DSpec>;
#[doc = "Register `D` writer"]
pub type W = crate::W<DSpec>;
#[doc = "Field `RT` reader - Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register"]
pub type RtR = crate::FieldReader;
#[doc = "Field `RT` writer - Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register"]
pub type RtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register"]
    #[inline(always)]
    pub fn rt(&self) -> RtR {
        RtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reads return the contents of the read-only receive data register and writes go to the write-only transmit data register"]
    #[inline(always)]
    #[must_use]
    pub fn rt(&mut self) -> RtW<DSpec> {
        RtW::new(self, 0)
    }
}
#[doc = "UART Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSpec;
impl crate::RegisterSpec for DSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`d::R`](R) reader structure"]
impl crate::Readable for DSpec {}
#[doc = "`write(|w| ..)` method takes [`d::W`](W) writer structure"]
impl crate::Writable for DSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets D to value 0"]
impl crate::Resettable for DSpec {
    const RESET_VALUE: u8 = 0;
}
