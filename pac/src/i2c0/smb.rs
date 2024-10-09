#[doc = "Register `SMB` reader"]
pub type R = crate::R<SmbSpec>;
#[doc = "Register `SMB` writer"]
pub type W = crate::W<SmbSpec>;
#[doc = "SHTF2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shtf2ie {
    #[doc = "0: SHTF2 interrupt is disabled"]
    B0 = 0,
    #[doc = "1: SHTF2 interrupt is enabled"]
    B1 = 1,
}
impl From<Shtf2ie> for bool {
    #[inline(always)]
    fn from(variant: Shtf2ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTF2IE` reader - SHTF2 Interrupt Enable"]
pub type Shtf2ieR = crate::BitReader<Shtf2ie>;
impl Shtf2ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shtf2ie {
        match self.bits {
            false => Shtf2ie::B0,
            true => Shtf2ie::B1,
        }
    }
    #[doc = "SHTF2 interrupt is disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Shtf2ie::B0
    }
    #[doc = "SHTF2 interrupt is enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Shtf2ie::B1
    }
}
#[doc = "Field `SHTF2IE` writer - SHTF2 Interrupt Enable"]
pub type Shtf2ieW<'a, REG> = crate::BitWriter<'a, REG, Shtf2ie>;
impl<'a, REG> Shtf2ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SHTF2 interrupt is disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Shtf2ie::B0)
    }
    #[doc = "SHTF2 interrupt is enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Shtf2ie::B1)
    }
}
#[doc = "SCL High Timeout Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shtf2 {
    #[doc = "0: No SCL high and SDA low timeout occurs"]
    B0 = 0,
    #[doc = "1: SCL high and SDA low timeout occurs"]
    B1 = 1,
}
impl From<Shtf2> for bool {
    #[inline(always)]
    fn from(variant: Shtf2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTF2` reader - SCL High Timeout Flag 2"]
pub type Shtf2R = crate::BitReader<Shtf2>;
impl Shtf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shtf2 {
        match self.bits {
            false => Shtf2::B0,
            true => Shtf2::B1,
        }
    }
    #[doc = "No SCL high and SDA low timeout occurs"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Shtf2::B0
    }
    #[doc = "SCL high and SDA low timeout occurs"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Shtf2::B1
    }
}
#[doc = "Field `SHTF2` writer - SCL High Timeout Flag 2"]
pub type Shtf2W<'a, REG> = crate::BitWriter<'a, REG, Shtf2>;
impl<'a, REG> Shtf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No SCL high and SDA low timeout occurs"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Shtf2::B0)
    }
    #[doc = "SCL high and SDA low timeout occurs"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Shtf2::B1)
    }
}
#[doc = "SCL High Timeout Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shtf1 {
    #[doc = "0: No SCL high and SDA high timeout occurs"]
    B0 = 0,
    #[doc = "1: SCL high and SDA high timeout occurs"]
    B1 = 1,
}
impl From<Shtf1> for bool {
    #[inline(always)]
    fn from(variant: Shtf1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHTF1` reader - SCL High Timeout Flag 1"]
pub type Shtf1R = crate::BitReader<Shtf1>;
impl Shtf1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shtf1 {
        match self.bits {
            false => Shtf1::B0,
            true => Shtf1::B1,
        }
    }
    #[doc = "No SCL high and SDA high timeout occurs"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Shtf1::B0
    }
    #[doc = "SCL high and SDA high timeout occurs"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Shtf1::B1
    }
}
#[doc = "SCL Low Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sltf {
    #[doc = "0: No low timeout occurs"]
    B0 = 0,
    #[doc = "1: Low timeout occurs"]
    B1 = 1,
}
impl From<Sltf> for bool {
    #[inline(always)]
    fn from(variant: Sltf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLTF` reader - SCL Low Timeout Flag"]
pub type SltfR = crate::BitReader<Sltf>;
impl SltfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sltf {
        match self.bits {
            false => Sltf::B0,
            true => Sltf::B1,
        }
    }
    #[doc = "No low timeout occurs"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sltf::B0
    }
    #[doc = "Low timeout occurs"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sltf::B1
    }
}
#[doc = "Field `SLTF` writer - SCL Low Timeout Flag"]
pub type SltfW<'a, REG> = crate::BitWriter<'a, REG, Sltf>;
impl<'a, REG> SltfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No low timeout occurs"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sltf::B0)
    }
    #[doc = "Low timeout occurs"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sltf::B1)
    }
}
#[doc = "Timeout Counter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcksel {
    #[doc = "0: Timeout counter counts at the frequency of the I2C module clock / 64"]
    B0 = 0,
    #[doc = "1: Timeout counter counts at the frequency of the I2C module clock"]
    B1 = 1,
}
impl From<Tcksel> for bool {
    #[inline(always)]
    fn from(variant: Tcksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCKSEL` reader - Timeout Counter Clock Select"]
pub type TckselR = crate::BitReader<Tcksel>;
impl TckselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcksel {
        match self.bits {
            false => Tcksel::B0,
            true => Tcksel::B1,
        }
    }
    #[doc = "Timeout counter counts at the frequency of the I2C module clock / 64"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcksel::B0
    }
    #[doc = "Timeout counter counts at the frequency of the I2C module clock"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcksel::B1
    }
}
#[doc = "Field `TCKSEL` writer - Timeout Counter Clock Select"]
pub type TckselW<'a, REG> = crate::BitWriter<'a, REG, Tcksel>;
impl<'a, REG> TckselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timeout counter counts at the frequency of the I2C module clock / 64"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcksel::B0)
    }
    #[doc = "Timeout counter counts at the frequency of the I2C module clock"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcksel::B1)
    }
}
#[doc = "Second I2C Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Siicaen {
    #[doc = "0: I2C address register 2 matching is disabled"]
    B0 = 0,
    #[doc = "1: I2C address register 2 matching is enabled"]
    B1 = 1,
}
impl From<Siicaen> for bool {
    #[inline(always)]
    fn from(variant: Siicaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIICAEN` reader - Second I2C Address Enable"]
pub type SiicaenR = crate::BitReader<Siicaen>;
impl SiicaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Siicaen {
        match self.bits {
            false => Siicaen::B0,
            true => Siicaen::B1,
        }
    }
    #[doc = "I2C address register 2 matching is disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Siicaen::B0
    }
    #[doc = "I2C address register 2 matching is enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Siicaen::B1
    }
}
#[doc = "Field `SIICAEN` writer - Second I2C Address Enable"]
pub type SiicaenW<'a, REG> = crate::BitWriter<'a, REG, Siicaen>;
impl<'a, REG> SiicaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C address register 2 matching is disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Siicaen::B0)
    }
    #[doc = "I2C address register 2 matching is enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Siicaen::B1)
    }
}
#[doc = "SMBus Alert Response Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alerten {
    #[doc = "0: SMBus alert response address matching is disabled"]
    B0 = 0,
    #[doc = "1: SMBus alert response address matching is enabled"]
    B1 = 1,
}
impl From<Alerten> for bool {
    #[inline(always)]
    fn from(variant: Alerten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTEN` reader - SMBus Alert Response Address Enable"]
pub type AlertenR = crate::BitReader<Alerten>;
impl AlertenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alerten {
        match self.bits {
            false => Alerten::B0,
            true => Alerten::B1,
        }
    }
    #[doc = "SMBus alert response address matching is disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Alerten::B0
    }
    #[doc = "SMBus alert response address matching is enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Alerten::B1
    }
}
#[doc = "Field `ALERTEN` writer - SMBus Alert Response Address Enable"]
pub type AlertenW<'a, REG> = crate::BitWriter<'a, REG, Alerten>;
impl<'a, REG> AlertenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBus alert response address matching is disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Alerten::B0)
    }
    #[doc = "SMBus alert response address matching is enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Alerten::B1)
    }
}
#[doc = "Fast NACK/ACK Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fack {
    #[doc = "0: An ACK or NACK is sent on the following receiving data byte"]
    B0 = 0,
    #[doc = "1: Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    B1 = 1,
}
impl From<Fack> for bool {
    #[inline(always)]
    fn from(variant: Fack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FACK` reader - Fast NACK/ACK Enable"]
pub type FackR = crate::BitReader<Fack>;
impl FackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fack {
        match self.bits {
            false => Fack::B0,
            true => Fack::B1,
        }
    }
    #[doc = "An ACK or NACK is sent on the following receiving data byte"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fack::B0
    }
    #[doc = "Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fack::B1
    }
}
#[doc = "Field `FACK` writer - Fast NACK/ACK Enable"]
pub type FackW<'a, REG> = crate::BitWriter<'a, REG, Fack>;
impl<'a, REG> FackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "An ACK or NACK is sent on the following receiving data byte"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fack::B0)
    }
    #[doc = "Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fack::B1)
    }
}
impl R {
    #[doc = "Bit 0 - SHTF2 Interrupt Enable"]
    #[inline(always)]
    pub fn shtf2ie(&self) -> Shtf2ieR {
        Shtf2ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL High Timeout Flag 2"]
    #[inline(always)]
    pub fn shtf2(&self) -> Shtf2R {
        Shtf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCL High Timeout Flag 1"]
    #[inline(always)]
    pub fn shtf1(&self) -> Shtf1R {
        Shtf1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCL Low Timeout Flag"]
    #[inline(always)]
    pub fn sltf(&self) -> SltfR {
        SltfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout Counter Clock Select"]
    #[inline(always)]
    pub fn tcksel(&self) -> TckselR {
        TckselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Second I2C Address Enable"]
    #[inline(always)]
    pub fn siicaen(&self) -> SiicaenR {
        SiicaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus Alert Response Address Enable"]
    #[inline(always)]
    pub fn alerten(&self) -> AlertenR {
        AlertenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast NACK/ACK Enable"]
    #[inline(always)]
    pub fn fack(&self) -> FackR {
        FackR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHTF2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shtf2ie(&mut self) -> Shtf2ieW<SmbSpec> {
        Shtf2ieW::new(self, 0)
    }
    #[doc = "Bit 1 - SCL High Timeout Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn shtf2(&mut self) -> Shtf2W<SmbSpec> {
        Shtf2W::new(self, 1)
    }
    #[doc = "Bit 3 - SCL Low Timeout Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sltf(&mut self) -> SltfW<SmbSpec> {
        SltfW::new(self, 3)
    }
    #[doc = "Bit 4 - Timeout Counter Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn tcksel(&mut self) -> TckselW<SmbSpec> {
        TckselW::new(self, 4)
    }
    #[doc = "Bit 5 - Second I2C Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn siicaen(&mut self) -> SiicaenW<SmbSpec> {
        SiicaenW::new(self, 5)
    }
    #[doc = "Bit 6 - SMBus Alert Response Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alerten(&mut self) -> AlertenW<SmbSpec> {
        AlertenW::new(self, 6)
    }
    #[doc = "Bit 7 - Fast NACK/ACK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fack(&mut self) -> FackW<SmbSpec> {
        FackW::new(self, 7)
    }
}
#[doc = "I2C SMBus Control and Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`smb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmbSpec;
impl crate::RegisterSpec for SmbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`smb::R`](R) reader structure"]
impl crate::Readable for SmbSpec {}
#[doc = "`write(|w| ..)` method takes [`smb::W`](W) writer structure"]
impl crate::Writable for SmbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SMB to value 0"]
impl crate::Resettable for SmbSpec {
    const RESET_VALUE: u8 = 0;
}
