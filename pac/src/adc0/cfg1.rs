#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Input Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adiclk {
    #[doc = "0: Bus clock"]
    B00 = 0,
    #[doc = "1: Bus clock divided by 2(BUSCLK/DIV2)"]
    B01 = 1,
    #[doc = "2: Alternate clock (ALTCLK)"]
    B10 = 2,
    #[doc = "3: Asynchronous clock (ADACK)"]
    B11 = 3,
}
impl From<Adiclk> for u8 {
    #[inline(always)]
    fn from(variant: Adiclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adiclk {
    type Ux = u8;
}
impl crate::IsEnum for Adiclk {}
#[doc = "Field `ADICLK` reader - Input Clock Select"]
pub type AdiclkR = crate::FieldReader<Adiclk>;
impl AdiclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adiclk {
        match self.bits {
            0 => Adiclk::B00,
            1 => Adiclk::B01,
            2 => Adiclk::B10,
            3 => Adiclk::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Adiclk::B00
    }
    #[doc = "Bus clock divided by 2(BUSCLK/DIV2)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Adiclk::B01
    }
    #[doc = "Alternate clock (ALTCLK)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Adiclk::B10
    }
    #[doc = "Asynchronous clock (ADACK)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Adiclk::B11
    }
}
#[doc = "Field `ADICLK` writer - Input Clock Select"]
pub type AdiclkW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adiclk, crate::Safe>;
impl<'a, REG> AdiclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Adiclk::B00)
    }
    #[doc = "Bus clock divided by 2(BUSCLK/DIV2)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Adiclk::B01)
    }
    #[doc = "Alternate clock (ALTCLK)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Adiclk::B10)
    }
    #[doc = "Asynchronous clock (ADACK)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Adiclk::B11)
    }
}
#[doc = "Conversion mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: When DIFF=0:It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    B00 = 0,
    #[doc = "1: When DIFF=0:It is single-ended 12-bit conversion ; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    B01 = 1,
    #[doc = "2: When DIFF=0:It is single-ended 10-bit conversion. ; when DIFF=1, it is differential 11-bit conversion with 2's complement output"]
    B10 = 2,
    #[doc = "3: When DIFF=0:It is single-ended 16-bit conversion..; when DIFF=1, it is differential 16-bit conversion with 2's complement output"]
    B11 = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Conversion mode selection"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::B00,
            1 => Mode::B01,
            2 => Mode::B10,
            3 => Mode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "When DIFF=0:It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Mode::B00
    }
    #[doc = "When DIFF=0:It is single-ended 12-bit conversion ; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Mode::B01
    }
    #[doc = "When DIFF=0:It is single-ended 10-bit conversion. ; when DIFF=1, it is differential 11-bit conversion with 2's complement output"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Mode::B10
    }
    #[doc = "When DIFF=0:It is single-ended 16-bit conversion..; when DIFF=1, it is differential 16-bit conversion with 2's complement output"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Mode::B11
    }
}
#[doc = "Field `MODE` writer - Conversion mode selection"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When DIFF=0:It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::B00)
    }
    #[doc = "When DIFF=0:It is single-ended 12-bit conversion ; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::B01)
    }
    #[doc = "When DIFF=0:It is single-ended 10-bit conversion. ; when DIFF=1, it is differential 11-bit conversion with 2's complement output"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::B10)
    }
    #[doc = "When DIFF=0:It is single-ended 16-bit conversion..; when DIFF=1, it is differential 16-bit conversion with 2's complement output"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::B11)
    }
}
#[doc = "Sample Time Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adlsmp {
    #[doc = "0: Short sample time."]
    B0 = 0,
    #[doc = "1: Long sample time."]
    B1 = 1,
}
impl From<Adlsmp> for bool {
    #[inline(always)]
    fn from(variant: Adlsmp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADLSMP` reader - Sample Time Configuration"]
pub type AdlsmpR = crate::BitReader<Adlsmp>;
impl AdlsmpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adlsmp {
        match self.bits {
            false => Adlsmp::B0,
            true => Adlsmp::B1,
        }
    }
    #[doc = "Short sample time."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adlsmp::B0
    }
    #[doc = "Long sample time."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adlsmp::B1
    }
}
#[doc = "Field `ADLSMP` writer - Sample Time Configuration"]
pub type AdlsmpW<'a, REG> = crate::BitWriter<'a, REG, Adlsmp>;
impl<'a, REG> AdlsmpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Short sample time."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsmp::B0)
    }
    #[doc = "Long sample time."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsmp::B1)
    }
}
#[doc = "Clock Divide Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adiv {
    #[doc = "0: The divide ratio is 1 and the clock rate is input clock."]
    B00 = 0,
    #[doc = "1: The divide ratio is 2 and the clock rate is (input clock)/2."]
    B01 = 1,
    #[doc = "2: The divide ratio is 4 and the clock rate is (input clock)/4."]
    B10 = 2,
    #[doc = "3: The divide ratio is 8 and the clock rate is (input clock)/8."]
    B11 = 3,
}
impl From<Adiv> for u8 {
    #[inline(always)]
    fn from(variant: Adiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adiv {
    type Ux = u8;
}
impl crate::IsEnum for Adiv {}
#[doc = "Field `ADIV` reader - Clock Divide Select"]
pub type AdivR = crate::FieldReader<Adiv>;
impl AdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adiv {
        match self.bits {
            0 => Adiv::B00,
            1 => Adiv::B01,
            2 => Adiv::B10,
            3 => Adiv::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Adiv::B00
    }
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Adiv::B01
    }
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Adiv::B10
    }
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Adiv::B11
    }
}
#[doc = "Field `ADIV` writer - Clock Divide Select"]
pub type AdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adiv, crate::Safe>;
impl<'a, REG> AdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Adiv::B00)
    }
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Adiv::B01)
    }
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Adiv::B10)
    }
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Adiv::B11)
    }
}
#[doc = "Low-Power Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adlpc {
    #[doc = "0: Normal power configuration."]
    B0 = 0,
    #[doc = "1: Low-power configuration. The power is reduced at the expense of maximum clock speed."]
    B1 = 1,
}
impl From<Adlpc> for bool {
    #[inline(always)]
    fn from(variant: Adlpc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADLPC` reader - Low-Power Configuration"]
pub type AdlpcR = crate::BitReader<Adlpc>;
impl AdlpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adlpc {
        match self.bits {
            false => Adlpc::B0,
            true => Adlpc::B1,
        }
    }
    #[doc = "Normal power configuration."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adlpc::B0
    }
    #[doc = "Low-power configuration. The power is reduced at the expense of maximum clock speed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adlpc::B1
    }
}
#[doc = "Field `ADLPC` writer - Low-Power Configuration"]
pub type AdlpcW<'a, REG> = crate::BitWriter<'a, REG, Adlpc>;
impl<'a, REG> AdlpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal power configuration."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adlpc::B0)
    }
    #[doc = "Low-power configuration. The power is reduced at the expense of maximum clock speed."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adlpc::B1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    pub fn adiclk(&self) -> AdiclkR {
        AdiclkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Sample Time Configuration"]
    #[inline(always)]
    pub fn adlsmp(&self) -> AdlsmpR {
        AdlsmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    pub fn adiv(&self) -> AdivR {
        AdivR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    pub fn adlpc(&self) -> AdlpcR {
        AdlpcR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn adiclk(&mut self) -> AdiclkW<Cfg1Spec> {
        AdiclkW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<Cfg1Spec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 4 - Sample Time Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adlsmp(&mut self) -> AdlsmpW<Cfg1Spec> {
        AdlsmpW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    #[must_use]
    pub fn adiv(&mut self) -> AdivW<Cfg1Spec> {
        AdivW::new(self, 5)
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adlpc(&mut self) -> AdlpcW<Cfg1Spec> {
        AdlpcW::new(self, 7)
    }
}
#[doc = "ADC Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0;
}
