#[doc = "Register `FCNFG` reader"]
pub type R = crate::R<FcnfgSpec>;
#[doc = "Register `FCNFG` writer"]
pub type W = crate::W<FcnfgSpec>;
#[doc = "Erase Suspend\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erssusp {
    #[doc = "0: No suspend requested"]
    B0 = 0,
    #[doc = "1: Suspend the current Erase Flash Sector command execution."]
    B1 = 1,
}
impl From<Erssusp> for bool {
    #[inline(always)]
    fn from(variant: Erssusp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERSSUSP` reader - Erase Suspend"]
pub type ErssuspR = crate::BitReader<Erssusp>;
impl ErssuspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erssusp {
        match self.bits {
            false => Erssusp::B0,
            true => Erssusp::B1,
        }
    }
    #[doc = "No suspend requested"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Erssusp::B0
    }
    #[doc = "Suspend the current Erase Flash Sector command execution."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Erssusp::B1
    }
}
#[doc = "Field `ERSSUSP` writer - Erase Suspend"]
pub type ErssuspW<'a, REG> = crate::BitWriter<'a, REG, Erssusp>;
impl<'a, REG> ErssuspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No suspend requested"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Erssusp::B0)
    }
    #[doc = "Suspend the current Erase Flash Sector command execution."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Erssusp::B1)
    }
}
#[doc = "Erase All Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ersareq {
    #[doc = "0: No request or request complete"]
    B0 = 0,
    #[doc = "1: Request to: run the Erase All Blocks command, verify the erased state, program the security byte in the Flash Configuration Field to the unsecure state, and release MCU security by setting the FSEC\\[SEC\\]
field to the unsecure state."]
    B1 = 1,
}
impl From<Ersareq> for bool {
    #[inline(always)]
    fn from(variant: Ersareq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERSAREQ` reader - Erase All Request"]
pub type ErsareqR = crate::BitReader<Ersareq>;
impl ErsareqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ersareq {
        match self.bits {
            false => Ersareq::B0,
            true => Ersareq::B1,
        }
    }
    #[doc = "No request or request complete"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ersareq::B0
    }
    #[doc = "Request to: run the Erase All Blocks command, verify the erased state, program the security byte in the Flash Configuration Field to the unsecure state, and release MCU security by setting the FSEC\\[SEC\\]
field to the unsecure state."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ersareq::B1
    }
}
#[doc = "Read Collision Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdcollie {
    #[doc = "0: Read collision error interrupt disabled"]
    B0 = 0,
    #[doc = "1: Read collision error interrupt enabled. An interrupt request is generated whenever a flash memory read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    B1 = 1,
}
impl From<Rdcollie> for bool {
    #[inline(always)]
    fn from(variant: Rdcollie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDCOLLIE` reader - Read Collision Error Interrupt Enable"]
pub type RdcollieR = crate::BitReader<Rdcollie>;
impl RdcollieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdcollie {
        match self.bits {
            false => Rdcollie::B0,
            true => Rdcollie::B1,
        }
    }
    #[doc = "Read collision error interrupt disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rdcollie::B0
    }
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever a flash memory read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rdcollie::B1
    }
}
#[doc = "Field `RDCOLLIE` writer - Read Collision Error Interrupt Enable"]
pub type RdcollieW<'a, REG> = crate::BitWriter<'a, REG, Rdcollie>;
impl<'a, REG> RdcollieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read collision error interrupt disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdcollie::B0)
    }
    #[doc = "Read collision error interrupt enabled. An interrupt request is generated whenever a flash memory read collision error is detected (see the description of FSTAT\\[RDCOLERR\\])."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdcollie::B1)
    }
}
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccie {
    #[doc = "0: Command complete interrupt disabled"]
    B0 = 0,
    #[doc = "1: Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\]
flag is set."]
    B1 = 1,
}
impl From<Ccie> for bool {
    #[inline(always)]
    fn from(variant: Ccie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIE` reader - Command Complete Interrupt Enable"]
pub type CcieR = crate::BitReader<Ccie>;
impl CcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccie {
        match self.bits {
            false => Ccie::B0,
            true => Ccie::B1,
        }
    }
    #[doc = "Command complete interrupt disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ccie::B0
    }
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\]
flag is set."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ccie::B1
    }
}
#[doc = "Field `CCIE` writer - Command Complete Interrupt Enable"]
pub type CcieW<'a, REG> = crate::BitWriter<'a, REG, Ccie>;
impl<'a, REG> CcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command complete interrupt disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccie::B0)
    }
    #[doc = "Command complete interrupt enabled. An interrupt request is generated whenever the FSTAT\\[CCIF\\]
flag is set."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccie::B1)
    }
}
impl R {
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline(always)]
    pub fn erssusp(&self) -> ErssuspR {
        ErssuspR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Erase All Request"]
    #[inline(always)]
    pub fn ersareq(&self) -> ErsareqR {
        ErsareqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline(always)]
    pub fn rdcollie(&self) -> RdcollieR {
        RdcollieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccie(&self) -> CcieR {
        CcieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Erase Suspend"]
    #[inline(always)]
    #[must_use]
    pub fn erssusp(&mut self) -> ErssuspW<FcnfgSpec> {
        ErssuspW::new(self, 4)
    }
    #[doc = "Bit 6 - Read Collision Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdcollie(&mut self) -> RdcollieW<FcnfgSpec> {
        RdcollieW::new(self, 6)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self) -> CcieW<FcnfgSpec> {
        CcieW::new(self, 7)
    }
}
#[doc = "Flash Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcnfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcnfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcnfgSpec;
impl crate::RegisterSpec for FcnfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fcnfg::R`](R) reader structure"]
impl crate::Readable for FcnfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fcnfg::W`](W) writer structure"]
impl crate::Writable for FcnfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FCNFG to value 0"]
impl crate::Resettable for FcnfgSpec {
    const RESET_VALUE: u8 = 0;
}
