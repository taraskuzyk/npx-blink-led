#[doc = "Register `BAUD` reader"]
pub type R = crate::R<BaudSpec>;
#[doc = "Register `BAUD` writer"]
pub type W = crate::W<BaudSpec>;
#[doc = "Field `SBR` reader - Baud Rate Modulo Divisor."]
pub type SbrR = crate::FieldReader<u16>;
#[doc = "Field `SBR` writer - Baud Rate Modulo Divisor."]
pub type SbrW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Stop Bit Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbns {
    #[doc = "0: One stop bit."]
    B0 = 0,
    #[doc = "1: Two stop bits."]
    B1 = 1,
}
impl From<Sbns> for bool {
    #[inline(always)]
    fn from(variant: Sbns) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBNS` reader - Stop Bit Number Select"]
pub type SbnsR = crate::BitReader<Sbns>;
impl SbnsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbns {
        match self.bits {
            false => Sbns::B0,
            true => Sbns::B1,
        }
    }
    #[doc = "One stop bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sbns::B0
    }
    #[doc = "Two stop bits."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sbns::B1
    }
}
#[doc = "Field `SBNS` writer - Stop Bit Number Select"]
pub type SbnsW<'a, REG> = crate::BitWriter<'a, REG, Sbns>;
impl<'a, REG> SbnsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One stop bit."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sbns::B0)
    }
    #[doc = "Two stop bits."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sbns::B1)
    }
}
#[doc = "RX Input Active Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxedgie {
    #[doc = "0: Hardware interrupts from LPUART_STAT\\[RXEDGIF\\]
disabled (use polling)."]
    B0 = 0,
    #[doc = "1: Hardware interrupt requested when LPUART_STAT\\[RXEDGIF\\]
flag is 1."]
    B1 = 1,
}
impl From<Rxedgie> for bool {
    #[inline(always)]
    fn from(variant: Rxedgie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEDGIE` reader - RX Input Active Edge Interrupt Enable"]
pub type RxedgieR = crate::BitReader<Rxedgie>;
impl RxedgieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxedgie {
        match self.bits {
            false => Rxedgie::B0,
            true => Rxedgie::B1,
        }
    }
    #[doc = "Hardware interrupts from LPUART_STAT\\[RXEDGIF\\]
disabled (use polling)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxedgie::B0
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[RXEDGIF\\]
flag is 1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxedgie::B1
    }
}
#[doc = "Field `RXEDGIE` writer - RX Input Active Edge Interrupt Enable"]
pub type RxedgieW<'a, REG> = crate::BitWriter<'a, REG, Rxedgie>;
impl<'a, REG> RxedgieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware interrupts from LPUART_STAT\\[RXEDGIF\\]
disabled (use polling)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedgie::B0)
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[RXEDGIF\\]
flag is 1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedgie::B1)
    }
}
#[doc = "LIN Break Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lbkdie {
    #[doc = "0: Hardware interrupts from LPUART_STAT\\[LBKDIF\\]
disabled (use polling)."]
    B0 = 0,
    #[doc = "1: Hardware interrupt requested when LPUART_STAT\\[LBKDIF\\]
