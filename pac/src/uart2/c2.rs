#[doc = "Register `C2` reader"]
pub type R = crate::R<C2Spec>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2Spec>;
#[doc = "Send Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbk {
    #[doc = "0: Normal transmitter operation."]
    B0 = 0,
    #[doc = "1: Queue break characters to be sent."]
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
    #[doc = "Queue break characters to be sent."]
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
    #[doc = "Queue break characters to be sent."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbk::B1)
    }
}
#[doc = "Receiver Wakeup Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwu {
    #[doc = "0: Normal operation."]
    B0 = 0,
    #[doc = "1: RWU enables the wakeup function and inhibits further receiver interrupt requests. Normally, hardware wakes the receiver by automatically clearing RWU."]
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
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rwu::B0
    }
    #[doc = "RWU enables the wakeup function and inhibits further receiver interrupt requests. Normally, hardware wakes the receiver by automatically clearing RWU."]
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
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwu::B0)
    }
    #[doc = "RWU enables the wakeup function and inhibits further receiver interrupt requests. Normally, hardware wakes the receiver by automatically clearing RWU."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwu::B1)
    }
}
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Re {
    #[doc = "0: Receiver off."]
    B0 = 0,
    #[doc = "1: Receiver on."]
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
    #[doc = "Receiver off."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Re::B0
    }
    #[doc = "Receiver on."]
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
    #[doc = "Receiver off."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Re::B0)
    }
    #[doc = "Receiver on."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Re::B1)
    }
}
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te {
    #[doc = "0: Transmitter off."]
    B0 = 0,
    #[doc = "1: Transmitter on."]
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
    #[doc = "Transmitter off."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Te::B0
    }
    #[doc = "Transmitter on."]
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
    #[doc = "Transmitter off."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Te::B0)
    }
    #[doc = "Transmitter on."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Te::B1)
    }
}
#[doc = "Idle Line Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilie {
    #[doc = "0: IDLE interrupt requests disabled."]
    B0 = 0,
    #[doc = "1: IDLE interrupt requests enabled."]
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
    #[doc = "IDLE interrupt requests disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ilie::B0
    }
    #[doc = "IDLE interrupt requests enabled."]
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
    #[doc = "IDLE interrupt requests disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ilie::B0)
    }
    #[doc = "IDLE interrupt requests enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ilie::B1)
    }
}
#[doc = "Receiver Full Interrupt or DMA Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rie {
    #[doc = "0: RDRF interrupt and DMA transfer requests disabled."]
    B0 = 0,
    #[doc = "1: RDRF interrupt or DMA transfer requests enabled."]
    B1 = 1,
}
impl From<Rie> for bool {
    #[inline(always)]
    fn from(variant: Rie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIE` reader - Receiver Full Interrupt or DMA Transfer Enable"]
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
    #[doc = "RDRF interrupt and DMA transfer requests disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rie::B0
    }
    #[doc = "RDRF interrupt or DMA transfer requests enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rie::B1
    }
}
#[doc = "Field `RIE` writer - Receiver Full Interrupt or DMA Transfer Enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG, Rie>;
impl<'a, REG> RieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RDRF interrupt and DMA transfer requests disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::B0)
    }
    #[doc = "RDRF interrupt or DMA transfer requests enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rie::B1)
    }
}
#[doc = "Transmission Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcie {
    #[doc = "0: TC interrupt requests disabled."]
    B0 = 0,
    #[doc = "1: TC interrupt requests enabled."]
    B1 = 1,
}
impl From<Tcie> for bool {
    #[inline(always)]
    fn from(variant: Tcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transmission Complete Interrupt Enable"]
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
    #[doc = "TC interrupt requests disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcie::B0
    }
    #[doc = "TC interrupt requests enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcie::B1
    }
}
#[doc = "Field `TCIE` writer - Transmission Complete Interrupt Enable"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG, Tcie>;
impl<'a, REG> TcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC interrupt requests disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B0)
    }
    #[doc = "TC interrupt requests enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcie::B1)
    }
}
#[doc = "Transmitter Interrupt or DMA Transfer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tie {
    #[doc = "0: TDRE interrupt and DMA transfer requests disabled."]
    B0 = 0,
    #[doc = "1: TDRE interrupt or DMA transfer requests enabled."]
    B1 = 1,
}
impl From<Tie> for bool {
    #[inline(always)]
    fn from(variant: Tie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Transmitter Interrupt or DMA Transfer Enable."]
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
    #[doc = "TDRE interrupt and DMA transfer requests disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tie::B0
    }
    #[doc = "TDRE interrupt or DMA transfer requests enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tie::B1
    }
}
#[doc = "Field `TIE` writer - Transmitter Interrupt or DMA Transfer Enable."]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG, Tie>;
impl<'a, REG> TieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TDRE interrupt and DMA transfer requests disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B0)
    }
    #[doc = "TDRE interrupt or DMA transfer requests enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tie::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Send Break"]
    #[inline(always)]
    pub fn sbk(&self) -> SbkR {
        SbkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&self) -> RwuR {
        RwuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle Line Interrupt Enable"]
    #[inline(always)]
    pub fn ilie(&self) -> IlieR {
        IlieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Full Interrupt or DMA Transfer Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter Interrupt or DMA Transfer Enable."]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send Break"]
    #[inline(always)]
    #[must_use]
    pub fn sbk(&mut self) -> SbkW<C2Spec> {
        SbkW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver Wakeup Control"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RwuW<C2Spec> {
        RwuW::new(self, 1)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> ReW<C2Spec> {
        ReW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<C2Spec> {
        TeW::new(self, 3)
    }
    #[doc = "Bit 4 - Idle Line Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ilie(&mut self) -> IlieW<C2Spec> {
        IlieW::new(self, 4)
    }
    #[doc = "Bit 5 - Receiver Full Interrupt or DMA Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RieW<C2Spec> {
        RieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmission Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TcieW<C2Spec> {
        TcieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmitter Interrupt or DMA Transfer Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TieW<C2Spec> {
        TieW::new(self, 7)
    }
}
#[doc = "UART Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2Spec;
impl crate::RegisterSpec for C2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c2::R`](R) reader structure"]
impl crate::Readable for C2Spec {}
#[doc = "`write(|w| ..)` method takes [`c2::W`](W) writer structure"]
impl crate::Writable for C2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2Spec {
    const RESET_VALUE: u8 = 0;
}
