#[doc = "Register `C1` reader"]
pub type R = crate::R<C1Spec>;
#[doc = "Register `C1` writer"]
pub type W = crate::W<C1Spec>;
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
    #[doc = "0: Parity function disabled."]
    B0 = 0,
    #[doc = "1: Parity function enabled."]
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
    #[doc = "Parity function disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pe::B0
    }
    #[doc = "Parity function enabled."]
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
    #[doc = "Parity function disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::B0)
    }
    #[doc = "Parity function enabled."]
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
    #[doc = "0: Idle line wakeup."]
    B0 = 0,
    #[doc = "1: Address mark wakeup."]
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
    #[doc = "Idle line wakeup."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wake::B0
    }
    #[doc = "Address mark wakeup."]
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
    #[doc = "Idle line wakeup."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wake::B0)
    }
    #[doc = "Address mark wakeup."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wake::B1)
    }
}
#[doc = "9-bit or 8-bit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M {
    #[doc = "0: Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop."]
    B0 = 0,
    #[doc = "1: Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop."]
    B1 = 1,
}
impl From<M> for bool {
    #[inline(always)]
    fn from(variant: M) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M` reader - 9-bit or 8-bit Mode Select"]
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
    #[doc = "Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == M::B0
    }
    #[doc = "Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == M::B1
    }
}
#[doc = "Field `M` writer - 9-bit or 8-bit Mode Select"]
pub type MW<'a, REG> = crate::BitWriter<'a, REG, M>;
impl<'a, REG> MW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(M::B0)
    }
    #[doc = "Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(M::B1)
    }
}
#[doc = "Receiver Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsrc {
    #[doc = "0: Selects internal loop back mode. The receiver input is internally connected to transmitter output."]
    B0 = 0,
    #[doc = "1: Single wire UART mode where the receiver input is connected to the transmit pin input signal."]
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
    #[doc = "Selects internal loop back mode. The receiver input is internally connected to transmitter output."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rsrc::B0
    }
    #[doc = "Single wire UART mode where the receiver input is connected to the transmit pin input signal."]
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
    #[doc = "Selects internal loop back mode. The receiver input is internally connected to transmitter output."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rsrc::B0)
    }
    #[doc = "Single wire UART mode where the receiver input is connected to the transmit pin input signal."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rsrc::B1)
    }
}
#[doc = "Loop Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Loops {
    #[doc = "0: Normal operation."]
    B0 = 0,
    #[doc = "1: Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC."]
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
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Loops::B0
    }
    #[doc = "Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC."]
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
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Loops::B0)
    }
    #[doc = "Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Loops::B1)
    }
}
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
    #[doc = "Bit 4 - 9-bit or 8-bit Mode Select"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&self) -> RsrcR {
        RsrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&self) -> LoopsR {
        LoopsR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PtW<C1Spec> {
        PtW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<C1Spec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn ilt(&mut self) -> IltW<C1Spec> {
        IltW::new(self, 2)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WakeW<C1Spec> {
        WakeW::new(self, 3)
    }
    #[doc = "Bit 4 - 9-bit or 8-bit Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> MW<C1Spec> {
        MW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn rsrc(&mut self) -> RsrcW<C1Spec> {
        RsrcW::new(self, 5)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn loops(&mut self) -> LoopsW<C1Spec> {
        LoopsW::new(self, 7)
    }
}
#[doc = "UART Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Spec;
impl crate::RegisterSpec for C1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c1::R`](R) reader structure"]
impl crate::Readable for C1Spec {}
#[doc = "`write(|w| ..)` method takes [`c1::W`](W) writer structure"]
impl crate::Writable for C1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1Spec {
    const RESET_VALUE: u8 = 0;
}
