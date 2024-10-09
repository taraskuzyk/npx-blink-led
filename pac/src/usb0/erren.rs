#[doc = "Register `ERREN` reader"]
pub type R = crate::R<ErrenSpec>;
#[doc = "Register `ERREN` writer"]
pub type W = crate::W<ErrenSpec>;
#[doc = "PIDERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Piderren {
    #[doc = "0: Disables the PIDERR interrupt."]
    B0 = 0,
    #[doc = "1: Enters the PIDERR interrupt."]
    B1 = 1,
}
impl From<Piderren> for bool {
    #[inline(always)]
    fn from(variant: Piderren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIDERREN` reader - PIDERR Interrupt Enable"]
pub type PiderrenR = crate::BitReader<Piderren>;
impl PiderrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Piderren {
        match self.bits {
            false => Piderren::B0,
            true => Piderren::B1,
        }
    }
    #[doc = "Disables the PIDERR interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Piderren::B0
    }
    #[doc = "Enters the PIDERR interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Piderren::B1
    }
}
#[doc = "Field `PIDERREN` writer - PIDERR Interrupt Enable"]
pub type PiderrenW<'a, REG> = crate::BitWriter<'a, REG, Piderren>;
impl<'a, REG> PiderrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the PIDERR interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Piderren::B0)
    }
    #[doc = "Enters the PIDERR interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Piderren::B1)
    }
}
#[doc = "CRC5/EOF Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crc5eofen {
    #[doc = "0: Disables the CRC5/EOF interrupt."]
    B0 = 0,
    #[doc = "1: Enables the CRC5/EOF interrupt."]
    B1 = 1,
}
impl From<Crc5eofen> for bool {
    #[inline(always)]
    fn from(variant: Crc5eofen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC5EOFEN` reader - CRC5/EOF Interrupt Enable"]
pub type Crc5eofenR = crate::BitReader<Crc5eofen>;
impl Crc5eofenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crc5eofen {
        match self.bits {
            false => Crc5eofen::B0,
            true => Crc5eofen::B1,
        }
    }
    #[doc = "Disables the CRC5/EOF interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Crc5eofen::B0
    }
    #[doc = "Enables the CRC5/EOF interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Crc5eofen::B1
    }
}
#[doc = "Field `CRC5EOFEN` writer - CRC5/EOF Interrupt Enable"]
pub type Crc5eofenW<'a, REG> = crate::BitWriter<'a, REG, Crc5eofen>;
impl<'a, REG> Crc5eofenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the CRC5/EOF interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Crc5eofen::B0)
    }
    #[doc = "Enables the CRC5/EOF interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Crc5eofen::B1)
    }
}
#[doc = "CRC16 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crc16en {
    #[doc = "0: Disables the CRC16 interrupt."]
    B0 = 0,
    #[doc = "1: Enables the CRC16 interrupt."]
    B1 = 1,
}
impl From<Crc16en> for bool {
    #[inline(always)]
    fn from(variant: Crc16en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC16EN` reader - CRC16 Interrupt Enable"]
pub type Crc16enR = crate::BitReader<Crc16en>;
impl Crc16enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crc16en {
        match self.bits {
            false => Crc16en::B0,
            true => Crc16en::B1,
        }
    }
    #[doc = "Disables the CRC16 interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Crc16en::B0
    }
    #[doc = "Enables the CRC16 interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Crc16en::B1
    }
}
#[doc = "Field `CRC16EN` writer - CRC16 Interrupt Enable"]
pub type Crc16enW<'a, REG> = crate::BitWriter<'a, REG, Crc16en>;
impl<'a, REG> Crc16enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the CRC16 interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Crc16en::B0)
    }
    #[doc = "Enables the CRC16 interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Crc16en::B1)
    }
}
#[doc = "DFN8 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dfn8en {
    #[doc = "0: Disables the DFN8 interrupt."]
    B0 = 0,
    #[doc = "1: Enables the DFN8 interrupt."]
    B1 = 1,
}
impl From<Dfn8en> for bool {
    #[inline(always)]
    fn from(variant: Dfn8en) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DFN8EN` reader - DFN8 Interrupt Enable"]
pub type Dfn8enR = crate::BitReader<Dfn8en>;
impl Dfn8enR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dfn8en {
        match self.bits {
            false => Dfn8en::B0,
            true => Dfn8en::B1,
        }
    }
    #[doc = "Disables the DFN8 interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dfn8en::B0
    }
    #[doc = "Enables the DFN8 interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dfn8en::B1
    }
}
#[doc = "Field `DFN8EN` writer - DFN8 Interrupt Enable"]
pub type Dfn8enW<'a, REG> = crate::BitWriter<'a, REG, Dfn8en>;
impl<'a, REG> Dfn8enW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the DFN8 interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dfn8en::B0)
    }
    #[doc = "Enables the DFN8 interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dfn8en::B1)
    }
}
#[doc = "BTOERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Btoerren {
    #[doc = "0: Disables the BTOERR interrupt."]
    B0 = 0,
    #[doc = "1: Enables the BTOERR interrupt."]
    B1 = 1,
}
impl From<Btoerren> for bool {
    #[inline(always)]
    fn from(variant: Btoerren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTOERREN` reader - BTOERR Interrupt Enable"]
pub type BtoerrenR = crate::BitReader<Btoerren>;
impl BtoerrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Btoerren {
        match self.bits {
            false => Btoerren::B0,
            true => Btoerren::B1,
        }
    }
    #[doc = "Disables the BTOERR interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Btoerren::B0
    }
    #[doc = "Enables the BTOERR interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Btoerren::B1
    }
}
#[doc = "Field `BTOERREN` writer - BTOERR Interrupt Enable"]
pub type BtoerrenW<'a, REG> = crate::BitWriter<'a, REG, Btoerren>;
impl<'a, REG> BtoerrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the BTOERR interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Btoerren::B0)
    }
    #[doc = "Enables the BTOERR interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Btoerren::B1)
    }
}
#[doc = "DMAERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaerren {
    #[doc = "0: Disables the DMAERR interrupt."]
    B0 = 0,
    #[doc = "1: Enables the DMAERR interrupt."]
    B1 = 1,
}
impl From<Dmaerren> for bool {
    #[inline(always)]
    fn from(variant: Dmaerren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAERREN` reader - DMAERR Interrupt Enable"]
pub type DmaerrenR = crate::BitReader<Dmaerren>;
impl DmaerrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaerren {
        match self.bits {
            false => Dmaerren::B0,
            true => Dmaerren::B1,
        }
    }
    #[doc = "Disables the DMAERR interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dmaerren::B0
    }
    #[doc = "Enables the DMAERR interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dmaerren::B1
    }
}
#[doc = "Field `DMAERREN` writer - DMAERR Interrupt Enable"]
pub type DmaerrenW<'a, REG> = crate::BitWriter<'a, REG, Dmaerren>;
impl<'a, REG> DmaerrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the DMAERR interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaerren::B0)
    }
    #[doc = "Enables the DMAERR interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaerren::B1)
    }
}
#[doc = "BTSERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Btserren {
    #[doc = "0: Disables the BTSERR interrupt."]
    B0 = 0,
    #[doc = "1: Enables the BTSERR interrupt."]
    B1 = 1,
}
impl From<Btserren> for bool {
    #[inline(always)]
    fn from(variant: Btserren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BTSERREN` reader - BTSERR Interrupt Enable"]
pub type BtserrenR = crate::BitReader<Btserren>;
impl BtserrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Btserren {
        match self.bits {
            false => Btserren::B0,
            true => Btserren::B1,
        }
    }
    #[doc = "Disables the BTSERR interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Btserren::B0
    }
    #[doc = "Enables the BTSERR interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Btserren::B1
    }
}
#[doc = "Field `BTSERREN` writer - BTSERR Interrupt Enable"]
pub type BtserrenW<'a, REG> = crate::BitWriter<'a, REG, Btserren>;
impl<'a, REG> BtserrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the BTSERR interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Btserren::B0)
    }
    #[doc = "Enables the BTSERR interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Btserren::B1)
    }
}
impl R {
    #[doc = "Bit 0 - PIDERR Interrupt Enable"]
    #[inline(always)]
    pub fn piderren(&self) -> PiderrenR {
        PiderrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC5/EOF Interrupt Enable"]
    #[inline(always)]
    pub fn crc5eofen(&self) -> Crc5eofenR {
        Crc5eofenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC16 Interrupt Enable"]
    #[inline(always)]
    pub fn crc16en(&self) -> Crc16enR {
        Crc16enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DFN8 Interrupt Enable"]
    #[inline(always)]
    pub fn dfn8en(&self) -> Dfn8enR {
        Dfn8enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BTOERR Interrupt Enable"]
    #[inline(always)]
    pub fn btoerren(&self) -> BtoerrenR {
        BtoerrenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAERR Interrupt Enable"]
    #[inline(always)]
    pub fn dmaerren(&self) -> DmaerrenR {
        DmaerrenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - BTSERR Interrupt Enable"]
    #[inline(always)]
    pub fn btserren(&self) -> BtserrenR {
        BtserrenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PIDERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn piderren(&mut self) -> PiderrenW<ErrenSpec> {
        PiderrenW::new(self, 0)
    }
    #[doc = "Bit 1 - CRC5/EOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crc5eofen(&mut self) -> Crc5eofenW<ErrenSpec> {
        Crc5eofenW::new(self, 1)
    }
    #[doc = "Bit 2 - CRC16 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crc16en(&mut self) -> Crc16enW<ErrenSpec> {
        Crc16enW::new(self, 2)
    }
    #[doc = "Bit 3 - DFN8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfn8en(&mut self) -> Dfn8enW<ErrenSpec> {
        Dfn8enW::new(self, 3)
    }
    #[doc = "Bit 4 - BTOERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn btoerren(&mut self) -> BtoerrenW<ErrenSpec> {
        BtoerrenW::new(self, 4)
    }
    #[doc = "Bit 5 - DMAERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaerren(&mut self) -> DmaerrenW<ErrenSpec> {
        DmaerrenW::new(self, 5)
    }
    #[doc = "Bit 7 - BTSERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn btserren(&mut self) -> BtserrenW<ErrenSpec> {
        BtserrenW::new(self, 7)
    }
}
#[doc = "Error Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`erren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErrenSpec;
impl crate::RegisterSpec for ErrenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`erren::R`](R) reader structure"]
impl crate::Readable for ErrenSpec {}
#[doc = "`write(|w| ..)` method takes [`erren::W`](W) writer structure"]
impl crate::Writable for ErrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ERREN to value 0"]
impl crate::Resettable for ErrenSpec {
    const RESET_VALUE: u8 = 0;
}
