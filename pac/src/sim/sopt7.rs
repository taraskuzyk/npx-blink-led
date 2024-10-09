#[doc = "Register `SOPT7` reader"]
pub type R = crate::R<Sopt7Spec>;
#[doc = "Register `SOPT7` writer"]
pub type W = crate::W<Sopt7Spec>;
#[doc = "ADC0 Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adc0trgsel {
    #[doc = "0: External trigger pin input (EXTRG_IN)"]
    B0000 = 0,
    #[doc = "1: CMP0 output"]
    B0001 = 1,
    #[doc = "4: PIT trigger 0"]
    B0100 = 4,
    #[doc = "5: PIT trigger 1"]
    B0101 = 5,
    #[doc = "8: TPM0 overflow"]
    B1000 = 8,
    #[doc = "9: TPM1 overflow"]
    B1001 = 9,
    #[doc = "10: TPM2 overflow"]
    B1010 = 10,
    #[doc = "12: RTC alarm"]
    B1100 = 12,
    #[doc = "13: RTC seconds"]
    B1101 = 13,
    #[doc = "14: LPTMR0 trigger"]
    B1110 = 14,
}
impl From<Adc0trgsel> for u8 {
    #[inline(always)]
    fn from(variant: Adc0trgsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adc0trgsel {
    type Ux = u8;
}
impl crate::IsEnum for Adc0trgsel {}
#[doc = "Field `ADC0TRGSEL` reader - ADC0 Trigger Select"]
pub type Adc0trgselR = crate::FieldReader<Adc0trgsel>;
impl Adc0trgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adc0trgsel> {
        match self.bits {
            0 => Some(Adc0trgsel::B0000),
            1 => Some(Adc0trgsel::B0001),
            4 => Some(Adc0trgsel::B0100),
            5 => Some(Adc0trgsel::B0101),
            8 => Some(Adc0trgsel::B1000),
            9 => Some(Adc0trgsel::B1001),
            10 => Some(Adc0trgsel::B1010),
            12 => Some(Adc0trgsel::B1100),
            13 => Some(Adc0trgsel::B1101),
            14 => Some(Adc0trgsel::B1110),
            _ => None,
        }
    }
    #[doc = "External trigger pin input (EXTRG_IN)"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Adc0trgsel::B0000
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Adc0trgsel::B0001
    }
    #[doc = "PIT trigger 0"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Adc0trgsel::B0100
    }
    #[doc = "PIT trigger 1"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Adc0trgsel::B0101
    }
    #[doc = "TPM0 overflow"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Adc0trgsel::B1000
    }
    #[doc = "TPM1 overflow"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Adc0trgsel::B1001
    }
    #[doc = "TPM2 overflow"]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == Adc0trgsel::B1010
    }
    #[doc = "RTC alarm"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == Adc0trgsel::B1100
    }
    #[doc = "RTC seconds"]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == Adc0trgsel::B1101
    }
    #[doc = "LPTMR0 trigger"]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == Adc0trgsel::B1110
    }
}
#[doc = "Field `ADC0TRGSEL` writer - ADC0 Trigger Select"]
pub type Adc0trgselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Adc0trgsel>;
impl<'a, REG> Adc0trgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External trigger pin input (EXTRG_IN)"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::B0000)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::B0001)
    }
    #[doc = "PIT trigger 0"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::B0100)
    }
    #[doc = "PIT trigger 1"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::B0101)
    }
    #[doc = "TPM0 overflow"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::B1000)
    }
    #[doc = "TPM1 overflow"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::B1001)
    }
    #[doc = "TPM2 overflow"]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::B1010)
    }
    #[doc = "RTC alarm"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::B1100)
    }
    #[doc = "RTC seconds"]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::B1101)
    }
    #[doc = "LPTMR0 trigger"]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0trgsel::B1110)
    }
}
#[doc = "ADC0 Pretrigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0pretrgsel {
    #[doc = "0: Pre-trigger ADHDWTSA is selected, thus ADC0 will use ADC0_SC1A configuration for the next ADC conversion and store the result in ADC0_RA register."]
    B0 = 0,
    #[doc = "1: Pre-trigger ADHDWTSB is selected, thus ADC0 will use ADC0_SC1B configuration for the next ADC conversion and store the result in ADC0_RB register."]
    B1 = 1,
}
impl From<Adc0pretrgsel> for bool {
    #[inline(always)]
    fn from(variant: Adc0pretrgsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0PRETRGSEL` reader - ADC0 Pretrigger Select"]
pub type Adc0pretrgselR = crate::BitReader<Adc0pretrgsel>;
impl Adc0pretrgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0pretrgsel {
        match self.bits {
            false => Adc0pretrgsel::B0,
            true => Adc0pretrgsel::B1,
        }
    }
    #[doc = "Pre-trigger ADHDWTSA is selected, thus ADC0 will use ADC0_SC1A configuration for the next ADC conversion and store the result in ADC0_RA register."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adc0pretrgsel::B0
    }
    #[doc = "Pre-trigger ADHDWTSB is selected, thus ADC0 will use ADC0_SC1B configuration for the next ADC conversion and store the result in ADC0_RB register."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adc0pretrgsel::B1
    }
}
#[doc = "Field `ADC0PRETRGSEL` writer - ADC0 Pretrigger Select"]
pub type Adc0pretrgselW<'a, REG> = crate::BitWriter<'a, REG, Adc0pretrgsel>;
impl<'a, REG> Adc0pretrgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pre-trigger ADHDWTSA is selected, thus ADC0 will use ADC0_SC1A configuration for the next ADC conversion and store the result in ADC0_RA register."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0pretrgsel::B0)
    }
    #[doc = "Pre-trigger ADHDWTSB is selected, thus ADC0 will use ADC0_SC1B configuration for the next ADC conversion and store the result in ADC0_RB register."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0pretrgsel::B1)
    }
}
#[doc = "ADC0 Alternate Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adc0alttrgen {
    #[doc = "0: ADC ADHWT trigger comes from TPM1 channel 0 and channel1. Prior to the assertion of TPM1 channel 0, a pre-trigger pulse will be sent to ADHWTSA to initiate an ADC acquisition using ADCx_SC1A configuration and store ADC conversion in ADCx_RA Register. Prior to the assertion of TPM1 channel 1 a pre-trigger pulse will be sent to ADHWTSB to initiate an ADC acquisition using ADCx_SC1Bconfiguration and store ADC conversion in ADCx_RB Register."]
    B0 = 0,
    #[doc = "1: ADC ADHWT trigger comes from a peripheral event selected by ADC0TRGSEL bits.ADC0PRETRGSEL bit will select the optional ADHWTSA or ADHWTSB select lines for choosing the ADCx_SC1x config and ADCx_Rx result regsiter to store the ADC conversion."]
    B1 = 1,
}
impl From<Adc0alttrgen> for bool {
    #[inline(always)]
    fn from(variant: Adc0alttrgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC0ALTTRGEN` reader - ADC0 Alternate Trigger Enable"]
pub type Adc0alttrgenR = crate::BitReader<Adc0alttrgen>;
impl Adc0alttrgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adc0alttrgen {
        match self.bits {
            false => Adc0alttrgen::B0,
            true => Adc0alttrgen::B1,
        }
    }
    #[doc = "ADC ADHWT trigger comes from TPM1 channel 0 and channel1. Prior to the assertion of TPM1 channel 0, a pre-trigger pulse will be sent to ADHWTSA to initiate an ADC acquisition using ADCx_SC1A configuration and store ADC conversion in ADCx_RA Register. Prior to the assertion of TPM1 channel 1 a pre-trigger pulse will be sent to ADHWTSB to initiate an ADC acquisition using ADCx_SC1Bconfiguration and store ADC conversion in ADCx_RB Register."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adc0alttrgen::B0
    }
    #[doc = "ADC ADHWT trigger comes from a peripheral event selected by ADC0TRGSEL bits.ADC0PRETRGSEL bit will select the optional ADHWTSA or ADHWTSB select lines for choosing the ADCx_SC1x config and ADCx_Rx result regsiter to store the ADC conversion."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adc0alttrgen::B1
    }
}
#[doc = "Field `ADC0ALTTRGEN` writer - ADC0 Alternate Trigger Enable"]
pub type Adc0alttrgenW<'a, REG> = crate::BitWriter<'a, REG, Adc0alttrgen>;
impl<'a, REG> Adc0alttrgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADC ADHWT trigger comes from TPM1 channel 0 and channel1. Prior to the assertion of TPM1 channel 0, a pre-trigger pulse will be sent to ADHWTSA to initiate an ADC acquisition using ADCx_SC1A configuration and store ADC conversion in ADCx_RA Register. Prior to the assertion of TPM1 channel 1 a pre-trigger pulse will be sent to ADHWTSB to initiate an ADC acquisition using ADCx_SC1Bconfiguration and store ADC conversion in ADCx_RB Register."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0alttrgen::B0)
    }
    #[doc = "ADC ADHWT trigger comes from a peripheral event selected by ADC0TRGSEL bits.ADC0PRETRGSEL bit will select the optional ADHWTSA or ADHWTSB select lines for choosing the ADCx_SC1x config and ADCx_Rx result regsiter to store the ADC conversion."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adc0alttrgen::B1)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC0 Trigger Select"]
    #[inline(always)]
    pub fn adc0trgsel(&self) -> Adc0trgselR {
        Adc0trgselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - ADC0 Pretrigger Select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&self) -> Adc0pretrgselR {
        Adc0pretrgselR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 Alternate Trigger Enable"]
    #[inline(always)]
    pub fn adc0alttrgen(&self) -> Adc0alttrgenR {
        Adc0alttrgenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC0 Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0trgsel(&mut self) -> Adc0trgselW<Sopt7Spec> {
        Adc0trgselW::new(self, 0)
    }
    #[doc = "Bit 4 - ADC0 Pretrigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0pretrgsel(&mut self) -> Adc0pretrgselW<Sopt7Spec> {
        Adc0pretrgselW::new(self, 4)
    }
    #[doc = "Bit 7 - ADC0 Alternate Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0alttrgen(&mut self) -> Adc0alttrgenW<Sopt7Spec> {
        Adc0alttrgenW::new(self, 7)
    }
}
#[doc = "System Options Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`sopt7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopt7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sopt7Spec;
impl crate::RegisterSpec for Sopt7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sopt7::R`](R) reader structure"]
impl crate::Readable for Sopt7Spec {}
#[doc = "`write(|w| ..)` method takes [`sopt7::W`](W) writer structure"]
impl crate::Writable for Sopt7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOPT7 to value 0"]
impl crate::Resettable for Sopt7Spec {
    const RESET_VALUE: u32 = 0;
}
