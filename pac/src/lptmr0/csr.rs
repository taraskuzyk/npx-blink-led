#[doc = "Register `CSR` reader"]
pub type R = crate::R<CsrSpec>;
#[doc = "Register `CSR` writer"]
pub type W = crate::W<CsrSpec>;
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ten {
    #[doc = "0: LPTMR is disabled and internal logic is reset."]
    B0 = 0,
    #[doc = "1: LPTMR is enabled."]
    B1 = 1,
}
impl From<Ten> for bool {
    #[inline(always)]
    fn from(variant: Ten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEN` reader - Timer Enable"]
pub type TenR = crate::BitReader<Ten>;
impl TenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ten {
        match self.bits {
            false => Ten::B0,
            true => Ten::B1,
        }
    }
    #[doc = "LPTMR is disabled and internal logic is reset."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ten::B0
    }
    #[doc = "LPTMR is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ten::B1
    }
}
#[doc = "Field `TEN` writer - Timer Enable"]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG, Ten>;
impl<'a, REG> TenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTMR is disabled and internal logic is reset."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::B0)
    }
    #[doc = "LPTMR is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ten::B1)
    }
}
#[doc = "Timer Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tms {
    #[doc = "0: Time Counter mode."]
    B0 = 0,
    #[doc = "1: Pulse Counter mode."]
    B1 = 1,
}
impl From<Tms> for bool {
    #[inline(always)]
    fn from(variant: Tms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMS` reader - Timer Mode Select"]
pub type TmsR = crate::BitReader<Tms>;
impl TmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tms {
        match self.bits {
            false => Tms::B0,
            true => Tms::B1,
        }
    }
    #[doc = "Time Counter mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tms::B0
    }
    #[doc = "Pulse Counter mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tms::B1
    }
}
#[doc = "Field `TMS` writer - Timer Mode Select"]
pub type TmsW<'a, REG> = crate::BitWriter<'a, REG, Tms>;
impl<'a, REG> TmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time Counter mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tms::B0)
    }
    #[doc = "Pulse Counter mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tms::B1)
    }
}
#[doc = "Timer Free-Running Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tfc {
    #[doc = "0: CNR is reset whenever TCF is set."]
    B0 = 0,
    #[doc = "1: CNR is reset on overflow."]
    B1 = 1,
}
impl From<Tfc> for bool {
    #[inline(always)]
    fn from(variant: Tfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFC` reader - Timer Free-Running Counter"]
pub type TfcR = crate::BitReader<Tfc>;
impl TfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tfc {
        match self.bits {
            false => Tfc::B0,
            true => Tfc::B1,
        }
    }
    #[doc = "CNR is reset whenever TCF is set."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tfc::B0
    }
    #[doc = "CNR is reset on overflow."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tfc::B1
    }
}
#[doc = "Field `TFC` writer - Timer Free-Running Counter"]
pub type TfcW<'a, REG> = crate::BitWriter<'a, REG, Tfc>;
impl<'a, REG> TfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CNR is reset whenever TCF is set."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tfc::B0)
    }
    #[doc = "CNR is reset on overflow."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tfc::B1)
    }
}
#[doc = "Timer Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpp {
    #[doc = "0: Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
    B0 = 0,
    #[doc = "1: Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
    B1 = 1,
}
impl From<Tpp> for bool {
    #[inline(always)]
    fn from(variant: Tpp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPP` reader - Timer Pin Polarity"]
pub type TppR = crate::BitReader<Tpp>;
impl TppR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpp {
        match self.bits {
            false => Tpp::B0,
            true => Tpp::B1,
        }
    }
    #[doc = "Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tpp::B0
    }
    #[doc = "Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tpp::B1
    }
}
#[doc = "Field `TPP` writer - Timer Pin Polarity"]
pub type TppW<'a, REG> = crate::BitWriter<'a, REG, Tpp>;
impl<'a, REG> TppW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpp::B0)
    }
    #[doc = "Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpp::B1)
    }
}
#[doc = "Timer Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tps {
    #[doc = "0: Pulse counter input 0 is selected."]
    B00 = 0,
    #[doc = "1: Pulse counter input 1 is selected."]
    B01 = 1,
    #[doc = "2: Pulse counter input 2 is selected."]
    B10 = 2,
    #[doc = "3: Pulse counter input 3 is selected."]
    B11 = 3,
}
impl From<Tps> for u8 {
    #[inline(always)]
    fn from(variant: Tps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tps {
    type Ux = u8;
}
impl crate::IsEnum for Tps {}
#[doc = "Field `TPS` reader - Timer Pin Select"]
pub type TpsR = crate::FieldReader<Tps>;
impl TpsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tps {
        match self.bits {
            0 => Tps::B00,
            1 => Tps::B01,
            2 => Tps::B10,
            3 => Tps::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Pulse counter input 0 is selected."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Tps::B00
    }
    #[doc = "Pulse counter input 1 is selected."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Tps::B01
    }
    #[doc = "Pulse counter input 2 is selected."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Tps::B10
    }
    #[doc = "Pulse counter input 3 is selected."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Tps::B11
    }
}
#[doc = "Field `TPS` writer - Timer Pin Select"]
pub type TpsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tps, crate::Safe>;
impl<'a, REG> TpsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pulse counter input 0 is selected."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Tps::B00)
    }
    #[doc = "Pulse counter input 1 is selected."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Tps::B01)
    }
    #[doc = "Pulse counter input 2 is selected."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Tps::B10)
    }
    #[doc = "Pulse counter input 3 is selected."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Tps::B11)
    }
}
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    #[doc = "0: Timer interrupt disabled."]
    B0 = 0,
    #[doc = "1: Timer interrupt enabled."]
    B1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Timer Interrupt Enable"]
