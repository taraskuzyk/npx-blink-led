#[doc = "Register `C1` reader"]
pub type R = crate::R<C1Spec>;
#[doc = "Register `C1` writer"]
pub type W = crate::W<C1Spec>;
#[doc = "LSB First (shifter direction)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lsbfe {
    #[doc = "0: SPI serial data transfers start with the most significant bit."]
    B0 = 0,
    #[doc = "1: SPI serial data transfers start with the least significant bit."]
    B1 = 1,
}
impl From<Lsbfe> for bool {
    #[inline(always)]
    fn from(variant: Lsbfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBFE` reader - LSB First (shifter direction)"]
pub type LsbfeR = crate::BitReader<Lsbfe>;
impl LsbfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lsbfe {
        match self.bits {
            false => Lsbfe::B0,
            true => Lsbfe::B1,
        }
    }
    #[doc = "SPI serial data transfers start with the most significant bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lsbfe::B0
    }
    #[doc = "SPI serial data transfers start with the least significant bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lsbfe::B1
    }
}
#[doc = "Field `LSBFE` writer - LSB First (shifter direction)"]
pub type LsbfeW<'a, REG> = crate::BitWriter<'a, REG, Lsbfe>;
impl<'a, REG> LsbfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI serial data transfers start with the most significant bit."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbfe::B0)
    }
    #[doc = "SPI serial data transfers start with the least significant bit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lsbfe::B1)
    }
}
#[doc = "Slave Select Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssoe {
    #[doc = "0: When C2\\[MODFEN\\]
is 0: In master mode, SS pin function is general-purpose I/O (not SPI). In slave mode, SS pin function is slave select input. When C2\\[MODFEN\\]
is 1: In master mode, SS pin function is SS input for mode fault. In slave mode, SS pin function is slave select input."]
    B0 = 0,
    #[doc = "1: When C2\\[MODFEN\\]
is 0: In master mode, SS pin function is general-purpose I/O (not SPI). In slave mode, SS pin function is slave select input. When C2\\[MODFEN\\]
is 1: In master mode, SS pin function is automatic SS output. In slave mode: SS pin function is slave select input."]
    B1 = 1,
}
impl From<Ssoe> for bool {
    #[inline(always)]
    fn from(variant: Ssoe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOE` reader - Slave Select Output Enable"]
pub type SsoeR = crate::BitReader<Ssoe>;
impl SsoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssoe {
        match self.bits {
            false => Ssoe::B0,
            true => Ssoe::B1,
        }
    }
    #[doc = "When C2\\[MODFEN\\]
is 0: In master mode, SS pin function is general-purpose I/O (not SPI). In slave mode, SS pin function is slave select input. When C2\\[MODFEN\\]
is 1: In master mode, SS pin function is SS input for mode fault. In slave mode, SS pin function is slave select input."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ssoe::B0
    }
    #[doc = "When C2\\[MODFEN\\]
is 0: In master mode, SS pin function is general-purpose I/O (not SPI). In slave mode, SS pin function is slave select input. When C2\\[MODFEN\\]
is 1: In master mode, SS pin function is automatic SS output. In slave mode: SS pin function is slave select input."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ssoe::B1
    }
}
#[doc = "Field `SSOE` writer - Slave Select Output Enable"]
pub type SsoeW<'a, REG> = crate::BitWriter<'a, REG, Ssoe>;
impl<'a, REG> SsoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When C2\\[MODFEN\\]
is 0: In master mode, SS pin function is general-purpose I/O (not SPI). In slave mode, SS pin function is slave select input. When C2\\[MODFEN\\]
is 1: In master mode, SS pin function is SS input for mode fault. In slave mode, SS pin function is slave select input."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssoe::B0)
    }
    #[doc = "When C2\\[MODFEN\\]
