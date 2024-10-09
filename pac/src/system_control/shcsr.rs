#[doc = "Register `SHCSR` reader"]
pub type R = crate::R<ShcsrSpec>;
#[doc = "Register `SHCSR` writer"]
pub type W = crate::W<ShcsrSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Svcallpended {
    #[doc = "0: exception is not pending"]
    B0 = 0,
    #[doc = "1: exception is pending"]
    B1 = 1,
}
impl From<Svcallpended> for bool {
    #[inline(always)]
    fn from(variant: Svcallpended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVCALLPENDED` reader - no description available"]
pub type SvcallpendedR = crate::BitReader<Svcallpended>;
impl SvcallpendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Svcallpended {
        match self.bits {
            false => Svcallpended::B0,
            true => Svcallpended::B1,
        }
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Svcallpended::B0
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Svcallpended::B1
    }
}
#[doc = "Field `SVCALLPENDED` writer - no description available"]
pub type SvcallpendedW<'a, REG> = crate::BitWriter<'a, REG, Svcallpended>;
impl<'a, REG> SvcallpendedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Svcallpended::B0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Svcallpended::B1)
    }
}
impl R {
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SvcallpendedR {
        SvcallpendedR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn svcallpended(&mut self) -> SvcallpendedW<ShcsrSpec> {
        SvcallpendedW::new(self, 15)
    }
}
#[doc = "System Handler Control and State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShcsrSpec;
impl crate::RegisterSpec for ShcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shcsr::R`](R) reader structure"]
impl crate::Readable for ShcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`shcsr::W`](W) writer structure"]
impl crate::Writable for ShcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHCSR to value 0"]
impl crate::Resettable for ShcsrSpec {
    const RESET_VALUE: u32 = 0;
}
