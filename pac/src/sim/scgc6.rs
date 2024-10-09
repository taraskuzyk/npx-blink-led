#[doc = "Register `SCGC6` reader"]
pub type R = crate::R<Scgc6Spec>;
#[doc = "Register `SCGC6` writer"]
pub type W = crate::W<Scgc6Spec>;
#[doc = "Flash Memory Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ftf {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Ftf> for bool {
    #[inline(always)]
    fn from(variant: Ftf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FTF` reader - Flash Memory Clock Gate Control"]
pub type FtfR = crate::BitReader<Ftf>;
impl FtfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ftf {
        match self.bits {
            false => Ftf::B0,
            true => Ftf::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ftf::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ftf::B1
    }
}
#[doc = "Field `FTF` writer - Flash Memory Clock Gate Control"]
pub type FtfW<'a, REG> = crate::BitWriter<'a, REG, Ftf>;
impl<'a, REG> FtfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ftf::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ftf::B1)
    }
}
#[doc = "DMA Mux Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmamux {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Dmamux> for bool {
    #[inline(always)]
    fn from(variant: Dmamux) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMUX` reader - DMA Mux Clock Gate Control"]
pub type DmamuxR = crate::BitReader<Dmamux>;
impl DmamuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmamux {
        match self.bits {
            false => Dmamux::B0,
            true => Dmamux::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dmamux::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dmamux::B1
    }
}
#[doc = "Field `DMAMUX` writer - DMA Mux Clock Gate Control"]
pub type DmamuxW<'a, REG> = crate::BitWriter<'a, REG, Dmamux>;
impl<'a, REG> DmamuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamux::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamux::B1)
    }
}
#[doc = "I2S Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2s {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<I2s> for bool {
    #[inline(always)]
    fn from(variant: I2s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2S` reader - I2S Clock Gate Control"]
pub type I2sR = crate::BitReader<I2s>;
impl I2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2s {
        match self.bits {
            false => I2s::B0,
            true => I2s::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == I2s::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == I2s::B1
    }
}
#[doc = "Field `I2S` writer - I2S Clock Gate Control"]
pub type I2sW<'a, REG> = crate::BitWriter<'a, REG, I2s>;
impl<'a, REG> I2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(I2s::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(I2s::B1)
    }
}
#[doc = "PIT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pit {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Pit> for bool {
    #[inline(always)]
    fn from(variant: Pit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIT` reader - PIT Clock Gate Control"]
pub type PitR = crate::BitReader<Pit>;
impl PitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pit {
        match self.bits {
            false => Pit::B0,
            true => Pit::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pit::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pit::B1
    }
}
#[doc = "Field `PIT` writer - PIT Clock Gate Control"]
pub type PitW<'a, REG> = crate::BitWriter<'a, REG, Pit>;
impl<'a, REG> PitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pit::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pit::B1)
    }
}
#[doc = "TPM0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm0 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Tpm0> for bool {
    #[inline(always)]
    fn from(variant: Tpm0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM0` reader - TPM0 Clock Gate Control"]
pub type Tpm0R = crate::BitReader<Tpm0>;
impl Tpm0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpm0 {
        match self.bits {
            false => Tpm0::B0,
            true => Tpm0::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tpm0::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tpm0::B1
    }
}
#[doc = "Field `TPM0` writer - TPM0 Clock Gate Control"]
pub type Tpm0W<'a, REG> = crate::BitWriter<'a, REG, Tpm0>;
impl<'a, REG> Tpm0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm0::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm0::B1)
    }
}
#[doc = "TPM1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm1 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Tpm1> for bool {
    #[inline(always)]
    fn from(variant: Tpm1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM1` reader - TPM1 Clock Gate Control"]
pub type Tpm1R = crate::BitReader<Tpm1>;
impl Tpm1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpm1 {
        match self.bits {
            false => Tpm1::B0,
            true => Tpm1::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tpm1::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tpm1::B1
    }
}
#[doc = "Field `TPM1` writer - TPM1 Clock Gate Control"]
pub type Tpm1W<'a, REG> = crate::BitWriter<'a, REG, Tpm1>;
impl<'a, REG> Tpm1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm1::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm1::B1)
    }
}
#[doc = "TPM2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm2 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Tpm2> for bool {
    #[inline(always)]
    fn from(variant: Tpm2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM2` reader - TPM2 Clock Gate Control"]
pub type Tpm2R = crate::BitReader<Tpm2>;
impl Tpm2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpm2 {
        match self.bits {
            false => Tpm2::B0,
            true => Tpm2::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tpm2::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tpm2::B1
    }
}
#[doc = "Field `TPM2` writer - TPM2 Clock Gate Control"]
pub type Tpm2W<'a, REG> = crate::BitWriter<'a, REG, Tpm2>;
impl<'a, REG> Tpm2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm2::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm2::B1)
    }
}
#[doc = "ADC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Adc0> for bool {
    #[inline(always)]
    fn from(variant: Adc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0` reader - ADC0 Clock Gate Control"]
pub type Adc0R = crate::BitReader<Adc0>;
impl Adc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0 {
        match self.bits {
            false => Adc0::B0,
            true => Adc0::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adc0::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adc0::B1
    }
}
#[doc = "Field `ADC0` writer - ADC0 Clock Gate Control"]
pub type Adc0W<'a, REG> = crate::BitWriter<'a, REG, Adc0>;
impl<'a, REG> Adc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0::B1)
    }
}
#[doc = "RTC Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtc {
    #[doc = "0: Access and interrupts disabled"]
    B0 = 0,
    #[doc = "1: Access and interrupts enabled"]
    B1 = 1,
}
impl From<Rtc> for bool {
    #[inline(always)]
    fn from(variant: Rtc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTC` reader - RTC Access Control"]
pub type RtcR = crate::BitReader<Rtc>;
impl RtcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtc {
        match self.bits {
            false => Rtc::B0,
            true => Rtc::B1,
        }
    }
    #[doc = "Access and interrupts disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rtc::B0
    }
    #[doc = "Access and interrupts enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rtc::B1
    }
}
#[doc = "Field `RTC` writer - RTC Access Control"]
pub type RtcW<'a, REG> = crate::BitWriter<'a, REG, Rtc>;
impl<'a, REG> RtcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access and interrupts disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::B0)
    }
    #[doc = "Access and interrupts enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtc::B1)
    }
}
#[doc = "DAC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dac0 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Dac0> for bool {
    #[inline(always)]
    fn from(variant: Dac0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DAC0` reader - DAC0 Clock Gate Control"]
pub type Dac0R = crate::BitReader<Dac0>;
impl Dac0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dac0 {
        match self.bits {
            false => Dac0::B0,
            true => Dac0::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dac0::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dac0::B1
    }
}
#[doc = "Field `DAC0` writer - DAC0 Clock Gate Control"]
pub type Dac0W<'a, REG> = crate::BitWriter<'a, REG, Dac0>;
impl<'a, REG> Dac0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dac0::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dac0::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    pub fn ftf(&self) -> FtfR {
        FtfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    pub fn dmamux(&self) -> DmamuxR {
        DmamuxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 15 - I2S Clock Gate Control"]
    #[inline(always)]
    pub fn i2s(&self) -> I2sR {
        I2sR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&self) -> PitR {
        PitR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - TPM0 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm0(&self) -> Tpm0R {
        Tpm0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TPM1 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm1(&self) -> Tpm1R {
        Tpm1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TPM2 Clock Gate Control"]
    #[inline(always)]
    pub fn tpm2(&self) -> Tpm2R {
        Tpm2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    pub fn adc0(&self) -> Adc0R {
        Adc0R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC Access Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RtcR {
        RtcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&self) -> Dac0R {
        Dac0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FtfW<Scgc6Spec> {
        FtfW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux(&mut self) -> DmamuxW<Scgc6Spec> {
        DmamuxW::new(self, 1)
    }
    #[doc = "Bit 15 - I2S Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn i2s(&mut self) -> I2sW<Scgc6Spec> {
        I2sW::new(self, 15)
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn pit(&mut self) -> PitW<Scgc6Spec> {
        PitW::new(self, 23)
    }
    #[doc = "Bit 24 - TPM0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn tpm0(&mut self) -> Tpm0W<Scgc6Spec> {
        Tpm0W::new(self, 24)
    }
    #[doc = "Bit 25 - TPM1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn tpm1(&mut self) -> Tpm1W<Scgc6Spec> {
        Tpm1W::new(self, 25)
    }
    #[doc = "Bit 26 - TPM2 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn tpm2(&mut self) -> Tpm2W<Scgc6Spec> {
        Tpm2W::new(self, 26)
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> Adc0W<Scgc6Spec> {
        Adc0W::new(self, 27)
    }
    #[doc = "Bit 29 - RTC Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RtcW<Scgc6Spec> {
        RtcW::new(self, 29)
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dac0(&mut self) -> Dac0W<Scgc6Spec> {
        Dac0W::new(self, 31)
    }
}
#[doc = "System Clock Gating Control Register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc6Spec;
impl crate::RegisterSpec for Scgc6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc6::R`](R) reader structure"]
impl crate::Readable for Scgc6Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc6::W`](W) writer structure"]
impl crate::Writable for Scgc6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGC6 to value 0x01"]
impl crate::Resettable for Scgc6Spec {
    const RESET_VALUE: u32 = 0x01;
}
