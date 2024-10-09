#[doc = "Register `RCR5` reader"]
pub type R = crate::R<Rcr5Spec>;
#[doc = "Register `RCR5` writer"]
pub type W = crate::W<Rcr5Spec>;
#[doc = "Field `FBT` reader - First Bit Shifted"]
pub type FbtR = crate::FieldReader;
#[doc = "Field `FBT` writer - First Bit Shifted"]
pub type FbtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `W0W` reader - Word 0 Width"]
pub type W0wR = crate::FieldReader;
#[doc = "Field `W0W` writer - Word 0 Width"]
pub type W0wW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `WNW` reader - Word N Width"]
pub type WnwR = crate::FieldReader;
#[doc = "Field `WNW` writer - Word N Width"]
pub type WnwW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 8:12 - First Bit Shifted"]
    #[inline(always)]
    pub fn fbt(&self) -> FbtR {
        FbtR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Word 0 Width"]
    #[inline(always)]
    pub fn w0w(&self) -> W0wR {
        W0wR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Word N Width"]
    #[inline(always)]
    pub fn wnw(&self) -> WnwR {
        WnwR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:12 - First Bit Shifted"]
    #[inline(always)]
    #[must_use]
    pub fn fbt(&mut self) -> FbtW<Rcr5Spec> {
        FbtW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Word 0 Width"]
    #[inline(always)]
    #[must_use]
    pub fn w0w(&mut self) -> W0wW<Rcr5Spec> {
        W0wW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Word N Width"]
    #[inline(always)]
    #[must_use]
    pub fn wnw(&mut self) -> WnwW<Rcr5Spec> {
        WnwW::new(self, 24)
    }
}
#[doc = "SAI Receive Configuration 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcr5Spec;
impl crate::RegisterSpec for Rcr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr5::R`](R) reader structure"]
impl crate::Readable for Rcr5Spec {}
#[doc = "`write(|w| ..)` method takes [`rcr5::W`](W) writer structure"]
impl crate::Writable for Rcr5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR5 to value 0"]
impl crate::Resettable for Rcr5Spec {
    const RESET_VALUE: u32 = 0;
}
