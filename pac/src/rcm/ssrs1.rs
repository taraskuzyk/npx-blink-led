#[doc = "Register `SSRS1` reader"]
pub type R = crate::R<Ssrs1Spec>;
#[doc = "Register `SSRS1` writer"]
pub type W = crate::W<Ssrs1Spec>;
#[doc = "Sticky Core Lockup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slockup {
    #[doc = "0: Reset not caused by core LOCKUP event"]
    B0 = 0,
    #[doc = "1: Reset caused by core LOCKUP event"]
    B1 = 1,
}
impl From<Slockup> for bool {
    #[inline(always)]
    fn from(variant: Slockup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLOCKUP` reader - Sticky Core Lockup"]
pub type SlockupR = crate::BitReader<Slockup>;
impl SlockupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slockup {
        match self.bits {
            false => Slockup::B0,
            true => Slockup::B1,
        }
    }
    #[doc = "Reset not caused by core LOCKUP event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Slockup::B0
    }
    #[doc = "Reset caused by core LOCKUP event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Slockup::B1
    }
}
#[doc = "Field `SLOCKUP` writer - Sticky Core Lockup"]
pub type SlockupW<'a, REG> = crate::BitWriter<'a, REG, Slockup>;
impl<'a, REG> SlockupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not caused by core LOCKUP event"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Slockup::B0)
    }
    #[doc = "Reset caused by core LOCKUP event"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Slockup::B1)
    }
}
#[doc = "Sticky Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssw {
    #[doc = "0: Reset not caused by software setting of SYSRESETREQ bit"]
    B0 = 0,
    #[doc = "1: Reset caused by software setting of SYSRESETREQ bit"]
    B1 = 1,
}
impl From<Ssw> for bool {
    #[inline(always)]
    fn from(variant: Ssw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSW` reader - Sticky Software"]
pub type SswR = crate::BitReader<Ssw>;
impl SswR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssw {
        match self.bits {
            false => Ssw::B0,
            true => Ssw::B1,
        }
    }
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ssw::B0
    }
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ssw::B1
    }
}
#[doc = "Field `SSW` writer - Sticky Software"]
pub type SswW<'a, REG> = crate::BitWriter<'a, REG, Ssw>;
impl<'a, REG> SswW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssw::B0)
    }
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssw::B1)
    }
}
#[doc = "Sticky MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmdmAp {
    #[doc = "0: Reset not caused by host debugger system setting of the System Reset Request bit"]
    B0 = 0,
    #[doc = "1: Reset caused by host debugger system setting of the System Reset Request bit"]
    B1 = 1,
}
impl From<SmdmAp> for bool {
    #[inline(always)]
    fn from(variant: SmdmAp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMDM_AP` reader - Sticky MDM-AP System Reset Request"]
pub type SmdmApR = crate::BitReader<SmdmAp>;
impl SmdmApR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SmdmAp {
        match self.bits {
            false => SmdmAp::B0,
            true => SmdmAp::B1,
        }
    }
    #[doc = "Reset not caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == SmdmAp::B0
    }
    #[doc = "Reset caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == SmdmAp::B1
    }
}
#[doc = "Field `SMDM_AP` writer - Sticky MDM-AP System Reset Request"]
pub type SmdmApW<'a, REG> = crate::BitWriter<'a, REG, SmdmAp>;
impl<'a, REG> SmdmApW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(SmdmAp::B0)
    }
    #[doc = "Reset caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(SmdmAp::B1)
    }
}
#[doc = "Sticky Stop Mode Acknowledge Error Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssackerr {
    #[doc = "0: Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    B0 = 0,
    #[doc = "1: Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    B1 = 1,
}
impl From<Ssackerr> for bool {
    #[inline(always)]
    fn from(variant: Ssackerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSACKERR` reader - Sticky Stop Mode Acknowledge Error Reset"]
pub type SsackerrR = crate::BitReader<Ssackerr>;
impl SsackerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssackerr {
        match self.bits {
            false => Ssackerr::B0,
            true => Ssackerr::B1,
        }
    }
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ssackerr::B0
    }
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ssackerr::B1
    }
}
#[doc = "Field `SSACKERR` writer - Sticky Stop Mode Acknowledge Error Reset"]
pub type SsackerrW<'a, REG> = crate::BitWriter<'a, REG, Ssackerr>;
impl<'a, REG> SsackerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssackerr::B0)
    }
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssackerr::B1)
    }
}
impl R {
    #[doc = "Bit 1 - Sticky Core Lockup"]
    #[inline(always)]
    pub fn slockup(&self) -> SlockupR {
        SlockupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sticky Software"]
    #[inline(always)]
    pub fn ssw(&self) -> SswR {
        SswR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sticky MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn smdm_ap(&self) -> SmdmApR {
        SmdmApR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Sticky Stop Mode Acknowledge Error Reset"]
    #[inline(always)]
    pub fn ssackerr(&self) -> SsackerrR {
        SsackerrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sticky Core Lockup"]
    #[inline(always)]
    #[must_use]
    pub fn slockup(&mut self) -> SlockupW<Ssrs1Spec> {
        SlockupW::new(self, 1)
    }
    #[doc = "Bit 2 - Sticky Software"]
    #[inline(always)]
    #[must_use]
    pub fn ssw(&mut self) -> SswW<Ssrs1Spec> {
        SswW::new(self, 2)
    }
    #[doc = "Bit 3 - Sticky MDM-AP System Reset Request"]
    #[inline(always)]
    #[must_use]
    pub fn smdm_ap(&mut self) -> SmdmApW<Ssrs1Spec> {
        SmdmApW::new(self, 3)
    }
    #[doc = "Bit 5 - Sticky Stop Mode Acknowledge Error Reset"]
    #[inline(always)]
    #[must_use]
    pub fn ssackerr(&mut self) -> SsackerrW<Ssrs1Spec> {
        SsackerrW::new(self, 5)
    }
}
#[doc = "Sticky System Reset Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ssrs1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrs1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ssrs1Spec;
impl crate::RegisterSpec for Ssrs1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ssrs1::R`](R) reader structure"]
impl crate::Readable for Ssrs1Spec {}
#[doc = "`write(|w| ..)` method takes [`ssrs1::W`](W) writer structure"]
impl crate::Writable for Ssrs1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SSRS1 to value 0"]
impl crate::Resettable for Ssrs1Spec {
    const RESET_VALUE: u8 = 0;
}
