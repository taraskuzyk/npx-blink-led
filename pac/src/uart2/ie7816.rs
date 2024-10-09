#[doc = "Register `IE7816` reader"]
pub type R = crate::R<Ie7816Spec>;
#[doc = "Register `IE7816` writer"]
pub type W = crate::W<Ie7816Spec>;
#[doc = "Receive Threshold Exceeded Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxte {
    #[doc = "0: The assertion of IS7816\\[RXT\\]
does not result in the generation of an interrupt."]
    B0 = 0,
    #[doc = "1: The assertion of IS7816\\[RXT\\]
results in the generation of an interrupt."]
    B1 = 1,
}
impl From<Rxte> for bool {
    #[inline(always)]
    fn from(variant: Rxte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXTE` reader - Receive Threshold Exceeded Interrupt Enable"]
pub type RxteR = crate::BitReader<Rxte>;
impl RxteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxte {
        match self.bits {
            false => Rxte::B0,
            true => Rxte::B1,
        }
    }
    #[doc = "The assertion of IS7816\\[RXT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxte::B0
    }
    #[doc = "The assertion of IS7816\\[RXT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxte::B1
    }
}
#[doc = "Field `RXTE` writer - Receive Threshold Exceeded Interrupt Enable"]
pub type RxteW<'a, REG> = crate::BitWriter<'a, REG, Rxte>;
impl<'a, REG> RxteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The assertion of IS7816\\[RXT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxte::B0)
    }
    #[doc = "The assertion of IS7816\\[RXT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxte::B1)
    }
}
#[doc = "Transmit Threshold Exceeded Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txte {
    #[doc = "0: The assertion of IS7816\\[TXT\\]
does not result in the generation of an interrupt."]
    B0 = 0,
    #[doc = "1: The assertion of IS7816\\[TXT\\]
results in the generation of an interrupt."]
    B1 = 1,
}
impl From<Txte> for bool {
    #[inline(always)]
    fn from(variant: Txte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXTE` reader - Transmit Threshold Exceeded Interrupt Enable"]
pub type TxteR = crate::BitReader<Txte>;
impl TxteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txte {
        match self.bits {
            false => Txte::B0,
            true => Txte::B1,
        }
    }
    #[doc = "The assertion of IS7816\\[TXT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txte::B0
    }
    #[doc = "The assertion of IS7816\\[TXT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txte::B1
    }
}
#[doc = "Field `TXTE` writer - Transmit Threshold Exceeded Interrupt Enable"]
pub type TxteW<'a, REG> = crate::BitWriter<'a, REG, Txte>;
impl<'a, REG> TxteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The assertion of IS7816\\[TXT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Txte::B0)
    }
    #[doc = "The assertion of IS7816\\[TXT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Txte::B1)
    }
}
#[doc = "Guard Timer Violated Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gtve {
    #[doc = "0: The assertion of IS7816\\[GTV\\]
does not result in the generation of an interrupt."]
    B0 = 0,
    #[doc = "1: The assertion of IS7816\\[GTV\\]
