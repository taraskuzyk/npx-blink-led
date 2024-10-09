#[doc = "Register `SC` reader"]
pub type R = crate::R<ScSpec>;
#[doc = "Register `SC` writer"]
pub type W = crate::W<ScSpec>;
#[doc = "Prescale Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ps {
    #[doc = "0: Divide by 1"]
    B000 = 0,
    #[doc = "1: Divide by 2"]
    B001 = 1,
    #[doc = "2: Divide by 4"]
    B010 = 2,
    #[doc = "3: Divide by 8"]
    B011 = 3,
    #[doc = "4: Divide by 16"]
    B100 = 4,
    #[doc = "5: Divide by 32"]
    B101 = 5,
    #[doc = "6: Divide by 64"]
    B110 = 6,
    #[doc = "7: Divide by 128"]
    B111 = 7,
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ps {
    type Ux = u8;
}
impl crate::IsEnum for Ps {}
#[doc = "Field `PS` reader - Prescale Factor Selection"]
pub type PsR = crate::FieldReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            0 => Ps::B000,
            1 => Ps::B001,
            2 => Ps::B010,
            3 => Ps::B011,
            4 => Ps::B100,
            5 => Ps::B101,
            6 => Ps::B110,
            7 => Ps::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Ps::B000
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Ps::B001
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Ps::B010
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Ps::B011
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Ps::B100
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Ps::B101
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Ps::B110
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Ps::B111
    }
}
#[doc = "Field `PS` writer - Prescale Factor Selection"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ps, crate::Safe>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B000)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B001)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B010)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B011)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B100)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B101)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B110)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B111)
    }
}
#[doc = "Clock Mode Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmod {
    #[doc = "0: TPM counter is disabled"]
    B00 = 0,
    #[doc = "1: TPM counter increments on every TPM counter clock"]
    B01 = 1,
    #[doc = "2: TPM counter increments on rising edge of TPM_EXTCLK synchronized to the TPM counter clock"]
    B10 = 2,
}
impl From<Cmod> for u8 {
    #[inline(always)]
    fn from(variant: Cmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmod {
    type Ux = u8;
}
impl crate::IsEnum for Cmod {}
#[doc = "Field `CMOD` reader - Clock Mode Selection"]
pub type CmodR = crate::FieldReader<Cmod>;
impl CmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmod> {
        match self.bits {
            0 => Some(Cmod::B00),
            1 => Some(Cmod::B01),
            2 => Some(Cmod::B10),
            _ => None,
        }
    }
    #[doc = "TPM counter is disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Cmod::B00
    }
    #[doc = "TPM counter increments on every TPM counter clock"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Cmod::B01
    }
    #[doc = "TPM counter increments on rising edge of TPM_EXTCLK synchronized to the TPM counter clock"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Cmod::B10
    }
}
#[doc = "Field `CMOD` writer - Clock Mode Selection"]
pub type CmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmod>;
impl<'a, REG> CmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TPM counter is disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Cmod::B00)
    }
    #[doc = "TPM counter increments on every TPM counter clock"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Cmod::B01)
    }
    #[doc = "TPM counter increments on rising edge of TPM_EXTCLK synchronized to the TPM counter clock"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Cmod::B10)
    }
}
#[doc = "Center-Aligned PWM Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpwms {
    #[doc = "0: TPM counter operates in up counting mode."]
    B0 = 0,
    #[doc = "1: TPM counter operates in up-down counting mode."]
    B1 = 1,
}
impl From<Cpwms> for bool {
    #[inline(always)]
    fn from(variant: Cpwms) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPWMS` reader - Center-Aligned PWM Select"]
pub type CpwmsR = crate::BitReader<Cpwms>;
impl CpwmsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpwms {
        match self.bits {
            false => Cpwms::B0,
            true => Cpwms::B1,
        }
    }
    #[doc = "TPM counter operates in up counting mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cpwms::B0
    }
    #[doc = "TPM counter operates in up-down counting mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cpwms::B1
    }
}
#[doc = "Field `CPWMS` writer - Center-Aligned PWM Select"]
pub type CpwmsW<'a, REG> = crate::BitWriter<'a, REG, Cpwms>;
impl<'a, REG> CpwmsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TPM counter operates in up counting mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpwms::B0)
    }
    #[doc = "TPM counter operates in up-down counting mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpwms::B1)
    }
}
#[doc = "Timer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toie {
    #[doc = "0: Disable TOF interrupts. Use software polling or DMA request."]
    B0 = 0,
    #[doc = "1: Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    B1 = 1,
}
impl From<Toie> for bool {
    #[inline(always)]
    fn from(variant: Toie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIE` reader - Timer Overflow Interrupt Enable"]
pub type ToieR = crate::BitReader<Toie>;
impl ToieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Toie {
        match self.bits {
            false => Toie::B0,
            true => Toie::B1,
        }
    }
    #[doc = "Disable TOF interrupts. Use software polling or DMA request."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Toie::B0
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Toie::B1
    }
}
#[doc = "Field `TOIE` writer - Timer Overflow Interrupt Enable"]
pub type ToieW<'a, REG> = crate::BitWriter<'a, REG, Toie>;
impl<'a, REG> ToieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable TOF interrupts. Use software polling or DMA request."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Toie::B0)
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Toie::B1)
    }
}
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tof {
    #[doc = "0: TPM counter has not overflowed."]
    B0 = 0,
    #[doc = "1: TPM counter has overflowed."]
    B1 = 1,
}
impl From<Tof> for bool {
    #[inline(always)]
    fn from(variant: Tof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` reader - Timer Overflow Flag"]
pub type TofR = crate::BitReader<Tof>;
impl TofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tof {
        match self.bits {
            false => Tof::B0,
            true => Tof::B1,
        }
    }
    #[doc = "TPM counter has not overflowed."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tof::B0
    }
    #[doc = "TPM counter has overflowed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tof::B1
    }
}
#[doc = "Field `TOF` writer - Timer Overflow Flag"]
pub type TofW<'a, REG> = crate::BitWriter<'a, REG, Tof>;
impl<'a, REG> TofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TPM counter has not overflowed."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tof::B0)
    }
    #[doc = "TPM counter has overflowed."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tof::B1)
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "0: Disables DMA transfers."]
    B0 = 0,
    #[doc = "1: Enables DMA transfers."]
    B1 = 1,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA Enable"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            false => Dma::B0,
            true => Dma::B1,
        }
    }
    #[doc = "Disables DMA transfers."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dma::B0
    }
    #[doc = "Enables DMA transfers."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dma::B1
    }
}
#[doc = "Field `DMA` writer - DMA Enable"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables DMA transfers."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::B0)
    }
    #[doc = "Enables DMA transfers."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Clock Mode Selection"]
    #[inline(always)]
    pub fn cmod(&self) -> CmodR {
        CmodR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&self) -> CpwmsR {
        CpwmsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&self) -> ToieR {
        ToieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TofR {
        TofR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<ScSpec> {
        PsW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Clock Mode Selection"]
    #[inline(always)]
    #[must_use]
    pub fn cmod(&mut self) -> CmodW<ScSpec> {
        CmodW::new(self, 3)
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline(always)]
    #[must_use]
    pub fn cpwms(&mut self) -> CpwmsW<ScSpec> {
        CpwmsW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> ToieW<ScSpec> {
        ToieW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TofW<ScSpec> {
        TofW::new(self, 7)
    }
    #[doc = "Bit 8 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<ScSpec> {
        DmaW::new(self, 8)
    }
}
#[doc = "Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScSpec;
impl crate::RegisterSpec for ScSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc::R`](R) reader structure"]
impl crate::Readable for ScSpec {}
#[doc = "`write(|w| ..)` method takes [`sc::W`](W) writer structure"]
impl crate::Writable for ScSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for ScSpec {
    const RESET_VALUE: u32 = 0;
}
