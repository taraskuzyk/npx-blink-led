#[doc = "Register `TMR` reader"]
pub type R = crate::R<TmrSpec>;
#[doc = "Register `TMR` writer"]
pub type W = crate::W<TmrSpec>;
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twm0 {
    #[doc = "0: Word N is enabled."]
    B00 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    B01 = 1,
}
impl From<Twm0> for bool {
    #[inline(always)]
    fn from(variant: Twm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWM0` reader - Transmit Word Mask"]
pub type Twm0R = crate::BitReader<Twm0>;
impl Twm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Twm0 {
        match self.bits {
            false => Twm0::B00,
            true => Twm0::B01,
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Twm0::B00
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Twm0::B01
    }
}
#[doc = "Field `TWM0` writer - Transmit Word Mask"]
pub type Twm0W<'a, REG> = crate::BitWriter<'a, REG, Twm0>;
impl<'a, REG> Twm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Twm0::B00)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Twm0::B01)
    }
}
#[doc = "Transmit Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Twm1 {
    #[doc = "0: Word N is enabled."]
    B00 = 0,
    #[doc = "1: Word N is masked. The transmit data pins are tri-stated when masked."]
    B01 = 1,
}
impl From<Twm1> for bool {
    #[inline(always)]
    fn from(variant: Twm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TWM1` reader - Transmit Word Mask"]
pub type Twm1R = crate::BitReader<Twm1>;
impl Twm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Twm1 {
        match self.bits {
            false => Twm1::B00,
            true => Twm1::B01,
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Twm1::B00
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Twm1::B01
    }
}
#[doc = "Field `TWM1` writer - Transmit Word Mask"]
pub type Twm1W<'a, REG> = crate::BitWriter<'a, REG, Twm1>;
impl<'a, REG> Twm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Twm1::B00)
    }
    #[doc = "Word N is masked. The transmit data pins are tri-stated when masked."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Twm1::B01)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm0(&self) -> Twm0R {
        Twm0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Word Mask"]
    #[inline(always)]
    pub fn twm1(&self) -> Twm1R {
        Twm1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm0(&mut self) -> Twm0W<TmrSpec> {
        Twm0W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn twm1(&mut self) -> Twm1W<TmrSpec> {
        Twm1W::new(self, 1)
    }
}
#[doc = "SAI Transmit Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmrSpec;
impl crate::RegisterSpec for TmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmr::R`](R) reader structure"]
impl crate::Readable for TmrSpec {}
#[doc = "`write(|w| ..)` method takes [`tmr::W`](W) writer structure"]
impl crate::Writable for TmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMR to value 0"]
impl crate::Resettable for TmrSpec {
    const RESET_VALUE: u32 = 0;
}
