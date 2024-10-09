#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pt {
    #[doc = "0: Even parity."]
    B0 = 0,
    #[doc = "1: Odd parity."]
    B1 = 1,
}
impl From<Pt> for bool {
    #[inline(always)]
    fn from(variant: Pt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PT` reader - Parity Type"]
pub type PtR = crate::BitReader<Pt>;
impl PtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pt {
        match self.bits {
            false => Pt::B0,
            true => Pt::B1,
        }
    }
    #[doc = "Even parity."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pt::B0
    }
    #[doc = "Odd parity."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pt::B1
    }
}
#[doc = "Field `PT` writer - Parity Type"]
pub type PtW<'a, REG> = crate::BitWriter<'a, REG, Pt>;
impl<'a, REG> PtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pt::B0)
    }
    #[doc = "Odd parity."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pt::B1)
    }
}
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: No hardware parity generation or checking."]
    B0 = 0,
    #[doc = "1: Parity enabled."]
    B1 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Enable"]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::B0,
            true => Pe::B1,
        }
    }
    #[doc = "No hardware parity generation or checking."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pe::B0
    }
    #[doc = "Parity enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pe::B1
    }
}
#[doc = "Field `PE` writer - Parity Enable"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No hardware parity generation or checking."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::B0)
    }
    #[doc = "Parity enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::B1)
    }
}
#[doc = "Idle Line Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilt {
    #[doc = "0: Idle character bit count starts after start bit."]
    B0 = 0,
    #[doc = "1: Idle character bit count starts after stop bit."]
    B1 = 1,
}
impl From<Ilt> for bool {
    #[inline(always)]
    fn from(variant: Ilt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILT` reader - Idle Line Type Select"]
pub type IltR = crate::BitReader<Ilt>;
impl IltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilt {
        match self.bits {
            false => Ilt::B0,
            true => Ilt::B1,
        }
    }
    #[doc = "Idle character bit count starts after start bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ilt::B0
    }
    #[doc = "Idle character bit count starts after stop bit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ilt::B1
    }
}
#[doc = "Field `ILT` writer - Idle Line Type Select"]
pub type IltW<'a, REG> = crate::BitWriter<'a, REG, Ilt>;
impl<'a, REG> IltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Idle character bit count starts after start bit."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ilt::B0)
    }
    #[doc = "Idle character bit count starts after stop bit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ilt::B1)
    }
}
#[doc = "Receiver Wakeup Method Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wake {
    #[doc = "0: Configures RWU for idle-line wakeup."]
    B0 = 0,
    #[doc = "1: Configures RWU with address-mark wakeup."]
    B1 = 1,
}
impl From<Wake> for bool {
    #[inline(always)]
    fn from(variant: Wake) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKE` reader - Receiver Wakeup Method Select"]
pub type WakeR = crate::BitReader<Wake>;
impl WakeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wake {
        match self.bits {
            false => Wake::B0,
            true => Wake::B1,
        }
    }
    #[doc = "Configures RWU for idle-line wakeup."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wake::B0
    }
    #[doc = "Configures RWU with address-mark wakeup."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wake::B1
    }
}
#[doc = "Field `WAKE` writer - Receiver Wakeup Method Select"]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG, Wake>;
impl<'a, REG> WakeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configures RWU for idle-line wakeup."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wake::B0)
    }
    #[doc = "Configures RWU with address-mark wakeup."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wake::B1)
    }
}
#[doc = "9-Bit or 8-Bit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M {
    #[doc = "0: Receiver and transmitter use 8-bit data characters."]
    B0 = 0,
    #[doc = "1: Receiver and transmitter use 9-bit data characters."]
    B1 = 1,
}
impl From<M> for bool {
    #[inline(always)]
    fn from(variant: M) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M` reader - 9-Bit or 8-Bit Mode Select"]
pub type MR = crate::BitReader<M>;
impl MR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M {
        match self.bits {
            false => M::B0,
            true => M::B1,
        }
    }
    #[doc = "Receiver and transmitter use 8-bit data characters."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == M::B0
    }
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == M::B1
    }
}
#[doc = "Field `M` writer - 9-Bit or 8-Bit Mode Select"]
pub type MW<'a, REG> = crate::BitWriter<'a, REG, M>;
impl<'a, REG> MW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver and transmitter use 8-bit data characters."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(M::B0)
    }
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(M::B1)
    }
}
#[doc = "Receiver Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsrc {
    #[doc = "0: Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the LPUART_RX pin."]
    B0 = 0,
    #[doc = "1: Single-wire LPUART mode where the LPUART_TX pin is connected to the transmitter output and receiver input."]
    B1 = 1,
}
impl From<Rsrc> for bool {
    #[inline(always)]
    fn from(variant: Rsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSRC` reader - Receiver Source Select"]
pub type RsrcR = crate::BitReader<Rsrc>;
impl RsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rsrc {
        match self.bits {
            false => Rsrc::B0,
            true => Rsrc::B1,
        }
    }
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the LPUART_RX pin."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rsrc::B0
    }
    #[doc = "Single-wire LPUART mode where the LPUART_TX pin is connected to the transmitter output and receiver input."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rsrc::B1
    }
}
#[doc = "Field `RSRC` writer - Receiver Source Select"]
pub type RsrcW<'a, REG> = crate::BitWriter<'a, REG, Rsrc>;
impl<'a, REG> RsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the LPUART_RX pin."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsrc::B0)
    }
    #[doc = "Single-wire LPUART mode where the LPUART_TX pin is connected to the transmitter output and receiver input."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsrc::B1)
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dozeen {
    #[doc = "0: LPUART is enabled in Doze mode."]
    B0 = 0,
    #[doc = "1: LPUART is disabled in Doze mode."]
    B1 = 1,
}
impl From<Dozeen> for bool {
    #[inline(always)]
    fn from(variant: Dozeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZEEN` reader - Doze Enable"]
pub type DozeenR = crate::BitReader<Dozeen>;
impl DozeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dozeen {
        match self.bits {
            false => Dozeen::B0,
            true => Dozeen::B1,
        }
    }
    #[doc = "LPUART is enabled in Doze mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dozeen::B0
    }
    #[doc = "LPUART is disabled in Doze mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dozeen::B1
    }
}
#[doc = "Field `DOZEEN` writer - Doze Enable"]
pub type DozeenW<'a, REG> = crate::BitWriter<'a, REG, Dozeen>;
impl<'a, REG> DozeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART is enabled in Doze mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dozeen::B0)
    }
    #[doc = "LPUART is disabled in Doze mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dozeen::B1)
    }
}
#[doc = "Loop Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loops {
    #[doc = "0: Normal operation - LPUART_RX and LPUART_TX use separate pins."]
    B0 = 0,
    #[doc = "1: Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    B1 = 1,
}
impl From<Loops> for bool {
    #[inline(always)]
    fn from(variant: Loops) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOOPS` reader - Loop Mode Select"]
pub type LoopsR = crate::BitReader<Loops>;
impl LoopsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Loops {
        match self.bits {
            false => Loops::B0,
            true => Loops::B1,
        }
    }
    #[doc = "Normal operation - LPUART_RX and LPUART_TX use separate pins."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Loops::B0
    }
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Loops::B1
    }
}
#[doc = "Field `LOOPS` writer - Loop Mode Select"]
pub type LoopsW<'a, REG> = crate::BitWriter<'a, REG, Loops>;
impl<'a, REG> LoopsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation - LPUART_RX and LPUART_TX use separate pins."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Loops::B0)
    }
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Loops::B1)
    }
}
#[doc = "Idle Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Idlecfg {
    #[doc = "0: 1 idle character"]
    B000 = 0,
    #[doc = "1: 2 idle characters"]
    B001 = 1,
    #[doc = "2: 4 idle characters"]
    B010 = 2,
    #[doc = "3: 8 idle characters"]
    B011 = 3,
    #[doc = "4: 16 idle characters"]
    B100 = 4,
    #[doc = "5: 32 idle characters"]
    B101 = 5,
    #[doc = "6: 64 idle characters"]
    B110 = 6,
    #[doc = "7: 128 idle characters"]
    B111 = 7,
}
impl From<Idlecfg> for u8 {
    #[inline(always)]
    fn from(variant: Idlecfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Idlecfg {
    type Ux = u8;
}
impl crate::IsEnum for Idlecfg {}
#[doc = "Field `IDLECFG` reader - Idle Configuration"]
pub type IdlecfgR = crate::FieldReader<Idlecfg>;
impl IdlecfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idlecfg {
        match self.bits {
            0 => Idlecfg::B000,
            1 => Idlecfg::B001,
            2 => Idlecfg::B010,
            3 => Idlecfg::B011,
            4 => Idlecfg::B100,
            5 => Idlecfg::B101,
            6 => Idlecfg::B110,
            7 => Idlecfg::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "1 idle character"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Idlecfg::B000
    }
    #[doc = "2 idle characters"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Idlecfg::B001
    }
    #[doc = "4 idle characters"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Idlecfg::B010
    }
    #[doc = "8 idle characters"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Idlecfg::B011
    }
    #[doc = "16 idle characters"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Idlecfg::B100
    }
    #[doc = "32 idle characters"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Idlecfg::B101
    }
    #[doc = "64 idle characters"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Idlecfg::B110
    }
    #[doc = "128 idle characters"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Idlecfg::B111
    }
}
#[doc = "Field `IDLECFG` writer - Idle Configuration"]
pub type IdlecfgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Idlecfg, crate::Safe>;
impl<'a, REG> IdlecfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 idle character"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Idlecfg::B000)
    }
    #[doc = "2 idle characters"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Idlecfg::B001)
    }
    #[doc = "4 idle characters"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Idlecfg::B010)
    }
    #[doc = "8 idle characters"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Idlecfg::B011)
    }
    #[doc = "16 idle characters"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Idlecfg::B100)
    }
    #[doc = "32 idle characters"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Idlecfg::B101)
    }
    #[doc = "64 idle characters"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Idlecfg::B110)
    }
    #[doc = "128 idle characters"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Idlecfg::B111)
    }
}
#[doc = "Match 2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ma2ie {
    #[doc = "0: MA2F interrupt disabled"]
    B0 = 0,
    #[doc = "1: MA2F interrupt enabled"]
    B1 = 1,
}
impl From<Ma2ie> for bool {
    #[inline(always)]
    fn from(variant: Ma2ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MA2IE` reader - Match 2 Interrupt Enable"]
pub type Ma2ieR = crate::BitReader<Ma2ie>;
impl Ma2ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ma2ie {
        match self.bits {
            false => Ma2ie::B0,
            true => Ma2ie::B1,
        }
    }
    #[doc = "MA2F interrupt disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ma2ie::B0
    }
    #[doc = "MA2F interrupt enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ma2ie::B1
    }
}
#[doc = "Field `MA2IE` writer - Match 2 Interrupt Enable"]
pub type Ma2ieW<'a, REG> = crate::BitWriter<'a, REG, Ma2ie>;
impl<'a, REG> Ma2ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MA2F interrupt disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ma2ie::B0)
    }
    #[doc = "MA2F interrupt enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ma2ie::B1)
    }
}
#[doc = "Match 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ma1ie {
    #[doc = "0: MA1F interrupt disabled"]
    B0 = 0,
    #[doc = "1: MA1F interrupt enabled"]
    B1 = 1,
}
impl From<Ma1ie> for bool {
    #[inline(always)]
    fn from(variant: Ma1ie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MA1IE` reader - Match 1 Interrupt Enable"]
pub type Ma1ieR = crate::BitReader<Ma1ie>;
impl Ma1ieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ma1ie {
        match self.bits {
            false => Ma1ie::B0,
            true => Ma1ie::B1,
        }
    }
    #[doc = "MA1F interrupt disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ma1ie::B0
    }
    #[doc = "MA1F interrupt enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ma1ie::B1
    }
}
#[doc = "Field `MA1IE` writer - Match 1 Interrupt Enable"]
pub type Ma1ieW<'a, REG> = crate::BitWriter<'a, REG, Ma1ie>;
impl<'a, REG> Ma1ieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MA1F interrupt disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ma1ie::B0)
    }
    #[doc = "MA1F interrupt enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ma1ie::B1)
    }
}
#[doc = "Send Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbk {
    #[doc = "0: Normal transmitter operation."]
    B0 = 0,
    #[doc = "1: Queue break character(s) to be sent."]
    B1 = 1,
}
impl From<Sbk> for bool {
    #[inline(always)]
    fn from(variant: Sbk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBK` reader - Send Break"]
pub type SbkR = crate::BitReader<Sbk>;
impl SbkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbk {
        match self.bits {
            false => Sbk::B0,
            true => Sbk::B1,
        }
    }
    #[doc = "Normal transmitter operation."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sbk::B0
    }
    #[doc = "Queue break character(s) to be sent."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sbk::B1
    }
}
#[doc = "Field `SBK` writer - Send Break"]
pub type SbkW<'a, REG> = crate::BitWriter<'a, REG, Sbk>;
impl<'a, REG> SbkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal transmitter operation."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sbk::B0)
    }
    #[doc = "Queue break character(s) to be sent."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbk::B1)
    }
}
#[doc = "Receiver Wakeup Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwu {
    #[doc = "0: Normal receiver operation."]
    B0 = 0,
    #[doc = "1: LPUART receiver in standby waiting for wakeup condition."]
    B1 = 1,
}
impl From<Rwu> for bool {
    #[inline(always)]
    fn from(variant: Rwu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWU` reader - Receiver Wakeup Control"]
pub type RwuR = crate::BitReader<Rwu>;
impl RwuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwu {
        match self.bits {
            false => Rwu::B0,
            true => Rwu::B1,
        }
    }
    #[doc = "Normal receiver operation."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rwu::B0
    }
    #[doc = "LPUART receiver in standby waiting for wakeup condition."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rwu::B1
    }
}
#[doc = "Field `RWU` writer - Receiver Wakeup Control"]
pub type RwuW<'a, REG> = crate::BitWriter<'a, REG, Rwu>;
impl<'a, REG> RwuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal receiver operation."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwu::B0)
    }
    #[doc = "LPUART receiver in standby waiting for wakeup condition."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwu::B1)
    }
}
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Re {
    #[doc = "0: Receiver disabled."]
    B0 = 0,
    #[doc = "1: Receiver enabled."]
    B1 = 1,
}
impl From<Re> for bool {
    #[inline(always)]
    fn from(variant: Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RE` reader - Receiver Enable"]
pub type ReR = crate::BitReader<Re>;
impl ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Re {
        match self.bits {
            false => Re::B0,
            true => Re::B1,
        }
    }
    #[doc = "Receiver disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Re::B0
    }
    #[doc = "Receiver enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Re::B1
    }
}
#[doc = "Field `RE` writer - Receiver Enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG, Re>;
impl<'a, REG> ReW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Re::B0)
    }
    #[doc = "Receiver enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Re::B1)
    }
}
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te {
    #[doc = "0: Transmitter disabled."]
    B0 = 0,
    #[doc = "1: Transmitter enabled."]
    B1 = 1,
}
impl From<Te> for bool {
    #[inline(always)]
    fn from(variant: Te) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TE` reader - Transmitter Enable"]
pub type TeR = crate::BitReader<Te>;
impl TeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Te {
        match self.bits {
            false => Te::B0,
            true => Te::B1,
        }
    }
    #[doc = "Transmitter disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Te::B0
    }
    #[doc = "Transmitter enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Te::B1
    }
}
#[doc = "Field `TE` writer - Transmitter Enable"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG, Te>;
impl<'a, REG> TeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Te::B0)
    }
    #[doc = "Transmitter enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Te::B1)
    }
}
#[doc = "Idle Line Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilie {
    #[doc = "0: Hardware interrupts from IDLE disabled; use polling."]
    B0 = 0,
    #[doc = "1: Hardware interrupt requested when IDLE flag is 1."]
    B1 = 1,
}
impl From<Ilie> for bool {
    #[inline(always)]
    fn from(variant: Ilie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIE` reader - Idle Line Interrupt Enable"]
pub type IlieR = crate::BitReader<Ilie>;
impl IlieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilie {
        match self.bits {
            false => Ilie::B0,
            true => Ilie::B1,
        }
    }
    #[doc = "Hardware interrupts from IDLE disabled; use polling."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ilie::B0
    }
    #[doc = "Hardware interrupt requested when IDLE flag is 1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ilie::B1
    }
}
#[doc = "Field `ILIE` writer - Idle Line Interrupt Enable"]
pub type IlieW<'a, REG> = crate::BitWriter<'a, REG, Ilie>;
impl<'a, REG> IlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware interrupts from IDLE disabled; use polling."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ilie::B0)
    }
    #[doc = "Hardware interrupt requested when IDLE flag is 1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ilie::B1)
    }
}
#[doc = "Receiver Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    #[doc = "0: Hardware interrupts from RDRF disabled; use polling."]
    B0 = 0,
    #[doc = "1: Hardware interrupt requested when RDRF flag is 1."]
    B1 = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIE` reader - Receiver Interrupt Enable"]
pub type RieR = crate::BitReader<Rie>;
impl RieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rie {
        match self.bits {
            false => Rie::B0,
            true => Rie::B1,
        }
    }
    #[doc = "Hardware interrupts from RDRF disabled; use polling."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rie::B0
    }
    #[doc = "Hardware interrupt requested when RDRF flag is 1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rie::B1
    }
}
#[doc = "Field `RIE` writer - Receiver Interrupt Enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware interrupts from RDRF disabled; use polling."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::B0)
    }
    #[doc = "Hardware interrupt requested when RDRF flag is 1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::B1)
    }
}
#[doc = "Transmission Complete Interrupt Enable for\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: Hardware interrupts from TC disabled; use polling."]
    B0 = 0,
    #[doc = "1: Hardware interrupt requested when TC flag is 1."]
    B1 = 1,
}
impl From<Tcie> for bool {
    #[inline(always)]
    fn from(variant: Tcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transmission Complete Interrupt Enable for"]
pub type TcieR = crate::BitReader<Tcie>;
impl TcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcie {
        match self.bits {
            false => Tcie::B0,
            true => Tcie::B1,
        }
    }
    #[doc = "Hardware interrupts from TC disabled; use polling."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcie::B0
    }
    #[doc = "Hardware interrupt requested when TC flag is 1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcie::B1
    }
}
#[doc = "Field `TCIE` writer - Transmission Complete Interrupt Enable for"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG, Tcie>;
impl<'a, REG> TcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware interrupts from TC disabled; use polling."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0)
    }
    #[doc = "Hardware interrupt requested when TC flag is 1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B1)
    }
}
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    #[doc = "0: Hardware interrupts from TDRE disabled; use polling."]
    B0 = 0,
    #[doc = "1: Hardware interrupt requested when TDRE flag is 1."]
    B1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
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
    #[doc = "Hardware interrupts from TDRE disabled; use polling."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tie::B0
    }
    #[doc = "Hardware interrupt requested when TDRE flag is 1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tie::B1
    }
}
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware interrupts from TDRE disabled; use polling."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B0)
    }
    #[doc = "Hardware interrupt requested when TDRE flag is 1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B1)
    }
}
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peie {
    #[doc = "0: PF interrupts disabled; use polling)."]
    B0 = 0,
    #[doc = "1: Hardware interrupt requested when PF is set."]
    B1 = 1,
}
impl From<Peie> for bool {
    #[inline(always)]
    fn from(variant: Peie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEIE` reader - Parity Error Interrupt Enable"]
