#[doc = "Register `C1` reader"]
pub type R = crate::R<C1Spec>;
#[doc = "Register `C1` writer"]
pub type W = crate::W<C1Spec>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: All DMA signalling disabled."]
    B0 = 0,
    #[doc = "1: DMA transfer is enabled. While SMB\\[FACK\\]
= 0, the following conditions trigger the DMA request: a data byte is received, and either address or data is transmitted. (ACK/NACK is automatic) the first byte received matches the A1 register or is a general call address. If any address matching occurs, S\\[IAAS\\]
and S\\[TCF\\]
are set. If the direction of transfer is known from master to slave, then it is not required to check S\\[SRW\\]. With this assumption, DMA can also be used in this case. In other cases, if the master reads data from the slave, then it is required to rewrite the C1 register operation. With this assumption, DMA cannot be used. When FACK = 1, an address or a data byte is transmitted."]
    B1 = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
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
    #[doc = "All DMA signalling disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dmaen::B0
    }
    #[doc = "DMA transfer is enabled. While SMB\\[FACK\\]
= 0, the following conditions trigger the DMA request: a data byte is received, and either address or data is transmitted. (ACK/NACK is automatic) the first byte received matches the A1 register or is a general call address. If any address matching occurs, S\\[IAAS\\]
and S\\[TCF\\]
are set. If the direction of transfer is known from master to slave, then it is not required to check S\\[SRW\\]. With this assumption, DMA can also be used in this case. In other cases, if the master reads data from the slave, then it is required to rewrite the C1 register operation. With this assumption, DMA cannot be used. When FACK = 1, an address or a data byte is transmitted."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dmaen::B1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All DMA signalling disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::B0)
    }
    #[doc = "DMA transfer is enabled. While SMB\\[FACK\\]
