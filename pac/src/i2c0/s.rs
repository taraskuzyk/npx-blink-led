#[doc = "Register `S` reader"]
pub type R = crate::R<SSpec>;
#[doc = "Register `S` writer"]
pub type W = crate::W<SSpec>;
#[doc = "Receive Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxak {
    #[doc = "0: Acknowledge signal was received after the completion of one byte of data transmission on the bus"]
    B0 = 0,
    #[doc = "1: No acknowledge signal detected"]
    B1 = 1,
}
impl From<Rxak> for bool {
    #[inline(always)]
    fn from(variant: Rxak) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXAK` reader - Receive Acknowledge"]
pub type RxakR = crate::BitReader<Rxak>;
impl RxakR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxak {
        match self.bits {
            false => Rxak::B0,
            true => Rxak::B1,
        }
    }
    #[doc = "Acknowledge signal was received after the completion of one byte of data transmission on the bus"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxak::B0
    }
    #[doc = "No acknowledge signal detected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxak::B1
    }
}
#[doc = "Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iicif {
    #[doc = "0: No interrupt pending"]
    B0 = 0,
    #[doc = "1: Interrupt pending"]
    B1 = 1,
}
impl From<Iicif> for bool {
    #[inline(always)]
    fn from(variant: Iicif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IICIF` reader - Interrupt Flag"]
pub type IicifR = crate::BitReader<Iicif>;
impl IicifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iicif {
        match self.bits {
            false => Iicif::B0,
            true => Iicif::B1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Iicif::B0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Iicif::B1
    }
}
#[doc = "Field `IICIF` writer - Interrupt Flag"]
pub type IicifW<'a, REG> = crate::BitWriter<'a, REG, Iicif>;
impl<'a, REG> IicifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Iicif::B0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Iicif::B1)
    }
}
#[doc = "Slave Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srw {
    #[doc = "0: Slave receive, master writing to slave"]
    B0 = 0,
    #[doc = "1: Slave transmit, master reading from slave"]
    B1 = 1,
}
impl From<Srw> for bool {
    #[inline(always)]
    fn from(variant: Srw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRW` reader - Slave Read/Write"]
pub type SrwR = crate::BitReader<Srw>;
impl SrwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srw {
        match self.bits {
            false => Srw::B0,
            true => Srw::B1,
        }
    }
    #[doc = "Slave receive, master writing to slave"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Srw::B0
    }
    #[doc = "Slave transmit, master reading from slave"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Srw::B1
    }
}
#[doc = "Range Address Match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ram {
    #[doc = "0: Not addressed"]
    B0 = 0,
    #[doc = "1: Addressed as a slave"]
    B1 = 1,
}
impl From<Ram> for bool {
    #[inline(always)]
    fn from(variant: Ram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAM` reader - Range Address Match"]
pub type RamR = crate::BitReader<Ram>;
impl RamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram {
        match self.bits {
            false => Ram::B0,
            true => Ram::B1,
        }
    }
    #[doc = "Not addressed"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ram::B0
    }
    #[doc = "Addressed as a slave"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ram::B1
    }
}
#[doc = "Field `RAM` writer - Range Address Match"]
pub type RamW<'a, REG> = crate::BitWriter<'a, REG, Ram>;
impl<'a, REG> RamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not addressed"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ram::B0)
    }
    #[doc = "Addressed as a slave"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ram::B1)
    }
}
#[doc = "Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arbl {
    #[doc = "0: Standard bus operation."]
    B0 = 0,
    #[doc = "1: Loss of arbitration."]
    B1 = 1,
}
impl From<Arbl> for bool {
    #[inline(always)]
    fn from(variant: Arbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBL` reader - Arbitration Lost"]
pub type ArblR = crate::BitReader<Arbl>;
impl ArblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Arbl {
        match self.bits {
            false => Arbl::B0,
            true => Arbl::B1,
        }
    }
    #[doc = "Standard bus operation."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Arbl::B0
    }
    #[doc = "Loss of arbitration."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Arbl::B1
    }
}
#[doc = "Field `ARBL` writer - Arbitration Lost"]
pub type ArblW<'a, REG> = crate::BitWriter<'a, REG, Arbl>;
impl<'a, REG> ArblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard bus operation."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Arbl::B0)
    }
    #[doc = "Loss of arbitration."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Arbl::B1)
    }
}
#[doc = "Bus Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Busy {
    #[doc = "0: Bus is idle"]
    B0 = 0,
    #[doc = "1: Bus is busy"]
    B1 = 1,
}
impl From<Busy> for bool {
    #[inline(always)]
    fn from(variant: Busy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BUSY` reader - Bus Busy"]
pub type BusyR = crate::BitReader<Busy>;
impl BusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Busy {
        match self.bits {
            false => Busy::B0,
            true => Busy::B1,
        }
    }
    #[doc = "Bus is idle"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Busy::B0
    }
    #[doc = "Bus is busy"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Busy::B1
    }
}
#[doc = "Addressed As A Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iaas {
    #[doc = "0: Not addressed"]
    B0 = 0,
    #[doc = "1: Addressed as a slave"]
    B1 = 1,
}
impl From<Iaas> for bool {
    #[inline(always)]
    fn from(variant: Iaas) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IAAS` reader - Addressed As A Slave"]
pub type IaasR = crate::BitReader<Iaas>;
impl IaasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iaas {
        match self.bits {
            false => Iaas::B0,
            true => Iaas::B1,
        }
    }
    #[doc = "Not addressed"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Iaas::B0
    }
    #[doc = "Addressed as a slave"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Iaas::B1
    }
}
#[doc = "Field `IAAS` writer - Addressed As A Slave"]
pub type IaasW<'a, REG> = crate::BitWriter<'a, REG, Iaas>;
impl<'a, REG> IaasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not addressed"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Iaas::B0)
    }
    #[doc = "Addressed as a slave"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Iaas::B1)
    }
}
#[doc = "Transfer Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcf {
    #[doc = "0: Transfer in progress"]
    B0 = 0,
    #[doc = "1: Transfer complete"]
    B1 = 1,
}
impl From<Tcf> for bool {
    #[inline(always)]
    fn from(variant: Tcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` reader - Transfer Complete Flag"]
pub type TcfR = crate::BitReader<Tcf>;
impl TcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcf {
        match self.bits {
            false => Tcf::B0,
            true => Tcf::B1,
        }
    }
    #[doc = "Transfer in progress"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcf::B0
    }
    #[doc = "Transfer complete"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcf::B1
    }
}
impl R {
    #[doc = "Bit 0 - Receive Acknowledge"]
    #[inline(always)]
    pub fn rxak(&self) -> RxakR {
        RxakR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Flag"]
    #[inline(always)]
    pub fn iicif(&self) -> IicifR {
        IicifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Read/Write"]
    #[inline(always)]
    pub fn srw(&self) -> SrwR {
        SrwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Range Address Match"]
    #[inline(always)]
    pub fn ram(&self) -> RamR {
        RamR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn arbl(&self) -> ArblR {
        ArblR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Addressed As A Slave"]
    #[inline(always)]
    pub fn iaas(&self) -> IaasR {
        IaasR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn iicif(&mut self) -> IicifW<SSpec> {
        IicifW::new(self, 1)
    }
    #[doc = "Bit 3 - Range Address Match"]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RamW<SSpec> {
        RamW::new(self, 3)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    #[must_use]
    pub fn arbl(&mut self) -> ArblW<SSpec> {
        ArblW::new(self, 4)
    }
    #[doc = "Bit 6 - Addressed As A Slave"]
    #[inline(always)]
    #[must_use]
    pub fn iaas(&mut self) -> IaasW<SSpec> {
        IaasW::new(self, 6)
    }
}
#[doc = "I2C Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets S to value 0x80"]
impl crate::Resettable for SSpec {
    const RESET_VALUE: u8 = 0x80;
}
