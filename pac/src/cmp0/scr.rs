#[doc = "Register `SCR` reader"]
pub type R = crate::R<ScrSpec>;
#[doc = "Register `SCR` writer"]
pub type W = crate::W<ScrSpec>;
#[doc = "Field `COUT` reader - Analog Comparator Output"]
pub type CoutR = crate::BitReader;
#[doc = "Analog Comparator Flag Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cff {
    #[doc = "0: Falling-edge on COUT has not been detected."]
    B0 = 0,
    #[doc = "1: Falling-edge on COUT has occurred."]
    B1 = 1,
}
impl From<Cff> for bool {
    #[inline(always)]
    fn from(variant: Cff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFF` reader - Analog Comparator Flag Falling"]
pub type CffR = crate::BitReader<Cff>;
impl CffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cff {
        match self.bits {
            false => Cff::B0,
            true => Cff::B1,
        }
    }
    #[doc = "Falling-edge on COUT has not been detected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cff::B0
    }
    #[doc = "Falling-edge on COUT has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cff::B1
    }
}
#[doc = "Field `CFF` writer - Analog Comparator Flag Falling"]
pub type CffW<'a, REG> = crate::BitWriter<'a, REG, Cff>;
impl<'a, REG> CffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Falling-edge on COUT has not been detected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cff::B0)
    }
    #[doc = "Falling-edge on COUT has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cff::B1)
    }
}
#[doc = "Analog Comparator Flag Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfr {
    #[doc = "0: Rising-edge on COUT has not been detected."]
    B0 = 0,
    #[doc = "1: Rising-edge on COUT has occurred."]
    B1 = 1,
}
impl From<Cfr> for bool {
    #[inline(always)]
    fn from(variant: Cfr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFR` reader - Analog Comparator Flag Rising"]
pub type CfrR = crate::BitReader<Cfr>;
impl CfrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfr {
        match self.bits {
            false => Cfr::B0,
            true => Cfr::B1,
        }
    }
    #[doc = "Rising-edge on COUT has not been detected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cfr::B0
    }
    #[doc = "Rising-edge on COUT has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cfr::B1
    }
}
#[doc = "Field `CFR` writer - Analog Comparator Flag Rising"]
pub type CfrW<'a, REG> = crate::BitWriter<'a, REG, Cfr>;
impl<'a, REG> CfrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rising-edge on COUT has not been detected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfr::B0)
    }
    #[doc = "Rising-edge on COUT has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfr::B1)
    }
}
#[doc = "Comparator Interrupt Enable Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ief {
    #[doc = "0: Interrupt is disabled."]
    B0 = 0,
    #[doc = "1: Interrupt is enabled."]
    B1 = 1,
}
impl From<Ief> for bool {
    #[inline(always)]
    fn from(variant: Ief) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IEF` reader - Comparator Interrupt Enable Falling"]
pub type IefR = crate::BitReader<Ief>;
impl IefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ief {
        match self.bits {
            false => Ief::B0,
            true => Ief::B1,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ief::B0
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ief::B1
    }
}
#[doc = "Field `IEF` writer - Comparator Interrupt Enable Falling"]
pub type IefW<'a, REG> = crate::BitWriter<'a, REG, Ief>;
impl<'a, REG> IefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ief::B0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ief::B1)
    }
}
#[doc = "Comparator Interrupt Enable Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ier {
    #[doc = "0: Interrupt is disabled."]
    B0 = 0,
    #[doc = "1: Interrupt is enabled."]
    B1 = 1,
}
impl From<Ier> for bool {
    #[inline(always)]
    fn from(variant: Ier) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IER` reader - Comparator Interrupt Enable Rising"]
pub type IerR = crate::BitReader<Ier>;
impl IerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ier {
        match self.bits {
            false => Ier::B0,
            true => Ier::B1,
        }
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ier::B0
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ier::B1
    }
}
#[doc = "Field `IER` writer - Comparator Interrupt Enable Rising"]
pub type IerW<'a, REG> = crate::BitWriter<'a, REG, Ier>;
impl<'a, REG> IerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ier::B0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ier::B1)
    }
}
#[doc = "DMA Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: DMA is disabled."]
    B0 = 0,
    #[doc = "1: DMA is enabled."]
    B1 = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable Control"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::B0,
            true => Dmaen::B1,
        }
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dmaen::B0
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dmaen::B1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable Control"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::B0)
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Analog Comparator Output"]
    #[inline(always)]
    pub fn cout(&self) -> CoutR {
        CoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator Flag Falling"]
    #[inline(always)]
    pub fn cff(&self) -> CffR {
        CffR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator Flag Rising"]
    #[inline(always)]
    pub fn cfr(&self) -> CfrR {
        CfrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub fn ief(&self) -> IefR {
        IefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub fn ier(&self) -> IerR {
        IerR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Enable Control"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Analog Comparator Flag Falling"]
    #[inline(always)]
    #[must_use]
    pub fn cff(&mut self) -> CffW<ScrSpec> {
        CffW::new(self, 1)
    }
    #[doc = "Bit 2 - Analog Comparator Flag Rising"]
    #[inline(always)]
    #[must_use]
    pub fn cfr(&mut self) -> CfrW<ScrSpec> {
        CfrW::new(self, 2)
    }
    #[doc = "Bit 3 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    #[must_use]
    pub fn ief(&mut self) -> IefW<ScrSpec> {
        IefW::new(self, 3)
    }
    #[doc = "Bit 4 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    #[must_use]
    pub fn ier(&mut self) -> IerW<ScrSpec> {
        IerW::new(self, 4)
    }
    #[doc = "Bit 6 - DMA Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<ScrSpec> {
        DmaenW::new(self, 6)
    }
}
#[doc = "CMP Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScrSpec;
impl crate::RegisterSpec for ScrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scr::R`](R) reader structure"]
impl crate::Readable for ScrSpec {}
#[doc = "`write(|w| ..)` method takes [`scr::W`](W) writer structure"]
impl crate::Writable for ScrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for ScrSpec {
    const RESET_VALUE: u8 = 0;
}
