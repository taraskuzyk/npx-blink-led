#[doc = "Register `TCSR` reader"]
pub type R = crate::R<TcsrSpec>;
#[doc = "Register `TCSR` writer"]
pub type W = crate::W<TcsrSpec>;
#[doc = "FIFO Warning DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwde {
    #[doc = "0: Disables the DMA request."]
    B0 = 0,
    #[doc = "1: Enables the DMA request."]
    B1 = 1,
}
impl From<Fwde> for bool {
    #[inline(always)]
    fn from(variant: Fwde) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWDE` reader - FIFO Warning DMA Enable"]
pub type FwdeR = crate::BitReader<Fwde>;
impl FwdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwde {
        match self.bits {
            false => Fwde::B0,
            true => Fwde::B1,
        }
    }
    #[doc = "Disables the DMA request."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fwde::B0
    }
    #[doc = "Enables the DMA request."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fwde::B1
    }
}
#[doc = "Field `FWDE` writer - FIFO Warning DMA Enable"]
pub type FwdeW<'a, REG> = crate::BitWriter<'a, REG, Fwde>;
impl<'a, REG> FwdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the DMA request."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fwde::B0)
    }
    #[doc = "Enables the DMA request."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fwde::B1)
    }
}
#[doc = "FIFO Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwie {
    #[doc = "0: Disables the interrupt."]
    B0 = 0,
    #[doc = "1: Enables the interrupt."]
    B1 = 1,
}
impl From<Fwie> for bool {
    #[inline(always)]
    fn from(variant: Fwie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWIE` reader - FIFO Warning Interrupt Enable"]
pub type FwieR = crate::BitReader<Fwie>;
impl FwieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwie {
        match self.bits {
            false => Fwie::B0,
            true => Fwie::B1,
        }
    }
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fwie::B0
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fwie::B1
    }
}
#[doc = "Field `FWIE` writer - FIFO Warning Interrupt Enable"]
pub type FwieW<'a, REG> = crate::BitWriter<'a, REG, Fwie>;
impl<'a, REG> FwieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fwie::B0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fwie::B1)
    }
}
#[doc = "FIFO Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Feie {
    #[doc = "0: Disables the interrupt."]
    B0 = 0,
    #[doc = "1: Enables the interrupt."]
    B1 = 1,
}
impl From<Feie> for bool {
    #[inline(always)]
    fn from(variant: Feie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEIE` reader - FIFO Error Interrupt Enable"]
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
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Feie::B0
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Feie::B1
    }
}
#[doc = "Field `FEIE` writer - FIFO Error Interrupt Enable"]
pub type FeieW<'a, REG> = crate::BitWriter<'a, REG, Feie>;
impl<'a, REG> FeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Feie::B0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Feie::B1)
    }
}
#[doc = "Sync Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Seie {
    #[doc = "0: Disables interrupt."]
    B0 = 0,
    #[doc = "1: Enables interrupt."]
    B1 = 1,
}
impl From<Seie> for bool {
    #[inline(always)]
    fn from(variant: Seie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEIE` reader - Sync Error Interrupt Enable"]
pub type SeieR = crate::BitReader<Seie>;
impl SeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Seie {
        match self.bits {
            false => Seie::B0,
            true => Seie::B1,
        }
    }
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Seie::B0
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Seie::B1
    }
}
#[doc = "Field `SEIE` writer - Sync Error Interrupt Enable"]
pub type SeieW<'a, REG> = crate::BitWriter<'a, REG, Seie>;
impl<'a, REG> SeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Seie::B0)
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Seie::B1)
    }
}
#[doc = "Word Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wsie {
    #[doc = "0: Disables interrupt."]
    B0 = 0,
    #[doc = "1: Enables interrupt."]
    B1 = 1,
}
impl From<Wsie> for bool {
    #[inline(always)]
    fn from(variant: Wsie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSIE` reader - Word Start Interrupt Enable"]
pub type WsieR = crate::BitReader<Wsie>;
impl WsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wsie {
        match self.bits {
            false => Wsie::B0,
            true => Wsie::B1,
        }
    }
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wsie::B0
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wsie::B1
    }
}
#[doc = "Field `WSIE` writer - Word Start Interrupt Enable"]
pub type WsieW<'a, REG> = crate::BitWriter<'a, REG, Wsie>;
impl<'a, REG> WsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wsie::B0)
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wsie::B1)
    }
}
#[doc = "FIFO Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fwf {
    #[doc = "0: No enabled transmit FIFO is empty."]
    B0 = 0,
    #[doc = "1: Enabled transmit FIFO is empty."]
    B1 = 1,
}
impl From<Fwf> for bool {
    #[inline(always)]
    fn from(variant: Fwf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FWF` reader - FIFO Warning Flag"]
pub type FwfR = crate::BitReader<Fwf>;
impl FwfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fwf {
        match self.bits {
            false => Fwf::B0,
            true => Fwf::B1,
        }
    }
    #[doc = "No enabled transmit FIFO is empty."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fwf::B0
    }
    #[doc = "Enabled transmit FIFO is empty."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fwf::B1
    }
}
#[doc = "FIFO Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fef {
    #[doc = "0: Transmit underrun not detected."]
    B0 = 0,
    #[doc = "1: Transmit underrun detected."]
    B1 = 1,
}
impl From<Fef> for bool {
    #[inline(always)]
    fn from(variant: Fef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEF` reader - FIFO Error Flag"]
pub type FefR = crate::BitReader<Fef>;
impl FefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fef {
        match self.bits {
            false => Fef::B0,
            true => Fef::B1,
        }
    }
    #[doc = "Transmit underrun not detected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fef::B0
    }
    #[doc = "Transmit underrun detected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fef::B1
    }
}
#[doc = "Field `FEF` writer - FIFO Error Flag"]
pub type FefW<'a, REG> = crate::BitWriter<'a, REG, Fef>;
impl<'a, REG> FefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit underrun not detected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fef::B0)
    }
    #[doc = "Transmit underrun detected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fef::B1)
    }
}
#[doc = "Sync Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sef {
    #[doc = "0: Sync error not detected."]
    B0 = 0,
    #[doc = "1: Frame sync error detected."]
    B1 = 1,
}
impl From<Sef> for bool {
    #[inline(always)]
    fn from(variant: Sef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEF` reader - Sync Error Flag"]
pub type SefR = crate::BitReader<Sef>;
impl SefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sef {
        match self.bits {
            false => Sef::B0,
            true => Sef::B1,
        }
    }
    #[doc = "Sync error not detected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sef::B0
    }
    #[doc = "Frame sync error detected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sef::B1
    }
}
#[doc = "Field `SEF` writer - Sync Error Flag"]
pub type SefW<'a, REG> = crate::BitWriter<'a, REG, Sef>;
impl<'a, REG> SefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sync error not detected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sef::B0)
    }
    #[doc = "Frame sync error detected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sef::B1)
    }
}
#[doc = "Word Start Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wsf {
    #[doc = "0: Start of word not detected."]
    B0 = 0,
    #[doc = "1: Start of word detected."]
    B1 = 1,
}
impl From<Wsf> for bool {
    #[inline(always)]
    fn from(variant: Wsf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WSF` reader - Word Start Flag"]
pub type WsfR = crate::BitReader<Wsf>;
impl WsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wsf {
        match self.bits {
            false => Wsf::B0,
            true => Wsf::B1,
        }
    }
    #[doc = "Start of word not detected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wsf::B0
    }
    #[doc = "Start of word detected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wsf::B1
    }
}
#[doc = "Field `WSF` writer - Word Start Flag"]
pub type WsfW<'a, REG> = crate::BitWriter<'a, REG, Wsf>;
impl<'a, REG> WsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start of word not detected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wsf::B0)
    }
    #[doc = "Start of word detected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wsf::B1)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr {
    #[doc = "0: No effect."]
    B0 = 0,
    #[doc = "1: Software reset."]
    B1 = 1,
}
impl From<Sr> for bool {
    #[inline(always)]
    fn from(variant: Sr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Software Reset"]
pub type SrR = crate::BitReader<Sr>;
impl SrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr {
        match self.bits {
            false => Sr::B0,
            true => Sr::B1,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sr::B0
    }
    #[doc = "Software reset."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sr::B1
    }
}
#[doc = "Field `SR` writer - Software Reset"]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG, Sr>;
impl<'a, REG> SrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sr::B0)
    }
    #[doc = "Software reset."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sr::B1)
    }
}
#[doc = "FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fr {
    #[doc = "0: No effect."]
    B0 = 0,
    #[doc = "1: FIFO reset."]
    B1 = 1,
}
impl From<Fr> for bool {
    #[inline(always)]
    fn from(variant: Fr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FR` reader - FIFO Reset"]
pub type FrR = crate::BitReader<Fr>;
impl FrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fr {
        match self.bits {
            false => Fr::B0,
            true => Fr::B1,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fr::B0
    }
    #[doc = "FIFO reset."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fr::B1
    }
}
#[doc = "Field `FR` writer - FIFO Reset"]
pub type FrW<'a, REG> = crate::BitWriter<'a, REG, Fr>;
impl<'a, REG> FrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::B0)
    }
    #[doc = "FIFO reset."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fr::B1)
    }
}
#[doc = "Bit Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bce {
    #[doc = "0: Transmit bit clock is disabled."]
    B0 = 0,
    #[doc = "1: Transmit bit clock is enabled."]
    B1 = 1,
}
impl From<Bce> for bool {
    #[inline(always)]
    fn from(variant: Bce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BCE` reader - Bit Clock Enable"]
pub type BceR = crate::BitReader<Bce>;
impl BceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bce {
        match self.bits {
            false => Bce::B0,
            true => Bce::B1,
        }
    }
    #[doc = "Transmit bit clock is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bce::B0
    }
    #[doc = "Transmit bit clock is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bce::B1
    }
}
#[doc = "Field `BCE` writer - Bit Clock Enable"]
pub type BceW<'a, REG> = crate::BitWriter<'a, REG, Bce>;
impl<'a, REG> BceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmit bit clock is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bce::B0)
    }
    #[doc = "Transmit bit clock is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bce::B1)
    }
}
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbge {
    #[doc = "0: Transmitter is disabled in Debug mode, after completing the current frame."]
    B0 = 0,
    #[doc = "1: Transmitter is enabled in Debug mode."]
    B1 = 1,
}
impl From<Dbge> for bool {
    #[inline(always)]
    fn from(variant: Dbge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGE` reader - Debug Enable"]
pub type DbgeR = crate::BitReader<Dbge>;
impl DbgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbge {
        match self.bits {
            false => Dbge::B0,
            true => Dbge::B1,
        }
    }
    #[doc = "Transmitter is disabled in Debug mode, after completing the current frame."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dbge::B0
    }
    #[doc = "Transmitter is enabled in Debug mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dbge::B1
    }
}
#[doc = "Field `DBGE` writer - Debug Enable"]
pub type DbgeW<'a, REG> = crate::BitWriter<'a, REG, Dbge>;
impl<'a, REG> DbgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter is disabled in Debug mode, after completing the current frame."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbge::B0)
    }
    #[doc = "Transmitter is enabled in Debug mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbge::B1)
    }
}
#[doc = "Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stope {
    #[doc = "0: Transmitter disabled in Stop mode."]
    B0 = 0,
    #[doc = "1: Transmitter enabled in Stop mode."]
    B1 = 1,
}
impl From<Stope> for bool {
    #[inline(always)]
    fn from(variant: Stope) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPE` reader - Stop Enable"]
pub type StopeR = crate::BitReader<Stope>;
impl StopeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stope {
        match self.bits {
            false => Stope::B0,
            true => Stope::B1,
        }
    }
    #[doc = "Transmitter disabled in Stop mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stope::B0
    }
    #[doc = "Transmitter enabled in Stop mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stope::B1
    }
}
#[doc = "Field `STOPE` writer - Stop Enable"]
pub type StopeW<'a, REG> = crate::BitWriter<'a, REG, Stope>;
impl<'a, REG> StopeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Transmitter disabled in Stop mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stope::B0)
    }
    #[doc = "Transmitter enabled in Stop mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stope::B1)
    }
}
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Te {
    #[doc = "0: Transmitter is disabled."]
    B0 = 0,
    #[doc = "1: Transmitter is enabled, or transmitter has been disabled and has not yet reached end of frame."]
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
    #[doc = "Transmitter is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Te::B0
    }
    #[doc = "Transmitter is enabled, or transmitter has been disabled and has not yet reached end of frame."]
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
    #[doc = "Transmitter is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Te::B0)
    }
    #[doc = "Transmitter is enabled, or transmitter has been disabled and has not yet reached end of frame."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Te::B1)
    }
}
impl R {
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline(always)]
    pub fn fwde(&self) -> FwdeR {
        FwdeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub fn fwie(&self) -> FwieR {
        FwieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FeieR {
        FeieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&self) -> SeieR {
        SeieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline(always)]
    pub fn wsie(&self) -> WsieR {
        WsieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - FIFO Warning Flag"]
    #[inline(always)]
    pub fn fwf(&self) -> FwfR {
        FwfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FefR {
        FefR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline(always)]
    pub fn sef(&self) -> SefR {
        SefR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline(always)]
    pub fn wsf(&self) -> WsfR {
        WsfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FIFO Reset"]
    #[inline(always)]
    pub fn fr(&self) -> FrR {
        FrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline(always)]
    pub fn bce(&self) -> BceR {
        BceR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&self) -> DbgeR {
        DbgeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline(always)]
    pub fn stope(&self) -> StopeR {
        StopeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwde(&mut self) -> FwdeW<TcsrSpec> {
        FwdeW::new(self, 1)
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwie(&mut self) -> FwieW<TcsrSpec> {
        FwieW::new(self, 9)
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FeieW<TcsrSpec> {
        FeieW::new(self, 10)
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SeieW<TcsrSpec> {
        SeieW::new(self, 11)
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wsie(&mut self) -> WsieW<TcsrSpec> {
        WsieW::new(self, 12)
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FefW<TcsrSpec> {
        FefW::new(self, 18)
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sef(&mut self) -> SefW<TcsrSpec> {
        SefW::new(self, 19)
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wsf(&mut self) -> WsfW<TcsrSpec> {
        WsfW::new(self, 20)
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SrW<TcsrSpec> {
        SrW::new(self, 24)
    }
    #[doc = "Bit 25 - FIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn fr(&mut self) -> FrW<TcsrSpec> {
        FrW::new(self, 25)
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bce(&mut self) -> BceW<TcsrSpec> {
        BceW::new(self, 28)
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbge(&mut self) -> DbgeW<TcsrSpec> {
        DbgeW::new(self, 29)
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stope(&mut self) -> StopeW<TcsrSpec> {
        StopeW::new(self, 30)
    }
    #[doc = "Bit 31 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TeW<TcsrSpec> {
        TeW::new(self, 31)
    }
}
#[doc = "SAI Transmit Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcsrSpec;
impl crate::RegisterSpec for TcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcsr::R`](R) reader structure"]
impl crate::Readable for TcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcsr::W`](W) writer structure"]
impl crate::Writable for TcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCSR to value 0"]
impl crate::Resettable for TcsrSpec {
    const RESET_VALUE: u32 = 0;
}
