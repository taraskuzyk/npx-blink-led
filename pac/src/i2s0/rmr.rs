#[doc = "Register `RMR` reader"]
pub type R = crate::R<RmrSpec>;
#[doc = "Register `RMR` writer"]
pub type W = crate::W<RmrSpec>;
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwm0 {
    #[doc = "0: Word N is enabled."]
    B00 = 0,
    #[doc = "1: Word N is masked."]
    B01 = 1,
}
impl From<Rwm0> for bool {
    #[inline(always)]
    fn from(variant: Rwm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWM0` reader - Receive Word Mask"]
pub type Rwm0R = crate::BitReader<Rwm0>;
impl Rwm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwm0 {
        match self.bits {
            false => Rwm0::B00,
            true => Rwm0::B01,
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rwm0::B00
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rwm0::B01
    }
}
#[doc = "Field `RWM0` writer - Receive Word Mask"]
pub type Rwm0W<'a, REG> = crate::BitWriter<'a, REG, Rwm0>;
impl<'a, REG> Rwm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rwm0::B00)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rwm0::B01)
    }
}
#[doc = "Receive Word Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwm1 {
    #[doc = "0: Word N is enabled."]
    B00 = 0,
    #[doc = "1: Word N is masked."]
    B01 = 1,
}
impl From<Rwm1> for bool {
    #[inline(always)]
    fn from(variant: Rwm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWM1` reader - Receive Word Mask"]
pub type Rwm1R = crate::BitReader<Rwm1>;
impl Rwm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwm1 {
        match self.bits {
            false => Rwm1::B00,
            true => Rwm1::B01,
        }
    }
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rwm1::B00
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rwm1::B01
    }
}
#[doc = "Field `RWM1` writer - Receive Word Mask"]
pub type Rwm1W<'a, REG> = crate::BitWriter<'a, REG, Rwm1>;
impl<'a, REG> Rwm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Word N is enabled."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rwm1::B00)
    }
    #[doc = "Word N is masked."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rwm1::B01)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm0(&self) -> Rwm0R {
        Rwm0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Word Mask"]
    #[inline(always)]
    pub fn rwm1(&self) -> Rwm1R {
        Rwm1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm0(&mut self) -> Rwm0W<RmrSpec> {
        Rwm0W::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Word Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rwm1(&mut self) -> Rwm1W<RmrSpec> {
        Rwm1W::new(self, 1)
    }
}
#[doc = "SAI Receive Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RmrSpec;
impl crate::RegisterSpec for RmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmr::R`](R) reader structure"]
impl crate::Readable for RmrSpec {}
#[doc = "`write(|w| ..)` method takes [`rmr::W`](W) writer structure"]
impl crate::Writable for RmrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMR to value 0"]
impl crate::Resettable for RmrSpec {
    const RESET_VALUE: u32 = 0;
}