flag is 1."]
    B1 = 1,
}
impl From<Lbkdie> for bool {
    #[inline(always)]
    fn from(variant: Lbkdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LBKDIE` reader - LIN Break Detect Interrupt Enable"]
pub type LbkdieR = crate::BitReader<Lbkdie>;
impl LbkdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lbkdie {
        match self.bits {
            false => Lbkdie::B0,
            true => Lbkdie::B1,
        }
    }
    #[doc = "Hardware interrupts from LPUART_STAT\\[LBKDIF\\]
disabled (use polling)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lbkdie::B0
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[LBKDIF\\]
flag is 1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lbkdie::B1
    }
}
#[doc = "Field `LBKDIE` writer - LIN Break Detect Interrupt Enable"]
pub type LbkdieW<'a, REG> = crate::BitWriter<'a, REG, Lbkdie>;
impl<'a, REG> LbkdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware interrupts from LPUART_STAT\\[LBKDIF\\]
disabled (use polling)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lbkdie::B0)
    }
    #[doc = "Hardware interrupt requested when LPUART_STAT\\[LBKDIF\\]
flag is 1."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lbkdie::B1)
    }
}
#[doc = "Resynchronization Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resyncdis {
    #[doc = "0: Resynchronization during received data word is supported"]
    B0 = 0,
    #[doc = "1: Resynchronization during received data word is disabled"]
    B1 = 1,
}
impl From<Resyncdis> for bool {
    #[inline(always)]
    fn from(variant: Resyncdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESYNCDIS` reader - Resynchronization Disable"]
pub type ResyncdisR = crate::BitReader<Resyncdis>;
impl ResyncdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resyncdis {
        match self.bits {
            false => Resyncdis::B0,
            true => Resyncdis::B1,
        }
    }
    #[doc = "Resynchronization during received data word is supported"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Resyncdis::B0
    }
    #[doc = "Resynchronization during received data word is disabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Resyncdis::B1
    }
}
#[doc = "Field `RESYNCDIS` writer - Resynchronization Disable"]
pub type ResyncdisW<'a, REG> = crate::BitWriter<'a, REG, Resyncdis>;
impl<'a, REG> ResyncdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Resynchronization during received data word is supported"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Resyncdis::B0)
    }
    #[doc = "Resynchronization during received data word is disabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Resyncdis::B1)
    }
}
#[doc = "Both Edge Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bothedge {
    #[doc = "0: Receiver samples input data using the rising edge of the baud rate clock."]
    B0 = 0,
    #[doc = "1: Receiver samples input data using the rising and falling edge of the baud rate clock."]
    B1 = 1,
}
impl From<Bothedge> for bool {
    #[inline(always)]
    fn from(variant: Bothedge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOTHEDGE` reader - Both Edge Sampling"]
pub type BothedgeR = crate::BitReader<Bothedge>;
impl BothedgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bothedge {
        match self.bits {
            false => Bothedge::B0,
            true => Bothedge::B1,
        }
    }
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bothedge::B0
    }
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bothedge::B1
    }
}
#[doc = "Field `BOTHEDGE` writer - Both Edge Sampling"]
pub type BothedgeW<'a, REG> = crate::BitWriter<'a, REG, Bothedge>;
impl<'a, REG> BothedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver samples input data using the rising edge of the baud rate clock."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bothedge::B0)
    }
    #[doc = "Receiver samples input data using the rising and falling edge of the baud rate clock."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bothedge::B1)
    }
}
#[doc = "Match Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Matcfg {
    #[doc = "0: Address Match Wakeup"]
    B00 = 0,
    #[doc = "1: Idle Match Wakeup"]
    B01 = 1,
    #[doc = "2: Match On and Match Off"]
    B10 = 2,
    #[doc = "3: Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    B11 = 3,
}
impl From<Matcfg> for u8 {
    #[inline(always)]
    fn from(variant: Matcfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Matcfg {
    type Ux = u8;
}
impl crate::IsEnum for Matcfg {}
#[doc = "Field `MATCFG` reader - Match Configuration"]
pub type MatcfgR = crate::FieldReader<Matcfg>;
impl MatcfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Matcfg {
        match self.bits {
            0 => Matcfg::B00,
            1 => Matcfg::B01,
            2 => Matcfg::B10,
            3 => Matcfg::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Address Match Wakeup"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Matcfg::B00
    }
    #[doc = "Idle Match Wakeup"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Matcfg::B01
    }
    #[doc = "Match On and Match Off"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Matcfg::B10
    }
    #[doc = "Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Matcfg::B11
    }
}
#[doc = "Field `MATCFG` writer - Match Configuration"]
pub type MatcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Matcfg, crate::Safe>;
impl<'a, REG> MatcfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address Match Wakeup"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Matcfg::B00)
    }
    #[doc = "Idle Match Wakeup"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Matcfg::B01)
    }
    #[doc = "Match On and Match Off"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Matcfg::B10)
    }
    #[doc = "Enables RWU on Data Match and Match On/Off for transmitter CTS input"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Matcfg::B11)
    }
}
#[doc = "Receiver Full DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdmae {
    #[doc = "0: DMA request disabled."]
    B0 = 0,
    #[doc = "1: DMA request enabled."]
    B1 = 1,
}
impl From<Rdmae> for bool {
    #[inline(always)]
    fn from(variant: Rdmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDMAE` reader - Receiver Full DMA Enable"]
pub type RdmaeR = crate::BitReader<Rdmae>;
impl RdmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdmae {
        match self.bits {
            false => Rdmae::B0,
            true => Rdmae::B1,
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rdmae::B0
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rdmae::B1
    }
}
#[doc = "Field `RDMAE` writer - Receiver Full DMA Enable"]
pub type RdmaeW<'a, REG> = crate::BitWriter<'a, REG, Rdmae>;
impl<'a, REG> RdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdmae::B0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdmae::B1)
    }
}
#[doc = "Transmitter DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdmae {
    #[doc = "0: DMA request disabled."]
    B0 = 0,
    #[doc = "1: DMA request enabled."]
    B1 = 1,
}
impl From<Tdmae> for bool {
    #[inline(always)]
    fn from(variant: Tdmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMAE` reader - Transmitter DMA Enable"]
pub type TdmaeR = crate::BitReader<Tdmae>;
impl TdmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdmae {
        match self.bits {
            false => Tdmae::B0,
            true => Tdmae::B1,
        }
    }
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tdmae::B0
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tdmae::B1
    }
}
#[doc = "Field `TDMAE` writer - Transmitter DMA Enable"]
pub type TdmaeW<'a, REG> = crate::BitWriter<'a, REG, Tdmae>;
impl<'a, REG> TdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmae::B0)
    }
    #[doc = "DMA request enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmae::B1)
    }
}
#[doc = "Field `OSR` reader - Over Sampling Ratio"]
pub type OsrR = crate::FieldReader;
#[doc = "Field `OSR` writer - Over Sampling Ratio"]
pub type OsrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "10-bit Mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M10 {
    #[doc = "0: Receiver and transmitter use 8-bit or 9-bit data characters."]
    B0 = 0,
    #[doc = "1: Receiver and transmitter use 10-bit data characters."]
    B1 = 1,
}
impl From<M10> for bool {
    #[inline(always)]
    fn from(variant: M10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M10` reader - 10-bit Mode select"]
pub type M10R = crate::BitReader<M10>;
impl M10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> M10 {
        match self.bits {
            false => M10::B0,
            true => M10::B1,
        }
    }
    #[doc = "Receiver and transmitter use 8-bit or 9-bit data characters."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == M10::B0
    }
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == M10::B1
    }
}
#[doc = "Field `M10` writer - 10-bit Mode select"]
pub type M10W<'a, REG> = crate::BitWriter<'a, REG, M10>;
impl<'a, REG> M10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receiver and transmitter use 8-bit or 9-bit data characters."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(M10::B0)
    }
    #[doc = "Receiver and transmitter use 10-bit data characters."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(M10::B1)
    }
}
#[doc = "Match Address Mode Enable 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maen2 {
    #[doc = "0: Normal operation."]
    B0 = 0,
    #[doc = "1: Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    B1 = 1,
}
impl From<Maen2> for bool {
    #[inline(always)]
    fn from(variant: Maen2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAEN2` reader - Match Address Mode Enable 2"]
pub type Maen2R = crate::BitReader<Maen2>;
impl Maen2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maen2 {
        match self.bits {
            false => Maen2::B0,
            true => Maen2::B1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Maen2::B0
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Maen2::B1
    }
}
#[doc = "Field `MAEN2` writer - Match Address Mode Enable 2"]
pub type Maen2W<'a, REG> = crate::BitWriter<'a, REG, Maen2>;
impl<'a, REG> Maen2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Maen2::B0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA2\\]."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Maen2::B1)
    }
}
#[doc = "Match Address Mode Enable 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Maen1 {
    #[doc = "0: Normal operation."]
    B0 = 0,
    #[doc = "1: Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    B1 = 1,
}
impl From<Maen1> for bool {
    #[inline(always)]
    fn from(variant: Maen1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MAEN1` reader - Match Address Mode Enable 1"]
pub type Maen1R = crate::BitReader<Maen1>;
impl Maen1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maen1 {
        match self.bits {
            false => Maen1::B0,
            true => Maen1::B1,
        }
    }
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Maen1::B0
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Maen1::B1
    }
}
#[doc = "Field `MAEN1` writer - Match Address Mode Enable 1"]
pub type Maen1W<'a, REG> = crate::BitWriter<'a, REG, Maen1>;
impl<'a, REG> Maen1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Maen1::B0)
    }
    #[doc = "Enables automatic address matching or data matching mode for MATCH\\[MA1\\]."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Maen1::B1)
    }
}
impl R {
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline(always)]
    pub fn sbr(&self) -> SbrR {
        SbrR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&self) -> SbnsR {
        SbnsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub fn rxedgie(&self) -> RxedgieR {
        RxedgieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lbkdie(&self) -> LbkdieR {
        LbkdieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline(always)]
    pub fn resyncdis(&self) -> ResyncdisR {
        ResyncdisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline(always)]
    pub fn bothedge(&self) -> BothedgeR {
        BothedgeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline(always)]
    pub fn matcfg(&self) -> MatcfgR {
        MatcfgR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline(always)]
    pub fn rdmae(&self) -> RdmaeR {
        RdmaeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline(always)]
    pub fn tdmae(&self) -> TdmaeR {
        TdmaeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr(&self) -> OsrR {
        OsrR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline(always)]
    pub fn m10(&self) -> M10R {
        M10R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline(always)]
    pub fn maen2(&self) -> Maen2R {
        Maen2R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline(always)]
    pub fn maen1(&self) -> Maen1R {
        Maen1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - Baud Rate Modulo Divisor."]
    #[inline(always)]
    #[must_use]
    pub fn sbr(&mut self) -> SbrW<BaudSpec> {
        SbrW::new(self, 0)
    }
    #[doc = "Bit 13 - Stop Bit Number Select"]
    #[inline(always)]
    #[must_use]
    pub fn sbns(&mut self) -> SbnsW<BaudSpec> {
        SbnsW::new(self, 13)
    }
    #[doc = "Bit 14 - RX Input Active Edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxedgie(&mut self) -> RxedgieW<BaudSpec> {
        RxedgieW::new(self, 14)
    }
    #[doc = "Bit 15 - LIN Break Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbkdie(&mut self) -> LbkdieW<BaudSpec> {
        LbkdieW::new(self, 15)
    }
    #[doc = "Bit 16 - Resynchronization Disable"]
    #[inline(always)]
    #[must_use]
    pub fn resyncdis(&mut self) -> ResyncdisW<BaudSpec> {
        ResyncdisW::new(self, 16)
    }
    #[doc = "Bit 17 - Both Edge Sampling"]
    #[inline(always)]
    #[must_use]
    pub fn bothedge(&mut self) -> BothedgeW<BaudSpec> {
        BothedgeW::new(self, 17)
    }
    #[doc = "Bits 18:19 - Match Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn matcfg(&mut self) -> MatcfgW<BaudSpec> {
        MatcfgW::new(self, 18)
    }
    #[doc = "Bit 21 - Receiver Full DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdmae(&mut self) -> RdmaeW<BaudSpec> {
        RdmaeW::new(self, 21)
    }
    #[doc = "Bit 23 - Transmitter DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdmae(&mut self) -> TdmaeW<BaudSpec> {
        TdmaeW::new(self, 23)
    }
    #[doc = "Bits 24:28 - Over Sampling Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OsrW<BaudSpec> {
        OsrW::new(self, 24)
    }
    #[doc = "Bit 29 - 10-bit Mode select"]
    #[inline(always)]
    #[must_use]
    pub fn m10(&mut self) -> M10W<BaudSpec> {
        M10W::new(self, 29)
    }
    #[doc = "Bit 30 - Match Address Mode Enable 2"]
    #[inline(always)]
    #[must_use]
    pub fn maen2(&mut self) -> Maen2W<BaudSpec> {
        Maen2W::new(self, 30)
    }
    #[doc = "Bit 31 - Match Address Mode Enable 1"]
    #[inline(always)]
    #[must_use]
    pub fn maen1(&mut self) -> Maen1W<BaudSpec> {
        Maen1W::new(self, 31)
    }
}
#[doc = "LPUART Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BaudSpec;
impl crate::RegisterSpec for BaudSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`baud::R`](R) reader structure"]
impl crate::Readable for BaudSpec {}
#[doc = "`write(|w| ..)` method takes [`baud::W`](W) writer structure"]
impl crate::Writable for BaudSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BAUD to value 0x0f00_0004"]
impl crate::Resettable for BaudSpec {
    const RESET_VALUE: u32 = 0x0f00_0004;
}
