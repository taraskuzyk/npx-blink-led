#[doc = "Register `PLACR` reader"]
pub type R = crate::R<PlacrSpec>;
#[doc = "Register `PLACR` writer"]
pub type W = crate::W<PlacrSpec>;
#[doc = "Arbitration select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arb {
    #[doc = "0: Fixed-priority arbitration for the crossbar masters"]
    B0 = 0,
    #[doc = "1: Round-robin arbitration for the crossbar masters"]
    B1 = 1,
}
impl From<Arb> for bool {
    #[inline(always)]
    fn from(variant: Arb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARB` reader - Arbitration select"]
pub type ArbR = crate::BitReader<Arb>;
impl ArbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arb {
        match self.bits {
            false => Arb::B0,
            true => Arb::B1,
        }
    }
    #[doc = "Fixed-priority arbitration for the crossbar masters"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Arb::B0
    }
    #[doc = "Round-robin arbitration for the crossbar masters"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Arb::B1
    }
}
#[doc = "Field `ARB` writer - Arbitration select"]
pub type ArbW<'a, REG> = crate::BitWriter<'a, REG, Arb>;
impl<'a, REG> ArbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fixed-priority arbitration for the crossbar masters"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Arb::B0)
    }
    #[doc = "Round-robin arbitration for the crossbar masters"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Arb::B1)
    }
}
#[doc = "Field `CFCC` reader - Clear Flash Controller Cache"]
pub type CfccR = crate::BitReader;
#[doc = "Field `CFCC` writer - Clear Flash Controller Cache"]
pub type CfccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Disable Flash Controller Data Caching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfcda {
    #[doc = "0: Enable flash controller data caching"]
    B0 = 0,
    #[doc = "1: Disable flash controller data caching."]
    B1 = 1,
}
impl From<Dfcda> for bool {
    #[inline(always)]
    fn from(variant: Dfcda) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFCDA` reader - Disable Flash Controller Data Caching"]
pub type DfcdaR = crate::BitReader<Dfcda>;
impl DfcdaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfcda {
        match self.bits {
            false => Dfcda::B0,
            true => Dfcda::B1,
        }
    }
    #[doc = "Enable flash controller data caching"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dfcda::B0
    }
    #[doc = "Disable flash controller data caching."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dfcda::B1
    }
}
#[doc = "Field `DFCDA` writer - Disable Flash Controller Data Caching"]
pub type DfcdaW<'a, REG> = crate::BitWriter<'a, REG, Dfcda>;
impl<'a, REG> DfcdaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable flash controller data caching"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcda::B0)
    }
    #[doc = "Disable flash controller data caching."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcda::B1)
    }
}
#[doc = "Disable Flash Controller Instruction Caching\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfcic {
    #[doc = "0: Enable flash controller instruction caching."]
    B0 = 0,
    #[doc = "1: Disable flash controller instruction caching."]
    B1 = 1,
}
impl From<Dfcic> for bool {
    #[inline(always)]
    fn from(variant: Dfcic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFCIC` reader - Disable Flash Controller Instruction Caching"]
pub type DfcicR = crate::BitReader<Dfcic>;
impl DfcicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfcic {
        match self.bits {
            false => Dfcic::B0,
            true => Dfcic::B1,
        }
    }
    #[doc = "Enable flash controller instruction caching."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dfcic::B0
    }
    #[doc = "Disable flash controller instruction caching."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dfcic::B1
    }
}
#[doc = "Field `DFCIC` writer - Disable Flash Controller Instruction Caching"]
pub type DfcicW<'a, REG> = crate::BitWriter<'a, REG, Dfcic>;
impl<'a, REG> DfcicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable flash controller instruction caching."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcic::B0)
    }
    #[doc = "Disable flash controller instruction caching."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcic::B1)
    }
}
#[doc = "Disable Flash Controller Cache\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfcc {
    #[doc = "0: Enable flash controller cache."]
    B0 = 0,
    #[doc = "1: Disable flash controller cache."]
    B1 = 1,
}
impl From<Dfcc> for bool {
    #[inline(always)]
    fn from(variant: Dfcc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFCC` reader - Disable Flash Controller Cache"]
pub type DfccR = crate::BitReader<Dfcc>;
impl DfccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfcc {
        match self.bits {
            false => Dfcc::B0,
            true => Dfcc::B1,
        }
    }
    #[doc = "Enable flash controller cache."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dfcc::B0
    }
    #[doc = "Disable flash controller cache."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dfcc::B1
    }
}
#[doc = "Field `DFCC` writer - Disable Flash Controller Cache"]
pub type DfccW<'a, REG> = crate::BitWriter<'a, REG, Dfcc>;
impl<'a, REG> DfccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable flash controller cache."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcc::B0)
    }
    #[doc = "Disable flash controller cache."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcc::B1)
    }
}
#[doc = "Enable Flash Data Speculation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Efds {
    #[doc = "0: Disable flash data speculation."]
    B0 = 0,
    #[doc = "1: Enable flash data speculation."]
    B1 = 1,
}
impl From<Efds> for bool {
    #[inline(always)]
    fn from(variant: Efds) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EFDS` reader - Enable Flash Data Speculation"]
pub type EfdsR = crate::BitReader<Efds>;
impl EfdsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Efds {
        match self.bits {
            false => Efds::B0,
            true => Efds::B1,
        }
    }
    #[doc = "Disable flash data speculation."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Efds::B0
    }
    #[doc = "Enable flash data speculation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Efds::B1
    }
}
#[doc = "Field `EFDS` writer - Enable Flash Data Speculation"]
pub type EfdsW<'a, REG> = crate::BitWriter<'a, REG, Efds>;
impl<'a, REG> EfdsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable flash data speculation."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Efds::B0)
    }
    #[doc = "Enable flash data speculation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Efds::B1)
    }
}
#[doc = "Disable Flash Controller Speculation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfcs {
    #[doc = "0: Enable flash controller speculation."]
    B0 = 0,
    #[doc = "1: Disable flash controller speculation."]
    B1 = 1,
}
impl From<Dfcs> for bool {
    #[inline(always)]
    fn from(variant: Dfcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFCS` reader - Disable Flash Controller Speculation"]
pub type DfcsR = crate::BitReader<Dfcs>;
impl DfcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfcs {
        match self.bits {
            false => Dfcs::B0,
            true => Dfcs::B1,
        }
    }
    #[doc = "Enable flash controller speculation."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dfcs::B0
    }
    #[doc = "Disable flash controller speculation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dfcs::B1
    }
}
#[doc = "Field `DFCS` writer - Disable Flash Controller Speculation"]
pub type DfcsW<'a, REG> = crate::BitWriter<'a, REG, Dfcs>;
impl<'a, REG> DfcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable flash controller speculation."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcs::B0)
    }
    #[doc = "Disable flash controller speculation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfcs::B1)
    }
}
#[doc = "Enable Stalling Flash Controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Esfc {
    #[doc = "0: Disable stalling flash controller when flash is busy."]
    B0 = 0,
    #[doc = "1: Enable stalling flash controller when flash is busy."]
    B1 = 1,
}
impl From<Esfc> for bool {
    #[inline(always)]
    fn from(variant: Esfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ESFC` reader - Enable Stalling Flash Controller"]
pub type EsfcR = crate::BitReader<Esfc>;
impl EsfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Esfc {
        match self.bits {
            false => Esfc::B0,
            true => Esfc::B1,
        }
    }
    #[doc = "Disable stalling flash controller when flash is busy."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Esfc::B0
    }
    #[doc = "Enable stalling flash controller when flash is busy."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Esfc::B1
    }
}
#[doc = "Field `ESFC` writer - Enable Stalling Flash Controller"]
pub type EsfcW<'a, REG> = crate::BitWriter<'a, REG, Esfc>;
impl<'a, REG> EsfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable stalling flash controller when flash is busy."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Esfc::B0)
    }
    #[doc = "Enable stalling flash controller when flash is busy."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Esfc::B1)
    }
}
impl R {
    #[doc = "Bit 9 - Arbitration select"]
    #[inline(always)]
    pub fn arb(&self) -> ArbR {
        ArbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clear Flash Controller Cache"]
    #[inline(always)]
    pub fn cfcc(&self) -> CfccR {
        CfccR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disable Flash Controller Data Caching"]
    #[inline(always)]
    pub fn dfcda(&self) -> DfcdaR {
        DfcdaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disable Flash Controller Instruction Caching"]
    #[inline(always)]
    pub fn dfcic(&self) -> DfcicR {
        DfcicR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable Flash Controller Cache"]
    #[inline(always)]
    pub fn dfcc(&self) -> DfccR {
        DfccR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable Flash Data Speculation"]
    #[inline(always)]
    pub fn efds(&self) -> EfdsR {
        EfdsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Disable Flash Controller Speculation"]
    #[inline(always)]
    pub fn dfcs(&self) -> DfcsR {
        DfcsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Stalling Flash Controller"]
    #[inline(always)]
    pub fn esfc(&self) -> EsfcR {
        EsfcR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Arbitration select"]
    #[inline(always)]
    #[must_use]
    pub fn arb(&mut self) -> ArbW<PlacrSpec> {
        ArbW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear Flash Controller Cache"]
    #[inline(always)]
    #[must_use]
    pub fn cfcc(&mut self) -> CfccW<PlacrSpec> {
        CfccW::new(self, 10)
    }
    #[doc = "Bit 11 - Disable Flash Controller Data Caching"]
    #[inline(always)]
    #[must_use]
    pub fn dfcda(&mut self) -> DfcdaW<PlacrSpec> {
        DfcdaW::new(self, 11)
    }
    #[doc = "Bit 12 - Disable Flash Controller Instruction Caching"]
    #[inline(always)]
    #[must_use]
    pub fn dfcic(&mut self) -> DfcicW<PlacrSpec> {
        DfcicW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable Flash Controller Cache"]
    #[inline(always)]
    #[must_use]
    pub fn dfcc(&mut self) -> DfccW<PlacrSpec> {
        DfccW::new(self, 13)
    }
    #[doc = "Bit 14 - Enable Flash Data Speculation"]
    #[inline(always)]
    #[must_use]
    pub fn efds(&mut self) -> EfdsW<PlacrSpec> {
        EfdsW::new(self, 14)
    }
    #[doc = "Bit 15 - Disable Flash Controller Speculation"]
    #[inline(always)]
    #[must_use]
    pub fn dfcs(&mut self) -> DfcsW<PlacrSpec> {
        DfcsW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable Stalling Flash Controller"]
    #[inline(always)]
    #[must_use]
    pub fn esfc(&mut self) -> EsfcW<PlacrSpec> {
        EsfcW::new(self, 16)
    }
}
#[doc = "Platform Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`placr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`placr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlacrSpec;
impl crate::RegisterSpec for PlacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`placr::R`](R) reader structure"]
impl crate::Readable for PlacrSpec {}
#[doc = "`write(|w| ..)` method takes [`placr::W`](W) writer structure"]
impl crate::Writable for PlacrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLACR to value 0"]
impl crate::Resettable for PlacrSpec {
    const RESET_VALUE: u32 = 0;
}
