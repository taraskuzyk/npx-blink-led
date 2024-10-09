#[doc = "Register `S1` reader"]
pub type R = crate::R<S1Spec>;
#[doc = "Parity Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pf {
    #[doc = "0: No parity error detected since the last time this flag was cleared. If the receive buffer has a depth greater than 1, then there may be data in the receive buffer what was received with a parity error."]
    B0 = 0,
    #[doc = "1: At least one dataword was received with a parity error since the last time this flag was cleared."]
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
    #[doc = "No parity error detected since the last time this flag was cleared. If the receive buffer has a depth greater than 1, then there may be data in the receive buffer what was received with a parity error."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pf::B0
    }
    #[doc = "At least one dataword was received with a parity error since the last time this flag was cleared."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pf::B1
    }
}
#[doc = "Framing Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: No framing error detected."]
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
    #[doc = "No framing error detected."]
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
#[doc = "Noise Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nf {
    #[doc = "0: No noise detected since the last time this flag was cleared. If the receive buffer has a depth greater than 1 then there may be data in the receiver buffer that was received with noise."]
    B0 = 0,
    #[doc = "1: At least one dataword was received with noise detected since the last time the flag was cleared."]
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
    #[doc = "No noise detected since the last time this flag was cleared. If the receive buffer has a depth greater than 1 then there may be data in the receiver buffer that was received with noise."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Nf::B0
    }
    #[doc = "At least one dataword was received with noise detected since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Nf::B1
    }
}
#[doc = "Receiver Overrun Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Or {
    #[doc = "0: No overrun has occurred since the last time the flag was cleared."]
    B0 = 0,
    #[doc = "1: Overrun has occurred or the overrun flag has not been cleared since the last overrun occured."]
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
    #[doc = "No overrun has occurred since the last time the flag was cleared."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Or::B0
    }
    #[doc = "Overrun has occurred or the overrun flag has not been cleared since the last overrun occured."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Or::B1
    }
}
#[doc = "Idle Line Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idle {
    #[doc = "0: Receiver input is either active now or has never become active since the IDLE flag was last cleared."]
    B0 = 0,
    #[doc = "1: Receiver input has become idle or the flag has not been cleared since it last asserted."]
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
    #[doc = "Receiver input is either active now or has never become active since the IDLE flag was last cleared."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Idle::B0
    }
    #[doc = "Receiver input has become idle or the flag has not been cleared since it last asserted."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Idle::B1
    }
}
#[doc = "Receive Data Register Full Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdrf {
    #[doc = "0: The number of datawords in the receive buffer is less than the number indicated by RXWATER."]
    B0 = 0,
    #[doc = "1: The number of datawords in the receive buffer is equal to or greater than the number indicated by RXWATER at some point in time since this flag was last cleared."]
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
    #[doc = "The number of datawords in the receive buffer is less than the number indicated by RXWATER."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rdrf::B0
    }
    #[doc = "The number of datawords in the receive buffer is equal to or greater than the number indicated by RXWATER at some point in time since this flag was last cleared."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rdrf::B1
    }
}
#[doc = "Transmit Complete Flag\n\nValue on reset: 1"]
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
#[doc = "Field `TC` reader - Transmit Complete Flag"]
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
    #[doc = "0: The amount of data in the transmit buffer is greater than the value indicated by TWFIFO\\[TXWATER\\]."]
    B0 = 0,
    #[doc = "1: The amount of data in the transmit buffer is less than or equal to the value indicated by TWFIFO\\[TXWATER\\]
at some point in time since the flag has been cleared."]
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
    #[doc = "The amount of data in the transmit buffer is greater than the value indicated by TWFIFO\\[TXWATER\\]."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tdre::B0
    }
    #[doc = "The amount of data in the transmit buffer is less than or equal to the value indicated by TWFIFO\\[TXWATER\\]
at some point in time since the flag has been cleared."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tdre::B1
    }
}
impl R {
    #[doc = "Bit 0 - Parity Error Flag"]
    #[inline(always)]
    pub fn pf(&self) -> PfR {
        PfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing Error Flag"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise Flag"]
    #[inline(always)]
    pub fn nf(&self) -> NfR {
        NfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver Overrun Flag"]
    #[inline(always)]
    pub fn or(&self) -> OrR {
        OrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle Line Flag"]
    #[inline(always)]
    pub fn idle(&self) -> IdleR {
        IdleR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Data Register Full Flag"]
    #[inline(always)]
    pub fn rdrf(&self) -> RdrfR {
        RdrfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Complete Flag"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit Data Register Empty Flag"]
    #[inline(always)]
    pub fn tdre(&self) -> TdreR {
        TdreR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`s1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S1Spec;
impl crate::RegisterSpec for S1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s1::R`](R) reader structure"]
impl crate::Readable for S1Spec {}
#[doc = "`reset()` method sets S1 to value 0xc0"]
impl crate::Resettable for S1Spec {
    const RESET_VALUE: u8 = 0xc0;
}
