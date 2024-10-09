#[doc = "Register `GCR` reader"]
pub type R = crate::R<GcrSpec>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GcrSpec>;
#[doc = "LCD duty select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Duty {
    #[doc = "0: Use 1 BP (1/1 duty cycle)."]
    B000 = 0,
    #[doc = "1: Use 2 BP (1/2 duty cycle)."]
    B001 = 1,
    #[doc = "2: Use 3 BP (1/3 duty cycle)."]
    B010 = 2,
    #[doc = "3: Use 4 BP (1/4 duty cycle). (Default)"]
    B011 = 3,
    #[doc = "7: Use 8 BP (1/8 duty cycle)."]
    B111 = 7,
}
impl From<Duty> for u8 {
    #[inline(always)]
    fn from(variant: Duty) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Duty {
    type Ux = u8;
}
impl crate::IsEnum for Duty {}
#[doc = "Field `DUTY` reader - LCD duty select"]
pub type DutyR = crate::FieldReader<Duty>;
impl DutyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Duty> {
        match self.bits {
            0 => Some(Duty::B000),
            1 => Some(Duty::B001),
            2 => Some(Duty::B010),
            3 => Some(Duty::B011),
            7 => Some(Duty::B111),
            _ => None,
        }
    }
    #[doc = "Use 1 BP (1/1 duty cycle)."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Duty::B000
    }
    #[doc = "Use 2 BP (1/2 duty cycle)."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Duty::B001
    }
    #[doc = "Use 3 BP (1/3 duty cycle)."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Duty::B010
    }
    #[doc = "Use 4 BP (1/4 duty cycle). (Default)"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Duty::B011
    }
    #[doc = "Use 8 BP (1/8 duty cycle)."]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Duty::B111
    }
}
#[doc = "Field `DUTY` writer - LCD duty select"]
pub type DutyW<'a, REG> = crate::FieldWriter<'a, REG, 3, Duty>;
impl<'a, REG> DutyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use 1 BP (1/1 duty cycle)."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Duty::B000)
    }
    #[doc = "Use 2 BP (1/2 duty cycle)."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Duty::B001)
    }
    #[doc = "Use 3 BP (1/3 duty cycle)."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Duty::B010)
    }
    #[doc = "Use 4 BP (1/4 duty cycle). (Default)"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Duty::B011)
    }
    #[doc = "Use 8 BP (1/8 duty cycle)."]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Duty::B111)
    }
}
#[doc = "Field `LCLK` reader - LCD Clock Prescaler"]
pub type LclkR = crate::FieldReader;
#[doc = "Field `LCLK` writer - LCD Clock Prescaler"]
pub type LclkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "LCD Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    #[doc = "0: Selects the default clock as the LCD clock source."]
    B0 = 0,
    #[doc = "1: Selects output of the alternate clock source selection (see ALTSOURCE) as the LCD clock source."]
    B1 = 1,
}
impl From<Source> for bool {
    #[inline(always)]
    fn from(variant: Source) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOURCE` reader - LCD Clock Source Select"]
pub type SourceR = crate::BitReader<Source>;
impl SourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Source {
        match self.bits {
            false => Source::B0,
            true => Source::B1,
        }
    }
    #[doc = "Selects the default clock as the LCD clock source."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Source::B0
    }
    #[doc = "Selects output of the alternate clock source selection (see ALTSOURCE) as the LCD clock source."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Source::B1
    }
}
#[doc = "Field `SOURCE` writer - LCD Clock Source Select"]
pub type SourceW<'a, REG> = crate::BitWriter<'a, REG, Source>;
impl<'a, REG> SourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects the default clock as the LCD clock source."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B0)
    }
    #[doc = "Selects output of the alternate clock source selection (see ALTSOURCE) as the LCD clock source."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B1)
    }
}
#[doc = "LCD Driver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcden {
    #[doc = "0: All front plane and back plane pins are disabled. The LCD controller system is also disabled, and all LCD waveform generation clocks are stopped. V LL3 is connected to V DD internally. All LCD pins, LCD_Pn, enabled using the LCD Pin Enable register, output a low value."]
    B0 = 0,
    #[doc = "1: LCD controller driver system is enabled, and front plane and back plane waveforms are generated. All LCD pins, LCD_Pn, enabled if PAD_SAFE is clearusing the LCD Pin Enable register, output an LCD driver waveform. The back plane pins output an LCD driver back plane waveform based on the settings of DUTY\\[2:0\\]. Charge pump or resistor bias is enabled."]
    B1 = 1,
}
impl From<Lcden> for bool {
    #[inline(always)]
    fn from(variant: Lcden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDEN` reader - LCD Driver Enable"]
pub type LcdenR = crate::BitReader<Lcden>;
impl LcdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcden {
        match self.bits {
            false => Lcden::B0,
            true => Lcden::B1,
        }
    }
    #[doc = "All front plane and back plane pins are disabled. The LCD controller system is also disabled, and all LCD waveform generation clocks are stopped. V LL3 is connected to V DD internally. All LCD pins, LCD_Pn, enabled using the LCD Pin Enable register, output a low value."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lcden::B0
    }
    #[doc = "LCD controller driver system is enabled, and front plane and back plane waveforms are generated. All LCD pins, LCD_Pn, enabled if PAD_SAFE is clearusing the LCD Pin Enable register, output an LCD driver waveform. The back plane pins output an LCD driver back plane waveform based on the settings of DUTY\\[2:0\\]. Charge pump or resistor bias is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lcden::B1
    }
}
#[doc = "Field `LCDEN` writer - LCD Driver Enable"]
pub type LcdenW<'a, REG> = crate::BitWriter<'a, REG, Lcden>;
impl<'a, REG> LcdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All front plane and back plane pins are disabled. The LCD controller system is also disabled, and all LCD waveform generation clocks are stopped. V LL3 is connected to V DD internally. All LCD pins, LCD_Pn, enabled using the LCD Pin Enable register, output a low value."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcden::B0)
    }
    #[doc = "LCD controller driver system is enabled, and front plane and back plane waveforms are generated. All LCD pins, LCD_Pn, enabled if PAD_SAFE is clearusing the LCD Pin Enable register, output an LCD driver waveform. The back plane pins output an LCD driver back plane waveform based on the settings of DUTY\\[2:0\\]. Charge pump or resistor bias is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcden::B1)
    }
}
#[doc = "LCD Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdstp {
    #[doc = "0: Allows the LCD driver, charge pump, resistor bias network, and voltage regulator to continue running during Stop mode."]
    B0 = 0,
    #[doc = "1: Disables the LCD driver, charge pump, resistor bias network, and voltage regulator when MCU enters Stop mode."]
    B1 = 1,
}
impl From<Lcdstp> for bool {
    #[inline(always)]
    fn from(variant: Lcdstp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDSTP` reader - LCD Stop"]
pub type LcdstpR = crate::BitReader<Lcdstp>;
impl LcdstpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdstp {
        match self.bits {
            false => Lcdstp::B0,
            true => Lcdstp::B1,
        }
    }
    #[doc = "Allows the LCD driver, charge pump, resistor bias network, and voltage regulator to continue running during Stop mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lcdstp::B0
    }
    #[doc = "Disables the LCD driver, charge pump, resistor bias network, and voltage regulator when MCU enters Stop mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lcdstp::B1
    }
}
#[doc = "Field `LCDSTP` writer - LCD Stop"]
pub type LcdstpW<'a, REG> = crate::BitWriter<'a, REG, Lcdstp>;
impl<'a, REG> LcdstpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allows the LCD driver, charge pump, resistor bias network, and voltage regulator to continue running during Stop mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdstp::B0)
    }
    #[doc = "Disables the LCD driver, charge pump, resistor bias network, and voltage regulator when MCU enters Stop mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdstp::B1)
    }
}
#[doc = "LCD Doze enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcddoze {
    #[doc = "0: Allows the LCD driver, charge pump, resistor bias network, and voltage regulator to continue running during Doze mode."]
    B0 = 0,
    #[doc = "1: Disables the LCD driver, charge pump, resistor bias network, and voltage regulator when MCU enters Doze mode."]
    B1 = 1,
}
impl From<Lcddoze> for bool {
    #[inline(always)]
    fn from(variant: Lcddoze) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDDOZE` reader - LCD Doze enable"]
pub type LcddozeR = crate::BitReader<Lcddoze>;
impl LcddozeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcddoze {
        match self.bits {
            false => Lcddoze::B0,
            true => Lcddoze::B1,
        }
    }
    #[doc = "Allows the LCD driver, charge pump, resistor bias network, and voltage regulator to continue running during Doze mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lcddoze::B0
    }
    #[doc = "Disables the LCD driver, charge pump, resistor bias network, and voltage regulator when MCU enters Doze mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lcddoze::B1
    }
}
#[doc = "Field `LCDDOZE` writer - LCD Doze enable"]
pub type LcddozeW<'a, REG> = crate::BitWriter<'a, REG, Lcddoze>;
impl<'a, REG> LcddozeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Allows the LCD driver, charge pump, resistor bias network, and voltage regulator to continue running during Doze mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddoze::B0)
    }
    #[doc = "Disables the LCD driver, charge pump, resistor bias network, and voltage regulator when MCU enters Doze mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lcddoze::B1)
    }
}
#[doc = "Fast Frame Rate Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ffr {
    #[doc = "0: Standard Frame Rate LCD Frame Freq: 23.3 (min) 73.1 (max)"]
    B0 = 0,
    #[doc = "1: Fast Frame Rate (Standard Frame Rate x2) LCD Frame Freq: 46.6 (min) 146.2 (max)"]
    B1 = 1,
}
impl From<Ffr> for bool {
    #[inline(always)]
    fn from(variant: Ffr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFR` reader - Fast Frame Rate Select"]
pub type FfrR = crate::BitReader<Ffr>;
impl FfrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ffr {
        match self.bits {
            false => Ffr::B0,
            true => Ffr::B1,
        }
    }
    #[doc = "Standard Frame Rate LCD Frame Freq: 23.3 (min) 73.1 (max)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ffr::B0
    }
    #[doc = "Fast Frame Rate (Standard Frame Rate x2) LCD Frame Freq: 46.6 (min) 146.2 (max)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ffr::B1
    }
}
#[doc = "Field `FFR` writer - Fast Frame Rate Select"]
pub type FfrW<'a, REG> = crate::BitWriter<'a, REG, Ffr>;
impl<'a, REG> FfrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard Frame Rate LCD Frame Freq: 23.3 (min) 73.1 (max)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ffr::B0)
    }
    #[doc = "Fast Frame Rate (Standard Frame Rate x2) LCD Frame Freq: 46.6 (min) 146.2 (max)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ffr::B1)
    }
}
#[doc = "Selects the alternate clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Altsource {
    #[doc = "0: Select Alternate Clock Source 1 (default)"]
    B0 = 0,
    #[doc = "1: Select Alternate Clock Source 2"]
    B1 = 1,
}
impl From<Altsource> for bool {
    #[inline(always)]
    fn from(variant: Altsource) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALTSOURCE` reader - Selects the alternate clock source"]
pub type AltsourceR = crate::BitReader<Altsource>;
impl AltsourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Altsource {
        match self.bits {
            false => Altsource::B0,
            true => Altsource::B1,
        }
    }
    #[doc = "Select Alternate Clock Source 1 (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Altsource::B0
    }
    #[doc = "Select Alternate Clock Source 2"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Altsource::B1
    }
}
#[doc = "Field `ALTSOURCE` writer - Selects the alternate clock source"]
pub type AltsourceW<'a, REG> = crate::BitWriter<'a, REG, Altsource>;
impl<'a, REG> AltsourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Select Alternate Clock Source 1 (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Altsource::B0)
    }
    #[doc = "Select Alternate Clock Source 2"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Altsource::B1)
    }
}
#[doc = "LCD AlternateClock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Altdiv {
    #[doc = "0: Divide factor = 1 (No divide)"]
    B00 = 0,
    #[doc = "1: Divide factor = 8"]
    B01 = 1,
    #[doc = "2: Divide factor = 64"]
    B10 = 2,
    #[doc = "3: Divide factor = 512"]
    B11 = 3,
}
impl From<Altdiv> for u8 {
    #[inline(always)]
    fn from(variant: Altdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Altdiv {
    type Ux = u8;
}
impl crate::IsEnum for Altdiv {}
#[doc = "Field `ALTDIV` reader - LCD AlternateClock Divider"]
pub type AltdivR = crate::FieldReader<Altdiv>;
impl AltdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Altdiv {
        match self.bits {
            0 => Altdiv::B00,
            1 => Altdiv::B01,
            2 => Altdiv::B10,
            3 => Altdiv::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide factor = 1 (No divide)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Altdiv::B00
    }
    #[doc = "Divide factor = 8"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Altdiv::B01
    }
    #[doc = "Divide factor = 64"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Altdiv::B10
    }
    #[doc = "Divide factor = 512"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Altdiv::B11
    }
}
#[doc = "Field `ALTDIV` writer - LCD AlternateClock Divider"]
pub type AltdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Altdiv, crate::Safe>;
impl<'a, REG> AltdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide factor = 1 (No divide)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Altdiv::B00)
    }
    #[doc = "Divide factor = 8"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Altdiv::B01)
    }
    #[doc = "Divide factor = 64"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Altdiv::B10)
    }
    #[doc = "Divide factor = 512"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Altdiv::B11)
    }
}
#[doc = "LCD Fault Detection Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdcien {
    #[doc = "0: No interrupt request is generated by this event."]
    B0 = 0,
    #[doc = "1: When a fault is detected and FDCF bit is set, this event causes an interrupt request."]
    B1 = 1,
}
impl From<Fdcien> for bool {
    #[inline(always)]
    fn from(variant: Fdcien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDCIEN` reader - LCD Fault Detection Complete Interrupt Enable"]
pub type FdcienR = crate::BitReader<Fdcien>;
impl FdcienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdcien {
        match self.bits {
            false => Fdcien::B0,
            true => Fdcien::B1,
        }
    }
    #[doc = "No interrupt request is generated by this event."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fdcien::B0
    }
    #[doc = "When a fault is detected and FDCF bit is set, this event causes an interrupt request."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fdcien::B1
    }
}
#[doc = "Field `FDCIEN` writer - LCD Fault Detection Complete Interrupt Enable"]
pub type FdcienW<'a, REG> = crate::BitWriter<'a, REG, Fdcien>;
impl<'a, REG> FdcienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt request is generated by this event."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fdcien::B0)
    }
    #[doc = "When a fault is detected and FDCF bit is set, this event causes an interrupt request."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fdcien::B1)
    }
}
#[doc = "Pad Safe State Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Padsafe {
    #[doc = "0: LCD frontplane and backplane functions enabled according to other LCD control bits"]
    B0 = 0,
    #[doc = "1: LCD frontplane and backplane functions disabled"]
    B1 = 1,
}
impl From<Padsafe> for bool {
    #[inline(always)]
    fn from(variant: Padsafe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PADSAFE` reader - Pad Safe State Enable"]
pub type PadsafeR = crate::BitReader<Padsafe>;
impl PadsafeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Padsafe {
        match self.bits {
            false => Padsafe::B0,
            true => Padsafe::B1,
        }
    }
    #[doc = "LCD frontplane and backplane functions enabled according to other LCD control bits"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Padsafe::B0
    }
    #[doc = "LCD frontplane and backplane functions disabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Padsafe::B1
    }
}
#[doc = "Field `PADSAFE` writer - Pad Safe State Enable"]
pub type PadsafeW<'a, REG> = crate::BitWriter<'a, REG, Padsafe>;
impl<'a, REG> PadsafeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD frontplane and backplane functions enabled according to other LCD control bits"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Padsafe::B0)
    }
    #[doc = "LCD frontplane and backplane functions disabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Padsafe::B1)
    }
}
#[doc = "Voltage Supply Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vsupply {
    #[doc = "0: Drive VLL3 internally from VDD"]
    B0 = 0,
    #[doc = "1: Drive VLL3 externally from VDD or drive VLL internally from vIREG"]
    B1 = 1,
}
impl From<Vsupply> for bool {
    #[inline(always)]
    fn from(variant: Vsupply) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSUPPLY` reader - Voltage Supply Control"]
pub type VsupplyR = crate::BitReader<Vsupply>;
impl VsupplyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vsupply {
        match self.bits {
            false => Vsupply::B0,
            true => Vsupply::B1,
        }
    }
    #[doc = "Drive VLL3 internally from VDD"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Vsupply::B0
    }
    #[doc = "Drive VLL3 externally from VDD or drive VLL internally from vIREG"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Vsupply::B1
    }
}
#[doc = "Field `VSUPPLY` writer - Voltage Supply Control"]
pub type VsupplyW<'a, REG> = crate::BitWriter<'a, REG, Vsupply>;
impl<'a, REG> VsupplyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Drive VLL3 internally from VDD"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Vsupply::B0)
    }
    #[doc = "Drive VLL3 externally from VDD or drive VLL internally from vIREG"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Vsupply::B1)
    }
}
#[doc = "Field `LADJ` reader - Load Adjust"]
pub type LadjR = crate::FieldReader;
#[doc = "Field `LADJ` writer - Load Adjust"]
pub type LadjW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Charge Pump or Resistor Bias Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpsel {
    #[doc = "0: LCD charge pump is disabled. Resistor network selected. (The internal 1/3-bias is forced.)"]
    B0 = 0,
    #[doc = "1: LCD charge pump is selected. Resistor network disabled. (The internal 1/3-bias is forced.)"]
    B1 = 1,
}
impl From<Cpsel> for bool {
    #[inline(always)]
    fn from(variant: Cpsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPSEL` reader - Charge Pump or Resistor Bias Select"]
pub type CpselR = crate::BitReader<Cpsel>;
impl CpselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpsel {
        match self.bits {
            false => Cpsel::B0,
            true => Cpsel::B1,
        }
    }
    #[doc = "LCD charge pump is disabled. Resistor network selected. (The internal 1/3-bias is forced.)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cpsel::B0
    }
    #[doc = "LCD charge pump is selected. Resistor network disabled. (The internal 1/3-bias is forced.)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cpsel::B1
    }
}
#[doc = "Field `CPSEL` writer - Charge Pump or Resistor Bias Select"]
pub type CpselW<'a, REG> = crate::BitWriter<'a, REG, Cpsel>;
impl<'a, REG> CpselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD charge pump is disabled. Resistor network selected. (The internal 1/3-bias is forced.)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpsel::B0)
    }
    #[doc = "LCD charge pump is selected. Resistor network disabled. (The internal 1/3-bias is forced.)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpsel::B1)
    }
}
#[doc = "Field `RVTRIM` reader - Regulated Voltage Trim"]
pub type RvtrimR = crate::FieldReader;
#[doc = "Field `RVTRIM` writer - Regulated Voltage Trim"]
pub type RvtrimW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Regulated Voltage Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rven {
    #[doc = "0: Regulated voltage disabled."]
    B0 = 0,
    #[doc = "1: Regulated voltage enabled."]
    B1 = 1,
}
impl From<Rven> for bool {
    #[inline(always)]
    fn from(variant: Rven) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RVEN` reader - Regulated Voltage Enable"]
pub type RvenR = crate::BitReader<Rven>;
impl RvenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rven {
        match self.bits {
            false => Rven::B0,
            true => Rven::B1,
        }
    }
    #[doc = "Regulated voltage disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rven::B0
    }
    #[doc = "Regulated voltage enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rven::B1
    }
}
#[doc = "Field `RVEN` writer - Regulated Voltage Enable"]
pub type RvenW<'a, REG> = crate::BitWriter<'a, REG, Rven>;
impl<'a, REG> RvenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Regulated voltage disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rven::B0)
    }
    #[doc = "Regulated voltage enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rven::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - LCD duty select"]
    #[inline(always)]
    pub fn duty(&self) -> DutyR {
        DutyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - LCD Clock Prescaler"]
    #[inline(always)]
    pub fn lclk(&self) -> LclkR {
        LclkR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - LCD Clock Source Select"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LCD Driver Enable"]
    #[inline(always)]
    pub fn lcden(&self) -> LcdenR {
        LcdenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LCD Stop"]
    #[inline(always)]
    pub fn lcdstp(&self) -> LcdstpR {
        LcdstpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LCD Doze enable"]
    #[inline(always)]
    pub fn lcddoze(&self) -> LcddozeR {
        LcddozeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast Frame Rate Select"]
    #[inline(always)]
    pub fn ffr(&self) -> FfrR {
        FfrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects the alternate clock source"]
    #[inline(always)]
    pub fn altsource(&self) -> AltsourceR {
        AltsourceR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - LCD AlternateClock Divider"]
    #[inline(always)]
    pub fn altdiv(&self) -> AltdivR {
        AltdivR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LCD Fault Detection Complete Interrupt Enable"]
    #[inline(always)]
    pub fn fdcien(&self) -> FdcienR {
        FdcienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Pad Safe State Enable"]
    #[inline(always)]
    pub fn padsafe(&self) -> PadsafeR {
        PadsafeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Voltage Supply Control"]
    #[inline(always)]
    pub fn vsupply(&self) -> VsupplyR {
        VsupplyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Load Adjust"]
    #[inline(always)]
    pub fn ladj(&self) -> LadjR {
        LadjR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - Charge Pump or Resistor Bias Select"]
    #[inline(always)]
    pub fn cpsel(&self) -> CpselR {
        CpselR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Regulated Voltage Trim"]
    #[inline(always)]
    pub fn rvtrim(&self) -> RvtrimR {
        RvtrimR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Regulated Voltage Enable"]
    #[inline(always)]
    pub fn rven(&self) -> RvenR {
        RvenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LCD duty select"]
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DutyW<GcrSpec> {
        DutyW::new(self, 0)
    }
    #[doc = "Bits 3:5 - LCD Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn lclk(&mut self) -> LclkW<GcrSpec> {
        LclkW::new(self, 3)
    }
    #[doc = "Bit 6 - LCD Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SourceW<GcrSpec> {
        SourceW::new(self, 6)
    }
    #[doc = "Bit 7 - LCD Driver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcden(&mut self) -> LcdenW<GcrSpec> {
        LcdenW::new(self, 7)
    }
    #[doc = "Bit 8 - LCD Stop"]
    #[inline(always)]
    #[must_use]
    pub fn lcdstp(&mut self) -> LcdstpW<GcrSpec> {
        LcdstpW::new(self, 8)
    }
    #[doc = "Bit 9 - LCD Doze enable"]
    #[inline(always)]
    #[must_use]
    pub fn lcddoze(&mut self) -> LcddozeW<GcrSpec> {
        LcddozeW::new(self, 9)
    }
    #[doc = "Bit 10 - Fast Frame Rate Select"]
    #[inline(always)]
    #[must_use]
    pub fn ffr(&mut self) -> FfrW<GcrSpec> {
        FfrW::new(self, 10)
    }
    #[doc = "Bit 11 - Selects the alternate clock source"]
    #[inline(always)]
    #[must_use]
    pub fn altsource(&mut self) -> AltsourceW<GcrSpec> {
        AltsourceW::new(self, 11)
    }
    #[doc = "Bits 12:13 - LCD AlternateClock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn altdiv(&mut self) -> AltdivW<GcrSpec> {
        AltdivW::new(self, 12)
    }
    #[doc = "Bit 14 - LCD Fault Detection Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdcien(&mut self) -> FdcienW<GcrSpec> {
        FdcienW::new(self, 14)
    }
    #[doc = "Bit 15 - Pad Safe State Enable"]
    #[inline(always)]
    #[must_use]
    pub fn padsafe(&mut self) -> PadsafeW<GcrSpec> {
        PadsafeW::new(self, 15)
    }
    #[doc = "Bit 17 - Voltage Supply Control"]
    #[inline(always)]
    #[must_use]
    pub fn vsupply(&mut self) -> VsupplyW<GcrSpec> {
        VsupplyW::new(self, 17)
    }
    #[doc = "Bits 20:21 - Load Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn ladj(&mut self) -> LadjW<GcrSpec> {
        LadjW::new(self, 20)
    }
    #[doc = "Bit 23 - Charge Pump or Resistor Bias Select"]
    #[inline(always)]
    #[must_use]
    pub fn cpsel(&mut self) -> CpselW<GcrSpec> {
        CpselW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Regulated Voltage Trim"]
    #[inline(always)]
    #[must_use]
    pub fn rvtrim(&mut self) -> RvtrimW<GcrSpec> {
        RvtrimW::new(self, 24)
    }
    #[doc = "Bit 31 - Regulated Voltage Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rven(&mut self) -> RvenW<GcrSpec> {
        RvenW::new(self, 31)
    }
}
#[doc = "LCD General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcrSpec;
impl crate::RegisterSpec for GcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GCR to value 0x0830_0003"]
impl crate::Resettable for GcrSpec {
    const RESET_VALUE: u32 = 0x0830_0003;
}
