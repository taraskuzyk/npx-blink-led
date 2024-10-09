#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Register `STAT` writer"]
pub type W = crate::W<StatSpec>;
#[doc = "Match 2 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ma2f {
    #[doc = "0: Received data is not equal to MA2"]
    B0 = 0,
    #[doc = "1: Received data is equal to MA2"]
    B1 = 1,
}
impl From<Ma2f> for bool {
    #[inline(always)]
    fn from(variant: Ma2f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MA2F` reader - Match 2 Flag"]
pub type Ma2fR = crate::BitReader<Ma2f>;
impl Ma2fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ma2f {
        match self.bits {
            false => Ma2f::B0,
            true => Ma2f::B1,
        }
    }
    #[doc = "Received data is not equal to MA2"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ma2f::B0
    }
    #[doc = "Received data is equal to MA2"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ma2f::B1
    }
}
#[doc = "Field `MA2F` writer - Match 2 Flag"]
pub type Ma2fW<'a, REG> = crate::BitWriter<'a, REG, Ma2f>;
impl<'a, REG> Ma2fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Received data is not equal to MA2"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ma2f::B0)
    }
    #[doc = "Received data is equal to MA2"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ma2f::B1)
    }
}
#[doc = "Match 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ma1f {
    #[doc = "0: Received data is not equal to MA1"]
    B0 = 0,
    #[doc = "1: Received data is equal to MA1"]
    B1 = 1,
}
impl From<Ma1f> for bool {
    #[inline(always)]
    fn from(variant: Ma1f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MA1F` reader - Match 1 Flag"]
pub type Ma1fR = crate::BitReader<Ma1f>;
impl Ma1fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ma1f {
        match self.bits {
            false => Ma1f::B0,
            true => Ma1f::B1,
        }
    }
    #[doc = "Received data is not equal to MA1"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ma1f::B0
    }
    #[doc = "Received data is equal to MA1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ma1f::B1
    }
}
#[doc = "Field `MA1F` writer - Match 1 Flag"]
pub type Ma1fW<'a, REG> = crate::BitWriter<'a, REG, Ma1f>;
impl<'a, REG> Ma1fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Received data is not equal to MA1"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ma1f::B0)
    }
    #[doc = "Received data is equal to MA1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ma1f::B1)
    }
}
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pf {
    #[doc = "0: No parity error."]
    B0 = 0,
    #[doc = "1: Parity error."]
    B1 = 1,
}
impl From<Pf> for bool {
    #[inline(always)]
    fn from(variant: Pf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PF` reader - Parity Error Flag"]
pub type PfR = crate::BitReader<Pf>;
impl PfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pf {
        match self.bits {
            false => Pf::B0,
            true => Pf::B1,
        }
    }
    #[doc = "No parity error."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pf::B0
    }
    #[doc = "Parity error."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pf::B1
    }
}
#[doc = "Field `PF` writer - Parity Error Flag"]
pub type PfW<'a, REG> = crate::BitWriter<'a, REG, Pf>;
impl<'a, REG> PfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No parity error."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pf::B0)
    }
    #[doc = "Parity error."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pf::B1)
    }
}
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: No framing error detected. This does not guarantee the framing is correct."]
    B0 = 0,
    #[doc = "1: Framing error."]
    B1 = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error Flag"]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::B0,
            true => Fe::B1,
        }
    }
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fe::B0
    }
    #[doc = "Framing error."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fe::B1
    }
}
#[doc = "Field `FE` writer - Framing Error Flag"]
pub type FeW<'a, REG> = crate::BitWriter<'a, REG, Fe>;
impl<'a, REG> FeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No framing error detected. This does not guarantee the framing is correct."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::B0)
    }
    #[doc = "Framing error."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fe::B1)
    }
}
#[doc = "Noise Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nf {
    #[doc = "0: No noise detected."]
    B0 = 0,
    #[doc = "1: Noise detected in the received character in LPUART_DATA."]
    B1 = 1,
}
impl From<Nf> for bool {
    #[inline(always)]
    fn from(variant: Nf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NF` reader - Noise Flag"]
pub type NfR = crate::BitReader<Nf>;
impl NfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nf {
        match self.bits {
            false => Nf::B0,
            true => Nf::B1,
        }
    }
    #[doc = "No noise detected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Nf::B0
    }
    #[doc = "Noise detected in the received character in LPUART_DATA."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Nf::B1
    }
}
#[doc = "Field `NF` writer - Noise Flag"]
pub type NfW<'a, REG> = crate::BitWriter<'a, REG, Nf>;
impl<'a, REG> NfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No noise detected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::B0)
    }
    #[doc = "Noise detected in the received character in LPUART_DATA."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Nf::B1)
    }
}
#[doc = "Receiver Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Or {
    #[doc = "0: No overrun."]
    B0 = 0,
    #[doc = "1: Receive overrun (new LPUART data lost)."]
    B1 = 1,
}
impl From<Or> for bool {
    #[inline(always)]
    fn from(variant: Or) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OR` reader - Receiver Overrun Flag"]
pub type OrR = crate::BitReader<Or>;
impl OrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Or {
        match self.bits {
            false => Or::B0,
            true => Or::B1,
        }
    }
    #[doc = "No overrun."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Or::B0
    }
    #[doc = "Receive overrun (new LPUART data lost)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Or::B1
    }
}
#[doc = "Field `OR` writer - Receiver Overrun Flag"]
pub type OrW<'a, REG> = crate::BitWriter<'a, REG, Or>;
impl<'a, REG> OrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No overrun."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Or::B0)
    }
    #[doc = "Receive overrun (new LPUART data lost)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Or::B1)
    }
}
#[doc = "Idle Line Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: No idle line detected."]
    B0 = 0,
    #[doc = "1: Idle line was detected."]
    B1 = 1,
}
impl From<Idle> for bool {
    #[inline(always)]
    fn from(variant: Idle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLE` reader - Idle Line Flag"]
pub type IdleR = crate::BitReader<Idle>;
impl IdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idle {
        match self.bits {
            false => Idle::B0,
            true => Idle::B1,
        }
    }
    #[doc = "No idle line detected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Idle::B0
    }
    #[doc = "Idle line was detected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Idle::B1
    }
}
#[doc = "Field `IDLE` writer - Idle Line Flag"]
pub type IdleW<'a, REG> = crate::BitWriter<'a, REG, Idle>;
impl<'a, REG> IdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No idle line detected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::B0)
    }
    #[doc = "Idle line was detected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Idle::B1)
    }
}
#[doc = "Receive Data Register Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdrf {
    #[doc = "0: Receive data buffer empty."]
    B0 = 0,
    #[doc = "1: Receive data buffer full."]
    B1 = 1,
}
impl From<Rdrf> for bool {
    #[inline(always)]
    fn from(variant: Rdrf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDRF` reader - Receive Data Register Full Flag"]
pub type RdrfR = crate::BitReader<Rdrf>;
impl RdrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdrf {
        match self.bits {
            false => Rdrf::B0,
            true => Rdrf::B1,
        }
    }
    #[doc = "Receive data buffer empty."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rdrf::B0
    }
    #[doc = "Receive data buffer full."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rdrf::B1
    }
}
#[doc = "Transmission Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc {
    #[doc = "0: Transmitter active (sending data, a preamble, or a break)."]
    B0 = 0,
    #[doc = "1: Transmitter idle (transmission activity complete)."]
    B1 = 1,
}
impl From<Tc> for bool {
    #[inline(always)]
    fn from(variant: Tc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC` reader - Transmission Complete Flag"]
pub type TcR = crate::BitReader<Tc>;
impl TcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc {
        match self.bits {
            false => Tc::B0,
            true => Tc::B1,
        }
    }
    #[doc = "Transmitter active (sending data, a preamble, or a break)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tc::B0
    }
    #[doc = "Transmitter idle (transmission activity complete)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tc::B1
    }
}
#[doc = "Transmit Data Register Empty Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdre {
    #[doc = "0: Transmit data buffer full."]
    B0 = 0,
    #[doc = "1: Transmit data buffer empty."]
    B1 = 1,
}
impl From<Tdre> for bool {
    #[inline(always)]
    fn from(variant: Tdre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDRE` reader - Transmit Data Register Empty Flag"]
pub type TdreR = crate::BitReader<Tdre>;
impl TdreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdre {
        match self.bits {
            false => Tdre::B0,
            true => Tdre::B1,
        }
    }
    #[doc = "Transmit data buffer full."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tdre::B0
    }
    #[doc = "Transmit data buffer empty."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tdre::B1
    }
}
#[doc = "Receiver Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Raf {
    #[doc = "0: LPUART receiver idle waiting for a start bit."]
    B0 = 0,
    #[doc = "1: LPUART receiver active (LPUART_RX input not idle)."]
    B1 = 1,
}
impl From<Raf> for bool {
    #[inline(always)]
    fn from(variant: Raf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAF` reader - Receiver Active Flag"]
pub type RafR = crate::BitReader<Raf>;
impl RafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Raf {
        match self.bits {
            false => Raf::B0,
            true => Raf::B1,
        }
    }
    #[doc = "LPUART receiver idle waiting for a start bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Raf::B0
    }
    #[doc = "LPUART receiver active (LPUART_RX input not idle)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Raf::B1
    }
}
#[doc = "LIN Break Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbkde {
    #[doc = "0: Break character is detected at length 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1)."]
    B0 = 0,
    #[doc = "1: Break character is detected at length of 11 bit times (if M = 0, SBNS = 0) or 12 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 14 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 15 (if M10 = 1, SNBS = 1)."]
    B1 = 1,
}
impl From<Lbkde> for bool {
    #[inline(always)]
    fn from(variant: Lbkde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKDE` reader - LIN Break Detection Enable"]
pub type LbkdeR = crate::BitReader<Lbkde>;
impl LbkdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbkde {
        match self.bits {
            false => Lbkde::B0,
            true => Lbkde::B1,
        }
    }
    #[doc = "Break character is detected at length 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lbkde::B0
    }
    #[doc = "Break character is detected at length of 11 bit times (if M = 0, SBNS = 0) or 12 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 14 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 15 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lbkde::B1
    }
}
#[doc = "Field `LBKDE` writer - LIN Break Detection Enable"]
pub type LbkdeW<'a, REG> = crate::BitWriter<'a, REG, Lbkde>;
impl<'a, REG> LbkdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break character is detected at length 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbkde::B0)
    }
    #[doc = "Break character is detected at length of 11 bit times (if M = 0, SBNS = 0) or 12 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 14 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 15 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbkde::B1)
    }
}
#[doc = "Break Character Generation Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brk13 {
    #[doc = "0: Break character is transmitted with length of 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1)."]
    B0 = 0,
    #[doc = "1: Break character is transmitted with length of 13 bit times (if M = 0, SBNS = 0) or 14 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 15 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 16 (if M10 = 1, SNBS = 1)."]
    B1 = 1,
}
impl From<Brk13> for bool {
    #[inline(always)]
    fn from(variant: Brk13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK13` reader - Break Character Generation Length"]
pub type Brk13R = crate::BitReader<Brk13>;
impl Brk13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brk13 {
        match self.bits {
            false => Brk13::B0,
            true => Brk13::B1,
        }
    }
    #[doc = "Break character is transmitted with length of 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Brk13::B0
    }
    #[doc = "Break character is transmitted with length of 13 bit times (if M = 0, SBNS = 0) or 14 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 15 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 16 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Brk13::B1
    }
}
#[doc = "Field `BRK13` writer - Break Character Generation Length"]
pub type Brk13W<'a, REG> = crate::BitWriter<'a, REG, Brk13>;
impl<'a, REG> Brk13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break character is transmitted with length of 10 bit times (if M = 0, SBNS = 0) or 11 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 12 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 13 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Brk13::B0)
    }
    #[doc = "Break character is transmitted with length of 13 bit times (if M = 0, SBNS = 0) or 14 (if M = 1, SBNS = 0 or M = 0, SBNS = 1) or 15 (if M = 1, SBNS = 1 or M10 = 1, SNBS = 0) or 16 (if M10 = 1, SNBS = 1)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Brk13::B1)
    }
}
#[doc = "Receive Wake Up Idle Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwuid {
    #[doc = "0: During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not get set when an address does not match."]
    B0 = 0,
    #[doc = "1: During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does get set when an address does not match."]
    B1 = 1,
}
impl From<Rwuid> for bool {
    #[inline(always)]
    fn from(variant: Rwuid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWUID` reader - Receive Wake Up Idle Detect"]
pub type RwuidR = crate::BitReader<Rwuid>;
impl RwuidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwuid {
        match self.bits {
            false => Rwuid::B0,
            true => Rwuid::B1,
        }
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not get set when an address does not match."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rwuid::B0
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does get set when an address does not match."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rwuid::B1
    }
}
#[doc = "Field `RWUID` writer - Receive Wake Up Idle Detect"]
pub type RwuidW<'a, REG> = crate::BitWriter<'a, REG, Rwuid>;
impl<'a, REG> RwuidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "During receive standby state (RWU = 1), the IDLE bit does not get set upon detection of an idle character. During address match wakeup, the IDLE bit does not get set when an address does not match."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwuid::B0)
    }
    #[doc = "During receive standby state (RWU = 1), the IDLE bit gets set upon detection of an idle character. During address match wakeup, the IDLE bit does get set when an address does not match."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwuid::B1)
    }
}
#[doc = "Receive Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxinv {
    #[doc = "0: Receive data not inverted."]
    B0 = 0,
    #[doc = "1: Receive data inverted."]
    B1 = 1,
}
impl From<Rxinv> for bool {
    #[inline(always)]
    fn from(variant: Rxinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - Receive Data Inversion"]
pub type RxinvR = crate::BitReader<Rxinv>;
impl RxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxinv {
        match self.bits {
            false => Rxinv::B0,
            true => Rxinv::B1,
        }
    }
    #[doc = "Receive data not inverted."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxinv::B0
    }
    #[doc = "Receive data inverted."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxinv::B1
    }
}
#[doc = "Field `RXINV` writer - Receive Data Inversion"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG, Rxinv>;
impl<'a, REG> RxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive data not inverted."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::B0)
    }
    #[doc = "Receive data inverted."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::B1)
    }
}
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbf {
    #[doc = "0: LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    B0 = 0,
    #[doc = "1: MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\\[M\\], CTRL\\[PE\\]
and BAUD\\[M10\\]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL\\[M\\]
and CTRL\\[PE\\]."]
    B1 = 1,
}
impl From<Msbf> for bool {
    #[inline(always)]
    fn from(variant: Msbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBF` reader - MSB First"]
pub type MsbfR = crate::BitReader<Msbf>;
impl MsbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbf {
        match self.bits {
            false => Msbf::B0,
            true => Msbf::B1,
        }
    }
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Msbf::B0
    }
    #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\\[M\\], CTRL\\[PE\\]
and BAUD\\[M10\\]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL\\[M\\]
and CTRL\\[PE\\]."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Msbf::B1
    }
}
#[doc = "Field `MSBF` writer - MSB First"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG, Msbf>;
impl<'a, REG> MsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Msbf::B0)
    }
    #[doc = "MSB (bit9, bit8, bit7 or bit6) is the first bit that is transmitted following the start bit depending on the setting of CTRL\\[M\\], CTRL\\[PE\\]
