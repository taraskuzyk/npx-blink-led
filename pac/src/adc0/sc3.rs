#[doc = "Register `SC3` reader"]
pub type R = crate::R<Sc3Spec>;
#[doc = "Register `SC3` writer"]
pub type W = crate::W<Sc3Spec>;
#[doc = "Hardware Average Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Avgs {
    #[doc = "0: 4 samples averaged."]
    B00 = 0,
    #[doc = "1: 8 samples averaged."]
    B01 = 1,
    #[doc = "2: 16 samples averaged."]
    B10 = 2,
    #[doc = "3: 32 samples averaged."]
    B11 = 3,
}
impl From<Avgs> for u8 {
    #[inline(always)]
    fn from(variant: Avgs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Avgs {
    type Ux = u8;
}
impl crate::IsEnum for Avgs {}
#[doc = "Field `AVGS` reader - Hardware Average Select"]
pub type AvgsR = crate::FieldReader<Avgs>;
impl AvgsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avgs {
        match self.bits {
            0 => Avgs::B00,
            1 => Avgs::B01,
            2 => Avgs::B10,
            3 => Avgs::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "4 samples averaged."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Avgs::B00
    }
    #[doc = "8 samples averaged."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Avgs::B01
    }
    #[doc = "16 samples averaged."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Avgs::B10
    }
    #[doc = "32 samples averaged."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Avgs::B11
    }
}
#[doc = "Field `AVGS` writer - Hardware Average Select"]
pub type AvgsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Avgs, crate::Safe>;
impl<'a, REG> AvgsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4 samples averaged."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::B00)
    }
    #[doc = "8 samples averaged."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::B01)
    }
    #[doc = "16 samples averaged."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::B10)
    }
    #[doc = "32 samples averaged."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Avgs::B11)
    }
}
#[doc = "Hardware Average Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avge {
    #[doc = "0: Hardware average function disabled."]
    B0 = 0,
    #[doc = "1: Hardware average function enabled."]
    B1 = 1,
}
impl From<Avge> for bool {
    #[inline(always)]
    fn from(variant: Avge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVGE` reader - Hardware Average Enable"]
pub type AvgeR = crate::BitReader<Avge>;
impl AvgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avge {
        match self.bits {
            false => Avge::B0,
            true => Avge::B1,
        }
    }
    #[doc = "Hardware average function disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Avge::B0
    }
    #[doc = "Hardware average function enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Avge::B1
    }
}
#[doc = "Field `AVGE` writer - Hardware Average Enable"]
pub type AvgeW<'a, REG> = crate::BitWriter<'a, REG, Avge>;
impl<'a, REG> AvgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware average function disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Avge::B0)
    }
    #[doc = "Hardware average function enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Avge::B1)
    }
}
#[doc = "Continuous Conversion Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adco {
    #[doc = "0: One conversion or one set of conversions if the hardware average function is enabled, that is, AVGE=1, after initiating a conversion."]
    B0 = 0,
    #[doc = "1: Continuous conversions or sets of conversions if the hardware average function is enabled, that is, AVGE=1, after initiating a conversion."]
    B1 = 1,
}
impl From<Adco> for bool {
    #[inline(always)]
    fn from(variant: Adco) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCO` reader - Continuous Conversion Enable"]
pub type AdcoR = crate::BitReader<Adco>;
impl AdcoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adco {
        match self.bits {
            false => Adco::B0,
            true => Adco::B1,
        }
    }
    #[doc = "One conversion or one set of conversions if the hardware average function is enabled, that is, AVGE=1, after initiating a conversion."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adco::B0
    }
    #[doc = "Continuous conversions or sets of conversions if the hardware average function is enabled, that is, AVGE=1, after initiating a conversion."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adco::B1
    }
}
#[doc = "Field `ADCO` writer - Continuous Conversion Enable"]
pub type AdcoW<'a, REG> = crate::BitWriter<'a, REG, Adco>;
impl<'a, REG> AdcoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One conversion or one set of conversions if the hardware average function is enabled, that is, AVGE=1, after initiating a conversion."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adco::B0)
    }
    #[doc = "Continuous conversions or sets of conversions if the hardware average function is enabled, that is, AVGE=1, after initiating a conversion."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adco::B1)
    }
}
#[doc = "Calibration Failed Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Calf {
    #[doc = "0: Calibration completed normally."]
    B0 = 0,
    #[doc = "1: Calibration failed. ADC accuracy specifications are not guaranteed."]
    B1 = 1,
}
impl From<Calf> for bool {
    #[inline(always)]
    fn from(variant: Calf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALF` reader - Calibration Failed Flag"]
pub type CalfR = crate::BitReader<Calf>;
impl CalfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Calf {
        match self.bits {
            false => Calf::B0,
            true => Calf::B1,
        }
    }
    #[doc = "Calibration completed normally."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Calf::B0
    }
    #[doc = "Calibration failed. ADC accuracy specifications are not guaranteed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Calf::B1
    }
}
#[doc = "Field `CALF` writer - Calibration Failed Flag"]
pub type CalfW<'a, REG> = crate::BitWriter<'a, REG, Calf>;
impl<'a, REG> CalfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Calibration completed normally."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Calf::B0)
    }
    #[doc = "Calibration failed. ADC accuracy specifications are not guaranteed."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Calf::B1)
    }
}
#[doc = "Field `CAL` reader - Calibration"]
pub type CalR = crate::BitReader;
#[doc = "Field `CAL` writer - Calibration"]
pub type CalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Hardware Average Select"]
    #[inline(always)]
    pub fn avgs(&self) -> AvgsR {
        AvgsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Hardware Average Enable"]
    #[inline(always)]
    pub fn avge(&self) -> AvgeR {
        AvgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Continuous Conversion Enable"]
    #[inline(always)]
    pub fn adco(&self) -> AdcoR {
        AdcoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Calibration Failed Flag"]
    #[inline(always)]
    pub fn calf(&self) -> CalfR {
        CalfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CalR {
        CalR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Hardware Average Select"]
    #[inline(always)]
    #[must_use]
    pub fn avgs(&mut self) -> AvgsW<Sc3Spec> {
        AvgsW::new(self, 0)
    }
    #[doc = "Bit 2 - Hardware Average Enable"]
    #[inline(always)]
    #[must_use]
    pub fn avge(&mut self) -> AvgeW<Sc3Spec> {
        AvgeW::new(self, 2)
    }
    #[doc = "Bit 3 - Continuous Conversion Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adco(&mut self) -> AdcoW<Sc3Spec> {
        AdcoW::new(self, 3)
    }
    #[doc = "Bit 6 - Calibration Failed Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calf(&mut self) -> CalfW<Sc3Spec> {
        CalfW::new(self, 6)
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CalW<Sc3Spec> {
        CalW::new(self, 7)
    }
}
#[doc = "Status and Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sc3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sc3Spec;
impl crate::RegisterSpec for Sc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc3::R`](R) reader structure"]
impl crate::Readable for Sc3Spec {}
#[doc = "`write(|w| ..)` method takes [`sc3::W`](W) writer structure"]
impl crate::Writable for Sc3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SC3 to value 0"]
impl crate::Resettable for Sc3Spec {
    const RESET_VALUE: u32 = 0;
}
