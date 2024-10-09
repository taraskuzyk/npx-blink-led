#[doc = "Register `TIMCTL%s` reader"]
pub type R = crate::R<TimctlSpec>;
#[doc = "Register `TIMCTL%s` writer"]
pub type W = crate::W<TimctlSpec>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timod {
    #[doc = "0: Timer Disabled."]
    B00 = 0,
    #[doc = "1: Dual 8-bit counters baud/bit mode."]
    B01 = 1,
    #[doc = "2: Dual 8-bit counters PWM mode."]
    B10 = 2,
    #[doc = "3: Single 16-bit counter mode."]
    B11 = 3,
}
impl From<Timod> for u8 {
    #[inline(always)]
    fn from(variant: Timod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timod {
    type Ux = u8;
}
impl crate::IsEnum for Timod {}
#[doc = "Field `TIMOD` reader - Timer Mode"]
pub type TimodR = crate::FieldReader<Timod>;
impl TimodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timod {
        match self.bits {
            0 => Timod::B00,
            1 => Timod::B01,
            2 => Timod::B10,
            3 => Timod::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer Disabled."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Timod::B00
    }
    #[doc = "Dual 8-bit counters baud/bit mode."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Timod::B01
    }
    #[doc = "Dual 8-bit counters PWM mode."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Timod::B10
    }
    #[doc = "Single 16-bit counter mode."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Timod::B11
    }
}
#[doc = "Field `TIMOD` writer - Timer Mode"]
pub type TimodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Timod, crate::Safe>;
impl<'a, REG> TimodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer Disabled."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Timod::B00)
    }
    #[doc = "Dual 8-bit counters baud/bit mode."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Timod::B01)
    }
    #[doc = "Dual 8-bit counters PWM mode."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Timod::B10)
    }
    #[doc = "Single 16-bit counter mode."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Timod::B11)
    }
}
#[doc = "Timer Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pinpol {
    #[doc = "0: Pin is active high"]
    B0 = 0,
    #[doc = "1: Pin is active low"]
    B1 = 1,
}
impl From<Pinpol> for bool {
    #[inline(always)]
    fn from(variant: Pinpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINPOL` reader - Timer Pin Polarity"]
pub type PinpolR = crate::BitReader<Pinpol>;
impl PinpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pinpol {
        match self.bits {
            false => Pinpol::B0,
            true => Pinpol::B1,
        }
    }
    #[doc = "Pin is active high"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pinpol::B0
    }
    #[doc = "Pin is active low"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pinpol::B1
    }
}
#[doc = "Field `PINPOL` writer - Timer Pin Polarity"]
pub type PinpolW<'a, REG> = crate::BitWriter<'a, REG, Pinpol>;
impl<'a, REG> PinpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin is active high"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pinpol::B0)
    }
    #[doc = "Pin is active low"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pinpol::B1)
    }
}
#[doc = "Field `PINSEL` reader - Timer Pin Select"]
pub type PinselR = crate::FieldReader;
#[doc = "Field `PINSEL` writer - Timer Pin Select"]
pub type PinselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Timer Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pincfg {
    #[doc = "0: Timer pin output disabled"]
    B00 = 0,
    #[doc = "1: Timer pin open drain or bidirectional output enable"]
    B01 = 1,
    #[doc = "2: Timer pin bidirectional output data"]
    B10 = 2,
    #[doc = "3: Timer pin output"]
    B11 = 3,
}
impl From<Pincfg> for u8 {
    #[inline(always)]
    fn from(variant: Pincfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pincfg {
    type Ux = u8;
}
impl crate::IsEnum for Pincfg {}
#[doc = "Field `PINCFG` reader - Timer Pin Configuration"]
pub type PincfgR = crate::FieldReader<Pincfg>;
impl PincfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pincfg {
        match self.bits {
            0 => Pincfg::B00,
            1 => Pincfg::B01,
            2 => Pincfg::B10,
            3 => Pincfg::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer pin output disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Pincfg::B00
    }
    #[doc = "Timer pin open drain or bidirectional output enable"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Pincfg::B01
    }
    #[doc = "Timer pin bidirectional output data"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Pincfg::B10
    }
    #[doc = "Timer pin output"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Pincfg::B11
    }
}
#[doc = "Field `PINCFG` writer - Timer Pin Configuration"]
pub type PincfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pincfg, crate::Safe>;
impl<'a, REG> PincfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer pin output disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Pincfg::B00)
    }
    #[doc = "Timer pin open drain or bidirectional output enable"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Pincfg::B01)
    }
    #[doc = "Timer pin bidirectional output data"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Pincfg::B10)
    }
    #[doc = "Timer pin output"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Pincfg::B11)
    }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgsrc {
    #[doc = "0: External trigger selected"]
    B0 = 0,
    #[doc = "1: Internal trigger selected"]
    B1 = 1,
}
impl From<Trgsrc> for bool {
    #[inline(always)]
    fn from(variant: Trgsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGSRC` reader - Trigger Source"]
pub type TrgsrcR = crate::BitReader<Trgsrc>;
impl TrgsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc {
        match self.bits {
            false => Trgsrc::B0,
            true => Trgsrc::B1,
        }
    }
    #[doc = "External trigger selected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Trgsrc::B0
    }
    #[doc = "Internal trigger selected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Trgsrc::B1
    }
}
#[doc = "Field `TRGSRC` writer - Trigger Source"]
pub type TrgsrcW<'a, REG> = crate::BitWriter<'a, REG, Trgsrc>;
impl<'a, REG> TrgsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External trigger selected"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc::B0)
    }
    #[doc = "Internal trigger selected"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc::B1)
    }
}
#[doc = "Trigger Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgpol {
    #[doc = "0: Trigger active high"]
    B0 = 0,
    #[doc = "1: Trigger active low"]
    B1 = 1,
}
impl From<Trgpol> for bool {
    #[inline(always)]
    fn from(variant: Trgpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGPOL` reader - Trigger Polarity"]
pub type TrgpolR = crate::BitReader<Trgpol>;
impl TrgpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgpol {
        match self.bits {
            false => Trgpol::B0,
            true => Trgpol::B1,
        }
    }
    #[doc = "Trigger active high"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Trgpol::B0
    }
    #[doc = "Trigger active low"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Trgpol::B1
    }
}
#[doc = "Field `TRGPOL` writer - Trigger Polarity"]
pub type TrgpolW<'a, REG> = crate::BitWriter<'a, REG, Trgpol>;
impl<'a, REG> TrgpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger active high"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgpol::B0)
    }
    #[doc = "Trigger active low"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgpol::B1)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Select"]
pub type TrgselR = crate::FieldReader;
#[doc = "Field `TRGSEL` writer - Trigger Select"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn timod(&self) -> TimodR {
        TimodR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&self) -> PinpolR {
        PinpolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Timer Pin Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PinselR {
        PinselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PincfgR {
        PincfgR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TrgsrcR {
        TrgsrcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline(always)]
    pub fn trgpol(&self) -> TrgpolR {
        TrgpolR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn timod(&mut self) -> TimodW<TimctlSpec> {
        TimodW::new(self, 0)
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pinpol(&mut self) -> PinpolW<TimctlSpec> {
        PinpolW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Timer Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn pinsel(&mut self) -> PinselW<TimctlSpec> {
        PinselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pincfg(&mut self) -> PincfgW<TimctlSpec> {
        PincfgW::new(self, 16)
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TrgsrcW<TimctlSpec> {
        TrgsrcW::new(self, 22)
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn trgpol(&mut self) -> TrgpolW<TimctlSpec> {
        TrgpolW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TrgselW<TimctlSpec> {
        TrgselW::new(self, 24)
    }
}
#[doc = "Timer Control N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimctlSpec;
impl crate::RegisterSpec for TimctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timctl::R`](R) reader structure"]
impl crate::Readable for TimctlSpec {}
#[doc = "`write(|w| ..)` method takes [`timctl::W`](W) writer structure"]
impl crate::Writable for TimctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMCTL%s to value 0"]
impl crate::Resettable for TimctlSpec {
    const RESET_VALUE: u32 = 0;
}
