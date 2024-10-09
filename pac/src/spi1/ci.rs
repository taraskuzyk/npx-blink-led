#[doc = "Register `CI` reader"]
pub type R = crate::R<CiSpec>;
#[doc = "Register `CI` writer"]
pub type W = crate::W<CiSpec>;
#[doc = "Field `SPRFCI` reader - Receive FIFO full flag clear interrupt"]
pub type SprfciR = crate::BitReader;
#[doc = "Field `SPRFCI` writer - Receive FIFO full flag clear interrupt"]
pub type SprfciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPTEFCI` reader - Transmit FIFO empty flag clear interrupt"]
pub type SptefciR = crate::BitReader;
#[doc = "Field `SPTEFCI` writer - Transmit FIFO empty flag clear interrupt"]
pub type SptefciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNFULLFCI` reader - Receive FIFO nearly full flag clear interrupt"]
pub type RnfullfciR = crate::BitReader;
#[doc = "Field `RNFULLFCI` writer - Receive FIFO nearly full flag clear interrupt"]
pub type RnfullfciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TNEAREFCI` reader - Transmit FIFO nearly empty flag clear interrupt"]
pub type TnearefciR = crate::BitReader;
#[doc = "Field `TNEAREFCI` writer - Transmit FIFO nearly empty flag clear interrupt"]
pub type TnearefciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Receive FIFO overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfof {
    #[doc = "0: Receive FIFO overflow condition has not occurred"]
    B0 = 0,
    #[doc = "1: Receive FIFO overflow condition occurred"]
    B1 = 1,
}
impl From<Rxfof> for bool {
    #[inline(always)]
    fn from(variant: Rxfof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFOF` reader - Receive FIFO overflow flag"]
pub type RxfofR = crate::BitReader<Rxfof>;
impl RxfofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfof {
        match self.bits {
            false => Rxfof::B0,
            true => Rxfof::B1,
        }
    }
    #[doc = "Receive FIFO overflow condition has not occurred"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxfof::B0
    }
    #[doc = "Receive FIFO overflow condition occurred"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxfof::B1
    }
}
#[doc = "Transmit FIFO overflow flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfof {
    #[doc = "0: Transmit FIFO overflow condition has not occurred"]
    B0 = 0,
    #[doc = "1: Transmit FIFO overflow condition occurred"]
    B1 = 1,
}
impl From<Txfof> for bool {
    #[inline(always)]
    fn from(variant: Txfof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFOF` reader - Transmit FIFO overflow flag"]
pub type TxfofR = crate::BitReader<Txfof>;
impl TxfofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txfof {
        match self.bits {
            false => Txfof::B0,
            true => Txfof::B1,
        }
    }
    #[doc = "Transmit FIFO overflow condition has not occurred"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txfof::B0
    }
    #[doc = "Transmit FIFO overflow condition occurred"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txfof::B1
    }
}
#[doc = "Receive FIFO error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxferr {
    #[doc = "0: No receive FIFO error occurred"]
    B0 = 0,
    #[doc = "1: A receive FIFO error occurred"]
    B1 = 1,
}
impl From<Rxferr> for bool {
    #[inline(always)]
    fn from(variant: Rxferr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFERR` reader - Receive FIFO error flag"]
pub type RxferrR = crate::BitReader<Rxferr>;
impl RxferrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxferr {
        match self.bits {
            false => Rxferr::B0,
            true => Rxferr::B1,
        }
    }
    #[doc = "No receive FIFO error occurred"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxferr::B0
    }
    #[doc = "A receive FIFO error occurred"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxferr::B1
    }
}
#[doc = "Transmit FIFO error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txferr {
    #[doc = "0: No transmit FIFO error occurred"]
    B0 = 0,
    #[doc = "1: A transmit FIFO error occurred"]
    B1 = 1,
}
impl From<Txferr> for bool {
    #[inline(always)]
    fn from(variant: Txferr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFERR` reader - Transmit FIFO error flag"]
pub type TxferrR = crate::BitReader<Txferr>;
impl TxferrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txferr {
        match self.bits {
            false => Txferr::B0,
            true => Txferr::B1,
        }
    }
    #[doc = "No transmit FIFO error occurred"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txferr::B0
    }
    #[doc = "A transmit FIFO error occurred"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txferr::B1
    }
}
impl R {
    #[doc = "Bit 0 - Receive FIFO full flag clear interrupt"]
    #[inline(always)]
    pub fn sprfci(&self) -> SprfciR {
        SprfciR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO empty flag clear interrupt"]
    #[inline(always)]
    pub fn sptefci(&self) -> SptefciR {
        SptefciR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO nearly full flag clear interrupt"]
    #[inline(always)]
    pub fn rnfullfci(&self) -> RnfullfciR {
        RnfullfciR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit FIFO nearly empty flag clear interrupt"]
    #[inline(always)]
    pub fn tnearefci(&self) -> TnearefciR {
        TnearefciR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO overflow flag"]
    #[inline(always)]
    pub fn rxfof(&self) -> RxfofR {
        RxfofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO overflow flag"]
    #[inline(always)]
    pub fn txfof(&self) -> TxfofR {
        TxfofR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive FIFO error flag"]
    #[inline(always)]
    pub fn rxferr(&self) -> RxferrR {
        RxferrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO error flag"]
    #[inline(always)]
    pub fn txferr(&self) -> TxferrR {
        TxferrR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive FIFO full flag clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sprfci(&mut self) -> SprfciW<CiSpec> {
        SprfciW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO empty flag clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn sptefci(&mut self) -> SptefciW<CiSpec> {
        SptefciW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive FIFO nearly full flag clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rnfullfci(&mut self) -> RnfullfciW<CiSpec> {
        RnfullfciW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit FIFO nearly empty flag clear interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tnearefci(&mut self) -> TnearefciW<CiSpec> {
        TnearefciW::new(self, 3)
    }
}
#[doc = "SPI clear interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`ci::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CiSpec;
impl crate::RegisterSpec for CiSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ci::R`](R) reader structure"]
impl crate::Readable for CiSpec {}
#[doc = "`write(|w| ..)` method takes [`ci::W`](W) writer structure"]
impl crate::Writable for CiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CI to value 0"]
impl crate::Resettable for CiSpec {
    const RESET_VALUE: u8 = 0;
}
