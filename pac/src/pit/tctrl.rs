#[doc = "Register `TCTRL%s` reader"]
pub type R = crate::R<TctrlSpec>;
#[doc = "Register `TCTRL%s` writer"]
pub type W = crate::W<TctrlSpec>;
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ten {
    #[doc = "0: Timer n is disabled."]
    B0 = 0,
    #[doc = "1: Timer n is enabled."]
    B1 = 1,
}
impl From<Ten> for bool {
    #[inline(always)]
    fn from(variant: Ten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Timer Enable"]
pub type TenR = crate::BitReader<Ten>;
impl TenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ten {
        match self.bits {
            false => Ten::B0,
            true => Ten::B1,
        }
    }
    #[doc = "Timer n is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ten::B0
    }
    #[doc = "Timer n is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ten::B1
    }
}
#[doc = "Field `TEN` writer - Timer Enable"]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG, Ten>;
impl<'a, REG> TenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer n is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::B0)
    }
    #[doc = "Timer n is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::B1)
    }
}
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    #[doc = "0: Interrupt requests from Timer n are disabled."]
    B0 = 0,
    #[doc = "1: Interrupt will be requested whenever TIF is set."]
    B1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Timer Interrupt Enable"]
pub type TieR = crate::BitReader<Tie>;
impl TieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tie {
        match self.bits {
            false => Tie::B0,
            true => Tie::B1,
        }
    }
    #[doc = "Interrupt requests from Timer n are disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tie::B0
    }
    #[doc = "Interrupt will be requested whenever TIF is set."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tie::B1
    }
}
#[doc = "Field `TIE` writer - Timer Interrupt Enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt requests from Timer n are disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B0)
    }
    #[doc = "Interrupt will be requested whenever TIF is set."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B1)
    }
}
#[doc = "Chain Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chn {
    #[doc = "0: Timer is not chained."]
    B0 = 0,
    #[doc = "1: Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    B1 = 1,
}
impl From<Chn> for bool {
    #[inline(always)]
    fn from(variant: Chn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHN` reader - Chain Mode"]
pub type ChnR = crate::BitReader<Chn>;
impl ChnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chn {
        match self.bits {
            false => Chn::B0,
            true => Chn::B1,
        }
    }
    #[doc = "Timer is not chained."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Chn::B0
    }
    #[doc = "Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Chn::B1
    }
}
#[doc = "Field `CHN` writer - Chain Mode"]
pub type ChnW<'a, REG> = crate::BitWriter<'a, REG, Chn>;
impl<'a, REG> ChnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer is not chained."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Chn::B0)
    }
    #[doc = "Timer is chained to previous timer. For example, for Channel 2, if this field is set, Timer 2 is chained to Timer 1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Chn::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Chain Mode"]
    #[inline(always)]
    pub fn chn(&self) -> ChnR {
        ChnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TenW<TctrlSpec> {
        TenW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<TctrlSpec> {
        TieW::new(self, 1)
    }
    #[doc = "Bit 2 - Chain Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chn(&mut self) -> ChnW<TctrlSpec> {
        ChnW::new(self, 2)
    }
}
#[doc = "Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TctrlSpec;
impl crate::RegisterSpec for TctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tctrl::R`](R) reader structure"]
impl crate::Readable for TctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tctrl::W`](W) writer structure"]
impl crate::Writable for TctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCTRL%s to value 0"]
impl crate::Resettable for TctrlSpec {
    const RESET_VALUE: u32 = 0;
}
