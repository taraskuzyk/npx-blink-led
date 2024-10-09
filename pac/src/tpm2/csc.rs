#[doc = "Register `C%sSC` reader"]
pub type R = crate::R<CscSpec>;
#[doc = "Register `C%sSC` writer"]
pub type W = crate::W<CscSpec>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "0: Disable DMA transfers."]
    B0 = 0,
    #[doc = "1: Enable DMA transfers."]
    B1 = 1,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA Enable"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            false => Dma::B0,
            true => Dma::B1,
        }
    }
    #[doc = "Disable DMA transfers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dma::B0
    }
    #[doc = "Enable DMA transfers."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dma::B1
    }
}
#[doc = "Field `DMA` writer - DMA Enable"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable DMA transfers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::B0)
    }
    #[doc = "Enable DMA transfers."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::B1)
    }
}
#[doc = "Field `ELSA` reader - Edge or Level Select"]
pub type ElsaR = crate::BitReader;
#[doc = "Field `ELSA` writer - Edge or Level Select"]
pub type ElsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELSB` reader - Edge or Level Select"]
pub type ElsbR = crate::BitReader;
#[doc = "Field `ELSB` writer - Edge or Level Select"]
pub type ElsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSA` reader - Channel Mode Select"]
pub type MsaR = crate::BitReader;
#[doc = "Field `MSA` writer - Channel Mode Select"]
pub type MsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSB` reader - Channel Mode Select"]
pub type MsbR = crate::BitReader;
#[doc = "Field `MSB` writer - Channel Mode Select"]
pub type MsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Channel Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chie {
    #[doc = "0: Disable channel interrupts."]
    B0 = 0,
    #[doc = "1: Enable channel interrupts."]
    B1 = 1,
}
impl From<Chie> for bool {
    #[inline(always)]
    fn from(variant: Chie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHIE` reader - Channel Interrupt Enable"]
pub type ChieR = crate::BitReader<Chie>;
impl ChieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chie {
        match self.bits {
            false => Chie::B0,
            true => Chie::B1,
        }
    }
    #[doc = "Disable channel interrupts."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Chie::B0
    }
    #[doc = "Enable channel interrupts."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Chie::B1
    }
}
#[doc = "Field `CHIE` writer - Channel Interrupt Enable"]
pub type ChieW<'a, REG> = crate::BitWriter<'a, REG, Chie>;
impl<'a, REG> ChieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable channel interrupts."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Chie::B0)
    }
    #[doc = "Enable channel interrupts."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Chie::B1)
    }
}
#[doc = "Channel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chf {
    #[doc = "0: No channel event has occurred."]
    B0 = 0,
    #[doc = "1: A channel event has occurred."]
    B1 = 1,
}
impl From<Chf> for bool {
    #[inline(always)]
    fn from(variant: Chf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CHF` reader - Channel Flag"]
pub type ChfR = crate::BitReader<Chf>;
impl ChfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Chf {
        match self.bits {
            false => Chf::B0,
            true => Chf::B1,
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Chf::B0
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Chf::B1
    }
}
#[doc = "Field `CHF` writer - Channel Flag"]
pub type ChfW<'a, REG> = crate::BitWriter<'a, REG, Chf>;
impl<'a, REG> ChfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Chf::B0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Chf::B1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsa(&self) -> ElsaR {
        ElsaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsb(&self) -> ElsbR {
        ElsbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Mode Select"]
    #[inline(always)]
    pub fn msa(&self) -> MsaR {
        MsaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Mode Select"]
    #[inline(always)]
    pub fn msb(&self) -> MsbR {
        MsbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Interrupt Enable"]
    #[inline(always)]
    pub fn chie(&self) -> ChieR {
        ChieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Flag"]
    #[inline(always)]
    pub fn chf(&self) -> ChfR {
        ChfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<CscSpec> {
        DmaW::new(self, 0)
    }
    #[doc = "Bit 2 - Edge or Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn elsa(&mut self) -> ElsaW<CscSpec> {
        ElsaW::new(self, 2)
    }
    #[doc = "Bit 3 - Edge or Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn elsb(&mut self) -> ElsbW<CscSpec> {
        ElsbW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn msa(&mut self) -> MsaW<CscSpec> {
        MsaW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn msb(&mut self) -> MsbW<CscSpec> {
        MsbW::new(self, 5)
    }
    #[doc = "Bit 6 - Channel Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chie(&mut self) -> ChieW<CscSpec> {
        ChieW::new(self, 6)
    }
    #[doc = "Bit 7 - Channel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chf(&mut self) -> ChfW<CscSpec> {
        ChfW::new(self, 7)
    }
}
#[doc = "Channel (n) Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`csc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CscSpec;
impl crate::RegisterSpec for CscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csc::R`](R) reader structure"]
impl crate::Readable for CscSpec {}
#[doc = "`write(|w| ..)` method takes [`csc::W`](W) writer structure"]
impl crate::Writable for CscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C%sSC to value 0"]
impl crate::Resettable for CscSpec {
    const RESET_VALUE: u32 = 0;
}
