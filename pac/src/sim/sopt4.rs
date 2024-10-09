#[doc = "Register `SOPT4` reader"]
pub type R = crate::R<Sopt4Spec>;
#[doc = "Register `SOPT4` writer"]
pub type W = crate::W<Sopt4Spec>;
#[doc = "TPM1 channel 0 input capture source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tpm1ch0src {
    #[doc = "0: TPM1_CH0 signal"]
    B00 = 0,
    #[doc = "1: CMP0 output"]
    B01 = 1,
    #[doc = "3: USB start of frame pulse"]
    B11 = 3,
}
impl From<Tpm1ch0src> for u8 {
    #[inline(always)]
    fn from(variant: Tpm1ch0src) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tpm1ch0src {
    type Ux = u8;
}
impl crate::IsEnum for Tpm1ch0src {}
#[doc = "Field `TPM1CH0SRC` reader - TPM1 channel 0 input capture source select"]
pub type Tpm1ch0srcR = crate::FieldReader<Tpm1ch0src>;
impl Tpm1ch0srcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tpm1ch0src> {
        match self.bits {
            0 => Some(Tpm1ch0src::B00),
            1 => Some(Tpm1ch0src::B01),
            3 => Some(Tpm1ch0src::B11),
            _ => None,
        }
    }
    #[doc = "TPM1_CH0 signal"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Tpm1ch0src::B00
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Tpm1ch0src::B01
    }
    #[doc = "USB start of frame pulse"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Tpm1ch0src::B11
    }
}
#[doc = "Field `TPM1CH0SRC` writer - TPM1 channel 0 input capture source select"]
pub type Tpm1ch0srcW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tpm1ch0src>;
impl<'a, REG> Tpm1ch0srcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TPM1_CH0 signal"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm1ch0src::B00)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm1ch0src::B01)
    }
    #[doc = "USB start of frame pulse"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm1ch0src::B11)
    }
}
#[doc = "TPM2 Channel 0 Input Capture Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm2ch0src {
    #[doc = "0: TPM2_CH0 signal"]
    B0 = 0,
    #[doc = "1: CMP0 output"]
    B1 = 1,
}
impl From<Tpm2ch0src> for bool {
    #[inline(always)]
    fn from(variant: Tpm2ch0src) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM2CH0SRC` reader - TPM2 Channel 0 Input Capture Source Select"]
pub type Tpm2ch0srcR = crate::BitReader<Tpm2ch0src>;
impl Tpm2ch0srcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpm2ch0src {
        match self.bits {
            false => Tpm2ch0src::B0,
            true => Tpm2ch0src::B1,
        }
    }
    #[doc = "TPM2_CH0 signal"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tpm2ch0src::B0
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tpm2ch0src::B1
    }
}
#[doc = "Field `TPM2CH0SRC` writer - TPM2 Channel 0 Input Capture Source Select"]
pub type Tpm2ch0srcW<'a, REG> = crate::BitWriter<'a, REG, Tpm2ch0src>;
impl<'a, REG> Tpm2ch0srcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TPM2_CH0 signal"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm2ch0src::B0)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm2ch0src::B1)
    }
}
#[doc = "TPM0 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm0clksel {
    #[doc = "0: TPM0 external clock driven by TPM_CLKIN0 pin."]
    B0 = 0,
    #[doc = "1: TPM0 external clock driven by TPM_CLKIN1 pin."]
    B1 = 1,
}
impl From<Tpm0clksel> for bool {
    #[inline(always)]
    fn from(variant: Tpm0clksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM0CLKSEL` reader - TPM0 External Clock Pin Select"]
pub type Tpm0clkselR = crate::BitReader<Tpm0clksel>;
impl Tpm0clkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpm0clksel {
        match self.bits {
            false => Tpm0clksel::B0,
            true => Tpm0clksel::B1,
        }
    }
    #[doc = "TPM0 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tpm0clksel::B0
    }
    #[doc = "TPM0 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tpm0clksel::B1
    }
}
#[doc = "Field `TPM0CLKSEL` writer - TPM0 External Clock Pin Select"]
pub type Tpm0clkselW<'a, REG> = crate::BitWriter<'a, REG, Tpm0clksel>;
impl<'a, REG> Tpm0clkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TPM0 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm0clksel::B0)
    }
    #[doc = "TPM0 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm0clksel::B1)
    }
}
#[doc = "TPM1 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm1clksel {
    #[doc = "0: TPM1 external clock driven by TPM_CLKIN0 pin."]
    B0 = 0,
    #[doc = "1: TPM1 external clock driven by TPM_CLKIN1 pin."]
    B1 = 1,
}
impl From<Tpm1clksel> for bool {
    #[inline(always)]
    fn from(variant: Tpm1clksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM1CLKSEL` reader - TPM1 External Clock Pin Select"]
pub type Tpm1clkselR = crate::BitReader<Tpm1clksel>;
impl Tpm1clkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpm1clksel {
        match self.bits {
            false => Tpm1clksel::B0,
            true => Tpm1clksel::B1,
        }
    }
    #[doc = "TPM1 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tpm1clksel::B0
    }
    #[doc = "TPM1 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tpm1clksel::B1
    }
}
#[doc = "Field `TPM1CLKSEL` writer - TPM1 External Clock Pin Select"]
pub type Tpm1clkselW<'a, REG> = crate::BitWriter<'a, REG, Tpm1clksel>;
impl<'a, REG> Tpm1clkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TPM1 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm1clksel::B0)
    }
    #[doc = "TPM1 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm1clksel::B1)
    }
}
#[doc = "TPM2 External Clock Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm2clksel {
    #[doc = "0: TPM2 external clock driven by TPM_CLKIN0 pin."]
    B0 = 0,
    #[doc = "1: TPM2 external clock driven by TPM_CLKIN1 pin."]
    B1 = 1,
}
impl From<Tpm2clksel> for bool {
    #[inline(always)]
    fn from(variant: Tpm2clksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM2CLKSEL` reader - TPM2 External Clock Pin Select"]
pub type Tpm2clkselR = crate::BitReader<Tpm2clksel>;
impl Tpm2clkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpm2clksel {
        match self.bits {
            false => Tpm2clksel::B0,
            true => Tpm2clksel::B1,
        }
    }
    #[doc = "TPM2 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tpm2clksel::B0
    }
    #[doc = "TPM2 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tpm2clksel::B1
    }
}
#[doc = "Field `TPM2CLKSEL` writer - TPM2 External Clock Pin Select"]
pub type Tpm2clkselW<'a, REG> = crate::BitWriter<'a, REG, Tpm2clksel>;
impl<'a, REG> Tpm2clkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TPM2 external clock driven by TPM_CLKIN0 pin."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm2clksel::B0)
    }
    #[doc = "TPM2 external clock driven by TPM_CLKIN1 pin."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm2clksel::B1)
    }
}
impl R {
    #[doc = "Bits 18:19 - TPM1 channel 0 input capture source select"]
    #[inline(always)]
    pub fn tpm1ch0src(&self) -> Tpm1ch0srcR {
        Tpm1ch0srcR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - TPM2 Channel 0 Input Capture Source Select"]
    #[inline(always)]
    pub fn tpm2ch0src(&self) -> Tpm2ch0srcR {
        Tpm2ch0srcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - TPM0 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm0clksel(&self) -> Tpm0clkselR {
        Tpm0clkselR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm1clksel(&self) -> Tpm1clkselR {
        Tpm1clkselR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TPM2 External Clock Pin Select"]
    #[inline(always)]
    pub fn tpm2clksel(&self) -> Tpm2clkselR {
        Tpm2clkselR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:19 - TPM1 channel 0 input capture source select"]
    #[inline(always)]
    #[must_use]
    pub fn tpm1ch0src(&mut self) -> Tpm1ch0srcW<Sopt4Spec> {
        Tpm1ch0srcW::new(self, 18)
    }
    #[doc = "Bit 20 - TPM2 Channel 0 Input Capture Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn tpm2ch0src(&mut self) -> Tpm2ch0srcW<Sopt4Spec> {
        Tpm2ch0srcW::new(self, 20)
    }
    #[doc = "Bit 24 - TPM0 External Clock Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn tpm0clksel(&mut self) -> Tpm0clkselW<Sopt4Spec> {
        Tpm0clkselW::new(self, 24)
    }
    #[doc = "Bit 25 - TPM1 External Clock Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn tpm1clksel(&mut self) -> Tpm1clkselW<Sopt4Spec> {
        Tpm1clkselW::new(self, 25)
    }
    #[doc = "Bit 26 - TPM2 External Clock Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn tpm2clksel(&mut self) -> Tpm2clkselW<Sopt4Spec> {
        Tpm2clkselW::new(self, 26)
    }
}
#[doc = "System Options Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sopt4Spec;
impl crate::RegisterSpec for Sopt4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sopt4::R`](R) reader structure"]
impl crate::Readable for Sopt4Spec {}
#[doc = "`write(|w| ..)` method takes [`sopt4::W`](W) writer structure"]
impl crate::Writable for Sopt4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOPT4 to value 0"]
impl crate::Resettable for Sopt4Spec {
    const RESET_VALUE: u32 = 0;
}
