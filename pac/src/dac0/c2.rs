#[doc = "Register `C2` reader"]
pub type R = crate::R<C2Spec>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2Spec>;
#[doc = "Field `DACBFUP` reader - DAC Buffer Upper Limit"]
pub type DacbfupR = crate::BitReader;
#[doc = "Field `DACBFUP` writer - DAC Buffer Upper Limit"]
pub type DacbfupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DACBFRP` reader - DAC Buffer Read Pointer"]
pub type DacbfrpR = crate::BitReader;
#[doc = "Field `DACBFRP` writer - DAC Buffer Read Pointer"]
pub type DacbfrpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DAC Buffer Upper Limit"]
    #[inline(always)]
    pub fn dacbfup(&self) -> DacbfupR {
        DacbfupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DAC Buffer Read Pointer"]
    #[inline(always)]
    pub fn dacbfrp(&self) -> DacbfrpR {
        DacbfrpR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Upper Limit"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfup(&mut self) -> DacbfupW<C2Spec> {
        DacbfupW::new(self, 0)
    }
    #[doc = "Bit 4 - DAC Buffer Read Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfrp(&mut self) -> DacbfrpW<C2Spec> {
        DacbfrpW::new(self, 4)
    }
}
#[doc = "DAC Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2Spec;
impl crate::RegisterSpec for C2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c2::R`](R) reader structure"]
impl crate::Readable for C2Spec {}
#[doc = "`write(|w| ..)` method takes [`c2::W`](W) writer structure"]
impl crate::Writable for C2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C2 to value 0x01"]
impl crate::Resettable for C2Spec {
    const RESET_VALUE: u8 = 0x01;
}
