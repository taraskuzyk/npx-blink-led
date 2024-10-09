#[doc = "Register `SHIFTCTL%s` reader"]
pub type R = crate::R<ShiftctlSpec>;
#[doc = "Register `SHIFTCTL%s` writer"]
pub type W = crate::W<ShiftctlSpec>;
#[doc = "Shifter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smod {
    #[doc = "0: Disabled."]
    B000 = 0,
    #[doc = "1: Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    B001 = 1,
    #[doc = "2: Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    B010 = 2,
    #[doc = "4: Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    B100 = 4,
    #[doc = "5: Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    B101 = 5,
}
impl From<Smod> for u8 {
    #[inline(always)]
    fn from(variant: Smod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smod {
    type Ux = u8;
}
impl crate::IsEnum for Smod {}
#[doc = "Field `SMOD` reader - Shifter Mode"]
pub type SmodR = crate::FieldReader<Smod>;
impl SmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Smod> {
        match self.bits {
            0 => Some(Smod::B000),
            1 => Some(Smod::B001),
            2 => Some(Smod::B010),
            4 => Some(Smod::B100),
            5 => Some(Smod::B101),
            _ => None,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Smod::B000
    }
    #[doc = "Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Smod::B001
    }
    #[doc = "Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Smod::B010
    }
    #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Smod::B100
    }
    #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Smod::B101
    }
}
#[doc = "Field `SMOD` writer - Shifter Mode"]
pub type SmodW<'a, REG> = crate::FieldWriter<'a, REG, 3, Smod>;
impl<'a, REG> SmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B000)
    }
    #[doc = "Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B001)
    }
    #[doc = "Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B010)
    }
    #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B100)
    }
    #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B101)
    }
}
#[doc = "Shifter Pin Polarity\n\nValue on reset: 0"]
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
#[doc = "Field `PINPOL` reader - Shifter Pin Polarity"]
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
#[doc = "Field `PINPOL` writer - Shifter Pin Polarity"]
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
#[doc = "Field `PINSEL` reader - Shifter Pin Select"]
pub type PinselR = crate::FieldReader;
#[doc = "Field `PINSEL` writer - Shifter Pin Select"]
pub type PinselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Shifter Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pincfg {
    #[doc = "0: Shifter pin output disabled"]
    B00 = 0,
    #[doc = "1: Shifter pin open drain or bidirectional output enable"]
    B01 = 1,
    #[doc = "2: Shifter pin bidirectional output data"]
    B10 = 2,
    #[doc = "3: Shifter pin output"]
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
#[doc = "Field `PINCFG` reader - Shifter Pin Configuration"]
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
    #[doc = "Shifter pin output disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Pincfg::B00
    }
    #[doc = "Shifter pin open drain or bidirectional output enable"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Pincfg::B01
    }
    #[doc = "Shifter pin bidirectional output data"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Pincfg::B10
    }
    #[doc = "Shifter pin output"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Pincfg::B11
    }
}
#[doc = "Field `PINCFG` writer - Shifter Pin Configuration"]
pub type PincfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pincfg, crate::Safe>;
impl<'a, REG> PincfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Shifter pin output disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Pincfg::B00)
    }
    #[doc = "Shifter pin open drain or bidirectional output enable"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Pincfg::B01)
    }
    #[doc = "Shifter pin bidirectional output data"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Pincfg::B10)
    }
    #[doc = "Shifter pin output"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Pincfg::B11)
    }
}
#[doc = "Timer Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timpol {
    #[doc = "0: Shift on posedge of Shift clock"]
    B0 = 0,
    #[doc = "1: Shift on negedge of Shift clock"]
    B1 = 1,
}
impl From<Timpol> for bool {
    #[inline(always)]
    fn from(variant: Timpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMPOL` reader - Timer Polarity"]
pub type TimpolR = crate::BitReader<Timpol>;
impl TimpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timpol {
        match self.bits {
            false => Timpol::B0,
            true => Timpol::B1,
        }
    }
    #[doc = "Shift on posedge of Shift clock"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Timpol::B0
    }
    #[doc = "Shift on negedge of Shift clock"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Timpol::B1
    }
}
#[doc = "Field `TIMPOL` writer - Timer Polarity"]
pub type TimpolW<'a, REG> = crate::BitWriter<'a, REG, Timpol>;
impl<'a, REG> TimpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Shift on posedge of Shift clock"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Timpol::B0)
    }
    #[doc = "Shift on negedge of Shift clock"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Timpol::B1)
    }
}
#[doc = "Field `TIMSEL` reader - Timer Select"]
pub type TimselR = crate::FieldReader;
#[doc = "Field `TIMSEL` writer - Timer Select"]
pub type TimselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Shifter Mode"]
    #[inline(always)]
    pub fn smod(&self) -> SmodR {
        SmodR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Shifter Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&self) -> PinpolR {
        PinpolR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Shifter Pin Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PinselR {
        PinselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Shifter Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PincfgR {
        PincfgR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 23 - Timer Polarity"]
    #[inline(always)]
    pub fn timpol(&self) -> TimpolR {
        TimpolR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Timer Select"]
    #[inline(always)]
    pub fn timsel(&self) -> TimselR {
        TimselR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shifter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn smod(&mut self) -> SmodW<ShiftctlSpec> {
        SmodW::new(self, 0)
    }
    #[doc = "Bit 7 - Shifter Pin Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pinpol(&mut self) -> PinpolW<ShiftctlSpec> {
        PinpolW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Shifter Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn pinsel(&mut self) -> PinselW<ShiftctlSpec> {
        PinselW::new(self, 8)
    }
    #[doc = "Bits 16:17 - Shifter Pin Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pincfg(&mut self) -> PincfgW<ShiftctlSpec> {
        PincfgW::new(self, 16)
    }
    #[doc = "Bit 23 - Timer Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn timpol(&mut self) -> TimpolW<ShiftctlSpec> {
        TimpolW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Timer Select"]
    #[inline(always)]
    #[must_use]
    pub fn timsel(&mut self) -> TimselW<ShiftctlSpec> {
        TimselW::new(self, 24)
    }
}
#[doc = "Shifter Control N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftctlSpec;
impl crate::RegisterSpec for ShiftctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shiftctl::R`](R) reader structure"]
impl crate::Readable for ShiftctlSpec {}
#[doc = "`write(|w| ..)` method takes [`shiftctl::W`](W) writer structure"]
impl crate::Writable for ShiftctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTCTL%s to value 0"]
impl crate::Resettable for ShiftctlSpec {
    const RESET_VALUE: u32 = 0;
}
