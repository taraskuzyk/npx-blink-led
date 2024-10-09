#[doc = "Register `DAT%sH` reader"]
pub type R = crate::R<DathSpec>;
#[doc = "Register `DAT%sH` writer"]
pub type W = crate::W<DathSpec>;
#[doc = "Field `DATA1` reader - DATA1"]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - DATA1"]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DATA1"]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - DATA1"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> Data1W<DathSpec> {
        Data1W::new(self, 0)
    }
}
#[doc = "DAC Data High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dath::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dath::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DathSpec;
impl crate::RegisterSpec for DathSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dath::R`](R) reader structure"]
impl crate::Readable for DathSpec {}
#[doc = "`write(|w| ..)` method takes [`dath::W`](W) writer structure"]
impl crate::Writable for DathSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DAT%sH to value 0"]
impl crate::Resettable for DathSpec {
    const RESET_VALUE: u8 = 0;
}
