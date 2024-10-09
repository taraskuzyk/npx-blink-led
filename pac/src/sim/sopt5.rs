#[doc = "Register `SOPT5` reader"]
pub type R = crate::R<Sopt5Spec>;
#[doc = "Register `SOPT5` writer"]
pub type W = crate::W<Sopt5Spec>;
#[doc = "LPUART0 Transmit Data Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpuart0txsrc {
    #[doc = "0: LPUART0_TX pin"]
    B00 = 0,
    #[doc = "1: LPUART0_TX pin modulated with TPM1 channel 0 output"]
    B01 = 1,
    #[doc = "2: LPUART0_TX pin modulated with TPM2 channel 0 output"]
    B10 = 2,
}
impl From<Lpuart0txsrc> for u8 {
    #[inline(always)]
    fn from(variant: Lpuart0txsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpuart0txsrc {
    type Ux = u8;
}
impl crate::IsEnum for Lpuart0txsrc {}
#[doc = "Field `LPUART0TXSRC` reader - LPUART0 Transmit Data Source Select"]
pub type Lpuart0txsrcR = crate::FieldReader<Lpuart0txsrc>;
impl Lpuart0txsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lpuart0txsrc> {
        match self.bits {
            0 => Some(Lpuart0txsrc::B00),
            1 => Some(Lpuart0txsrc::B01),
            2 => Some(Lpuart0txsrc::B10),
            _ => None,
        }
    }
    #[doc = "LPUART0_TX pin"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lpuart0txsrc::B00
    }
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lpuart0txsrc::B01
    }
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lpuart0txsrc::B10
    }
}
#[doc = "Field `LPUART0TXSRC` writer - LPUART0 Transmit Data Source Select"]
pub type Lpuart0txsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpuart0txsrc>;
impl<'a, REG> Lpuart0txsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPUART0_TX pin"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0txsrc::B00)
    }
    #[doc = "LPUART0_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0txsrc::B01)
    }
    #[doc = "LPUART0_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0txsrc::B10)
    }
}
#[doc = "LPUART0 Receive Data Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart0rxsrc {
    #[doc = "0: LPUART_RX pin"]
    B0 = 0,
    #[doc = "1: CMP0 output"]
    B1 = 1,
}
impl From<Lpuart0rxsrc> for bool {
    #[inline(always)]
    fn from(variant: Lpuart0rxsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART0RXSRC` reader - LPUART0 Receive Data Source Select"]
pub type Lpuart0rxsrcR = crate::BitReader<Lpuart0rxsrc>;
impl Lpuart0rxsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart0rxsrc {
        match self.bits {
            false => Lpuart0rxsrc::B0,
            true => Lpuart0rxsrc::B1,
        }
    }
    #[doc = "LPUART_RX pin"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpuart0rxsrc::B0
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpuart0rxsrc::B1
    }
}
#[doc = "Field `LPUART0RXSRC` writer - LPUART0 Receive Data Source Select"]
pub type Lpuart0rxsrcW<'a, REG> = crate::BitWriter<'a, REG, Lpuart0rxsrc>;
impl<'a, REG> Lpuart0rxsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART_RX pin"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0rxsrc::B0)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0rxsrc::B1)
    }
}
#[doc = "LPUART1 Transmit Data Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpuart1txsrc {
    #[doc = "0: LPUART1_TX pin"]
    B00 = 0,
    #[doc = "1: LPUART1_TX pin modulated with TPM1 channel 0 output"]
    B01 = 1,
    #[doc = "2: LPUART1_TX pin modulated with TPM2 channel 0 output"]
    B10 = 2,
}
impl From<Lpuart1txsrc> for u8 {
    #[inline(always)]
    fn from(variant: Lpuart1txsrc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpuart1txsrc {
    type Ux = u8;
}
impl crate::IsEnum for Lpuart1txsrc {}
#[doc = "Field `LPUART1TXSRC` reader - LPUART1 Transmit Data Source Select"]
pub type Lpuart1txsrcR = crate::FieldReader<Lpuart1txsrc>;
impl Lpuart1txsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lpuart1txsrc> {
        match self.bits {
            0 => Some(Lpuart1txsrc::B00),
            1 => Some(Lpuart1txsrc::B01),
            2 => Some(Lpuart1txsrc::B10),
            _ => None,
        }
    }
    #[doc = "LPUART1_TX pin"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lpuart1txsrc::B00
    }
    #[doc = "LPUART1_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lpuart1txsrc::B01
    }
    #[doc = "LPUART1_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lpuart1txsrc::B10
    }
}
#[doc = "Field `LPUART1TXSRC` writer - LPUART1 Transmit Data Source Select"]
pub type Lpuart1txsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpuart1txsrc>;
impl<'a, REG> Lpuart1txsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPUART1_TX pin"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1txsrc::B00)
    }
    #[doc = "LPUART1_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1txsrc::B01)
    }
    #[doc = "LPUART1_TX pin modulated with TPM2 channel 0 output"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1txsrc::B10)
    }
}
#[doc = "LPUART1 Receive Data Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart1rxsrc {
    #[doc = "0: LPUART1_RX pin"]
    B0 = 0,
    #[doc = "1: CMP0 output"]
    B1 = 1,
}
impl From<Lpuart1rxsrc> for bool {
    #[inline(always)]
    fn from(variant: Lpuart1rxsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1RXSRC` reader - LPUART1 Receive Data Source Select"]
pub type Lpuart1rxsrcR = crate::BitReader<Lpuart1rxsrc>;
impl Lpuart1rxsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1rxsrc {
        match self.bits {
            false => Lpuart1rxsrc::B0,
            true => Lpuart1rxsrc::B1,
        }
    }
    #[doc = "LPUART1_RX pin"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpuart1rxsrc::B0
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpuart1rxsrc::B1
    }
}
#[doc = "Field `LPUART1RXSRC` writer - LPUART1 Receive Data Source Select"]
pub type Lpuart1rxsrcW<'a, REG> = crate::BitWriter<'a, REG, Lpuart1rxsrc>;
impl<'a, REG> Lpuart1rxsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPUART1_RX pin"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1rxsrc::B0)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1rxsrc::B1)
    }
}
#[doc = "LPUART0 Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart0ode {
    #[doc = "0: Open drain is disabled on LPUART0."]
    B0 = 0,
    #[doc = "1: Open drain is enabled on LPUART0."]
    B1 = 1,
}
impl From<Lpuart0ode> for bool {
    #[inline(always)]
    fn from(variant: Lpuart0ode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART0ODE` reader - LPUART0 Open Drain Enable"]
pub type Lpuart0odeR = crate::BitReader<Lpuart0ode>;
impl Lpuart0odeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart0ode {
        match self.bits {
            false => Lpuart0ode::B0,
            true => Lpuart0ode::B1,
        }
    }
    #[doc = "Open drain is disabled on LPUART0."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpuart0ode::B0
    }
    #[doc = "Open drain is enabled on LPUART0."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpuart0ode::B1
    }
}
#[doc = "Field `LPUART0ODE` writer - LPUART0 Open Drain Enable"]
pub type Lpuart0odeW<'a, REG> = crate::BitWriter<'a, REG, Lpuart0ode>;
impl<'a, REG> Lpuart0odeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Open drain is disabled on LPUART0."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0ode::B0)
    }
    #[doc = "Open drain is enabled on LPUART0."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart0ode::B1)
    }
}
#[doc = "LPUART1 Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpuart1ode {
    #[doc = "0: Open drain is disabled on LPUART1."]
    B0 = 0,
    #[doc = "1: Open drain is enabled on LPUART1"]
    B1 = 1,
}
impl From<Lpuart1ode> for bool {
    #[inline(always)]
    fn from(variant: Lpuart1ode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1ODE` reader - LPUART1 Open Drain Enable"]
pub type Lpuart1odeR = crate::BitReader<Lpuart1ode>;
impl Lpuart1odeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpuart1ode {
        match self.bits {
            false => Lpuart1ode::B0,
            true => Lpuart1ode::B1,
        }
    }
    #[doc = "Open drain is disabled on LPUART1."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpuart1ode::B0
    }
    #[doc = "Open drain is enabled on LPUART1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpuart1ode::B1
    }
}
#[doc = "Field `LPUART1ODE` writer - LPUART1 Open Drain Enable"]
pub type Lpuart1odeW<'a, REG> = crate::BitWriter<'a, REG, Lpuart1ode>;
impl<'a, REG> Lpuart1odeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Open drain is disabled on LPUART1."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1ode::B0)
    }
    #[doc = "Open drain is enabled on LPUART1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpuart1ode::B1)
    }
}
#[doc = "UART2 Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart2ode {
    #[doc = "0: Open drain is disabled on UART2"]
    B0 = 0,
    #[doc = "1: Open drain is enabled on UART2"]
    B1 = 1,
}
impl From<Uart2ode> for bool {
    #[inline(always)]
    fn from(variant: Uart2ode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2ODE` reader - UART2 Open Drain Enable"]
pub type Uart2odeR = crate::BitReader<Uart2ode>;
impl Uart2odeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart2ode {
        match self.bits {
            false => Uart2ode::B0,
            true => Uart2ode::B1,
        }
    }
    #[doc = "Open drain is disabled on UART2"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Uart2ode::B0
    }
    #[doc = "Open drain is enabled on UART2"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Uart2ode::B1
    }
}
#[doc = "Field `UART2ODE` writer - UART2 Open Drain Enable"]
pub type Uart2odeW<'a, REG> = crate::BitWriter<'a, REG, Uart2ode>;
impl<'a, REG> Uart2odeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Open drain is disabled on UART2"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2ode::B0)
    }
    #[doc = "Open drain is enabled on UART2"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2ode::B1)
    }
}
impl R {
    #[doc = "Bits 0:1 - LPUART0 Transmit Data Source Select"]
    #[inline(always)]
    pub fn lpuart0txsrc(&self) -> Lpuart0txsrcR {
        Lpuart0txsrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - LPUART0 Receive Data Source Select"]
    #[inline(always)]
    pub fn lpuart0rxsrc(&self) -> Lpuart0rxsrcR {
        Lpuart0rxsrcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - LPUART1 Transmit Data Source Select"]
    #[inline(always)]
    pub fn lpuart1txsrc(&self) -> Lpuart1txsrcR {
        Lpuart1txsrcR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - LPUART1 Receive Data Source Select"]
    #[inline(always)]
    pub fn lpuart1rxsrc(&self) -> Lpuart1rxsrcR {
        Lpuart1rxsrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - LPUART0 Open Drain Enable"]
    #[inline(always)]
    pub fn lpuart0ode(&self) -> Lpuart0odeR {
        Lpuart0odeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LPUART1 Open Drain Enable"]
    #[inline(always)]
    pub fn lpuart1ode(&self) -> Lpuart1odeR {
        Lpuart1odeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - UART2 Open Drain Enable"]
    #[inline(always)]
    pub fn uart2ode(&self) -> Uart2odeR {
        Uart2odeR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - LPUART0 Transmit Data Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart0txsrc(&mut self) -> Lpuart0txsrcW<Sopt5Spec> {
        Lpuart0txsrcW::new(self, 0)
    }
    #[doc = "Bit 2 - LPUART0 Receive Data Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart0rxsrc(&mut self) -> Lpuart0rxsrcW<Sopt5Spec> {
        Lpuart0rxsrcW::new(self, 2)
    }
    #[doc = "Bits 4:5 - LPUART1 Transmit Data Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1txsrc(&mut self) -> Lpuart1txsrcW<Sopt5Spec> {
        Lpuart1txsrcW::new(self, 4)
    }
    #[doc = "Bit 6 - LPUART1 Receive Data Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rxsrc(&mut self) -> Lpuart1rxsrcW<Sopt5Spec> {
        Lpuart1rxsrcW::new(self, 6)
    }
    #[doc = "Bit 16 - LPUART0 Open Drain Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart0ode(&mut self) -> Lpuart0odeW<Sopt5Spec> {
        Lpuart0odeW::new(self, 16)
    }
    #[doc = "Bit 17 - LPUART1 Open Drain Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1ode(&mut self) -> Lpuart1odeW<Sopt5Spec> {
        Lpuart1odeW::new(self, 17)
    }
    #[doc = "Bit 18 - UART2 Open Drain Enable"]
    #[inline(always)]
    #[must_use]
    pub fn uart2ode(&mut self) -> Uart2odeW<Sopt5Spec> {
        Uart2odeW::new(self, 18)
    }
}
#[doc = "System Options Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sopt5Spec;
impl crate::RegisterSpec for Sopt5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sopt5::R`](R) reader structure"]
impl crate::Readable for Sopt5Spec {}
#[doc = "`write(|w| ..)` method takes [`sopt5::W`](W) writer structure"]
impl crate::Writable for Sopt5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOPT5 to value 0"]
impl crate::Resettable for Sopt5Spec {
    const RESET_VALUE: u32 = 0;
}
