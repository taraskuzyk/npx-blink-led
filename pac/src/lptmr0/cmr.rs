#[doc = "Register `CMR` reader"]
pub type R = crate::R<CmrSpec>;
#[doc = "Register `CMR` writer"]
pub type W = crate::W<CmrSpec>;
#[doc = "Field `COMPARE` reader - Compare Value"]
pub type CompareR = crate::FieldReader<u16>;
#[doc = "Field `COMPARE` writer - Compare Value"]
pub type CompareW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Compare Value"]
    #[inline(always)]
    pub fn compare(&self) -> CompareR {
        CompareR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn compare(&mut self) -> CompareW<CmrSpec> {
        CompareW::new(self, 0)
    }
}
#[doc = "Low Power Timer Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmrSpec;
impl crate::RegisterSpec for CmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmr::R`](R) reader structure"]
impl crate::Readable for CmrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmr::W`](W) writer structure"]
impl crate::Writable for CmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CmrSpec {
    const RESET_VALUE: u32 = 0;
}