= 0, the following conditions trigger the DMA request: a data byte is received, and either address or data is transmitted. (ACK/NACK is automatic) the first byte received matches the A1 register or is a general call address. If any address matching occurs, S\\[IAAS\\]
and S\\[TCF\\]
are set. If the direction of transfer is known from master to slave, then it is not required to check S\\[SRW\\]. With this assumption, DMA can also be used in this case. In other cases, if the master reads data from the slave, then it is required to rewrite the C1 register operation. With this assumption, DMA cannot be used. When FACK = 1, an address or a data byte is transmitted."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::B1)
    }
}
#[doc = "Wakeup Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wuen {
    #[doc = "0: Normal operation. No interrupt generated when address matching in low power mode."]
    B0 = 0,
    #[doc = "1: Enables the wakeup function in low power mode."]
    B1 = 1,
}
impl From<Wuen> for bool {
    #[inline(always)]
    fn from(variant: Wuen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUEN` reader - Wakeup Enable"]
pub type WuenR = crate::BitReader<Wuen>;
impl WuenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wuen {
        match self.bits {
            false => Wuen::B0,
            true => Wuen::B1,
        }
    }
    #[doc = "Normal operation. No interrupt generated when address matching in low power mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wuen::B0
    }
    #[doc = "Enables the wakeup function in low power mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wuen::B1
    }
}
#[doc = "Field `WUEN` writer - Wakeup Enable"]
pub type WuenW<'a, REG> = crate::BitWriter<'a, REG, Wuen>;
impl<'a, REG> WuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation. No interrupt generated when address matching in low power mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wuen::B0)
    }
    #[doc = "Enables the wakeup function in low power mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wuen::B1)
    }
}
#[doc = "Field `RSTA` reader - Repeat START"]
pub type RstaR = crate::BitReader;
#[doc = "Field `RSTA` writer - Repeat START"]
pub type RstaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transmit Acknowledge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txak {
    #[doc = "0: An acknowledge signal is sent to the bus on the following receiving byte (if FACK is cleared) or the current receiving byte (if FACK is set)."]
    B0 = 0,
    #[doc = "1: No acknowledge signal is sent to the bus on the following receiving data byte (if FACK is cleared) or the current receiving data byte (if FACK is set)."]
    B1 = 1,
}
impl From<Txak> for bool {
    #[inline(always)]
    fn from(variant: Txak) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXAK` reader - Transmit Acknowledge Enable"]
pub type TxakR = crate::BitReader<Txak>;
impl TxakR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txak {
        match self.bits {
            false => Txak::B0,
            true => Txak::B1,
        }
    }
    #[doc = "An acknowledge signal is sent to the bus on the following receiving byte (if FACK is cleared) or the current receiving byte (if FACK is set)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txak::B0
    }
    #[doc = "No acknowledge signal is sent to the bus on the following receiving data byte (if FACK is cleared) or the current receiving data byte (if FACK is set)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txak::B1
    }
}
#[doc = "Field `TXAK` writer - Transmit Acknowledge Enable"]
pub type TxakW<'a, REG> = crate::BitWriter<'a, REG, Txak>;
impl<'a, REG> TxakW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An acknowledge signal is sent to the bus on the following receiving byte (if FACK is cleared) or the current receiving byte (if FACK is set)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Txak::B0)
    }
    #[doc = "No acknowledge signal is sent to the bus on the following receiving data byte (if FACK is cleared) or the current receiving data byte (if FACK is set)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Txak::B1)
    }
}
#[doc = "Transmit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tx {
    #[doc = "0: Receive"]
    B0 = 0,
    #[doc = "1: Transmit"]
    B1 = 1,
}
impl From<Tx> for bool {
    #[inline(always)]
    fn from(variant: Tx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TX` reader - Transmit Mode Select"]
pub type TxR = crate::BitReader<Tx>;
impl TxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tx {
        match self.bits {
            false => Tx::B0,
            true => Tx::B1,
        }
    }
    #[doc = "Receive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tx::B0
    }
    #[doc = "Transmit"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tx::B1
    }
}
#[doc = "Field `TX` writer - Transmit Mode Select"]
pub type TxW<'a, REG> = crate::BitWriter<'a, REG, Tx>;
impl<'a, REG> TxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::B0)
    }
    #[doc = "Transmit"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tx::B1)
    }
}
#[doc = "Master Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mst {
    #[doc = "0: Slave mode"]
    B0 = 0,
    #[doc = "1: Master mode"]
    B1 = 1,
}
impl From<Mst> for bool {
    #[inline(always)]
    fn from(variant: Mst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MST` reader - Master Mode Select"]
pub type MstR = crate::BitReader<Mst>;
impl MstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mst {
        match self.bits {
            false => Mst::B0,
            true => Mst::B1,
        }
    }
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Mst::B0
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Mst::B1
    }
}
#[doc = "Field `MST` writer - Master Mode Select"]
pub type MstW<'a, REG> = crate::BitWriter<'a, REG, Mst>;
impl<'a, REG> MstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Mst::B0)
    }
    #[doc = "Master mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Mst::B1)
    }
}
#[doc = "I2C Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicie {
    #[doc = "0: Disabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Iicie> for bool {
    #[inline(always)]
    fn from(variant: Iicie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICIE` reader - I2C Interrupt Enable"]
pub type IicieR = crate::BitReader<Iicie>;
impl IicieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicie {
        match self.bits {
            false => Iicie::B0,
            true => Iicie::B1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Iicie::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Iicie::B1
    }
}
#[doc = "Field `IICIE` writer - I2C Interrupt Enable"]
pub type IicieW<'a, REG> = crate::BitWriter<'a, REG, Iicie>;
impl<'a, REG> IicieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicie::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicie::B1)
    }
}
#[doc = "I2C Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicen {
    #[doc = "0: Disabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Iicen> for bool {
    #[inline(always)]
    fn from(variant: Iicen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICEN` reader - I2C Enable"]
pub type IicenR = crate::BitReader<Iicen>;
impl IicenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicen {
        match self.bits {
            false => Iicen::B0,
            true => Iicen::B1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Iicen::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Iicen::B1
    }
}
#[doc = "Field `IICEN` writer - I2C Enable"]
pub type IicenW<'a, REG> = crate::BitWriter<'a, REG, Iicen>;
impl<'a, REG> IicenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicen::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicen::B1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Enable"]
    #[inline(always)]
    pub fn wuen(&self) -> WuenR {
        WuenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Repeat START"]
    #[inline(always)]
    pub fn rsta(&self) -> RstaR {
        RstaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Acknowledge Enable"]
    #[inline(always)]
    pub fn txak(&self) -> TxakR {
        TxakR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Mode Select"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master Mode Select"]
    #[inline(always)]
    pub fn mst(&self) -> MstR {
        MstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Interrupt Enable"]
    #[inline(always)]
    pub fn iicie(&self) -> IicieR {
        IicieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C Enable"]
    #[inline(always)]
    pub fn iicen(&self) -> IicenR {
        IicenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<C1Spec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wuen(&mut self) -> WuenW<C1Spec> {
        WuenW::new(self, 1)
    }
    #[doc = "Bit 2 - Repeat START"]
    #[inline(always)]
    #[must_use]
    pub fn rsta(&mut self) -> RstaW<C1Spec> {
        RstaW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Acknowledge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txak(&mut self) -> TxakW<C1Spec> {
        TxakW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TxW<C1Spec> {
        TxW::new(self, 4)
    }
    #[doc = "Bit 5 - Master Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mst(&mut self) -> MstW<C1Spec> {
        MstW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iicie(&mut self) -> IicieW<C1Spec> {
        IicieW::new(self, 6)
    }
    #[doc = "Bit 7 - I2C Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iicen(&mut self) -> IicenW<C1Spec> {
        IicenW::new(self, 7)
    }
}
#[doc = "I2C Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1Spec {
    const RESET_VALUE: u8 = 0;
}