is 0: In master mode, SS pin function is general-purpose I/O (not SPI). In slave mode, SS pin function is slave select input. When C2\\[MODFEN\\]
is 1: In master mode, SS pin function is automatic SS output. In slave mode: SS pin function is slave select input."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssoe::B1)
    }
}
#[doc = "Clock Phase\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    #[doc = "0: First edge on SPSCK occurs at the middle of the first cycle of a data transfer."]
    B0 = 0,
    #[doc = "1: First edge on SPSCK occurs at the start of the first cycle of a data transfer."]
    B1 = 1,
}
impl From<Cpha> for bool {
    #[inline(always)]
    fn from(variant: Cpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CphaR = crate::BitReader<Cpha>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpha {
        match self.bits {
            false => Cpha::B0,
            true => Cpha::B1,
        }
    }
    #[doc = "First edge on SPSCK occurs at the middle of the first cycle of a data transfer."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cpha::B0
    }
    #[doc = "First edge on SPSCK occurs at the start of the first cycle of a data transfer."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cpha::B1
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cpha>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "First edge on SPSCK occurs at the middle of the first cycle of a data transfer."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::B0)
    }
    #[doc = "First edge on SPSCK occurs at the start of the first cycle of a data transfer."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::B1)
    }
}
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    #[doc = "0: Active-high SPI clock (idles low)"]
    B0 = 0,
    #[doc = "1: Active-low SPI clock (idles high)"]
    B1 = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CpolR = crate::BitReader<Cpol>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpol {
        match self.bits {
            false => Cpol::B0,
            true => Cpol::B1,
        }
    }
    #[doc = "Active-high SPI clock (idles low)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cpol::B0
    }
    #[doc = "Active-low SPI clock (idles high)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cpol::B1
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active-high SPI clock (idles low)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::B0)
    }
    #[doc = "Active-low SPI clock (idles high)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::B1)
    }
}
#[doc = "Master/Slave Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mstr {
    #[doc = "0: SPI module configured as a slave SPI device"]
    B0 = 0,
    #[doc = "1: SPI module configured as a master SPI device"]
    B1 = 1,
}
impl From<Mstr> for bool {
    #[inline(always)]
    fn from(variant: Mstr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSTR` reader - Master/Slave Mode Select"]
pub type MstrR = crate::BitReader<Mstr>;
impl MstrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mstr {
        match self.bits {
            false => Mstr::B0,
            true => Mstr::B1,
        }
    }
    #[doc = "SPI module configured as a slave SPI device"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Mstr::B0
    }
    #[doc = "SPI module configured as a master SPI device"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Mstr::B1
    }
}
#[doc = "Field `MSTR` writer - Master/Slave Mode Select"]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG, Mstr>;
impl<'a, REG> MstrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI module configured as a slave SPI device"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Mstr::B0)
    }
    #[doc = "SPI module configured as a master SPI device"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Mstr::B1)
    }
}
#[doc = "SPI Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sptie {
    #[doc = "0: Interrupts from SPTEF inhibited (use polling)"]
    B0 = 0,
    #[doc = "1: When SPTEF is 1, hardware interrupt requested"]
    B1 = 1,
}
impl From<Sptie> for bool {
    #[inline(always)]
    fn from(variant: Sptie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPTIE` reader - SPI Transmit Interrupt Enable"]
pub type SptieR = crate::BitReader<Sptie>;
impl SptieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sptie {
        match self.bits {
            false => Sptie::B0,
            true => Sptie::B1,
        }
    }
    #[doc = "Interrupts from SPTEF inhibited (use polling)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sptie::B0
    }
    #[doc = "When SPTEF is 1, hardware interrupt requested"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sptie::B1
    }
}
#[doc = "Field `SPTIE` writer - SPI Transmit Interrupt Enable"]
pub type SptieW<'a, REG> = crate::BitWriter<'a, REG, Sptie>;
impl<'a, REG> SptieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts from SPTEF inhibited (use polling)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sptie::B0)
    }
    #[doc = "When SPTEF is 1, hardware interrupt requested"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sptie::B1)
    }
}
#[doc = "SPI System Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spe {
    #[doc = "0: SPI system inactive"]
    B0 = 0,
    #[doc = "1: SPI system enabled"]
    B1 = 1,
}
impl From<Spe> for bool {
    #[inline(always)]
    fn from(variant: Spe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPE` reader - SPI System Enable"]
pub type SpeR = crate::BitReader<Spe>;
impl SpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spe {
        match self.bits {
            false => Spe::B0,
            true => Spe::B1,
        }
    }
    #[doc = "SPI system inactive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spe::B0
    }
    #[doc = "SPI system enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spe::B1
    }
}
#[doc = "Field `SPE` writer - SPI System Enable"]
pub type SpeW<'a, REG> = crate::BitWriter<'a, REG, Spe>;
impl<'a, REG> SpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI system inactive"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spe::B0)
    }
    #[doc = "SPI system enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spe::B1)
    }
}
#[doc = "SPI Interrupt Enable: for SPRF and MODF (when FIFO is not supported or not enabled) or for read FIFO (when FIFO is supported and enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spie {
    #[doc = "0: Interrupts from SPRF and MODF are inhibited-use polling (when FIFOMODE is not present or is 0) or Read FIFO Full Interrupts are disabled (when FIFOMODE is 1)"]
    B0 = 0,
    #[doc = "1: Request a hardware interrupt when SPRF or MODF is 1 (when FIFOMODE is not present or is 0) or Read FIFO Full Interrupts are enabled (when FIFOMODE is 1)"]
    B1 = 1,
}
impl From<Spie> for bool {
    #[inline(always)]
    fn from(variant: Spie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIE` reader - SPI Interrupt Enable: for SPRF and MODF (when FIFO is not supported or not enabled) or for read FIFO (when FIFO is supported and enabled)"]
pub type SpieR = crate::BitReader<Spie>;
impl SpieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spie {
        match self.bits {
            false => Spie::B0,
            true => Spie::B1,
        }
    }
    #[doc = "Interrupts from SPRF and MODF are inhibited-use polling (when FIFOMODE is not present or is 0) or Read FIFO Full Interrupts are disabled (when FIFOMODE is 1)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spie::B0
    }
    #[doc = "Request a hardware interrupt when SPRF or MODF is 1 (when FIFOMODE is not present or is 0) or Read FIFO Full Interrupts are enabled (when FIFOMODE is 1)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spie::B1
    }
}
#[doc = "Field `SPIE` writer - SPI Interrupt Enable: for SPRF and MODF (when FIFO is not supported or not enabled) or for read FIFO (when FIFO is supported and enabled)"]
pub type SpieW<'a, REG> = crate::BitWriter<'a, REG, Spie>;
impl<'a, REG> SpieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts from SPRF and MODF are inhibited-use polling (when FIFOMODE is not present or is 0) or Read FIFO Full Interrupts are disabled (when FIFOMODE is 1)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spie::B0)
    }
    #[doc = "Request a hardware interrupt when SPRF or MODF is 1 (when FIFOMODE is not present or is 0) or Read FIFO Full Interrupts are enabled (when FIFOMODE is 1)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spie::B1)
    }
}
impl R {
    #[doc = "Bit 0 - LSB First (shifter direction)"]
    #[inline(always)]
    pub fn lsbfe(&self) -> LsbfeR {
        LsbfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave Select Output Enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SsoeR {
        SsoeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master/Slave Mode Select"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn sptie(&self) -> SptieR {
        SptieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI System Enable"]
    #[inline(always)]
    pub fn spe(&self) -> SpeR {
        SpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Interrupt Enable: for SPRF and MODF (when FIFO is not supported or not enabled) or for read FIFO (when FIFO is supported and enabled)"]
    #[inline(always)]
    pub fn spie(&self) -> SpieR {
        SpieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSB First (shifter direction)"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfe(&mut self) -> LsbfeW<C1Spec> {
        LsbfeW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave Select Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SsoeW<C1Spec> {
        SsoeW::new(self, 1)
    }
    #[doc = "Bit 2 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CphaW<C1Spec> {
        CphaW::new(self, 2)
    }
    #[doc = "Bit 3 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CpolW<C1Spec> {
        CpolW::new(self, 3)
    }
    #[doc = "Bit 4 - Master/Slave Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MstrW<C1Spec> {
        MstrW::new(self, 4)
    }
    #[doc = "Bit 5 - SPI Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sptie(&mut self) -> SptieW<C1Spec> {
        SptieW::new(self, 5)
    }
    #[doc = "Bit 6 - SPI System Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SpeW<C1Spec> {
        SpeW::new(self, 6)
    }
    #[doc = "Bit 7 - SPI Interrupt Enable: for SPRF and MODF (when FIFO is not supported or not enabled) or for read FIFO (when FIFO is supported and enabled)"]
    #[inline(always)]
    #[must_use]
    pub fn spie(&mut self) -> SpieW<C1Spec> {
        SpieW::new(self, 7)
    }
}
#[doc = "SPI Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Spec;
impl crate::RegisterSpec for C1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c1::R`](R) reader structure"]
impl crate::Readable for C1Spec {}
#[doc = "`write(|w| ..)` method takes [`c1::W`](W) writer structure"]
impl crate::Writable for C1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C1 to value 0x04"]
impl crate::Resettable for C1Spec {
    const RESET_VALUE: u8 = 0x04;
}
