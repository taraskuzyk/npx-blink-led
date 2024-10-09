#[doc = "Register `C3` reader"]
pub type R = crate::R<C3Spec>;
#[doc = "Register `C3` writer"]
pub type W = crate::W<C3Spec>;
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Peie {
    #[doc = "0: PF interrupt requests are disabled."]
    B0 = 0,
    #[doc = "1: PF interrupt requests are enabled."]
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
    #[doc = "PF interrupt requests are disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Peie::B0
    }
    #[doc = "PF interrupt requests are enabled."]
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
    #[doc = "PF interrupt requests are disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::B0)
    }
    #[doc = "PF interrupt requests are enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Peie::B1)
    }
}
#[doc = "Framing Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feie {
    #[doc = "0: FE interrupt requests are disabled."]
    B0 = 0,
    #[doc = "1: FE interrupt requests are enabled."]
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
    #[doc = "FE interrupt requests are disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Feie::B0
    }
    #[doc = "FE interrupt requests are enabled."]
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
    #[doc = "FE interrupt requests are disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Feie::B0)
    }
    #[doc = "FE interrupt requests are enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Feie::B1)
    }
}
#[doc = "Noise Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Neie {
    #[doc = "0: NF interrupt requests are disabled."]
    B0 = 0,
    #[doc = "1: NF interrupt requests are enabled."]
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
    #[doc = "NF interrupt requests are disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Neie::B0
    }
    #[doc = "NF interrupt requests are enabled."]
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
    #[doc = "NF interrupt requests are disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Neie::B0)
    }
    #[doc = "NF interrupt requests are enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Neie::B1)
    }
}
#[doc = "Overrun Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orie {
    #[doc = "0: OR interrupts are disabled."]
    B0 = 0,
    #[doc = "1: OR interrupt requests are enabled."]
    B1 = 1,
}
impl From<Orie> for bool {
    #[inline(always)]
    fn from(variant: Orie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ORIE` reader - Overrun Error Interrupt Enable"]
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
    #[doc = "OR interrupts are disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Orie::B0
    }
    #[doc = "OR interrupt requests are enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Orie::B1
    }
}
#[doc = "Field `ORIE` writer - Overrun Error Interrupt Enable"]
pub type OrieW<'a, REG> = crate::BitWriter<'a, REG, Orie>;
impl<'a, REG> OrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OR interrupts are disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Orie::B0)
    }
    #[doc = "OR interrupt requests are enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Orie::B1)
    }
}
#[doc = "Transmit Data Inversion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txinv {
    #[doc = "0: Transmit data is not inverted."]
    B0 = 0,
    #[doc = "1: Transmit data is inverted."]
    B1 = 1,
}
impl From<Txinv> for bool {
    #[inline(always)]
    fn from(variant: Txinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - Transmit Data Inversion."]
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
    #[doc = "Transmit data is not inverted."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txinv::B0
    }
    #[doc = "Transmit data is inverted."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txinv::B1
    }
}
#[doc = "Field `TXINV` writer - Transmit Data Inversion."]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG, Txinv>;
impl<'a, REG> TxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit data is not inverted."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::B0)
    }
    #[doc = "Transmit data is inverted."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Txinv::B1)
    }
}
#[doc = "Transmitter Pin Data Direction in Single-Wire mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdir {
    #[doc = "0: TXD pin is an input in single wire mode."]
    B0 = 0,
    #[doc = "1: TXD pin is an output in single wire mode."]
    B1 = 1,
}
impl From<Txdir> for bool {
    #[inline(always)]
    fn from(variant: Txdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDIR` reader - Transmitter Pin Data Direction in Single-Wire mode"]
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
    #[doc = "TXD pin is an input in single wire mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txdir::B0
    }
    #[doc = "TXD pin is an output in single wire mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txdir::B1
    }
}
#[doc = "Field `TXDIR` writer - Transmitter Pin Data Direction in Single-Wire mode"]
pub type TxdirW<'a, REG> = crate::BitWriter<'a, REG, Txdir>;
impl<'a, REG> TxdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXD pin is an input in single wire mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Txdir::B0)
    }
    #[doc = "TXD pin is an output in single wire mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Txdir::B1)
    }
}
#[doc = "Field `T8` reader - Transmit Bit 8"]
pub type T8R = crate::BitReader;
#[doc = "Field `T8` writer - Transmit Bit 8"]
pub type T8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R8` reader - Received Bit 8"]
pub type R8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&self) -> PeieR {
        PeieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FeieR {
        FeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&self) -> NeieR {
        NeieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&self) -> OrieR {
        OrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Data Inversion."]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter Pin Data Direction in Single-Wire mode"]
    #[inline(always)]
    pub fn txdir(&self) -> TxdirR {
        TxdirR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit Bit 8"]
    #[inline(always)]
    pub fn t8(&self) -> T8R {
        T8R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Received Bit 8"]
    #[inline(always)]
    pub fn r8(&self) -> R8R {
        R8R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PeieW<C3Spec> {
        PeieW::new(self, 0)
    }
    #[doc = "Bit 1 - Framing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FeieW<C3Spec> {
        FeieW::new(self, 1)
    }
    #[doc = "Bit 2 - Noise Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn neie(&mut self) -> NeieW<C3Spec> {
        NeieW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn orie(&mut self) -> OrieW<C3Spec> {
        OrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Data Inversion."]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TxinvW<C3Spec> {
        TxinvW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmitter Pin Data Direction in Single-Wire mode"]
    #[inline(always)]
    #[must_use]
    pub fn txdir(&mut self) -> TxdirW<C3Spec> {
        TxdirW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn t8(&mut self) -> T8W<C3Spec> {
        T8W::new(self, 6)
    }
}
#[doc = "UART Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C3Spec;
impl crate::RegisterSpec for C3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c3::R`](R) reader structure"]
impl crate::Readable for C3Spec {}
#[doc = "`write(|w| ..)` method takes [`c3::W`](W) writer structure"]
impl crate::Writable for C3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C3 to value 0"]
impl crate::Resettable for C3Spec {
    const RESET_VALUE: u8 = 0;
}
