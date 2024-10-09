#[doc = "Register `TCR3` reader"]
pub type R = crate::R<Tcr3Spec>;
#[doc = "Register `TCR3` writer"]
pub type W = crate::W<Tcr3Spec>;
#[doc = "Field `WDFL` reader - Word Flag Configuration"]
pub type WdflR = crate::BitReader;
#[doc = "Field `WDFL` writer - Word Flag Configuration"]
pub type WdflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transmit Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tce {
    #[doc = "0: Transmit data channel N is disabled."]
    B0 = 0,
    #[doc = "1: Transmit data channel N is enabled."]
    B1 = 1,
}
impl From<Tce> for bool {
    #[inline(always)]
    fn from(variant: Tce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCE` reader - Transmit Channel Enable"]
pub type TceR = crate::BitReader<Tce>;
impl TceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tce {
        match self.bits {
            false => Tce::B0,
            true => Tce::B1,
        }
    }
    #[doc = "Transmit data channel N is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tce::B0
    }
    #[doc = "Transmit data channel N is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tce::B1
    }
}
#[doc = "Field `TCE` writer - Transmit Channel Enable"]
pub type TceW<'a, REG> = crate::BitWriter<'a, REG, Tce>;
impl<'a, REG> TceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit data channel N is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tce::B0)
    }
    #[doc = "Transmit data channel N is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tce::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&self) -> WdflR {
        WdflR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit Channel Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TceR {
        TceR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Word Flag Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdfl(&mut self) -> WdflW<Tcr3Spec> {
        WdflW::new(self, 0)
    }
    #[doc = "Bit 16 - Transmit Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TceW<Tcr3Spec> {
        TceW::new(self, 16)
    }
}
#[doc = "SAI Transmit Configuration 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcr3Spec;
impl crate::RegisterSpec for Tcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr3::R`](R) reader structure"]
impl crate::Readable for Tcr3Spec {}
#[doc = "`write(|w| ..)` method takes [`tcr3::W`](W) writer structure"]
impl crate::Writable for Tcr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCR3 to value 0"]
impl crate::Resettable for Tcr3Spec {
    const RESET_VALUE: u32 = 0;
}
