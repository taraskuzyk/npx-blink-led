#[doc = "Register `RCR3` reader"]
pub type R = crate::R<Rcr3Spec>;
#[doc = "Register `RCR3` writer"]
pub type W = crate::W<Rcr3Spec>;
#[doc = "Field `WDFL` reader - Word Flag Configuration"]
pub type WdflR = crate::BitReader;
#[doc = "Field `WDFL` writer - Word Flag Configuration"]
pub type WdflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rce {
    #[doc = "0: Receive data channel N is disabled."]
    B0 = 0,
    #[doc = "1: Receive data channel N is enabled."]
    B1 = 1,
}
impl From<Rce> for bool {
    #[inline(always)]
    fn from(variant: Rce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCE` reader - Receive Channel Enable"]
pub type RceR = crate::BitReader<Rce>;
impl RceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rce {
        match self.bits {
            false => Rce::B0,
            true => Rce::B1,
        }
    }
    #[doc = "Receive data channel N is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rce::B0
    }
    #[doc = "Receive data channel N is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rce::B1
    }
}
#[doc = "Field `RCE` writer - Receive Channel Enable"]
pub type RceW<'a, REG> = crate::BitWriter<'a, REG, Rce>;
impl<'a, REG> RceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive data channel N is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rce::B0)
    }
    #[doc = "Receive data channel N is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rce::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&self) -> WdflR {
        WdflR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Channel Enable"]
    #[inline(always)]
    pub fn rce(&self) -> RceR {
        RceR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Word Flag Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdfl(&mut self) -> WdflW<Rcr3Spec> {
        WdflW::new(self, 0)
    }
    #[doc = "Bit 16 - Receive Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rce(&mut self) -> RceW<Rcr3Spec> {
        RceW::new(self, 16)
    }
}
#[doc = "SAI Receive Configuration 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcr3Spec;
impl crate::RegisterSpec for Rcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr3::R`](R) reader structure"]
impl crate::Readable for Rcr3Spec {}
#[doc = "`write(|w| ..)` method takes [`rcr3::W`](W) writer structure"]
impl crate::Writable for Rcr3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR3 to value 0"]
impl crate::Resettable for Rcr3Spec {
    const RESET_VALUE: u32 = 0;
}
