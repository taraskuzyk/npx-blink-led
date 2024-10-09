#[doc = "Register `SRS0` reader"]
pub type R = crate::R<Srs0Spec>;
#[doc = "Low Leakage Wakeup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakeup {
    #[doc = "0: Reset not caused by LLWU module wakeup source"]
    B0 = 0,
    #[doc = "1: Reset caused by LLWU module wakeup source"]
    B1 = 1,
}
impl From<Wakeup> for bool {
    #[inline(always)]
    fn from(variant: Wakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP` reader - Low Leakage Wakeup Reset"]
pub type WakeupR = crate::BitReader<Wakeup>;
impl WakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakeup {
        match self.bits {
            false => Wakeup::B0,
            true => Wakeup::B1,
        }
    }
    #[doc = "Reset not caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wakeup::B0
    }
    #[doc = "Reset caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wakeup::B1
    }
}
#[doc = "Low-Voltage Detect Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvd {
    #[doc = "0: Reset not caused by LVD trip or POR"]
    B0 = 0,
    #[doc = "1: Reset caused by LVD trip or POR"]
    B1 = 1,
}
impl From<Lvd> for bool {
    #[inline(always)]
    fn from(variant: Lvd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVD` reader - Low-Voltage Detect Reset"]
pub type LvdR = crate::BitReader<Lvd>;
impl LvdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvd {
        match self.bits {
            false => Lvd::B0,
            true => Lvd::B1,
        }
    }
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lvd::B0
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lvd::B1
    }
}
#[doc = "Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wdog {
    #[doc = "0: Reset not caused by watchdog timeout"]
    B0 = 0,
    #[doc = "1: Reset caused by watchdog timeout"]
    B1 = 1,
}
impl From<Wdog> for bool {
    #[inline(always)]
    fn from(variant: Wdog) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDOG` reader - Watchdog"]
pub type WdogR = crate::BitReader<Wdog>;
impl WdogR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wdog {
        match self.bits {
            false => Wdog::B0,
            true => Wdog::B1,
        }
    }
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wdog::B0
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wdog::B1
    }
}
#[doc = "External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pin {
    #[doc = "0: Reset not caused by external reset pin"]
    B0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    B1 = 1,
}
impl From<Pin> for bool {
    #[inline(always)]
    fn from(variant: Pin) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN` reader - External Reset Pin"]
pub type PinR = crate::BitReader<Pin>;
impl PinR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pin {
        match self.bits {
            false => Pin::B0,
            true => Pin::B1,
        }
    }
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pin::B0
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pin::B1
    }
}
#[doc = "Power-On Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Por {
    #[doc = "0: Reset not caused by POR"]
    B0 = 0,
    #[doc = "1: Reset caused by POR"]
    B1 = 1,
}
impl From<Por> for bool {
    #[inline(always)]
    fn from(variant: Por) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POR` reader - Power-On Reset"]
pub type PorR = crate::BitReader<Por>;
impl PorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Por {
        match self.bits {
            false => Por::B0,
            true => Por::B1,
        }
    }
    #[doc = "Reset not caused by POR"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Por::B0
    }
    #[doc = "Reset caused by POR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Por::B1
    }
}
impl R {
    #[doc = "Bit 0 - Low Leakage Wakeup Reset"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn lvd(&self) -> LvdR {
        LvdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Watchdog"]
    #[inline(always)]
    pub fn wdog(&self) -> WdogR {
        WdogR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Reset Pin"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power-On Reset"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "System Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`srs0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srs0Spec;
impl crate::RegisterSpec for Srs0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`srs0::R`](R) reader structure"]
impl crate::Readable for Srs0Spec {}
#[doc = "`reset()` method sets SRS0 to value 0x82"]
impl crate::Resettable for Srs0Spec {
    const RESET_VALUE: u8 = 0x82;
}
