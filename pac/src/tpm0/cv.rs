#[doc = "Register `C%sV` reader"]
pub type R = crate::R<CvSpec>;
#[doc = "Register `C%sV` writer"]
pub type W = crate::W<CvSpec>;
#[doc = "Field `VAL` reader - Channel Value"]
pub type ValR = crate::FieldReader<u16>;
#[doc = "Field `VAL` writer - Channel Value"]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel Value"]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel Value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<CvSpec> {
        ValW::new(self, 0)
    }
}
#[doc = "Channel (n) Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CvSpec;
impl crate::RegisterSpec for CvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cv::R`](R) reader structure"]
impl crate::Readable for CvSpec {}
#[doc = "`write(|w| ..)` method takes [`cv::W`](W) writer structure"]
impl crate::Writable for CvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C%sV to value 0"]
impl crate::Resettable for CvSpec {
    const RESET_VALUE: u32 = 0;
}
