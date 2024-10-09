#[doc = "Register `CALIB` reader"]
pub type R = crate::R<CalibSpec>;
#[doc = "Field `TENMS` reader - Reload value to use for 10ms timing"]
pub type TenmsR = crate::FieldReader<u32>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skew {
    #[doc = "0: 10ms calibration value is exact"]
    B0 = 0,
    #[doc = "1: 10ms calibration value is inexact, because of the clock frequency"]
    B1 = 1,
}
impl From<Skew> for bool {
    #[inline(always)]
    fn from(variant: Skew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SKEW` reader - no description available"]
pub type SkewR = crate::BitReader<Skew>;
impl SkewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Skew {
        match self.bits {
            false => Skew::B0,
            true => Skew::B1,
        }
    }
    #[doc = "10ms calibration value is exact"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Skew::B0
    }
    #[doc = "10ms calibration value is inexact, because of the clock frequency"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Skew::B1
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noref {
    #[doc = "0: The reference clock is provided"]
    B0 = 0,
    #[doc = "1: The reference clock is not provided"]
    B1 = 1,
}
impl From<Noref> for bool {
    #[inline(always)]
    fn from(variant: Noref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOREF` reader - no description available"]
pub type NorefR = crate::BitReader<Noref>;
impl NorefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Noref {
        match self.bits {
            false => Noref::B0,
            true => Noref::B1,
        }
    }
    #[doc = "The reference clock is provided"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Noref::B0
    }
    #[doc = "The reference clock is not provided"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Noref::B1
    }
}
impl R {
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TenmsR {
        TenmsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn noref(&self) -> NorefR {
        NorefR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SysTick Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calib::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalibSpec;
impl crate::RegisterSpec for CalibSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calib::R`](R) reader structure"]
impl crate::Readable for CalibSpec {}
#[doc = "`reset()` method sets CALIB to value 0"]
impl crate::Resettable for CalibSpec {
    const RESET_VALUE: u32 = 0;
}
