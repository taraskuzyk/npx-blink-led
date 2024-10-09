#[doc = "Register `SRS1` reader"]
pub type R = crate::R<Srs1Spec>;
#[doc = "Core Lockup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lockup {
    #[doc = "0: Reset not caused by core LOCKUP event"]
    B0 = 0,
    #[doc = "1: Reset caused by core LOCKUP event"]
    B1 = 1,
}
impl From<Lockup> for bool {
    #[inline(always)]
    fn from(variant: Lockup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKUP` reader - Core Lockup"]
pub type LockupR = crate::BitReader<Lockup>;
impl LockupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lockup {
        match self.bits {
            false => Lockup::B0,
            true => Lockup::B1,
        }
    }
    #[doc = "Reset not caused by core LOCKUP event"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lockup::B0
    }
    #[doc = "Reset caused by core LOCKUP event"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lockup::B1
    }
}
#[doc = "Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sw {
    #[doc = "0: Reset not caused by software setting of SYSRESETREQ bit"]
    B0 = 0,
    #[doc = "1: Reset caused by software setting of SYSRESETREQ bit"]
    B1 = 1,
}
impl From<Sw> for bool {
    #[inline(always)]
    fn from(variant: Sw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SW` reader - Software"]
pub type SwR = crate::BitReader<Sw>;
impl SwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sw {
        match self.bits {
            false => Sw::B0,
            true => Sw::B1,
        }
    }
    #[doc = "Reset not caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sw::B0
    }
    #[doc = "Reset caused by software setting of SYSRESETREQ bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sw::B1
    }
}
#[doc = "MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MdmAp {
    #[doc = "0: Reset not caused by host debugger system setting of the System Reset Request bit"]
    B0 = 0,
    #[doc = "1: Reset caused by host debugger system setting of the System Reset Request bit"]
    B1 = 1,
}
impl From<MdmAp> for bool {
    #[inline(always)]
    fn from(variant: MdmAp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDM_AP` reader - MDM-AP System Reset Request"]
pub type MdmApR = crate::BitReader<MdmAp>;
impl MdmApR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MdmAp {
        match self.bits {
            false => MdmAp::B0,
            true => MdmAp::B1,
        }
    }
    #[doc = "Reset not caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == MdmAp::B0
    }
    #[doc = "Reset caused by host debugger system setting of the System Reset Request bit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == MdmAp::B1
    }
}
#[doc = "Stop Mode Acknowledge Error Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sackerr {
    #[doc = "0: Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    B0 = 0,
    #[doc = "1: Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    B1 = 1,
}
impl From<Sackerr> for bool {
    #[inline(always)]
    fn from(variant: Sackerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SACKERR` reader - Stop Mode Acknowledge Error Reset"]
pub type SackerrR = crate::BitReader<Sackerr>;
impl SackerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sackerr {
        match self.bits {
            false => Sackerr::B0,
            true => Sackerr::B1,
        }
    }
    #[doc = "Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sackerr::B0
    }
    #[doc = "Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sackerr::B1
    }
}
impl R {
    #[doc = "Bit 1 - Core Lockup"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn mdm_ap(&self) -> MdmApR {
        MdmApR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop Mode Acknowledge Error Reset"]
    #[inline(always)]
    pub fn sackerr(&self) -> SackerrR {
        SackerrR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "System Reset Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`srs1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Srs1Spec;
impl crate::RegisterSpec for Srs1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`srs1::R`](R) reader structure"]
impl crate::Readable for Srs1Spec {}
#[doc = "`reset()` method sets SRS1 to value 0"]
impl crate::Resettable for Srs1Spec {
    const RESET_VALUE: u8 = 0;
}