pub type PeieR = crate::BitReader<Peie>;
impl PeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Peie {
        match self.bits {
            false => Peie::B0,
            true => Peie::B1,
        }
    }
    #[doc = "PF interrupts disabled; use polling)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Peie::B0
    }
    #[doc = "Hardware interrupt requested when PF is set."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Peie::B1
    }
}
#[doc = "Field `PEIE` writer - Parity Error Interrupt Enable"]
pub type PeieW<'a, REG> = crate::BitWriter<'a, REG, Peie>;
impl<'a, REG> PeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PF interrupts disabled; use polling)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::B0)
    }
    #[doc = "Hardware interrupt requested when PF is set."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::B1)
    }
}
#[doc = "Framing Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feie {
    #[doc = "0: FE interrupts disabled; use polling."]
    B0 = 0,
    #[doc = "1: Hardware interrupt requested when FE is set."]
    B1 = 1,
}
impl From<Feie> for bool {
    #[inline(always)]
    fn from(variant: Feie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIE` reader - Framing Error Interrupt Enable"]
pub type FeieR = crate::BitReader<Feie>;
impl FeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Feie {
        match self.bits {
            false => Feie::B0,
            true => Feie::B1,
        }
    }
    #[doc = "FE interrupts disabled; use polling."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Feie::B0
    }
    #[doc = "Hardware interrupt requested when FE is set."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Feie::B1
    }
}
#[doc = "Field `FEIE` writer - Framing Error Interrupt Enable"]
pub type FeieW<'a, REG> = crate::BitWriter<'a, REG, Feie>;
impl<'a, REG> FeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FE interrupts disabled; use polling."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Feie::B0)
    }
    #[doc = "Hardware interrupt requested when FE is set."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Feie::B1)
    }
}
#[doc = "Noise Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Neie {
    #[doc = "0: NF interrupts disabled; use polling."]
    B0 = 0,
    #[doc = "1: Hardware interrupt requested when NF is set."]
    B1 = 1,
}
impl From<Neie> for bool {
    #[inline(always)]
    fn from(variant: Neie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NEIE` reader - Noise Error Interrupt Enable"]
pub type NeieR = crate::BitReader<Neie>;
impl NeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Neie {
        match self.bits {
            false => Neie::B0,
            true => Neie::B1,
        }
    }
    #[doc = "NF interrupts disabled; use polling."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Neie::B0
    }
    #[doc = "Hardware interrupt requested when NF is set."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Neie::B1
    }
}
#[doc = "Field `NEIE` writer - Noise Error Interrupt Enable"]
pub type NeieW<'a, REG> = crate::BitWriter<'a, REG, Neie>;
impl<'a, REG> NeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NF interrupts disabled; use polling."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Neie::B0)
    }
    #[doc = "Hardware interrupt requested when NF is set."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Neie::B1)
    }
}
#[doc = "Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orie {
    #[doc = "0: OR interrupts disabled; use polling."]
    B0 = 0,
    #[doc = "1: Hardware interrupt requested when OR is set."]
    B1 = 1,
}
impl From<Orie> for bool {
    #[inline(always)]
    fn from(variant: Orie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORIE` reader - Overrun Interrupt Enable"]
pub type OrieR = crate::BitReader<Orie>;
impl OrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Orie {
        match self.bits {
            false => Orie::B0,
            true => Orie::B1,
        }
    }
    #[doc = "OR interrupts disabled; use polling."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Orie::B0
    }
    #[doc = "Hardware interrupt requested when OR is set."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Orie::B1
    }
}
#[doc = "Field `ORIE` writer - Overrun Interrupt Enable"]
pub type OrieW<'a, REG> = crate::BitWriter<'a, REG, Orie>;
impl<'a, REG> OrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OR interrupts disabled; use polling."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Orie::B0)
    }
    #[doc = "Hardware interrupt requested when OR is set."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Orie::B1)
    }
}
#[doc = "Transmit Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txinv {
    #[doc = "0: Transmit data not inverted."]
    B0 = 0,
    #[doc = "1: Transmit data inverted."]
    B1 = 1,
}
impl From<Txinv> for bool {
    #[inline(always)]
    fn from(variant: Txinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - Transmit Data Inversion"]
pub type TxinvR = crate::BitReader<Txinv>;
impl TxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txinv {
        match self.bits {
            false => Txinv::B0,
            true => Txinv::B1,
        }
    }
    #[doc = "Transmit data not inverted."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txinv::B0
    }
    #[doc = "Transmit data inverted."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txinv::B1
    }
}
#[doc = "Field `TXINV` writer - Transmit Data Inversion"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG, Txinv>;
impl<'a, REG> TxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit data not inverted."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::B0)
    }
    #[doc = "Transmit data inverted."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::B1)
    }
}
#[doc = "LPUART_TX Pin Direction in Single-Wire Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdir {
    #[doc = "0: LPUART_TX pin is an input in single-wire mode."]
    B0 = 0,
    #[doc = "1: LPUART_TX pin is an output in single-wire mode."]
    B1 = 1,
}
impl From<Txdir> for bool {
    #[inline(always)]
    fn from(variant: Txdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDIR` reader - LPUART_TX Pin Direction in Single-Wire Mode"]
pub type TxdirR = crate::BitReader<Txdir>;
impl TxdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdir {
        match self.bits {
            false => Txdir::B0,
            true => Txdir::B1,
        }
    }
    #[doc = "LPUART_TX pin is an input in single-wire mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txdir::B0
    }
    #[doc = "LPUART_TX pin is an output in single-wire mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txdir::B1
    }
}
#[doc = "Field `TXDIR` writer - LPUART_TX Pin Direction in Single-Wire Mode"]
pub type TxdirW<'a, REG> = crate::BitWriter<'a, REG, Txdir>;
impl<'a, REG> TxdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART_TX pin is an input in single-wire mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Txdir::B0)
    }
    #[doc = "LPUART_TX pin is an output in single-wire mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Txdir::B1)
    }
}
#[doc = "Field `R9T8` reader - Receive Bit 9 / Transmit Bit 8"]
pub type R9t8R = crate::BitReader;
#[doc = "Field `R9T8` writer - Receive Bit 9 / Transmit Bit 8"]
pub type R9t8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R8T9` reader - Receive Bit 8 / Transmit Bit 9"]
pub type R8t9R = crate::BitReader;
#[doc = "Field `R8T9` writer - Receive Bit 8 / Transmit Bit 9"]
pub type R8t9W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&self) -> IltR {
        IltR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&self) -> RsrcR {
        RsrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&self) -> DozeenR {
        DozeenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&self) -> LoopsR {
        LoopsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Idle Configuration"]
    #[inline(always)]
    pub fn idlecfg(&self) -> IdlecfgR {
        IdlecfgR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 14 - Match 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ma2ie(&self) -> Ma2ieR {
        Ma2ieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ma1ie(&self) -> Ma1ieR {
        Ma1ieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Send Break"]
    #[inline(always)]
    pub fn sbk(&self) -> SbkR {
        SbkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Idle Line Interrupt Enable"]
    #[inline(always)]
    pub fn ilie(&self) -> IlieR {
        IlieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Receiver Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Complete Interrupt Enable for"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FeieR {
        FeieR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&self) -> NeieR {
        NeieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&self) -> OrieR {
        OrieR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit Data Inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - LPUART_TX Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub fn txdir(&self) -> TxdirR {
        TxdirR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Receive Bit 9 / Transmit Bit 8"]
    #[inline(always)]
    pub fn r9t8(&self) -> R9t8R {
        R9t8R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive Bit 8 / Transmit Bit 9"]
    #[inline(always)]
    pub fn r8t9(&self) -> R8t9R {
        R8t9R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PtW<CtrlSpec> {
        PtW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<CtrlSpec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn ilt(&mut self) -> IltW<CtrlSpec> {
        IltW::new(self, 2)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WakeW<CtrlSpec> {
        WakeW::new(self, 3)
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<CtrlSpec> {
        MW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn rsrc(&mut self) -> RsrcW<CtrlSpec> {
        RsrcW::new(self, 5)
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dozeen(&mut self) -> DozeenW<CtrlSpec> {
        DozeenW::new(self, 6)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn loops(&mut self) -> LoopsW<CtrlSpec> {
        LoopsW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Idle Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idlecfg(&mut self) -> IdlecfgW<CtrlSpec> {
        IdlecfgW::new(self, 8)
    }
    #[doc = "Bit 14 - Match 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ma2ie(&mut self) -> Ma2ieW<CtrlSpec> {
        Ma2ieW::new(self, 14)
    }
    #[doc = "Bit 15 - Match 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ma1ie(&mut self) -> Ma1ieW<CtrlSpec> {
        Ma1ieW::new(self, 15)
    }
    #[doc = "Bit 16 - Send Break"]
    #[inline(always)]
    #[must_use]
    pub fn sbk(&mut self) -> SbkW<CtrlSpec> {
        SbkW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Wakeup Control"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RwuW<CtrlSpec> {
        RwuW::new(self, 17)
    }
    #[doc = "Bit 18 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<CtrlSpec> {
        ReW::new(self, 18)
    }
    #[doc = "Bit 19 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<CtrlSpec> {
        TeW::new(self, 19)
    }
    #[doc = "Bit 20 - Idle Line Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ilie(&mut self) -> IlieW<CtrlSpec> {
        IlieW::new(self, 20)
    }
    #[doc = "Bit 21 - Receiver Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RieW<CtrlSpec> {
        RieW::new(self, 21)
    }
    #[doc = "Bit 22 - Transmission Complete Interrupt Enable for"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<CtrlSpec> {
        TcieW::new(self, 22)
    }
    #[doc = "Bit 23 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<CtrlSpec> {
        TieW::new(self, 23)
    }
    #[doc = "Bit 24 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PeieW<CtrlSpec> {
        PeieW::new(self, 24)
    }
    #[doc = "Bit 25 - Framing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FeieW<CtrlSpec> {
        FeieW::new(self, 25)
    }
    #[doc = "Bit 26 - Noise Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn neie(&mut self) -> NeieW<CtrlSpec> {
        NeieW::new(self, 26)
    }
    #[doc = "Bit 27 - Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn orie(&mut self) -> OrieW<CtrlSpec> {
        OrieW::new(self, 27)
    }
    #[doc = "Bit 28 - Transmit Data Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TxinvW<CtrlSpec> {
        TxinvW::new(self, 28)
    }
    #[doc = "Bit 29 - LPUART_TX Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    #[must_use]
    pub fn txdir(&mut self) -> TxdirW<CtrlSpec> {
        TxdirW::new(self, 29)
    }
    #[doc = "Bit 30 - Receive Bit 9 / Transmit Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn r9t8(&mut self) -> R9t8W<CtrlSpec> {
        R9t8W::new(self, 30)
    }
    #[doc = "Bit 31 - Receive Bit 8 / Transmit Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn r8t9(&mut self) -> R8t9W<CtrlSpec> {
        R8t9W::new(self, 31)
    }
}
#[doc = "LPUART Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
