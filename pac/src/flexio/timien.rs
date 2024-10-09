#[doc = "Register `TIMIEN` reader"]
pub type R = crate::R<TimienSpec>;
#[doc = "Register `TIMIEN` writer"]
pub type W = crate::W<TimienSpec>;
#[doc = "Timer Status Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Teie {
    #[doc = "0: Timer Status Flag interrupt is disabled"]
    B0000 = 0,
    #[doc = "1: Timer Status Flag interrupt is enabled"]
    B0001 = 1,
}
impl From<Teie> for u8 {
    #[inline(always)]
    fn from(variant: Teie) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Teie {
    type Ux = u8;
}
impl crate::IsEnum for Teie {}
#[doc = "Field `TEIE` reader - Timer Status Interrupt Enable"]
pub type TeieR = crate::FieldReader<Teie>;
impl TeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Teie> {
        match self.bits {
            0 => Some(Teie::B0000),
            1 => Some(Teie::B0001),
            _ => None,
        }
    }
    #[doc = "Timer Status Flag interrupt is disabled"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Teie::B0000
    }
    #[doc = "Timer Status Flag interrupt is enabled"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Teie::B0001
    }
}
#[doc = "Field `TEIE` writer - Timer Status Interrupt Enable"]
pub type TeieW<'a, REG> = crate::FieldWriter<'a, REG, 4, Teie>;
impl<'a, REG> TeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer Status Flag interrupt is disabled"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::B0000)
    }
    #[doc = "Timer Status Flag interrupt is enabled"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Teie::B0001)
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer Status Interrupt Enable"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TeieW<TimienSpec> {
        TeieW::new(self, 0)
    }
}
#[doc = "Timer Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimienSpec;
impl crate::RegisterSpec for TimienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timien::R`](R) reader structure"]
impl crate::Readable for TimienSpec {}
#[doc = "`write(|w| ..)` method takes [`timien::W`](W) writer structure"]
impl crate::Writable for TimienSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMIEN to value 0"]
impl crate::Resettable for TimienSpec {
    const RESET_VALUE: u32 = 0;
}
