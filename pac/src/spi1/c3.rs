#[doc = "Register `C3` reader"]
pub type R = crate::R<C3Spec>;
#[doc = "Register `C3` writer"]
pub type W = crate::W<C3Spec>;
#[doc = "FIFO mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifomode {
    #[doc = "0: Buffer mode disabled"]
    B0 = 0,
    #[doc = "1: Data available in the receive data buffer"]
    B1 = 1,
}
impl From<Fifomode> for bool {
    #[inline(always)]
    fn from(variant: Fifomode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOMODE` reader - FIFO mode enable"]
pub type FifomodeR = crate::BitReader<Fifomode>;
impl FifomodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifomode {
        match self.bits {
            false => Fifomode::B0,
            true => Fifomode::B1,
        }
    }
    #[doc = "Buffer mode disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fifomode::B0
    }
    #[doc = "Data available in the receive data buffer"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fifomode::B1
    }
}
#[doc = "Field `FIFOMODE` writer - FIFO mode enable"]
pub type FifomodeW<'a, REG> = crate::BitWriter<'a, REG, Fifomode>;
impl<'a, REG> FifomodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Buffer mode disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fifomode::B0)
    }
    #[doc = "Data available in the receive data buffer"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fifomode::B1)
    }
}
#[doc = "Receive FIFO nearly full interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rnfullien {
    #[doc = "0: No interrupt upon RNFULLF being set"]
    B0 = 0,
    #[doc = "1: Enable interrupts upon RNFULLF being set"]
    B1 = 1,
}
impl From<Rnfullien> for bool {
    #[inline(always)]
    fn from(variant: Rnfullien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNFULLIEN` reader - Receive FIFO nearly full interrupt enable"]
pub type RnfullienR = crate::BitReader<Rnfullien>;
impl RnfullienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rnfullien {
        match self.bits {
            false => Rnfullien::B0,
            true => Rnfullien::B1,
        }
    }
    #[doc = "No interrupt upon RNFULLF being set"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rnfullien::B0
    }
    #[doc = "Enable interrupts upon RNFULLF being set"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rnfullien::B1
    }
}
#[doc = "Field `RNFULLIEN` writer - Receive FIFO nearly full interrupt enable"]
pub type RnfullienW<'a, REG> = crate::BitWriter<'a, REG, Rnfullien>;
impl<'a, REG> RnfullienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt upon RNFULLF being set"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rnfullien::B0)
    }
    #[doc = "Enable interrupts upon RNFULLF being set"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rnfullien::B1)
    }
}
#[doc = "Transmit FIFO nearly empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tnearien {
    #[doc = "0: No interrupt upon TNEAREF being set"]
    B0 = 0,
    #[doc = "1: Enable interrupts upon TNEAREF being set"]
    B1 = 1,
}
impl From<Tnearien> for bool {
    #[inline(always)]
    fn from(variant: Tnearien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNEARIEN` reader - Transmit FIFO nearly empty interrupt enable"]
pub type TnearienR = crate::BitReader<Tnearien>;
impl TnearienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tnearien {
        match self.bits {
            false => Tnearien::B0,
            true => Tnearien::B1,
        }
    }
    #[doc = "No interrupt upon TNEAREF being set"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tnearien::B0
    }
    #[doc = "Enable interrupts upon TNEAREF being set"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tnearien::B1
    }
}
#[doc = "Field `TNEARIEN` writer - Transmit FIFO nearly empty interrupt enable"]
pub type TnearienW<'a, REG> = crate::BitWriter<'a, REG, Tnearien>;
impl<'a, REG> TnearienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt upon TNEAREF being set"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tnearien::B0)
    }
    #[doc = "Enable interrupts upon TNEAREF being set"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tnearien::B1)
    }
}
#[doc = "Interrupt clearing mechanism select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intclr {
    #[doc = "0: These interrupts are cleared when the corresponding flags are cleared depending on the state of the FIFOs"]
    B0 = 0,
    #[doc = "1: These interrupts are cleared by writing the corresponding bits in the CI register"]
    B1 = 1,
}
impl From<Intclr> for bool {
    #[inline(always)]
    fn from(variant: Intclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTCLR` reader - Interrupt clearing mechanism select"]
pub type IntclrR = crate::BitReader<Intclr>;
impl IntclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Intclr {
        match self.bits {
            false => Intclr::B0,
            true => Intclr::B1,
        }
    }
    #[doc = "These interrupts are cleared when the corresponding flags are cleared depending on the state of the FIFOs"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Intclr::B0
    }
    #[doc = "These interrupts are cleared by writing the corresponding bits in the CI register"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Intclr::B1
    }
}
#[doc = "Field `INTCLR` writer - Interrupt clearing mechanism select"]
pub type IntclrW<'a, REG> = crate::BitWriter<'a, REG, Intclr>;
impl<'a, REG> IntclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "These interrupts are cleared when the corresponding flags are cleared depending on the state of the FIFOs"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Intclr::B0)
    }
    #[doc = "These interrupts are cleared by writing the corresponding bits in the CI register"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Intclr::B1)
    }
}
#[doc = "Receive FIFO nearly full watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RnfullfMark {
    #[doc = "0: RNFULLF is set when the receive FIFO has 48 bits or more"]
    B0 = 0,
    #[doc = "1: RNFULLF is set when the receive FIFO has 32 bits or more"]
    B1 = 1,
}
impl From<RnfullfMark> for bool {
    #[inline(always)]
    fn from(variant: RnfullfMark) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RNFULLF_MARK` reader - Receive FIFO nearly full watermark"]
pub type RnfullfMarkR = crate::BitReader<RnfullfMark>;
impl RnfullfMarkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RnfullfMark {
        match self.bits {
            false => RnfullfMark::B0,
            true => RnfullfMark::B1,
        }
    }
    #[doc = "RNFULLF is set when the receive FIFO has 48 bits or more"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RnfullfMark::B0
    }
    #[doc = "RNFULLF is set when the receive FIFO has 32 bits or more"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RnfullfMark::B1
    }
}
#[doc = "Field `RNFULLF_MARK` writer - Receive FIFO nearly full watermark"]
pub type RnfullfMarkW<'a, REG> = crate::BitWriter<'a, REG, RnfullfMark>;
impl<'a, REG> RnfullfMarkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RNFULLF is set when the receive FIFO has 48 bits or more"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RnfullfMark::B0)
    }
    #[doc = "RNFULLF is set when the receive FIFO has 32 bits or more"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RnfullfMark::B1)
    }
}
#[doc = "Transmit FIFO nearly empty watermark\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TnearefMark {
    #[doc = "0: TNEAREF is set when the transmit FIFO has 16 bits or less"]
    B0 = 0,
    #[doc = "1: TNEAREF is set when the transmit FIFO has 32 bits or less"]
    B1 = 1,
}
impl From<TnearefMark> for bool {
    #[inline(always)]
    fn from(variant: TnearefMark) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TNEAREF_MARK` reader - Transmit FIFO nearly empty watermark"]
pub type TnearefMarkR = crate::BitReader<TnearefMark>;
impl TnearefMarkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TnearefMark {
        match self.bits {
            false => TnearefMark::B0,
            true => TnearefMark::B1,
        }
    }
    #[doc = "TNEAREF is set when the transmit FIFO has 16 bits or less"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == TnearefMark::B0
    }
    #[doc = "TNEAREF is set when the transmit FIFO has 32 bits or less"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == TnearefMark::B1
    }
}
#[doc = "Field `TNEAREF_MARK` writer - Transmit FIFO nearly empty watermark"]
pub type TnearefMarkW<'a, REG> = crate::BitWriter<'a, REG, TnearefMark>;
impl<'a, REG> TnearefMarkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TNEAREF is set when the transmit FIFO has 16 bits or less"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(TnearefMark::B0)
    }
    #[doc = "TNEAREF is set when the transmit FIFO has 32 bits or less"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(TnearefMark::B1)
    }
}
impl R {
    #[doc = "Bit 0 - FIFO mode enable"]
    #[inline(always)]
    pub fn fifomode(&self) -> FifomodeR {
        FifomodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive FIFO nearly full interrupt enable"]
    #[inline(always)]
    pub fn rnfullien(&self) -> RnfullienR {
        RnfullienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO nearly empty interrupt enable"]
    #[inline(always)]
    pub fn tnearien(&self) -> TnearienR {
        TnearienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt clearing mechanism select"]
    #[inline(always)]
    pub fn intclr(&self) -> IntclrR {
        IntclrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO nearly full watermark"]
    #[inline(always)]
    pub fn rnfullf_mark(&self) -> RnfullfMarkR {
        RnfullfMarkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO nearly empty watermark"]
    #[inline(always)]
    pub fn tnearef_mark(&self) -> TnearefMarkR {
        TnearefMarkR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn fifomode(&mut self) -> FifomodeW<C3Spec> {
        FifomodeW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive FIFO nearly full interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rnfullien(&mut self) -> RnfullienW<C3Spec> {
        RnfullienW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO nearly empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tnearien(&mut self) -> TnearienW<C3Spec> {
        TnearienW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt clearing mechanism select"]
    #[inline(always)]
    #[must_use]
    pub fn intclr(&mut self) -> IntclrW<C3Spec> {
        IntclrW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive FIFO nearly full watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rnfullf_mark(&mut self) -> RnfullfMarkW<C3Spec> {
        RnfullfMarkW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO nearly empty watermark"]
    #[inline(always)]
    #[must_use]
    pub fn tnearef_mark(&mut self) -> TnearefMarkW<C3Spec> {
        TnearefMarkW::new(self, 5)
    }
}
#[doc = "SPI control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
