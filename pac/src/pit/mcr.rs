#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Freeze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Frz {
    #[doc = "0: Timers continue to run in Debug mode."]
    B0 = 0,
    #[doc = "1: Timers are stopped in Debug mode."]
    B1 = 1,
}
impl From<Frz> for bool {
    #[inline(always)]
    fn from(variant: Frz) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRZ` reader - Freeze"]
pub type FrzR = crate::BitReader<Frz>;
impl FrzR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frz {
        match self.bits {
            false => Frz::B0,
            true => Frz::B1,
        }
    }
    #[doc = "Timers continue to run in Debug mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Frz::B0
    }
    #[doc = "Timers are stopped in Debug mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Frz::B1
    }
}
#[doc = "Field `FRZ` writer - Freeze"]
pub type FrzW<'a, REG> = crate::BitWriter<'a, REG, Frz>;
impl<'a, REG> FrzW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timers continue to run in Debug mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Frz::B0)
    }
    #[doc = "Timers are stopped in Debug mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Frz::B1)
    }
}
#[doc = "Module Disable - (PIT section)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mdis {
    #[doc = "0: Clock for standard PIT timers is enabled."]
    B0 = 0,
    #[doc = "1: Clock for standard PIT timers is disabled."]
    B1 = 1,
}
impl From<Mdis> for bool {
    #[inline(always)]
    fn from(variant: Mdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDIS` reader - Module Disable - (PIT section)"]
pub type MdisR = crate::BitReader<Mdis>;
impl MdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mdis {
        match self.bits {
            false => Mdis::B0,
            true => Mdis::B1,
        }
    }
    #[doc = "Clock for standard PIT timers is enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Mdis::B0
    }
    #[doc = "Clock for standard PIT timers is disabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Mdis::B1
    }
}
#[doc = "Field `MDIS` writer - Module Disable - (PIT section)"]
pub type MdisW<'a, REG> = crate::BitWriter<'a, REG, Mdis>;
impl<'a, REG> MdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock for standard PIT timers is enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Mdis::B0)
    }
    #[doc = "Clock for standard PIT timers is disabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Mdis::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Freeze"]
    #[inline(always)]
    pub fn frz(&self) -> FrzR {
        FrzR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable - (PIT section)"]
    #[inline(always)]
    pub fn mdis(&self) -> MdisR {
        MdisR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Freeze"]
    #[inline(always)]
    #[must_use]
    pub fn frz(&mut self) -> FrzW<McrSpec> {
        FrzW::new(self, 0)
    }
    #[doc = "Bit 1 - Module Disable - (PIT section)"]
    #[inline(always)]
    #[must_use]
    pub fn mdis(&mut self) -> MdisW<McrSpec> {
        MdisW::new(self, 1)
    }
}
#[doc = "PIT Module Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0x06"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0x06;
}
