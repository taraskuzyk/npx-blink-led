#[doc = "Register `FSTAT` reader"]
pub type R = crate::R<FstatSpec>;
#[doc = "Register `FSTAT` writer"]
pub type W = crate::W<FstatSpec>;
#[doc = "Field `MGSTAT0` reader - Memory Controller Command Completion Status Flag"]
pub type Mgstat0R = crate::BitReader;
#[doc = "Flash Protection Violation Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpviol {
    #[doc = "0: No protection violation detected"]
    B0 = 0,
    #[doc = "1: Protection violation detected"]
    B1 = 1,
}
impl From<Fpviol> for bool {
    #[inline(always)]
    fn from(variant: Fpviol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPVIOL` reader - Flash Protection Violation Flag"]
pub type FpviolR = crate::BitReader<Fpviol>;
impl FpviolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpviol {
        match self.bits {
            false => Fpviol::B0,
            true => Fpviol::B1,
        }
    }
    #[doc = "No protection violation detected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fpviol::B0
    }
    #[doc = "Protection violation detected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fpviol::B1
    }
}
#[doc = "Field `FPVIOL` writer - Flash Protection Violation Flag"]
pub type FpviolW<'a, REG> = crate::BitWriter<'a, REG, Fpviol>;
impl<'a, REG> FpviolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No protection violation detected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fpviol::B0)
    }
    #[doc = "Protection violation detected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fpviol::B1)
    }
}
#[doc = "Flash Access Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accerr {
    #[doc = "0: No access error detected"]
    B0 = 0,
    #[doc = "1: Access error detected"]
    B1 = 1,
}
impl From<Accerr> for bool {
    #[inline(always)]
    fn from(variant: Accerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACCERR` reader - Flash Access Error Flag"]
pub type AccerrR = crate::BitReader<Accerr>;
impl AccerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Accerr {
        match self.bits {
            false => Accerr::B0,
            true => Accerr::B1,
        }
    }
    #[doc = "No access error detected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Accerr::B0
    }
    #[doc = "Access error detected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Accerr::B1
    }
}
#[doc = "Field `ACCERR` writer - Flash Access Error Flag"]
pub type AccerrW<'a, REG> = crate::BitWriter<'a, REG, Accerr>;
impl<'a, REG> AccerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No access error detected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Accerr::B0)
    }
    #[doc = "Access error detected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Accerr::B1)
    }
}
#[doc = "Flash Read Collision Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdcolerr {
    #[doc = "0: No collision error detected"]
    B0 = 0,
    #[doc = "1: Collision error detected"]
    B1 = 1,
}
impl From<Rdcolerr> for bool {
    #[inline(always)]
    fn from(variant: Rdcolerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDCOLERR` reader - Flash Read Collision Error Flag"]
pub type RdcolerrR = crate::BitReader<Rdcolerr>;
impl RdcolerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdcolerr {
        match self.bits {
            false => Rdcolerr::B0,
            true => Rdcolerr::B1,
        }
    }
    #[doc = "No collision error detected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rdcolerr::B0
    }
    #[doc = "Collision error detected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rdcolerr::B1
    }
}
#[doc = "Field `RDCOLERR` writer - Flash Read Collision Error Flag"]
pub type RdcolerrW<'a, REG> = crate::BitWriter<'a, REG, Rdcolerr>;
impl<'a, REG> RdcolerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No collision error detected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdcolerr::B0)
    }
    #[doc = "Collision error detected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdcolerr::B1)
    }
}
#[doc = "Command Complete Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccif {
    #[doc = "0: Flash command in progress"]
    B0 = 0,
    #[doc = "1: Flash command has completed"]
    B1 = 1,
}
impl From<Ccif> for bool {
    #[inline(always)]
    fn from(variant: Ccif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIF` reader - Command Complete Interrupt Flag"]
pub type CcifR = crate::BitReader<Ccif>;
impl CcifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccif {
        match self.bits {
            false => Ccif::B0,
            true => Ccif::B1,
        }
    }
    #[doc = "Flash command in progress"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ccif::B0
    }
    #[doc = "Flash command has completed"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ccif::B1
    }
}
#[doc = "Field `CCIF` writer - Command Complete Interrupt Flag"]
pub type CcifW<'a, REG> = crate::BitWriter<'a, REG, Ccif>;
impl<'a, REG> CcifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash command in progress"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ccif::B0)
    }
    #[doc = "Flash command has completed"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ccif::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Controller Command Completion Status Flag"]
    #[inline(always)]
    pub fn mgstat0(&self) -> Mgstat0R {
        Mgstat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Protection Violation Flag"]
    #[inline(always)]
    pub fn fpviol(&self) -> FpviolR {
        FpviolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Access Error Flag"]
    #[inline(always)]
    pub fn accerr(&self) -> AccerrR {
        AccerrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Flash Read Collision Error Flag"]
    #[inline(always)]
    pub fn rdcolerr(&self) -> RdcolerrR {
        RdcolerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ccif(&self) -> CcifR {
        CcifR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Flash Protection Violation Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpviol(&mut self) -> FpviolW<FstatSpec> {
        FpviolW::new(self, 4)
    }
    #[doc = "Bit 5 - Flash Access Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn accerr(&mut self) -> AccerrW<FstatSpec> {
        AccerrW::new(self, 5)
    }
    #[doc = "Bit 6 - Flash Read Collision Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdcolerr(&mut self) -> RdcolerrW<FstatSpec> {
        RdcolerrW::new(self, 6)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccif(&mut self) -> CcifW<FstatSpec> {
        CcifW::new(self, 7)
    }
}
#[doc = "Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FstatSpec;
impl crate::RegisterSpec for FstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fstat::R`](R) reader structure"]
impl crate::Readable for FstatSpec {}
#[doc = "`write(|w| ..)` method takes [`fstat::W`](W) writer structure"]
impl crate::Writable for FstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FSTAT to value 0"]
impl crate::Resettable for FstatSpec {
    const RESET_VALUE: u8 = 0;
}
