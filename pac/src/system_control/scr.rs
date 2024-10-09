#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleeponexit {
    #[doc = "0: do not sleep when returning to Thread mode"]
    B0 = 0,
    #[doc = "1: enter sleep, or deep sleep, on return from an ISR"]
    B1 = 1,
}
impl From<Sleeponexit> for bool {
    #[inline(always)]
    fn from(variant: Sleeponexit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPONEXIT` reader - Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
pub type SleeponexitR = crate::BitReader<Sleeponexit>;
impl SleeponexitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleeponexit {
        match self.bits {
            false => Sleeponexit::B0,
            true => Sleeponexit::B1,
        }
    }
    #[doc = "do not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sleeponexit::B0
    }
    #[doc = "enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sleeponexit::B1
    }
}
#[doc = "Field `SLEEPONEXIT` writer - Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
pub type SleeponexitW<'a, REG> = crate::BitWriter<'a, REG, Sleeponexit>;
impl<'a, REG> SleeponexitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "do not sleep when returning to Thread mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sleeponexit::B0)
    }
    #[doc = "enter sleep, or deep sleep, on return from an ISR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sleeponexit::B1)
    }
}
#[doc = "Controls whether the processor uses sleep or deep sleep as its low power mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepdeep {
    #[doc = "0: sleep"]
    B0 = 0,
    #[doc = "1: deep sleep"]
    B1 = 1,
}
impl From<Sleepdeep> for bool {
    #[inline(always)]
    fn from(variant: Sleepdeep) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPDEEP` reader - Controls whether the processor uses sleep or deep sleep as its low power mode"]
pub type SleepdeepR = crate::BitReader<Sleepdeep>;
impl SleepdeepR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepdeep {
        match self.bits {
            false => Sleepdeep::B0,
            true => Sleepdeep::B1,
        }
    }
    #[doc = "sleep"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sleepdeep::B0
    }
    #[doc = "deep sleep"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sleepdeep::B1
    }
}
#[doc = "Field `SLEEPDEEP` writer - Controls whether the processor uses sleep or deep sleep as its low power mode"]
pub type SleepdeepW<'a, REG> = crate::BitWriter<'a, REG, Sleepdeep>;
impl<'a, REG> SleepdeepW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "sleep"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepdeep::B0)
    }
    #[doc = "deep sleep"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepdeep::B1)
    }
}
#[doc = "Send Event on Pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sevonpend {
    #[doc = "0: only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    B0 = 0,
    #[doc = "1: enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    B1 = 1,
}
impl From<Sevonpend> for bool {
    #[inline(always)]
    fn from(variant: Sevonpend) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEVONPEND` reader - Send Event on Pending bit"]
pub type SevonpendR = crate::BitReader<Sevonpend>;
impl SevonpendR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sevonpend {
        match self.bits {
            false => Sevonpend::B0,
            true => Sevonpend::B1,
        }
    }
    #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sevonpend::B0
    }
    #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sevonpend::B1
    }
}
#[doc = "Field `SEVONPEND` writer - Send Event on Pending bit"]
pub type SevonpendW<'a, REG> = crate::BitWriter<'a, REG, Sevonpend>;
impl<'a, REG> SevonpendW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonpend::B0)
    }
    #[doc = "enabled events and all interrupts, including disabled interrupts, can wakeup the processor"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sevonpend::B1)
    }
}
impl R {
    #[doc = "Bit 1 - Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
    #[inline(always)]
    pub fn sleeponexit(&self) -> SleeponexitR {
        SleeponexitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline(always)]
    pub fn sleepdeep(&self) -> SleepdeepR {
        SleepdeepR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    pub fn sevonpend(&self) -> SevonpendR {
        SevonpendR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Indicates sleep-on-exit when returning from Handler mode to Thread mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleeponexit(&mut self) -> SleeponexitW<ScrSpec> {
        SleeponexitW::new(self, 1)
    }
    #[doc = "Bit 2 - Controls whether the processor uses sleep or deep sleep as its low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleepdeep(&mut self) -> SleepdeepW<ScrSpec> {
        SleepdeepW::new(self, 2)
    }
    #[doc = "Bit 4 - Send Event on Pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn sevonpend(&mut self) -> SevonpendW<ScrSpec> {
        SevonpendW::new(self, 4)
    }
}
#[doc = "System Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {
    const RESET_VALUE: u32 = 0;
}
