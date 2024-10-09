#[doc = "Register `TCR2` reader"]
pub type R = crate::R<Tcr2Spec>;
#[doc = "Register `TCR2` writer"]
pub type W = crate::W<Tcr2Spec>;
#[doc = "Field `DIV` reader - Bit Clock Divide"]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Bit Clock Divide"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Bit Clock Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcd {
    #[doc = "0: Bit clock is generated externally in Slave mode."]
    B0 = 0,
    #[doc = "1: Bit clock is generated internally in Master mode."]
    B1 = 1,
}
impl From<Bcd> for bool {
    #[inline(always)]
    fn from(variant: Bcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCD` reader - Bit Clock Direction"]
pub type BcdR = crate::BitReader<Bcd>;
impl BcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcd {
        match self.bits {
            false => Bcd::B0,
            true => Bcd::B1,
        }
    }
    #[doc = "Bit clock is generated externally in Slave mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bcd::B0
    }
    #[doc = "Bit clock is generated internally in Master mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bcd::B1
    }
}
#[doc = "Field `BCD` writer - Bit Clock Direction"]
pub type BcdW<'a, REG> = crate::BitWriter<'a, REG, Bcd>;
impl<'a, REG> BcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit clock is generated externally in Slave mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bcd::B0)
    }
    #[doc = "Bit clock is generated internally in Master mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcd::B1)
    }
}
#[doc = "Bit Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcp {
    #[doc = "0: Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    B0 = 0,
    #[doc = "1: Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    B1 = 1,
}
impl From<Bcp> for bool {
    #[inline(always)]
    fn from(variant: Bcp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCP` reader - Bit Clock Polarity"]
pub type BcpR = crate::BitReader<Bcp>;
impl BcpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcp {
        match self.bits {
            false => Bcp::B0,
            true => Bcp::B1,
        }
    }
    #[doc = "Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bcp::B0
    }
    #[doc = "Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bcp::B1
    }
}
#[doc = "Field `BCP` writer - Bit Clock Polarity"]
pub type BcpW<'a, REG> = crate::BitWriter<'a, REG, Bcp>;
impl<'a, REG> BcpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bit clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bcp::B0)
    }
    #[doc = "Bit clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcp::B1)
    }
}
#[doc = "MCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msel {
    #[doc = "0: Bus Clock selected."]
    B00 = 0,
    #[doc = "1: Master Clock (MCLK) 1 option selected."]
    B01 = 1,
    #[doc = "2: Master Clock (MCLK) 2 option selected."]
    B10 = 2,
    #[doc = "3: Master Clock (MCLK) 3 option selected."]
    B11 = 3,
}
impl From<Msel> for u8 {
    #[inline(always)]
    fn from(variant: Msel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msel {
    type Ux = u8;
}
impl crate::IsEnum for Msel {}
#[doc = "Field `MSEL` reader - MCLK Select"]
pub type MselR = crate::FieldReader<Msel>;
impl MselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msel {
        match self.bits {
            0 => Msel::B00,
            1 => Msel::B01,
            2 => Msel::B10,
            3 => Msel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Bus Clock selected."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Msel::B00
    }
    #[doc = "Master Clock (MCLK) 1 option selected."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Msel::B01
    }
    #[doc = "Master Clock (MCLK) 2 option selected."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Msel::B10
    }
    #[doc = "Master Clock (MCLK) 3 option selected."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Msel::B11
    }
}
#[doc = "Field `MSEL` writer - MCLK Select"]
pub type MselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Msel, crate::Safe>;
impl<'a, REG> MselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bus Clock selected."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B00)
    }
    #[doc = "Master Clock (MCLK) 1 option selected."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B01)
    }
    #[doc = "Master Clock (MCLK) 2 option selected."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B10)
    }
    #[doc = "Master Clock (MCLK) 3 option selected."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B11)
    }
}
#[doc = "Bit Clock Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bci {
    #[doc = "0: No effect."]
    B0 = 0,
    #[doc = "1: Internal logic is clocked as if bit clock was externally generated."]
    B1 = 1,
}
impl From<Bci> for bool {
    #[inline(always)]
    fn from(variant: Bci) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCI` reader - Bit Clock Input"]
pub type BciR = crate::BitReader<Bci>;
impl BciR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bci {
        match self.bits {
            false => Bci::B0,
            true => Bci::B1,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bci::B0
    }
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bci::B1
    }
}
#[doc = "Field `BCI` writer - Bit Clock Input"]
pub type BciW<'a, REG> = crate::BitWriter<'a, REG, Bci>;
impl<'a, REG> BciW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bci::B0)
    }
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bci::B1)
    }
}
#[doc = "Bit Clock Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bcs {
    #[doc = "0: Use the normal bit clock source."]
    B0 = 0,
    #[doc = "1: Swap the bit clock source."]
    B1 = 1,
}
impl From<Bcs> for bool {
    #[inline(always)]
    fn from(variant: Bcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCS` reader - Bit Clock Swap"]
pub type BcsR = crate::BitReader<Bcs>;
impl BcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bcs {
        match self.bits {
            false => Bcs::B0,
            true => Bcs::B1,
        }
    }
    #[doc = "Use the normal bit clock source."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bcs::B0
    }
    #[doc = "Swap the bit clock source."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bcs::B1
    }
}
#[doc = "Field `BCS` writer - Bit Clock Swap"]
pub type BcsW<'a, REG> = crate::BitWriter<'a, REG, Bcs>;
impl<'a, REG> BcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use the normal bit clock source."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bcs::B0)
    }
    #[doc = "Swap the bit clock source."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bcs::B1)
    }
}
#[doc = "Synchronous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sync {
    #[doc = "0: Asynchronous mode."]
    B00 = 0,
    #[doc = "1: Synchronous with receiver."]
    B01 = 1,
    #[doc = "2: Synchronous with another SAI transmitter."]
    B10 = 2,
    #[doc = "3: Synchronous with another SAI receiver."]
    B11 = 3,
}
impl From<Sync> for u8 {
    #[inline(always)]
    fn from(variant: Sync) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sync {
    type Ux = u8;
}
impl crate::IsEnum for Sync {}
#[doc = "Field `SYNC` reader - Synchronous Mode"]
pub type SyncR = crate::FieldReader<Sync>;
impl SyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync {
        match self.bits {
            0 => Sync::B00,
            1 => Sync::B01,
            2 => Sync::B10,
            3 => Sync::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Asynchronous mode."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Sync::B00
    }
    #[doc = "Synchronous with receiver."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Sync::B01
    }
    #[doc = "Synchronous with another SAI transmitter."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Sync::B10
    }
    #[doc = "Synchronous with another SAI receiver."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Sync::B11
    }
}
#[doc = "Field `SYNC` writer - Synchronous Mode"]
pub type SyncW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sync, crate::Safe>;
impl<'a, REG> SyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Asynchronous mode."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::B00)
    }
    #[doc = "Synchronous with receiver."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::B01)
    }
    #[doc = "Synchronous with another SAI transmitter."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::B10)
    }
    #[doc = "Synchronous with another SAI receiver."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::B11)
    }
}
impl R {
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline(always)]
    pub fn bcd(&self) -> BcdR {
        BcdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline(always)]
    pub fn bcp(&self) -> BcpR {
        BcpR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline(always)]
    pub fn msel(&self) -> MselR {
        MselR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline(always)]
    pub fn bci(&self) -> BciR {
        BciR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline(always)]
    pub fn bcs(&self) -> BcsR {
        BcsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<Tcr2Spec> {
        DivW::new(self, 0)
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline(always)]
    #[must_use]
    pub fn bcd(&mut self) -> BcdW<Tcr2Spec> {
        BcdW::new(self, 24)
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bcp(&mut self) -> BcpW<Tcr2Spec> {
        BcpW::new(self, 25)
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline(always)]
    #[must_use]
    pub fn msel(&mut self) -> MselW<Tcr2Spec> {
        MselW::new(self, 26)
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline(always)]
    #[must_use]
    pub fn bci(&mut self) -> BciW<Tcr2Spec> {
        BciW::new(self, 28)
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline(always)]
    #[must_use]
    pub fn bcs(&mut self) -> BcsW<Tcr2Spec> {
        BcsW::new(self, 29)
    }
    #[doc = "Bits 30:31 - Synchronous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SyncW<Tcr2Spec> {
        SyncW::new(self, 30)
    }
}
#[doc = "SAI Transmit Configuration 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tcr2Spec;
impl crate::RegisterSpec for Tcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr2::R`](R) reader structure"]
impl crate::Readable for Tcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`tcr2::W`](W) writer structure"]
impl crate::Writable for Tcr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCR2 to value 0"]
impl crate::Resettable for Tcr2Spec {
    const RESET_VALUE: u32 = 0;
}
