#[doc = "Register `SOPT2` reader"]
pub type R = crate::R<Sopt2Spec>;
#[doc = "Register `SOPT2` writer"]
pub type W = crate::W<Sopt2Spec>;
#[doc = "RTC Clock Out Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtcclkoutsel {
    #[doc = "0: RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    B0 = 0,
    #[doc = "1: OSCERCLK clock is output on the RTC_CLKOUT pin."]
    B1 = 1,
}
impl From<Rtcclkoutsel> for bool {
    #[inline(always)]
    fn from(variant: Rtcclkoutsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCCLKOUTSEL` reader - RTC Clock Out Select"]
pub type RtcclkoutselR = crate::BitReader<Rtcclkoutsel>;
impl RtcclkoutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtcclkoutsel {
        match self.bits {
            false => Rtcclkoutsel::B0,
            true => Rtcclkoutsel::B1,
        }
    }
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rtcclkoutsel::B0
    }
    #[doc = "OSCERCLK clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rtcclkoutsel::B1
    }
}
#[doc = "Field `RTCCLKOUTSEL` writer - RTC Clock Out Select"]
pub type RtcclkoutselW<'a, REG> = crate::BitWriter<'a, REG, Rtcclkoutsel>;
impl<'a, REG> RtcclkoutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcclkoutsel::B0)
    }
    #[doc = "OSCERCLK clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rtcclkoutsel::B1)
    }
}
#[doc = "CLKOUT select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkoutsel {
    #[doc = "2: Bus clock"]
    B010 = 2,
    #[doc = "3: LPO clock (1 kHz)"]
    B011 = 3,
    #[doc = "4: LIRC_CLK"]
    B100 = 4,
    #[doc = "6: OSCERCLK"]
    B110 = 6,
    #[doc = "7: IRC48M clock (IRC48M clock can be output to PAD only when chip VDD is 2.7-3.6 V)"]
    B111 = 7,
}
impl From<Clkoutsel> for u8 {
    #[inline(always)]
    fn from(variant: Clkoutsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkoutsel {
    type Ux = u8;
}
impl crate::IsEnum for Clkoutsel {}
#[doc = "Field `CLKOUTSEL` reader - CLKOUT select"]
pub type ClkoutselR = crate::FieldReader<Clkoutsel>;
impl ClkoutselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkoutsel> {
        match self.bits {
            2 => Some(Clkoutsel::B010),
            3 => Some(Clkoutsel::B011),
            4 => Some(Clkoutsel::B100),
            6 => Some(Clkoutsel::B110),
            7 => Some(Clkoutsel::B111),
            _ => None,
        }
    }
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Clkoutsel::B010
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Clkoutsel::B011
    }
    #[doc = "LIRC_CLK"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Clkoutsel::B100
    }
    #[doc = "OSCERCLK"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Clkoutsel::B110
    }
    #[doc = "IRC48M clock (IRC48M clock can be output to PAD only when chip VDD is 2.7-3.6 V)"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Clkoutsel::B111
    }
}
#[doc = "Field `CLKOUTSEL` writer - CLKOUT select"]
pub type ClkoutselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Clkoutsel>;
impl<'a, REG> ClkoutselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::B010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::B011)
    }
    #[doc = "LIRC_CLK"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::B100)
    }
    #[doc = "OSCERCLK"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::B110)
    }
    #[doc = "IRC48M clock (IRC48M clock can be output to PAD only when chip VDD is 2.7-3.6 V)"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Clkoutsel::B111)
    }
}
#[doc = "USB clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbsrc {
    #[doc = "0: External bypass clock (USB_CLKIN)."]
    B0 = 0,
    #[doc = "1: IRC48M clock"]
    B1 = 1,
}
impl From<Usbsrc> for bool {
    #[inline(always)]
    fn from(variant: Usbsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBSRC` reader - USB clock source select"]
pub type UsbsrcR = crate::BitReader<Usbsrc>;
impl UsbsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbsrc {
        match self.bits {
            false => Usbsrc::B0,
            true => Usbsrc::B1,
        }
    }
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbsrc::B0
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbsrc::B1
    }
}
#[doc = "Field `USBSRC` writer - USB clock source select"]
pub type UsbsrcW<'a, REG> = crate::BitWriter<'a, REG, Usbsrc>;
impl<'a, REG> UsbsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsrc::B0)
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsrc::B1)
    }
}
#[doc = "FlexIO Module Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flexiosrc {
    #[doc = "0: Clock disabled"]
    B00 = 0,
    #[doc = "1: IRC48M clock"]
    B01 = 1,
    #[doc = "2: OSCERCLK clock"]
    B10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    B11 = 3,
}
impl From<Flexiosrc> for u8 {
    #[inline(always)]
    fn from(variant: Flexiosrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flexiosrc {
    type Ux = u8;
}
impl crate::IsEnum for Flexiosrc {}
#[doc = "Field `FLEXIOSRC` reader - FlexIO Module Clock Source Select"]
pub type FlexiosrcR = crate::FieldReader<Flexiosrc>;
impl FlexiosrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexiosrc {
        match self.bits {
            0 => Flexiosrc::B00,
            1 => Flexiosrc::B01,
            2 => Flexiosrc::B10,
            3 => Flexiosrc::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Flexiosrc::B00
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Flexiosrc::B01
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Flexiosrc::B10
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Flexiosrc::B11
    }
}
#[doc = "Field `FLEXIOSRC` writer - FlexIO Module Clock Source Select"]
pub type FlexiosrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flexiosrc, crate::Safe>;
impl<'a, REG> FlexiosrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Flexiosrc::B00)
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Flexiosrc::B01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Flexiosrc::B10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Flexiosrc::B11)
    }
}
#[doc = "TPM Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tpmsrc {
    #[doc = "0: Clock disabled"]
    B00 = 0,
    #[doc = "1: IRC48M clock"]
    B01 = 1,
    #[doc = "2: OSCERCLK clock"]
    B10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    B11 = 3,
}
impl From<Tpmsrc> for u8 {
    #[inline(always)]
    fn from(variant: Tpmsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tpmsrc {
    type Ux = u8;
}
impl crate::IsEnum for Tpmsrc {}
#[doc = "Field `TPMSRC` reader - TPM Clock Source Select"]
pub type TpmsrcR = crate::FieldReader<Tpmsrc>;
impl TpmsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpmsrc {
        match self.bits {
            0 => Tpmsrc::B00,
            1 => Tpmsrc::B01,
            2 => Tpmsrc::B10,
            3 => Tpmsrc::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Tpmsrc::B00
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Tpmsrc::B01
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Tpmsrc::B10
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Tpmsrc::B11
    }
}
#[doc = "Field `TPMSRC` writer - TPM Clock Source Select"]
pub type TpmsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tpmsrc, crate::Safe>;
impl<'a, REG> TpmsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Tpmsrc::B00)
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Tpmsrc::B01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Tpmsrc::B10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Tpmsrc::B11)
    }
}
#[doc = "LPUART0 Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpuart0src {
    #[doc = "0: Clock disabled"]
    B00 = 0,
    #[doc = "1: IRC48M clock"]
    B01 = 1,
    #[doc = "2: OSCERCLK clock"]
    B10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    B11 = 3,
}
impl From<Lpuart0src> for u8 {
    #[inline(always)]
    fn from(variant: Lpuart0src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpuart0src {
    type Ux = u8;
}
impl crate::IsEnum for Lpuart0src {}
#[doc = "Field `LPUART0SRC` reader - LPUART0 Clock Source Select"]
pub type Lpuart0srcR = crate::FieldReader<Lpuart0src>;
impl Lpuart0srcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart0src {
        match self.bits {
            0 => Lpuart0src::B00,
            1 => Lpuart0src::B01,
            2 => Lpuart0src::B10,
            3 => Lpuart0src::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lpuart0src::B00
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lpuart0src::B01
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lpuart0src::B10
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Lpuart0src::B11
    }
}
#[doc = "Field `LPUART0SRC` writer - LPUART0 Clock Source Select"]
pub type Lpuart0srcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpuart0src, crate::Safe>;
impl<'a, REG> Lpuart0srcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0src::B00)
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0src::B01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0src::B10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0src::B11)
    }
}
#[doc = "LPUART1 Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpuart1src {
    #[doc = "0: Clock disabled"]
    B00 = 0,
    #[doc = "1: IRC48M clock"]
    B01 = 1,
    #[doc = "2: OSCERCLK clock"]
    B10 = 2,
    #[doc = "3: MCGIRCLK clock"]
    B11 = 3,
}
impl From<Lpuart1src> for u8 {
    #[inline(always)]
    fn from(variant: Lpuart1src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpuart1src {
    type Ux = u8;
}
impl crate::IsEnum for Lpuart1src {}
#[doc = "Field `LPUART1SRC` reader - LPUART1 Clock Source Select"]
pub type Lpuart1srcR = crate::FieldReader<Lpuart1src>;
impl Lpuart1srcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1src {
        match self.bits {
            0 => Lpuart1src::B00,
            1 => Lpuart1src::B01,
            2 => Lpuart1src::B10,
            3 => Lpuart1src::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lpuart1src::B00
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lpuart1src::B01
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lpuart1src::B10
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Lpuart1src::B11
    }
}
#[doc = "Field `LPUART1SRC` writer - LPUART1 Clock Source Select"]
pub type Lpuart1srcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpuart1src, crate::Safe>;
impl<'a, REG> Lpuart1srcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1src::B00)
    }
    #[doc = "IRC48M clock"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1src::B01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1src::B10)
    }
    #[doc = "MCGIRCLK clock"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1src::B11)
    }
}
impl R {
    #[doc = "Bit 4 - RTC Clock Out Select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&self) -> RtcclkoutselR {
        RtcclkoutselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> ClkoutselR {
        ClkoutselR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&self) -> UsbsrcR {
        UsbsrcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 22:23 - FlexIO Module Clock Source Select"]
    #[inline(always)]
    pub fn flexiosrc(&self) -> FlexiosrcR {
        FlexiosrcR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - TPM Clock Source Select"]
    #[inline(always)]
    pub fn tpmsrc(&self) -> TpmsrcR {
        TpmsrcR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - LPUART0 Clock Source Select"]
    #[inline(always)]
    pub fn lpuart0src(&self) -> Lpuart0srcR {
        Lpuart0srcR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - LPUART1 Clock Source Select"]
    #[inline(always)]
    pub fn lpuart1src(&self) -> Lpuart1srcR {
        Lpuart1srcR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - RTC Clock Out Select"]
    #[inline(always)]
    #[must_use]
    pub fn rtcclkoutsel(&mut self) -> RtcclkoutselW<Sopt2Spec> {
        RtcclkoutselW::new(self, 4)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel(&mut self) -> ClkoutselW<Sopt2Spec> {
        ClkoutselW::new(self, 5)
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn usbsrc(&mut self) -> UsbsrcW<Sopt2Spec> {
        UsbsrcW::new(self, 18)
    }
    #[doc = "Bits 22:23 - FlexIO Module Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn flexiosrc(&mut self) -> FlexiosrcW<Sopt2Spec> {
        FlexiosrcW::new(self, 22)
    }
    #[doc = "Bits 24:25 - TPM Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn tpmsrc(&mut self) -> TpmsrcW<Sopt2Spec> {
        TpmsrcW::new(self, 24)
    }
    #[doc = "Bits 26:27 - LPUART0 Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart0src(&mut self) -> Lpuart0srcW<Sopt2Spec> {
        Lpuart0srcW::new(self, 26)
    }
    #[doc = "Bits 28:29 - LPUART1 Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1src(&mut self) -> Lpuart1srcW<Sopt2Spec> {
        Lpuart1srcW::new(self, 28)
    }
}
#[doc = "System Options Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sopt2Spec;
impl crate::RegisterSpec for Sopt2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sopt2::R`](R) reader structure"]
impl crate::Readable for Sopt2Spec {}
#[doc = "`write(|w| ..)` method takes [`sopt2::W`](W) writer structure"]
impl crate::Writable for Sopt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOPT2 to value 0"]
impl crate::Resettable for Sopt2Spec {
    const RESET_VALUE: u32 = 0;
}
