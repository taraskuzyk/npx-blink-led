#[doc = "Register `TFLG%s` reader"]
pub type R = crate::R<TflgSpec>;
#[doc = "Register `TFLG%s` writer"]
pub type W = crate::W<TflgSpec>;
#[doc = "Timer Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tif {
    #[doc = "0: Timeout has not yet occurred."]
    B0 = 0,
    #[doc = "1: Timeout has occurred."]
    B1 = 1,
}
impl From<Tif> for bool {
    #[inline(always)]
    fn from(variant: Tif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` reader - Timer Interrupt Flag"]
pub type TifR = crate::BitReader<Tif>;
impl TifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tif {
        match self.bits {
            false => Tif::B0,
            true => Tif::B1,
        }
    }
    #[doc = "Timeout has not yet occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tif::B0
    }
    #[doc = "Timeout has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tif::B1
    }
}
#[doc = "Field `TIF` writer - Timer Interrupt Flag"]
pub type TifW<'a, REG> = crate::BitWriter<'a, REG, Tif>;
impl<'a, REG> TifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout has not yet occurred."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tif::B0)
    }
    #[doc = "Timeout has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tif::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Interrupt Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tif(&mut self) -> TifW<TflgSpec> {
        TifW::new(self, 0)
    }
}
#[doc = "Timer Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tflg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tflg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TflgSpec;
impl crate::RegisterSpec for TflgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tflg::R`](R) reader structure"]
impl crate::Readable for TflgSpec {}
#[doc = "`write(|w| ..)` method takes [`tflg::W`](W) writer structure"]
impl crate::Writable for TflgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFLG%s to value 0"]
impl crate::Resettable for TflgSpec {
    const RESET_VALUE: u32 = 0;
}
