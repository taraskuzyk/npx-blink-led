#[doc = "Register `SOPT1` reader"]
pub type R = crate::R<Sopt1Spec>;
#[doc = "Register `SOPT1` writer"]
pub type W = crate::W<Sopt1Spec>;
#[doc = "32K oscillator clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osc32kout {
    #[doc = "0: ERCLK32K is not output."]
    B00 = 0,
    #[doc = "1: ERCLK32K is output on PTE0."]
    B01 = 1,
    #[doc = "2: ERCLK32K is output on PTE26."]
    B10 = 2,
}
impl From<Osc32kout> for u8 {
    #[inline(always)]
    fn from(variant: Osc32kout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Osc32kout {
    type Ux = u8;
}
impl crate::IsEnum for Osc32kout {}
#[doc = "Field `OSC32KOUT` reader - 32K oscillator clock output"]
pub type Osc32koutR = crate::FieldReader<Osc32kout>;
impl Osc32koutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Osc32kout> {
        match self.bits {
            0 => Some(Osc32kout::B00),
            1 => Some(Osc32kout::B01),
            2 => Some(Osc32kout::B10),
            _ => None,
        }
    }
    #[doc = "ERCLK32K is not output."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Osc32kout::B00
    }
    #[doc = "ERCLK32K is output on PTE0."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Osc32kout::B01
    }
    #[doc = "ERCLK32K is output on PTE26."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Osc32kout::B10
    }
}
#[doc = "Field `OSC32KOUT` writer - 32K oscillator clock output"]
pub type Osc32koutW<'a, REG> = crate::FieldWriter<'a, REG, 2, Osc32kout>;
impl<'a, REG> Osc32koutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ERCLK32K is not output."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Osc32kout::B00)
    }
    #[doc = "ERCLK32K is output on PTE0."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Osc32kout::B01)
    }
    #[doc = "ERCLK32K is output on PTE26."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Osc32kout::B10)
    }
}
#[doc = "32K Oscillator Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osc32ksel {
    #[doc = "0: System oscillator (OSC32KCLK)"]
    B00 = 0,
    #[doc = "2: RTC_CLKIN"]
    B10 = 2,
    #[doc = "3: LPO 1kHz"]
    B11 = 3,
}
impl From<Osc32ksel> for u8 {
    #[inline(always)]
    fn from(variant: Osc32ksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Osc32ksel {
    type Ux = u8;
}
impl crate::IsEnum for Osc32ksel {}
#[doc = "Field `OSC32KSEL` reader - 32K Oscillator Clock Select"]
pub type Osc32kselR = crate::FieldReader<Osc32ksel>;
impl Osc32kselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Osc32ksel> {
        match self.bits {
            0 => Some(Osc32ksel::B00),
            2 => Some(Osc32ksel::B10),
            3 => Some(Osc32ksel::B11),
            _ => None,
        }
    }
    #[doc = "System oscillator (OSC32KCLK)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Osc32ksel::B00
    }
    #[doc = "RTC_CLKIN"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Osc32ksel::B10
    }
    #[doc = "LPO 1kHz"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Osc32ksel::B11
    }
}
#[doc = "Field `OSC32KSEL` writer - 32K Oscillator Clock Select"]
pub type Osc32kselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Osc32ksel>;
impl<'a, REG> Osc32kselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "System oscillator (OSC32KCLK)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Osc32ksel::B00)
    }
    #[doc = "RTC_CLKIN"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Osc32ksel::B10)
    }
    #[doc = "LPO 1kHz"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Osc32ksel::B11)
    }
}
#[doc = "USB voltage regulator in standby mode during VLPR and VLPW modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbvstby {
    #[doc = "0: USB voltage regulator not in standby during VLPR and VLPW modes."]
    B0 = 0,
    #[doc = "1: USB voltage regulator in standby during VLPR and VLPW modes."]
    B1 = 1,
}
impl From<Usbvstby> for bool {
    #[inline(always)]
    fn from(variant: Usbvstby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBVSTBY` reader - USB voltage regulator in standby mode during VLPR and VLPW modes"]
pub type UsbvstbyR = crate::BitReader<Usbvstby>;
impl UsbvstbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbvstby {
        match self.bits {
            false => Usbvstby::B0,
            true => Usbvstby::B1,
        }
    }
    #[doc = "USB voltage regulator not in standby during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbvstby::B0
    }
    #[doc = "USB voltage regulator in standby during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbvstby::B1
    }
}
#[doc = "Field `USBVSTBY` writer - USB voltage regulator in standby mode during VLPR and VLPW modes"]
pub type UsbvstbyW<'a, REG> = crate::BitWriter<'a, REG, Usbvstby>;
impl<'a, REG> UsbvstbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB voltage regulator not in standby during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbvstby::B0)
    }
    #[doc = "USB voltage regulator in standby during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbvstby::B1)
    }
}
#[doc = "USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbsstby {
    #[doc = "0: USB voltage regulator not in standby during Stop, VLPS, LLS and VLLS modes."]
    B0 = 0,
    #[doc = "1: USB voltage regulator in standby during Stop, VLPS, LLS and VLLS modes."]
    B1 = 1,
}
impl From<Usbsstby> for bool {
    #[inline(always)]
    fn from(variant: Usbsstby) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBSSTBY` reader - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
pub type UsbsstbyR = crate::BitReader<Usbsstby>;
impl UsbsstbyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbsstby {
        match self.bits {
            false => Usbsstby::B0,
            true => Usbsstby::B1,
        }
    }
    #[doc = "USB voltage regulator not in standby during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbsstby::B0
    }
    #[doc = "USB voltage regulator in standby during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbsstby::B1
    }
}
#[doc = "Field `USBSSTBY` writer - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
pub type UsbsstbyW<'a, REG> = crate::BitWriter<'a, REG, Usbsstby>;
impl<'a, REG> UsbsstbyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB voltage regulator not in standby during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsstby::B0)
    }
    #[doc = "USB voltage regulator in standby during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsstby::B1)
    }
}
#[doc = "USB voltage regulator enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbregen {
    #[doc = "0: USB voltage regulator is disabled."]
    B0 = 0,
    #[doc = "1: USB voltage regulator is enabled."]
    B1 = 1,
}
impl From<Usbregen> for bool {
    #[inline(always)]
    fn from(variant: Usbregen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBREGEN` reader - USB voltage regulator enable"]
pub type UsbregenR = crate::BitReader<Usbregen>;
impl UsbregenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbregen {
        match self.bits {
            false => Usbregen::B0,
            true => Usbregen::B1,
        }
    }
    #[doc = "USB voltage regulator is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbregen::B0
    }
    #[doc = "USB voltage regulator is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbregen::B1
    }
}
#[doc = "Field `USBREGEN` writer - USB voltage regulator enable"]
pub type UsbregenW<'a, REG> = crate::BitWriter<'a, REG, Usbregen>;
impl<'a, REG> UsbregenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB voltage regulator is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbregen::B0)
    }
    #[doc = "USB voltage regulator is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbregen::B1)
    }
}
impl R {
    #[doc = "Bits 16:17 - 32K oscillator clock output"]
    #[inline(always)]
    pub fn osc32kout(&self) -> Osc32koutR {
        Osc32koutR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 32K Oscillator Clock Select"]
    #[inline(always)]
    pub fn osc32ksel(&self) -> Osc32kselR {
        Osc32kselR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 29 - USB voltage regulator in standby mode during VLPR and VLPW modes"]
    #[inline(always)]
    pub fn usbvstby(&self) -> UsbvstbyR {
        UsbvstbyR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn usbsstby(&self) -> UsbsstbyR {
        UsbsstbyR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB voltage regulator enable"]
    #[inline(always)]
    pub fn usbregen(&self) -> UsbregenR {
        UsbregenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - 32K oscillator clock output"]
    #[inline(always)]
    #[must_use]
    pub fn osc32kout(&mut self) -> Osc32koutW<Sopt1Spec> {
        Osc32koutW::new(self, 16)
    }
    #[doc = "Bits 18:19 - 32K Oscillator Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn osc32ksel(&mut self) -> Osc32kselW<Sopt1Spec> {
        Osc32kselW::new(self, 18)
    }
    #[doc = "Bit 29 - USB voltage regulator in standby mode during VLPR and VLPW modes"]
    #[inline(always)]
    #[must_use]
    pub fn usbvstby(&mut self) -> UsbvstbyW<Sopt1Spec> {
        UsbvstbyW::new(self, 29)
    }
    #[doc = "Bit 30 - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    #[must_use]
    pub fn usbsstby(&mut self) -> UsbsstbyW<Sopt1Spec> {
        UsbsstbyW::new(self, 30)
    }
    #[doc = "Bit 31 - USB voltage regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbregen(&mut self) -> UsbregenW<Sopt1Spec> {
        UsbregenW::new(self, 31)
    }
}
#[doc = "System Options Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sopt1Spec;
impl crate::RegisterSpec for Sopt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sopt1::R`](R) reader structure"]
impl crate::Readable for Sopt1Spec {}
#[doc = "`write(|w| ..)` method takes [`sopt1::W`](W) writer structure"]
impl crate::Writable for Sopt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOPT1 to value 0x8000_0000"]
impl crate::Resettable for Sopt1Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