results in the generation of an interrupt."]
    B1 = 1,
}
impl From<Gtve> for bool {
    #[inline(always)]
    fn from(variant: Gtve) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTVE` reader - Guard Timer Violated Interrupt Enable"]
pub type GtveR = crate::BitReader<Gtve>;
impl GtveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gtve {
        match self.bits {
            false => Gtve::B0,
            true => Gtve::B1,
        }
    }
    #[doc = "The assertion of IS7816\\[GTV\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gtve::B0
    }
    #[doc = "The assertion of IS7816\\[GTV\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gtve::B1
    }
}
#[doc = "Field `GTVE` writer - Guard Timer Violated Interrupt Enable"]
pub type GtveW<'a, REG> = crate::BitWriter<'a, REG, Gtve>;
impl<'a, REG> GtveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The assertion of IS7816\\[GTV\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gtve::B0)
    }
    #[doc = "The assertion of IS7816\\[GTV\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gtve::B1)
    }
}
#[doc = "ATR Duration Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adte {
    #[doc = "0: The assertion of IS7816\\[ADT\\]
does not result in the generation of an interrupt."]
    B0 = 0,
    #[doc = "1: The assertion of IS7816\\[ADT\\]
results in the generation of an interrupt."]
    B1 = 1,
}
impl From<Adte> for bool {
    #[inline(always)]
    fn from(variant: Adte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADTE` reader - ATR Duration Timer Interrupt Enable"]
pub type AdteR = crate::BitReader<Adte>;
impl AdteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adte {
        match self.bits {
            false => Adte::B0,
            true => Adte::B1,
        }
    }
    #[doc = "The assertion of IS7816\\[ADT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adte::B0
    }
    #[doc = "The assertion of IS7816\\[ADT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adte::B1
    }
}
#[doc = "Field `ADTE` writer - ATR Duration Timer Interrupt Enable"]
pub type AdteW<'a, REG> = crate::BitWriter<'a, REG, Adte>;
impl<'a, REG> AdteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The assertion of IS7816\\[ADT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adte::B0)
    }
    #[doc = "The assertion of IS7816\\[ADT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adte::B1)
    }
}
#[doc = "Initial Character Detected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Initde {
    #[doc = "0: The assertion of IS7816\\[INITD\\]
does not result in the generation of an interrupt."]
    B0 = 0,
    #[doc = "1: The assertion of IS7816\\[INITD\\]
results in the generation of an interrupt."]
    B1 = 1,
}
impl From<Initde> for bool {
    #[inline(always)]
    fn from(variant: Initde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INITDE` reader - Initial Character Detected Interrupt Enable"]
pub type InitdeR = crate::BitReader<Initde>;
impl InitdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Initde {
        match self.bits {
            false => Initde::B0,
            true => Initde::B1,
        }
    }
    #[doc = "The assertion of IS7816\\[INITD\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Initde::B0
    }
    #[doc = "The assertion of IS7816\\[INITD\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Initde::B1
    }
}
#[doc = "Field `INITDE` writer - Initial Character Detected Interrupt Enable"]
pub type InitdeW<'a, REG> = crate::BitWriter<'a, REG, Initde>;
impl<'a, REG> InitdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The assertion of IS7816\\[INITD\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Initde::B0)
    }
    #[doc = "The assertion of IS7816\\[INITD\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Initde::B1)
    }
}
#[doc = "Block Wait Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bwte {
    #[doc = "0: The assertion of IS7816\\[BWT\\]
does not result in the generation of an interrupt."]
    B0 = 0,
    #[doc = "1: The assertion of IS7816\\[BWT\\]
results in the generation of an interrupt."]
    B1 = 1,
}
impl From<Bwte> for bool {
    #[inline(always)]
    fn from(variant: Bwte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWTE` reader - Block Wait Timer Interrupt Enable"]
pub type BwteR = crate::BitReader<Bwte>;
impl BwteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bwte {
        match self.bits {
            false => Bwte::B0,
            true => Bwte::B1,
        }
    }
    #[doc = "The assertion of IS7816\\[BWT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bwte::B0
    }
    #[doc = "The assertion of IS7816\\[BWT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bwte::B1
    }
}
#[doc = "Field `BWTE` writer - Block Wait Timer Interrupt Enable"]
pub type BwteW<'a, REG> = crate::BitWriter<'a, REG, Bwte>;
impl<'a, REG> BwteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The assertion of IS7816\\[BWT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bwte::B0)
    }
    #[doc = "The assertion of IS7816\\[BWT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bwte::B1)
    }
}
#[doc = "Character Wait Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cwte {
    #[doc = "0: The assertion of IS7816\\[CWT\\]
does not result in the generation of an interrupt."]
    B0 = 0,
    #[doc = "1: The assertion of IS7816\\[CWT\\]