and BAUD\\[M10\\]. Further, the first bit received after the start bit is identified as bit9, bit8, bit7 or bit6 depending on the setting of CTRL\\[M\\]
and CTRL\\[PE\\]."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Msbf::B1)
    }
}
#[doc = "LPUART_RX Pin Active Edge Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxedgif {
    #[doc = "0: No active edge on the receive pin has occurred."]
    B0 = 0,
    #[doc = "1: An active edge on the receive pin has occurred."]
    B1 = 1,
}
impl From<Rxedgif> for bool {
    #[inline(always)]
    fn from(variant: Rxedgif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEDGIF` reader - LPUART_RX Pin Active Edge Interrupt Flag"]
pub type RxedgifR = crate::BitReader<Rxedgif>;
impl RxedgifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxedgif {
        match self.bits {
            false => Rxedgif::B0,
            true => Rxedgif::B1,
        }
    }
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxedgif::B0
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxedgif::B1
    }
}
#[doc = "Field `RXEDGIF` writer - LPUART_RX Pin Active Edge Interrupt Flag"]
pub type RxedgifW<'a, REG> = crate::BitWriter<'a, REG, Rxedgif>;
impl<'a, REG> RxedgifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedgif::B0)
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedgif::B1)
    }
}
#[doc = "LIN Break Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbkdif {
    #[doc = "0: No LIN break character has been detected."]
    B0 = 0,
    #[doc = "1: LIN break character has been detected."]
    B1 = 1,
}
impl From<Lbkdif> for bool {
    #[inline(always)]
    fn from(variant: Lbkdif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKDIF` reader - LIN Break Detect Interrupt Flag"]
pub type LbkdifR = crate::BitReader<Lbkdif>;
impl LbkdifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbkdif {
        match self.bits {
            false => Lbkdif::B0,
            true => Lbkdif::B1,
        }
    }
    #[doc = "No LIN break character has been detected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lbkdif::B0
    }
    #[doc = "LIN break character has been detected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lbkdif::B1
    }
}
#[doc = "Field `LBKDIF` writer - LIN Break Detect Interrupt Flag"]
pub type LbkdifW<'a, REG> = crate::BitWriter<'a, REG, Lbkdif>;
impl<'a, REG> LbkdifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No LIN break character has been detected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbkdif::B0)
    }
    #[doc = "LIN break character has been detected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbkdif::B1)
    }
}
impl R {
    #[doc = "Bit 14 - Match 2 Flag"]
    #[inline(always)]
    pub fn ma2f(&self) -> Ma2fR {
        Ma2fR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Match 1 Flag"]
    #[inline(always)]
    pub fn ma1f(&self) -> Ma1fR {
        Ma1fR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Parity Error Flag"]
    #[inline(always)]
    pub fn pf(&self) -> PfR {
        PfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Framing Error Flag"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Noise Flag"]
    #[inline(always)]
    pub fn nf(&self) -> NfR {
        NfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Receiver Overrun Flag"]
    #[inline(always)]
    pub fn or(&self) -> OrR {
        OrR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Idle Line Flag"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Receive Data Register Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Complete Flag"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit Data Register Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receiver Active Flag"]
    #[inline(always)]
    pub fn raf(&self) -> RafR {
        RafR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&self) -> LbkdeR {
        LbkdeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Break Character Generation Length"]
    #[inline(always)]
    pub fn brk13(&self) -> Brk13R {
        Brk13R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Receive Wake Up Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&self) -> RwuidR {
        RwuidR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MSB First"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - LPUART_RX Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&self) -> RxedgifR {
        RxedgifR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&self) -> LbkdifR {
        LbkdifR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Match 2 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ma2f(&mut self) -> Ma2fW<StatSpec> {
        Ma2fW::new(self, 14)
    }
    #[doc = "Bit 15 - Match 1 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ma1f(&mut self) -> Ma1fW<StatSpec> {
        Ma1fW::new(self, 15)
    }
    #[doc = "Bit 16 - Parity Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PfW<StatSpec> {
        PfW::new(self, 16)
    }
    #[doc = "Bit 17 - Framing Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fe(&mut self) -> FeW<StatSpec> {
        FeW::new(self, 17)
    }
    #[doc = "Bit 18 - Noise Flag"]
    #[inline(always)]
    #[must_use]
    pub fn nf(&mut self) -> NfW<StatSpec> {
        NfW::new(self, 18)
    }
    #[doc = "Bit 19 - Receiver Overrun Flag"]
    #[inline(always)]
    #[must_use]
    pub fn or(&mut self) -> OrW<StatSpec> {
        OrW::new(self, 19)
    }
    #[doc = "Bit 20 - Idle Line Flag"]
    #[inline(always)]
    #[must_use]
    pub fn idle(&mut self) -> IdleW<StatSpec> {
        IdleW::new(self, 20)
    }
    #[doc = "Bit 25 - LIN Break Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbkde(&mut self) -> LbkdeW<StatSpec> {
        LbkdeW::new(self, 25)
    }
    #[doc = "Bit 26 - Break Character Generation Length"]
    #[inline(always)]
    #[must_use]
    pub fn brk13(&mut self) -> Brk13W<StatSpec> {
        Brk13W::new(self, 26)
    }
    #[doc = "Bit 27 - Receive Wake Up Idle Detect"]
    #[inline(always)]
    #[must_use]
    pub fn rwuid(&mut self) -> RwuidW<StatSpec> {
        RwuidW::new(self, 27)
    }
    #[doc = "Bit 28 - Receive Data Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RxinvW<StatSpec> {
        RxinvW::new(self, 28)
    }
    #[doc = "Bit 29 - MSB First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MsbfW<StatSpec> {
        MsbfW::new(self, 29)
    }
    #[doc = "Bit 30 - LPUART_RX Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxedgif(&mut self) -> RxedgifW<StatSpec> {
        RxedgifW::new(self, 30)
    }
    #[doc = "Bit 31 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lbkdif(&mut self) -> LbkdifW<StatSpec> {
        LbkdifW::new(self, 31)
    }
}
#[doc = "LPUART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`write(|w| ..)` method takes [`stat::W`](W) writer structure"]
impl crate::Writable for StatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STAT to value 0x00c0_0000"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0x00c0_0000;
}
