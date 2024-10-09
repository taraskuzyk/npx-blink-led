#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swr {
    #[doc = "0: No effect."]
    B0 = 0,
    #[doc = "1: Resets all RTC registers except for the SWR bit . The SWR bit is cleared by POR and by software explicitly clearing it."]
    B1 = 1,
}
impl From<Swr> for bool {
    #[inline(always)]
    fn from(variant: Swr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWR` reader - Software Reset"]
pub type SwrR = crate::BitReader<Swr>;
impl SwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swr {
        match self.bits {
            false => Swr::B0,
            true => Swr::B1,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Swr::B0
    }
    #[doc = "Resets all RTC registers except for the SWR bit . The SWR bit is cleared by POR and by software explicitly clearing it."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Swr::B1
    }
}
#[doc = "Field `SWR` writer - Software Reset"]
pub type SwrW<'a, REG> = crate::BitWriter<'a, REG, Swr>;
impl<'a, REG> SwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Swr::B0)
    }
    #[doc = "Resets all RTC registers except for the SWR bit . The SWR bit is cleared by POR and by software explicitly clearing it."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Swr::B1)
    }
}
#[doc = "Wakeup Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wpe {
    #[doc = "0: Wakeup pin is disabled."]
    B0 = 0,
    #[doc = "1: Wakeup pin is enabled and wakeup pin asserts if the RTC interrupt asserts or the wakeup pin is turned on."]
    B1 = 1,
}
impl From<Wpe> for bool {
    #[inline(always)]
    fn from(variant: Wpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPE` reader - Wakeup Pin Enable"]
pub type WpeR = crate::BitReader<Wpe>;
impl WpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wpe {
        match self.bits {
            false => Wpe::B0,
            true => Wpe::B1,
        }
    }
    #[doc = "Wakeup pin is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wpe::B0
    }
    #[doc = "Wakeup pin is enabled and wakeup pin asserts if the RTC interrupt asserts or the wakeup pin is turned on."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wpe::B1
    }
}
#[doc = "Field `WPE` writer - Wakeup Pin Enable"]
pub type WpeW<'a, REG> = crate::BitWriter<'a, REG, Wpe>;
impl<'a, REG> WpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup pin is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wpe::B0)
    }
    #[doc = "Wakeup pin is enabled and wakeup pin asserts if the RTC interrupt asserts or the wakeup pin is turned on."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wpe::B1)
    }
}
#[doc = "Supervisor Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sup {
    #[doc = "0: Non-supervisor mode write accesses are not supported and generate a bus error."]
    B0 = 0,
    #[doc = "1: Non-supervisor mode write accesses are supported."]
    B1 = 1,
}
impl From<Sup> for bool {
    #[inline(always)]
    fn from(variant: Sup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUP` reader - Supervisor Access"]
pub type SupR = crate::BitReader<Sup>;
impl SupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sup {
        match self.bits {
            false => Sup::B0,
            true => Sup::B1,
        }
    }
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sup::B0
    }
    #[doc = "Non-supervisor mode write accesses are supported."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sup::B1
    }
}
#[doc = "Field `SUP` writer - Supervisor Access"]
pub type SupW<'a, REG> = crate::BitWriter<'a, REG, Sup>;
impl<'a, REG> SupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Non-supervisor mode write accesses are not supported and generate a bus error."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sup::B0)
    }
    #[doc = "Non-supervisor mode write accesses are supported."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sup::B1)
    }
}
#[doc = "Update Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Um {
    #[doc = "0: Registers cannot be written when locked."]
    B0 = 0,
    #[doc = "1: Registers can be written when locked under limited conditions."]
    B1 = 1,
}
impl From<Um> for bool {
    #[inline(always)]
    fn from(variant: Um) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UM` reader - Update Mode"]
pub type UmR = crate::BitReader<Um>;
impl UmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Um {
        match self.bits {
            false => Um::B0,
            true => Um::B1,
        }
    }
    #[doc = "Registers cannot be written when locked."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Um::B0
    }
    #[doc = "Registers can be written when locked under limited conditions."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Um::B1
    }
}
#[doc = "Field `UM` writer - Update Mode"]
pub type UmW<'a, REG> = crate::BitWriter<'a, REG, Um>;
impl<'a, REG> UmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Registers cannot be written when locked."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Um::B0)
    }
    #[doc = "Registers can be written when locked under limited conditions."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Um::B1)
    }
}
#[doc = "Wakeup Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wps {
    #[doc = "0: Wakeup pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
    B0 = 0,
    #[doc = "1: Wakeup pin instead outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
    B1 = 1,
}
impl From<Wps> for bool {
    #[inline(always)]
    fn from(variant: Wps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPS` reader - Wakeup Pin Select"]
pub type WpsR = crate::BitReader<Wps>;
impl WpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wps {
        match self.bits {
            false => Wps::B0,
            true => Wps::B1,
        }
    }
    #[doc = "Wakeup pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wps::B0
    }
    #[doc = "Wakeup pin instead outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wps::B1
    }
}
#[doc = "Field `WPS` writer - Wakeup Pin Select"]
pub type WpsW<'a, REG> = crate::BitWriter<'a, REG, Wps>;
impl<'a, REG> WpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wakeup pin asserts (active low, open drain) if the RTC interrupt asserts or the wakeup pin is turned on."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wps::B0)
    }
    #[doc = "Wakeup pin instead outputs the RTC 32kHz clock, provided the wakeup pin is turned on and the 32kHz clock is output to other peripherals."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wps::B1)
    }
}
#[doc = "Oscillator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Osce {
    #[doc = "0: 32.768 kHz oscillator is disabled."]
    B0 = 0,
    #[doc = "1: 32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
    B1 = 1,
}
impl From<Osce> for bool {
    #[inline(always)]
    fn from(variant: Osce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCE` reader - Oscillator Enable"]
pub type OsceR = crate::BitReader<Osce>;
impl OsceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Osce {
        match self.bits {
            false => Osce::B0,
            true => Osce::B1,
        }
    }
    #[doc = "32.768 kHz oscillator is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Osce::B0
    }
    #[doc = "32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Osce::B1
    }
}
#[doc = "Field `OSCE` writer - Oscillator Enable"]
pub type OsceW<'a, REG> = crate::BitWriter<'a, REG, Osce>;
impl<'a, REG> OsceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "32.768 kHz oscillator is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Osce::B0)
    }
    #[doc = "32.768 kHz oscillator is enabled. After setting this bit, wait the oscillator startup time before enabling the time counter to allow the 32.768 kHz clock time to stabilize."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Osce::B1)
    }
}
#[doc = "Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clko {
    #[doc = "0: The 32 kHz clock is output to other peripherals."]
    B0 = 0,
    #[doc = "1: The 32 kHz clock is not output to other peripherals."]
    B1 = 1,
}
impl From<Clko> for bool {
    #[inline(always)]
    fn from(variant: Clko) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKO` reader - Clock Output"]
pub type ClkoR = crate::BitReader<Clko>;
impl ClkoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clko {
        match self.bits {
            false => Clko::B0,
            true => Clko::B1,
        }
    }
    #[doc = "The 32 kHz clock is output to other peripherals."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Clko::B0
    }
    #[doc = "The 32 kHz clock is not output to other peripherals."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Clko::B1
    }
}
#[doc = "Field `CLKO` writer - Clock Output"]
pub type ClkoW<'a, REG> = crate::BitWriter<'a, REG, Clko>;
impl<'a, REG> ClkoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The 32 kHz clock is output to other peripherals."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Clko::B0)
    }
    #[doc = "The 32 kHz clock is not output to other peripherals."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Clko::B1)
    }
}
#[doc = "Oscillator 16pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sc16p {
    #[doc = "0: Disable the load."]
    B0 = 0,
    #[doc = "1: Enable the additional load."]
    B1 = 1,
}
impl From<Sc16p> for bool {
    #[inline(always)]
    fn from(variant: Sc16p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC16P` reader - Oscillator 16pF Load Configure"]
pub type Sc16pR = crate::BitReader<Sc16p>;
impl Sc16pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sc16p {
        match self.bits {
            false => Sc16p::B0,
            true => Sc16p::B1,
        }
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sc16p::B0
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sc16p::B1
    }
}
#[doc = "Field `SC16P` writer - Oscillator 16pF Load Configure"]
pub type Sc16pW<'a, REG> = crate::BitWriter<'a, REG, Sc16p>;
impl<'a, REG> Sc16pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sc16p::B0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sc16p::B1)
    }
}
#[doc = "Oscillator 8pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sc8p {
    #[doc = "0: Disable the load."]
    B0 = 0,
    #[doc = "1: Enable the additional load."]
    B1 = 1,
}
impl From<Sc8p> for bool {
    #[inline(always)]
    fn from(variant: Sc8p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC8P` reader - Oscillator 8pF Load Configure"]
pub type Sc8pR = crate::BitReader<Sc8p>;
impl Sc8pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sc8p {
        match self.bits {
            false => Sc8p::B0,
            true => Sc8p::B1,
        }
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sc8p::B0
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sc8p::B1
    }
}
#[doc = "Field `SC8P` writer - Oscillator 8pF Load Configure"]
pub type Sc8pW<'a, REG> = crate::BitWriter<'a, REG, Sc8p>;
impl<'a, REG> Sc8pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sc8p::B0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sc8p::B1)
    }
}
#[doc = "Oscillator 4pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sc4p {
    #[doc = "0: Disable the load."]
    B0 = 0,
    #[doc = "1: Enable the additional load."]
    B1 = 1,
}
impl From<Sc4p> for bool {
    #[inline(always)]
    fn from(variant: Sc4p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC4P` reader - Oscillator 4pF Load Configure"]
pub type Sc4pR = crate::BitReader<Sc4p>;
impl Sc4pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sc4p {
        match self.bits {
            false => Sc4p::B0,
            true => Sc4p::B1,
        }
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sc4p::B0
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sc4p::B1
    }
}
#[doc = "Field `SC4P` writer - Oscillator 4pF Load Configure"]
pub type Sc4pW<'a, REG> = crate::BitWriter<'a, REG, Sc4p>;
impl<'a, REG> Sc4pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sc4p::B0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sc4p::B1)
    }
}
#[doc = "Oscillator 2pF Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sc2p {
    #[doc = "0: Disable the load."]
    B0 = 0,
    #[doc = "1: Enable the additional load."]
    B1 = 1,
}
impl From<Sc2p> for bool {
    #[inline(always)]
    fn from(variant: Sc2p) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SC2P` reader - Oscillator 2pF Load Configure"]
pub type Sc2pR = crate::BitReader<Sc2p>;
impl Sc2pR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sc2p {
        match self.bits {
            false => Sc2p::B0,
            true => Sc2p::B1,
        }
    }
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sc2p::B0
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sc2p::B1
    }
}
#[doc = "Field `SC2P` writer - Oscillator 2pF Load Configure"]
pub type Sc2pW<'a, REG> = crate::BitWriter<'a, REG, Sc2p>;
impl<'a, REG> Sc2pW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the load."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sc2p::B0)
    }
    #[doc = "Enable the additional load."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sc2p::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SwrR {
        SwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Pin Enable"]
    #[inline(always)]
    pub fn wpe(&self) -> WpeR {
        WpeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline(always)]
    pub fn sup(&self) -> SupR {
        SupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline(always)]
    pub fn um(&self) -> UmR {
        UmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup Pin Select"]
    #[inline(always)]
    pub fn wps(&self) -> WpsR {
        WpsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Oscillator Enable"]
    #[inline(always)]
    pub fn osce(&self) -> OsceR {
        OsceR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock Output"]
    #[inline(always)]
    pub fn clko(&self) -> ClkoR {
        ClkoR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Oscillator 16pF Load Configure"]
    #[inline(always)]
    pub fn sc16p(&self) -> Sc16pR {
        Sc16pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Oscillator 8pF Load Configure"]
    #[inline(always)]
    pub fn sc8p(&self) -> Sc8pR {
        Sc8pR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Oscillator 4pF Load Configure"]
    #[inline(always)]
    pub fn sc4p(&self) -> Sc4pR {
        Sc4pR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Oscillator 2pF Load Configure"]
    #[inline(always)]
    pub fn sc2p(&self) -> Sc2pR {
        Sc2pR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SwrW<CrSpec> {
        SwrW::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpe(&mut self) -> WpeW<CrSpec> {
        WpeW::new(self, 1)
    }
    #[doc = "Bit 2 - Supervisor Access"]
    #[inline(always)]
    #[must_use]
    pub fn sup(&mut self) -> SupW<CrSpec> {
        SupW::new(self, 2)
    }
    #[doc = "Bit 3 - Update Mode"]
    #[inline(always)]
    #[must_use]
    pub fn um(&mut self) -> UmW<CrSpec> {
        UmW::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn wps(&mut self) -> WpsW<CrSpec> {
        WpsW::new(self, 4)
    }
    #[doc = "Bit 8 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osce(&mut self) -> OsceW<CrSpec> {
        OsceW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn clko(&mut self) -> ClkoW<CrSpec> {
        ClkoW::new(self, 9)
    }
    #[doc = "Bit 10 - Oscillator 16pF Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc16p(&mut self) -> Sc16pW<CrSpec> {
        Sc16pW::new(self, 10)
    }
    #[doc = "Bit 11 - Oscillator 8pF Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc8p(&mut self) -> Sc8pW<CrSpec> {
        Sc8pW::new(self, 11)
    }
    #[doc = "Bit 12 - Oscillator 4pF Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc4p(&mut self) -> Sc4pW<CrSpec> {
        Sc4pW::new(self, 12)
    }
    #[doc = "Bit 13 - Oscillator 2pF Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc2p(&mut self) -> Sc2pW<CrSpec> {
        Sc2pW::new(self, 13)
    }
}
#[doc = "RTC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