pub type TieR = crate::BitReader<Tie>;
impl TieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tie {
        match self.bits {
            false => Tie::B0,
            true => Tie::B1,
        }
    }
    #[doc = "Timer interrupt disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tie::B0
    }
    #[doc = "Timer interrupt enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tie::B1
    }
}
#[doc = "Field `TIE` writer - Timer Interrupt Enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer interrupt disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B0)
    }
    #[doc = "Timer interrupt enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B1)
    }
}
#[doc = "Timer Compare Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcf {
    #[doc = "0: The value of CNR is not equal to CMR and increments."]
    B0 = 0,
    #[doc = "1: The value of CNR is equal to CMR and increments."]
    B1 = 1,
}
impl From<Tcf> for bool {
    #[inline(always)]
    fn from(variant: Tcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCF` reader - Timer Compare Flag"]
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
    #[doc = "The value of CNR is not equal to CMR and increments."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcf::B0
    }
    #[doc = "The value of CNR is equal to CMR and increments."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcf::B1
    }
}
#[doc = "Field `TCF` writer - Timer Compare Flag"]
pub type TcfW<'a, REG> = crate::BitWriter<'a, REG, Tcf>;
impl<'a, REG> TcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The value of CNR is not equal to CMR and increments."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcf::B0)
    }
    #[doc = "The value of CNR is equal to CMR and increments."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcf::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Mode Select"]
    #[inline(always)]
    pub fn tms(&self) -> TmsR {
        TmsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer Free-Running Counter"]
    #[inline(always)]
    pub fn tfc(&self) -> TfcR {
        TfcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer Pin Polarity"]
    #[inline(always)]
    pub fn tpp(&self) -> TppR {
        TppR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Timer Pin Select"]
    #[inline(always)]
    pub fn tps(&self) -> TpsR {
        TpsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Compare Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TenW<CsrSpec> {
        TenW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tms(&mut self) -> TmsW<CsrSpec> {
        TmsW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer Free-Running Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tfc(&mut self) -> TfcW<CsrSpec> {
        TfcW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer Pin Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tpp(&mut self) -> TppW<CsrSpec> {
        TppW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Timer Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TpsW<CsrSpec> {
        TpsW::new(self, 4)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<CsrSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer Compare Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TcfW<CsrSpec> {
        TcfW::new(self, 7)
    }
}
#[doc = "Low Power Timer Control Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsrSpec;
impl crate::RegisterSpec for CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr::R`](R) reader structure"]
impl crate::Readable for CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`csr::W`](W) writer structure"]
impl crate::Writable for CsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CsrSpec {
    const RESET_VALUE: u32 = 0;
}
