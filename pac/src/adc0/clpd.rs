#[doc = "Register `CLPD` reader"]
pub type R = crate::R<ClpdSpec>;
#[doc = "Register `CLPD` writer"]
pub type W = crate::W<ClpdSpec>;
#[doc = "Field `CLPD` reader - Calibration Value"]
pub type ClpdR = crate::FieldReader;
#[doc = "Field `CLPD` writer - Calibration Value"]
pub type ClpdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clpd(&self) -> ClpdR {
        ClpdR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clpd(&mut self) -> ClpdW<ClpdSpec> {
        ClpdW::new(self, 0)
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clpd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clpd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClpdSpec;
impl crate::RegisterSpec for ClpdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clpd::R`](R) reader structure"]
impl crate::Readable for ClpdSpec {}
#[doc = "`write(|w| ..)` method takes [`clpd::W`](W) writer structure"]
impl crate::Writable for ClpdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLPD to value 0x0a"]
impl crate::Resettable for ClpdSpec {
    const RESET_VALUE: u32 = 0x0a;
}
