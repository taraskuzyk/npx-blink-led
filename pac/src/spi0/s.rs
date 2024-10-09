#[doc = "Register `S` reader"]
pub type R = crate::R<SSpec>;
#[doc = "Register `S` writer"]
pub type W = crate::W<SSpec>;
#[doc = "Master Mode Fault Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modf {
    #[doc = "0: No mode fault error"]
    B0 = 0,
    #[doc = "1: Mode fault error detected"]
    B1 = 1,
}
impl From<Modf> for bool {
    #[inline(always)]
    fn from(variant: Modf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODF` reader - Master Mode Fault Flag"]
pub type ModfR = crate::BitReader<Modf>;
impl ModfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modf {
        match self.bits {
            false => Modf::B0,
            true => Modf::B1,
        }
    }
    #[doc = "No mode fault error"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Modf::B0
    }
    #[doc = "Mode fault error detected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Modf::B1
    }
}
#[doc = "SPI Transmit Buffer Empty Flag (when FIFO is not supported or not enabled) or SPI transmit FIFO empty flag (when FIFO is supported and enabled)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sptef {
    #[doc = "0: SPI transmit buffer not empty (when FIFOMODE is not present or is 0) or SPI FIFO not empty (when FIFOMODE is 1)"]
    B0 = 0,
    #[doc = "1: SPI transmit buffer empty (when FIFOMODE is not present or is 0) or SPI FIFO empty (when FIFOMODE is 1)"]
    B1 = 1,
}
impl From<Sptef> for bool {
    #[inline(always)]
    fn from(variant: Sptef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPTEF` reader - SPI Transmit Buffer Empty Flag (when FIFO is not supported or not enabled) or SPI transmit FIFO empty flag (when FIFO is supported and enabled)"]
pub type SptefR = crate::BitReader<Sptef>;
impl SptefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sptef {
        match self.bits {
            false => Sptef::B0,
            true => Sptef::B1,
        }
    }
    #[doc = "SPI transmit buffer not empty (when FIFOMODE is not present or is 0) or SPI FIFO not empty (when FIFOMODE is 1)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sptef::B0
    }
    #[doc = "SPI transmit buffer empty (when FIFOMODE is not present or is 0) or SPI FIFO empty (when FIFOMODE is 1)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sptef::B1
    }
}
#[doc = "SPI Match Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spmf {
    #[doc = "0: Value in the receive data buffer does not match the value in the MH:ML registers"]
    B0 = 0,
    #[doc = "1: Value in the receive data buffer matches the value in the MH:ML registers"]
    B1 = 1,
}
impl From<Spmf> for bool {
    #[inline(always)]
    fn from(variant: Spmf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPMF` reader - SPI Match Flag"]
pub type SpmfR = crate::BitReader<Spmf>;
impl SpmfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spmf {
        match self.bits {
            false => Spmf::B0,
            true => Spmf::B1,
        }
    }
    #[doc = "Value in the receive data buffer does not match the value in the MH:ML registers"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spmf::B0
    }
    #[doc = "Value in the receive data buffer matches the value in the MH:ML registers"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spmf::B1
    }
}
#[doc = "Field `SPMF` writer - SPI Match Flag"]
pub type SpmfW<'a, REG> = crate::BitWriter<'a, REG, Spmf>;
impl<'a, REG> SpmfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Value in the receive data buffer does not match the value in the MH:ML registers"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spmf::B0)
    }
    #[doc = "Value in the receive data buffer matches the value in the MH:ML registers"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spmf::B1)
    }
}
#[doc = "SPI Read Buffer Full Flag (when FIFO is not supported or not enabled) or SPI read FIFO FULL flag (when FIFO is supported and enabled)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sprf {
    #[doc = "0: No data available in the receive data buffer (when FIFOMODE is not present or is 0) or Read FIFO is not full (when FIFOMODE is 1)"]
    B0 = 0,
    #[doc = "1: Data available in the receive data buffer (when FIFOMODE is not present or is 0) or Read FIFO is full (when FIFOMODE is 1)"]
    B1 = 1,
}
impl From<Sprf> for bool {
    #[inline(always)]
    fn from(variant: Sprf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPRF` reader - SPI Read Buffer Full Flag (when FIFO is not supported or not enabled) or SPI read FIFO FULL flag (when FIFO is supported and enabled)"]
pub type SprfR = crate::BitReader<Sprf>;
impl SprfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sprf {
        match self.bits {
            false => Sprf::B0,
            true => Sprf::B1,
        }
    }
    #[doc = "No data available in the receive data buffer (when FIFOMODE is not present or is 0) or Read FIFO is not full (when FIFOMODE is 1)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sprf::B0
    }
    #[doc = "Data available in the receive data buffer (when FIFOMODE is not present or is 0) or Read FIFO is full (when FIFOMODE is 1)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sprf::B1
    }
}
impl R {
    #[doc = "Bit 4 - Master Mode Fault Flag"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Transmit Buffer Empty Flag (when FIFO is not supported or not enabled) or SPI transmit FIFO empty flag (when FIFO is supported and enabled)"]
    #[inline(always)]
    pub fn sptef(&self) -> SptefR {
        SptefR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Match Flag"]
    #[inline(always)]
    pub fn spmf(&self) -> SpmfR {
        SpmfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Read Buffer Full Flag (when FIFO is not supported or not enabled) or SPI read FIFO FULL flag (when FIFO is supported and enabled)"]
    #[inline(always)]
    pub fn sprf(&self) -> SprfR {
        SprfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - SPI Match Flag"]
    #[inline(always)]
    #[must_use]
    pub fn spmf(&mut self) -> SpmfW<SSpec> {
        SpmfW::new(self, 6)
    }
}
#[doc = "SPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSpec;
impl crate::RegisterSpec for SSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s::R`](R) reader structure"]
impl crate::Readable for SSpec {}
#[doc = "`write(|w| ..)` method takes [`s::W`](W) writer structure"]
impl crate::Writable for SSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets S to value 0x20"]
impl crate::Resettable for SSpec {
    const RESET_VALUE: u8 = 0x20;
}