results in the generation of an interrupt."]
    B1 = 1,
}
impl From<Cwte> for bool {
    #[inline(always)]
    fn from(variant: Cwte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWTE` reader - Character Wait Timer Interrupt Enable"]
pub type CwteR = crate::BitReader<Cwte>;
impl CwteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cwte {
        match self.bits {
            false => Cwte::B0,
            true => Cwte::B1,
        }
    }
    #[doc = "The assertion of IS7816\\[CWT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cwte::B0
    }
    #[doc = "The assertion of IS7816\\[CWT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cwte::B1
    }
}
#[doc = "Field `CWTE` writer - Character Wait Timer Interrupt Enable"]
pub type CwteW<'a, REG> = crate::BitWriter<'a, REG, Cwte>;
impl<'a, REG> CwteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The assertion of IS7816\\[CWT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cwte::B0)
    }
    #[doc = "The assertion of IS7816\\[CWT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cwte::B1)
    }
}
#[doc = "Wait Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wte {
    #[doc = "0: The assertion of IS7816\\[WT\\]
does not result in the generation of an interrupt."]
    B0 = 0,
    #[doc = "1: The assertion of IS7816\\[WT\\]
results in the generation of an interrupt."]
    B1 = 1,
}
impl From<Wte> for bool {
    #[inline(always)]
    fn from(variant: Wte) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTE` reader - Wait Timer Interrupt Enable"]
pub type WteR = crate::BitReader<Wte>;
impl WteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wte {
        match self.bits {
            false => Wte::B0,
            true => Wte::B1,
        }
    }
    #[doc = "The assertion of IS7816\\[WT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wte::B0
    }
    #[doc = "The assertion of IS7816\\[WT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wte::B1
    }
}
#[doc = "Field `WTE` writer - Wait Timer Interrupt Enable"]
pub type WteW<'a, REG> = crate::BitWriter<'a, REG, Wte>;
impl<'a, REG> WteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The assertion of IS7816\\[WT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wte::B0)
    }
    #[doc = "The assertion of IS7816\\[WT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wte::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    pub fn rxte(&self) -> RxteR {
        RxteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    pub fn txte(&self) -> TxteR {
        TxteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt Enable"]
    #[inline(always)]
    pub fn gtve(&self) -> GtveR {
        GtveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ATR Duration Timer Interrupt Enable"]
    #[inline(always)]
    pub fn adte(&self) -> AdteR {
        AdteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt Enable"]
    #[inline(always)]
    pub fn initde(&self) -> InitdeR {
        InitdeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn bwte(&self) -> BwteR {
        BwteR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn cwte(&self) -> CwteR {
        CwteR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&self) -> WteR {
        WteR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxte(&mut self) -> RxteW<Ie7816Spec> {
        RxteW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txte(&mut self) -> TxteW<Ie7816Spec> {
        TxteW::new(self, 1)
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gtve(&mut self) -> GtveW<Ie7816Spec> {
        GtveW::new(self, 2)
    }
    #[doc = "Bit 3 - ATR Duration Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adte(&mut self) -> AdteW<Ie7816Spec> {
        AdteW::new(self, 3)
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn initde(&mut self) -> InitdeW<Ie7816Spec> {
        InitdeW::new(self, 4)
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwte(&mut self) -> BwteW<Ie7816Spec> {
        BwteW::new(self, 5)
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cwte(&mut self) -> CwteW<Ie7816Spec> {
        CwteW::new(self, 6)
    }
    #[doc = "Bit 7 - Wait Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wte(&mut self) -> WteW<Ie7816Spec> {
        WteW::new(self, 7)
    }
}
#[doc = "UART 7816 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie7816::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie7816::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ie7816Spec;
impl crate::RegisterSpec for Ie7816Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ie7816::R`](R) reader structure"]
impl crate::Readable for Ie7816Spec {}
#[doc = "`write(|w| ..)` method takes [`ie7816::W`](W) writer structure"]
impl crate::Writable for Ie7816Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets IE7816 to value 0"]
impl crate::Resettable for Ie7816Spec {
    const RESET_VALUE: u8 = 0;
}
