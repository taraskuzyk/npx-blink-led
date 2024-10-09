#[doc = "Register `SCGC5` reader"]
pub type R = crate::R<Scgc5Spec>;
#[doc = "Register `SCGC5` writer"]
pub type W = crate::W<Scgc5Spec>;
#[doc = "Low Power Timer Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lptmr {
    #[doc = "0: Access disabled"]
    B0 = 0,
    #[doc = "1: Access enabled"]
    B1 = 1,
}
impl From<Lptmr> for bool {
    #[inline(always)]
    fn from(variant: Lptmr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPTMR` reader - Low Power Timer Access Control"]
pub type LptmrR = crate::BitReader<Lptmr>;
impl LptmrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lptmr {
        match self.bits {
            false => Lptmr::B0,
            true => Lptmr::B1,
        }
    }
    #[doc = "Access disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lptmr::B0
    }
    #[doc = "Access enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lptmr::B1
    }
}
#[doc = "Field `LPTMR` writer - Low Power Timer Access Control"]
pub type LptmrW<'a, REG> = crate::BitWriter<'a, REG, Lptmr>;
impl<'a, REG> LptmrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Access disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lptmr::B0)
    }
    #[doc = "Access enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lptmr::B1)
    }
}
#[doc = "Port A Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porta {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Porta> for bool {
    #[inline(always)]
    fn from(variant: Porta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTA` reader - Port A Clock Gate Control"]
pub type PortaR = crate::BitReader<Porta>;
impl PortaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Porta {
        match self.bits {
            false => Porta::B0,
            true => Porta::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Porta::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Porta::B1
    }
}
#[doc = "Field `PORTA` writer - Port A Clock Gate Control"]
pub type PortaW<'a, REG> = crate::BitWriter<'a, REG, Porta>;
impl<'a, REG> PortaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Porta::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Porta::B1)
    }
}
#[doc = "Port B Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Portb {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Portb> for bool {
    #[inline(always)]
    fn from(variant: Portb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTB` reader - Port B Clock Gate Control"]
pub type PortbR = crate::BitReader<Portb>;
impl PortbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Portb {
        match self.bits {
            false => Portb::B0,
            true => Portb::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Portb::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Portb::B1
    }
}
#[doc = "Field `PORTB` writer - Port B Clock Gate Control"]
pub type PortbW<'a, REG> = crate::BitWriter<'a, REG, Portb>;
impl<'a, REG> PortbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Portb::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Portb::B1)
    }
}
#[doc = "Port C Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Portc {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Portc> for bool {
    #[inline(always)]
    fn from(variant: Portc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTC` reader - Port C Clock Gate Control"]
pub type PortcR = crate::BitReader<Portc>;
impl PortcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Portc {
        match self.bits {
            false => Portc::B0,
            true => Portc::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Portc::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Portc::B1
    }
}
#[doc = "Field `PORTC` writer - Port C Clock Gate Control"]
pub type PortcW<'a, REG> = crate::BitWriter<'a, REG, Portc>;
impl<'a, REG> PortcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Portc::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Portc::B1)
    }
}
#[doc = "Port D Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Portd {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Portd> for bool {
    #[inline(always)]
    fn from(variant: Portd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTD` reader - Port D Clock Gate Control"]
pub type PortdR = crate::BitReader<Portd>;
impl PortdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Portd {
        match self.bits {
            false => Portd::B0,
            true => Portd::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Portd::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Portd::B1
    }
}
#[doc = "Field `PORTD` writer - Port D Clock Gate Control"]
pub type PortdW<'a, REG> = crate::BitWriter<'a, REG, Portd>;
impl<'a, REG> PortdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Portd::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Portd::B1)
    }
}
#[doc = "Port E Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porte {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Porte> for bool {
    #[inline(always)]
    fn from(variant: Porte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORTE` reader - Port E Clock Gate Control"]
pub type PorteR = crate::BitReader<Porte>;
impl PorteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Porte {
        match self.bits {
            false => Porte::B0,
            true => Porte::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Porte::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Porte::B1
    }
}
#[doc = "Field `PORTE` writer - Port E Clock Gate Control"]
pub type PorteW<'a, REG> = crate::BitWriter<'a, REG, Porte>;
impl<'a, REG> PorteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Porte::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Porte::B1)
    }
}
#[doc = "Segment LCD Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slcd {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Slcd> for bool {
    #[inline(always)]
    fn from(variant: Slcd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLCD` reader - Segment LCD Clock Gate Control"]
pub type SlcdR = crate::BitReader<Slcd>;
impl SlcdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slcd {
        match self.bits {
            false => Slcd::B0,
            true => Slcd::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Slcd::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Slcd::B1
    }
}
#[doc = "Field `SLCD` writer - Segment LCD Clock Gate Control"]
pub type SlcdW<'a, REG> = crate::BitWriter<'a, REG, Slcd>;
impl<'a, REG> SlcdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Slcd::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Slcd::B1)
    }
}
#[doc = "LPUART0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart0 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Lpuart0> for bool {
    #[inline(always)]
    fn from(variant: Lpuart0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART0` reader - LPUART0 Clock Gate Control"]
pub type Lpuart0R = crate::BitReader<Lpuart0>;
impl Lpuart0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart0 {
        match self.bits {
            false => Lpuart0::B0,
            true => Lpuart0::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpuart0::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpuart0::B1
    }
}
#[doc = "Field `LPUART0` writer - LPUART0 Clock Gate Control"]
pub type Lpuart0W<'a, REG> = crate::BitWriter<'a, REG, Lpuart0>;
impl<'a, REG> Lpuart0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0::B1)
    }
}
#[doc = "LPUART1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart1 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Lpuart1> for bool {
    #[inline(always)]
    fn from(variant: Lpuart1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1` reader - LPUART1 Clock Gate Control"]
pub type Lpuart1R = crate::BitReader<Lpuart1>;
impl Lpuart1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1 {
        match self.bits {
            false => Lpuart1::B0,
            true => Lpuart1::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpuart1::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpuart1::B1
    }
}
#[doc = "Field `LPUART1` writer - LPUART1 Clock Gate Control"]
pub type Lpuart1W<'a, REG> = crate::BitWriter<'a, REG, Lpuart1>;
impl<'a, REG> Lpuart1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1::B1)
    }
}
#[doc = "FlexIO Module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexio {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Flexio> for bool {
    #[inline(always)]
    fn from(variant: Flexio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXIO` reader - FlexIO Module"]
pub type FlexioR = crate::BitReader<Flexio>;
impl FlexioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexio {
        match self.bits {
            false => Flexio::B0,
            true => Flexio::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Flexio::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Flexio::B1
    }
}
#[doc = "Field `FLEXIO` writer - FlexIO Module"]
pub type FlexioW<'a, REG> = crate::BitWriter<'a, REG, Flexio>;
impl<'a, REG> FlexioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Flexio::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Flexio::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Low Power Timer Access Control"]
    #[inline(always)]
    pub fn lptmr(&self) -> LptmrR {
        LptmrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 9 - Port A Clock Gate Control"]
    #[inline(always)]
    pub fn porta(&self) -> PortaR {
        PortaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port B Clock Gate Control"]
    #[inline(always)]
    pub fn portb(&self) -> PortbR {
        PortbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port C Clock Gate Control"]
    #[inline(always)]
    pub fn portc(&self) -> PortcR {
        PortcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port D Clock Gate Control"]
    #[inline(always)]
    pub fn portd(&self) -> PortdR {
        PortdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port E Clock Gate Control"]
    #[inline(always)]
    pub fn porte(&self) -> PorteR {
        PorteR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 19 - Segment LCD Clock Gate Control"]
    #[inline(always)]
    pub fn slcd(&self) -> SlcdR {
        SlcdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPUART0 Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart0(&self) -> Lpuart0R {
        Lpuart0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPUART1 Clock Gate Control"]
    #[inline(always)]
    pub fn lpuart1(&self) -> Lpuart1R {
        Lpuart1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - FlexIO Module"]
    #[inline(always)]
    pub fn flexio(&self) -> FlexioR {
        FlexioR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low Power Timer Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn lptmr(&mut self) -> LptmrW<Scgc5Spec> {
        LptmrW::new(self, 0)
    }
    #[doc = "Bit 9 - Port A Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn porta(&mut self) -> PortaW<Scgc5Spec> {
        PortaW::new(self, 9)
    }
    #[doc = "Bit 10 - Port B Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn portb(&mut self) -> PortbW<Scgc5Spec> {
        PortbW::new(self, 10)
    }
    #[doc = "Bit 11 - Port C Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn portc(&mut self) -> PortcW<Scgc5Spec> {
        PortcW::new(self, 11)
    }
    #[doc = "Bit 12 - Port D Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn portd(&mut self) -> PortdW<Scgc5Spec> {
        PortdW::new(self, 12)
    }
    #[doc = "Bit 13 - Port E Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn porte(&mut self) -> PorteW<Scgc5Spec> {
        PorteW::new(self, 13)
    }
    #[doc = "Bit 19 - Segment LCD Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn slcd(&mut self) -> SlcdW<Scgc5Spec> {
        SlcdW::new(self, 19)
    }
    #[doc = "Bit 20 - LPUART0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart0(&mut self) -> Lpuart0W<Scgc5Spec> {
        Lpuart0W::new(self, 20)
    }
    #[doc = "Bit 21 - LPUART1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1(&mut self) -> Lpuart1W<Scgc5Spec> {
        Lpuart1W::new(self, 21)
    }
    #[doc = "Bit 31 - FlexIO Module"]
    #[inline(always)]
    #[must_use]
    pub fn flexio(&mut self) -> FlexioW<Scgc5Spec> {
        FlexioW::new(self, 31)
    }
}
#[doc = "System Clock Gating Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc5Spec;
impl crate::RegisterSpec for Scgc5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc5::R`](R) reader structure"]
impl crate::Readable for Scgc5Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc5::W`](W) writer structure"]
impl crate::Writable for Scgc5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGC5 to value 0x0182"]
impl crate::Resettable for Scgc5Spec {
    const RESET_VALUE: u32 = 0x0182;
}
