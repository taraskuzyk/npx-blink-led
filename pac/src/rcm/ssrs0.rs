#[doc = "Register `SSRS0` reader"]
pub type R = crate::R<Ssrs0Spec>;
#[doc = "Register `SSRS0` writer"]
pub type W = crate::W<Ssrs0Spec>;
#[doc = "Sticky Low Leakage Wakeup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swakeup {
    #[doc = "0: Reset not caused by LLWU module wakeup source"]
    B0 = 0,
    #[doc = "1: Reset caused by LLWU module wakeup source"]
    B1 = 1,
}
impl From<Swakeup> for bool {
    #[inline(always)]
    fn from(variant: Swakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAKEUP` reader - Sticky Low Leakage Wakeup Reset"]
pub type SwakeupR = crate::BitReader<Swakeup>;
impl SwakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swakeup {
        match self.bits {
            false => Swakeup::B0,
            true => Swakeup::B1,
        }
    }
    #[doc = "Reset not caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Swakeup::B0
    }
    #[doc = "Reset caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Swakeup::B1
    }
}
#[doc = "Field `SWAKEUP` writer - Sticky Low Leakage Wakeup Reset"]
pub type SwakeupW<'a, REG> = crate::BitWriter<'a, REG, Swakeup>;
impl<'a, REG> SwakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Swakeup::B0)
    }
    #[doc = "Reset caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Swakeup::B1)
    }
}
#[doc = "Sticky Low-Voltage Detect Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slvd {
    #[doc = "0: Reset not caused by LVD trip or POR"]
    B0 = 0,
    #[doc = "1: Reset caused by LVD trip or POR"]
    B1 = 1,
}
impl From<Slvd> for bool {
    #[inline(always)]
    fn from(variant: Slvd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLVD` reader - Sticky Low-Voltage Detect Reset"]
pub type SlvdR = crate::BitReader<Slvd>;
impl SlvdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slvd {
        match self.bits {
            false => Slvd::B0,
            true => Slvd::B1,
        }
    }
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Slvd::B0
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Slvd::B1
    }
}
#[doc = "Field `SLVD` writer - Sticky Low-Voltage Detect Reset"]
pub type SlvdW<'a, REG> = crate::BitWriter<'a, REG, Slvd>;
impl<'a, REG> SlvdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Slvd::B0)
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Slvd::B1)
    }
}
#[doc = "Sticky Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swdog {
    #[doc = "0: Reset not caused by watchdog timeout"]
    B0 = 0,
    #[doc = "1: Reset caused by watchdog timeout"]
    B1 = 1,
}
impl From<Swdog> for bool {
    #[inline(always)]
    fn from(variant: Swdog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWDOG` reader - Sticky Watchdog"]
pub type SwdogR = crate::BitReader<Swdog>;
impl SwdogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swdog {
        match self.bits {
            false => Swdog::B0,
            true => Swdog::B1,
        }
    }
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Swdog::B0
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Swdog::B1
    }
}
#[doc = "Field `SWDOG` writer - Sticky Watchdog"]
pub type SwdogW<'a, REG> = crate::BitWriter<'a, REG, Swdog>;
impl<'a, REG> SwdogW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Swdog::B0)
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Swdog::B1)
    }
}
#[doc = "Sticky External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spin {
    #[doc = "0: Reset not caused by external reset pin"]
    B0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    B1 = 1,
}
impl From<Spin> for bool {
    #[inline(always)]
    fn from(variant: Spin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIN` reader - Sticky External Reset Pin"]
pub type SpinR = crate::BitReader<Spin>;
impl SpinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spin {
        match self.bits {
            false => Spin::B0,
            true => Spin::B1,
        }
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spin::B0
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spin::B1
    }
}
#[doc = "Field `SPIN` writer - Sticky External Reset Pin"]
pub type SpinW<'a, REG> = crate::BitWriter<'a, REG, Spin>;
impl<'a, REG> SpinW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spin::B0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spin::B1)
    }
}
#[doc = "Sticky Power-On Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spor {
    #[doc = "0: Reset not caused by POR"]
    B0 = 0,
    #[doc = "1: Reset caused by POR"]
    B1 = 1,
}
impl From<Spor> for bool {
    #[inline(always)]
    fn from(variant: Spor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOR` reader - Sticky Power-On Reset"]
pub type SporR = crate::BitReader<Spor>;
impl SporR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spor {
        match self.bits {
            false => Spor::B0,
            true => Spor::B1,
        }
    }
    #[doc = "Reset not caused by POR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spor::B0
    }
    #[doc = "Reset caused by POR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spor::B1
    }
}
#[doc = "Field `SPOR` writer - Sticky Power-On Reset"]
pub type SporW<'a, REG> = crate::BitWriter<'a, REG, Spor>;
impl<'a, REG> SporW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not caused by POR"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spor::B0)
    }
    #[doc = "Reset caused by POR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spor::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Sticky Low Leakage Wakeup Reset"]
    #[inline(always)]
    pub fn swakeup(&self) -> SwakeupR {
        SwakeupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn slvd(&self) -> SlvdR {
        SlvdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    pub fn swdog(&self) -> SwdogR {
        SwdogR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    pub fn spin(&self) -> SpinR {
        SpinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    pub fn spor(&self) -> SporR {
        SporR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sticky Low Leakage Wakeup Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swakeup(&mut self) -> SwakeupW<Ssrs0Spec> {
        SwakeupW::new(self, 0)
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    #[must_use]
    pub fn slvd(&mut self) -> SlvdW<Ssrs0Spec> {
        SlvdW::new(self, 1)
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    #[must_use]
    pub fn swdog(&mut self) -> SwdogW<Ssrs0Spec> {
        SwdogW::new(self, 5)
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    #[must_use]
    pub fn spin(&mut self) -> SpinW<Ssrs0Spec> {
        SpinW::new(self, 6)
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    #[must_use]
    pub fn spor(&mut self) -> SporW<Ssrs0Spec> {
        SporW::new(self, 7)
    }
}
#[doc = "Sticky System Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ssrs0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrs0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ssrs0Spec;
impl crate::RegisterSpec for Ssrs0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ssrs0::R`](R) reader structure"]
impl crate::Readable for Ssrs0Spec {}
#[doc = "`write(|w| ..)` method takes [`ssrs0::W`](W) writer structure"]
impl crate::Writable for Ssrs0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SSRS0 to value 0x82"]
impl crate::Resettable for Ssrs0Spec {
    const RESET_VALUE: u8 = 0x82;
}
