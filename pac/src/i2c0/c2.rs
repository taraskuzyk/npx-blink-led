#[doc = "Register `C2` reader"]
pub type R = crate::R<C2Spec>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2Spec>;
#[doc = "Field `AD` reader - Slave Address"]
pub type AdR = crate::FieldReader;
#[doc = "Field `AD` writer - Slave Address"]
pub type AdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Range Address Matching Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rmen {
    #[doc = "0: Range mode disabled. No address matching occurs for an address within the range of values of the A1 and RA registers."]
    B0 = 0,
    #[doc = "1: Range mode enabled. Address matching occurs when a slave receives an address within the range of values of the A1 and RA registers."]
    B1 = 1,
}
impl From<Rmen> for bool {
    #[inline(always)]
    fn from(variant: Rmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RMEN` reader - Range Address Matching Enable"]
pub type RmenR = crate::BitReader<Rmen>;
impl RmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rmen {
        match self.bits {
            false => Rmen::B0,
            true => Rmen::B1,
        }
    }
    #[doc = "Range mode disabled. No address matching occurs for an address within the range of values of the A1 and RA registers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rmen::B0
    }
    #[doc = "Range mode enabled. Address matching occurs when a slave receives an address within the range of values of the A1 and RA registers."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rmen::B1
    }
}
#[doc = "Field `RMEN` writer - Range Address Matching Enable"]
pub type RmenW<'a, REG> = crate::BitWriter<'a, REG, Rmen>;
impl<'a, REG> RmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Range mode disabled. No address matching occurs for an address within the range of values of the A1 and RA registers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rmen::B0)
    }
    #[doc = "Range mode enabled. Address matching occurs when a slave receives an address within the range of values of the A1 and RA registers."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rmen::B1)
    }
}
#[doc = "Slave Baud Rate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbrc {
    #[doc = "0: The slave baud rate follows the master baud rate and clock stretching may occur"]
    B0 = 0,
    #[doc = "1: Slave baud rate is independent of the master baud rate"]
    B1 = 1,
}
impl From<Sbrc> for bool {
    #[inline(always)]
    fn from(variant: Sbrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBRC` reader - Slave Baud Rate Control"]
pub type SbrcR = crate::BitReader<Sbrc>;
impl SbrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbrc {
        match self.bits {
            false => Sbrc::B0,
            true => Sbrc::B1,
        }
    }
    #[doc = "The slave baud rate follows the master baud rate and clock stretching may occur"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sbrc::B0
    }
    #[doc = "Slave baud rate is independent of the master baud rate"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sbrc::B1
    }
}
#[doc = "Field `SBRC` writer - Slave Baud Rate Control"]
pub type SbrcW<'a, REG> = crate::BitWriter<'a, REG, Sbrc>;
impl<'a, REG> SbrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The slave baud rate follows the master baud rate and clock stretching may occur"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sbrc::B0)
    }
    #[doc = "Slave baud rate is independent of the master baud rate"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbrc::B1)
    }
}
#[doc = "High Drive Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hdrs {
    #[doc = "0: Normal drive mode"]
    B0 = 0,
    #[doc = "1: High drive mode"]
    B1 = 1,
}
impl From<Hdrs> for bool {
    #[inline(always)]
    fn from(variant: Hdrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HDRS` reader - High Drive Select"]
pub type HdrsR = crate::BitReader<Hdrs>;
impl HdrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hdrs {
        match self.bits {
            false => Hdrs::B0,
            true => Hdrs::B1,
        }
    }
    #[doc = "Normal drive mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hdrs::B0
    }
    #[doc = "High drive mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hdrs::B1
    }
}
#[doc = "Field `HDRS` writer - High Drive Select"]
pub type HdrsW<'a, REG> = crate::BitWriter<'a, REG, Hdrs>;
impl<'a, REG> HdrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal drive mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hdrs::B0)
    }
    #[doc = "High drive mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hdrs::B1)
    }
}
#[doc = "Address Extension\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adext {
    #[doc = "0: 7-bit address scheme"]
    B0 = 0,
    #[doc = "1: 10-bit address scheme"]
    B1 = 1,
}
impl From<Adext> for bool {
    #[inline(always)]
    fn from(variant: Adext) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADEXT` reader - Address Extension"]
pub type AdextR = crate::BitReader<Adext>;
impl AdextR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adext {
        match self.bits {
            false => Adext::B0,
            true => Adext::B1,
        }
    }
    #[doc = "7-bit address scheme"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adext::B0
    }
    #[doc = "10-bit address scheme"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adext::B1
    }
}
#[doc = "Field `ADEXT` writer - Address Extension"]
pub type AdextW<'a, REG> = crate::BitWriter<'a, REG, Adext>;
impl<'a, REG> AdextW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "7-bit address scheme"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adext::B0)
    }
    #[doc = "10-bit address scheme"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adext::B1)
    }
}
#[doc = "General Call Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gcaen {
    #[doc = "0: Disabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Gcaen> for bool {
    #[inline(always)]
    fn from(variant: Gcaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GCAEN` reader - General Call Address Enable"]
pub type GcaenR = crate::BitReader<Gcaen>;
impl GcaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gcaen {
        match self.bits {
            false => Gcaen::B0,
            true => Gcaen::B1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gcaen::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gcaen::B1
    }
}
#[doc = "Field `GCAEN` writer - General Call Address Enable"]
pub type GcaenW<'a, REG> = crate::BitWriter<'a, REG, Gcaen>;
impl<'a, REG> GcaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gcaen::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gcaen::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave Address"]
    #[inline(always)]
    pub fn ad(&self) -> AdR {
        AdR::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Range Address Matching Enable"]
    #[inline(always)]
    pub fn rmen(&self) -> RmenR {
        RmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Baud Rate Control"]
    #[inline(always)]
    pub fn sbrc(&self) -> SbrcR {
        SbrcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High Drive Select"]
    #[inline(always)]
    pub fn hdrs(&self) -> HdrsR {
        HdrsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Address Extension"]
    #[inline(always)]
    pub fn adext(&self) -> AdextR {
        AdextR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - General Call Address Enable"]
    #[inline(always)]
    pub fn gcaen(&self) -> GcaenR {
        GcaenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn ad(&mut self) -> AdW<C2Spec> {
        AdW::new(self, 0)
    }
    #[doc = "Bit 3 - Range Address Matching Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmen(&mut self) -> RmenW<C2Spec> {
        RmenW::new(self, 3)
    }
    #[doc = "Bit 4 - Slave Baud Rate Control"]
    #[inline(always)]
    #[must_use]
    pub fn sbrc(&mut self) -> SbrcW<C2Spec> {
        SbrcW::new(self, 4)
    }
    #[doc = "Bit 5 - High Drive Select"]
    #[inline(always)]
    #[must_use]
    pub fn hdrs(&mut self) -> HdrsW<C2Spec> {
        HdrsW::new(self, 5)
    }
    #[doc = "Bit 6 - Address Extension"]
    #[inline(always)]
    #[must_use]
    pub fn adext(&mut self) -> AdextW<C2Spec> {
        AdextW::new(self, 6)
    }
    #[doc = "Bit 7 - General Call Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcaen(&mut self) -> GcaenW<C2Spec> {
        GcaenW::new(self, 7)
    }
}
#[doc = "I2C Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2Spec;
impl crate::RegisterSpec for C2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c2::R`](R) reader structure"]
impl crate::Readable for C2Spec {}
#[doc = "`write(|w| ..)` method takes [`c2::W`](W) writer structure"]
impl crate::Writable for C2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2Spec {
    const RESET_VALUE: u8 = 0;
}
