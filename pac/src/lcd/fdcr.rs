#[doc = "Register `FDCR` reader"]
pub type R = crate::R<FdcrSpec>;
#[doc = "Register `FDCR` writer"]
pub type W = crate::W<FdcrSpec>;
#[doc = "Fault Detect Pin ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fdpinid {
    #[doc = "0: Fault detection for LCD_P0 pin."]
    B000000 = 0,
    #[doc = "1: Fault detection for LCD_P1 pin."]
    B000001 = 1,
    #[doc = "63: Fault detection for LCD_P63 pin."]
    B111111 = 63,
}
impl From<Fdpinid> for u8 {
    #[inline(always)]
    fn from(variant: Fdpinid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fdpinid {
    type Ux = u8;
}
impl crate::IsEnum for Fdpinid {}
#[doc = "Field `FDPINID` reader - Fault Detect Pin ID"]
pub type FdpinidR = crate::FieldReader<Fdpinid>;
impl FdpinidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fdpinid> {
        match self.bits {
            0 => Some(Fdpinid::B000000),
            1 => Some(Fdpinid::B000001),
            63 => Some(Fdpinid::B111111),
            _ => None,
        }
    }
    #[doc = "Fault detection for LCD_P0 pin."]
    #[inline(always)]
    pub fn is_b000000(&self) -> bool {
        *self == Fdpinid::B000000
    }
    #[doc = "Fault detection for LCD_P1 pin."]
    #[inline(always)]
    pub fn is_b000001(&self) -> bool {
        *self == Fdpinid::B000001
    }
    #[doc = "Fault detection for LCD_P63 pin."]
    #[inline(always)]
    pub fn is_b111111(&self) -> bool {
        *self == Fdpinid::B111111
    }
}
#[doc = "Field `FDPINID` writer - Fault Detect Pin ID"]
pub type FdpinidW<'a, REG> = crate::FieldWriter<'a, REG, 6, Fdpinid>;
impl<'a, REG> FdpinidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Fault detection for LCD_P0 pin."]
    #[inline(always)]
    pub fn b000000(self) -> &'a mut crate::W<REG> {
        self.variant(Fdpinid::B000000)
    }
    #[doc = "Fault detection for LCD_P1 pin."]
    #[inline(always)]
    pub fn b000001(self) -> &'a mut crate::W<REG> {
        self.variant(Fdpinid::B000001)
    }
    #[doc = "Fault detection for LCD_P63 pin."]
    #[inline(always)]
    pub fn b111111(self) -> &'a mut crate::W<REG> {
        self.variant(Fdpinid::B111111)
    }
}
#[doc = "Fault Detect Back Plane Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdbpen {
    #[doc = "0: Type of the selected pin under fault detect test is front plane."]
    B0 = 0,
    #[doc = "1: Type of the selected pin under fault detect test is back plane."]
    B1 = 1,
}
impl From<Fdbpen> for bool {
    #[inline(always)]
    fn from(variant: Fdbpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDBPEN` reader - Fault Detect Back Plane Enable"]
pub type FdbpenR = crate::BitReader<Fdbpen>;
impl FdbpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdbpen {
        match self.bits {
            false => Fdbpen::B0,
            true => Fdbpen::B1,
        }
    }
    #[doc = "Type of the selected pin under fault detect test is front plane."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fdbpen::B0
    }
    #[doc = "Type of the selected pin under fault detect test is back plane."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fdbpen::B1
    }
}
#[doc = "Field `FDBPEN` writer - Fault Detect Back Plane Enable"]
pub type FdbpenW<'a, REG> = crate::BitWriter<'a, REG, Fdbpen>;
impl<'a, REG> FdbpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Type of the selected pin under fault detect test is front plane."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fdbpen::B0)
    }
    #[doc = "Type of the selected pin under fault detect test is back plane."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fdbpen::B1)
    }
}
#[doc = "Fault Detect Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fden {
    #[doc = "0: Disable fault detection."]
    B0 = 0,
    #[doc = "1: Enable fault detection."]
    B1 = 1,
}
impl From<Fden> for bool {
    #[inline(always)]
    fn from(variant: Fden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDEN` reader - Fault Detect Enable"]
pub type FdenR = crate::BitReader<Fden>;
impl FdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fden {
        match self.bits {
            false => Fden::B0,
            true => Fden::B1,
        }
    }
    #[doc = "Disable fault detection."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fden::B0
    }
    #[doc = "Enable fault detection."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fden::B1
    }
}
#[doc = "Field `FDEN` writer - Fault Detect Enable"]
pub type FdenW<'a, REG> = crate::BitWriter<'a, REG, Fden>;
impl<'a, REG> FdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable fault detection."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fden::B0)
    }
    #[doc = "Enable fault detection."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fden::B1)
    }
}
#[doc = "Fault Detect Sample Window Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fdsww {
    #[doc = "0: Sample window width is 4 sample clock cycles."]
    B000 = 0,
    #[doc = "1: Sample window width is 8 sample clock cycles."]
    B001 = 1,
    #[doc = "2: Sample window width is 16 sample clock cycles."]
    B010 = 2,
    #[doc = "3: Sample window width is 32 sample clock cycles."]
    B011 = 3,
    #[doc = "4: Sample window width is 64 sample clock cycles."]
    B100 = 4,
    #[doc = "5: Sample window width is 128 sample clock cycles."]
    B101 = 5,
    #[doc = "6: Sample window width is 256 sample clock cycles."]
    B110 = 6,
    #[doc = "7: Sample window width is 512 sample clock cycles."]
    B111 = 7,
}
impl From<Fdsww> for u8 {
    #[inline(always)]
    fn from(variant: Fdsww) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fdsww {
    type Ux = u8;
}
impl crate::IsEnum for Fdsww {}
#[doc = "Field `FDSWW` reader - Fault Detect Sample Window Width"]
pub type FdswwR = crate::FieldReader<Fdsww>;
impl FdswwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdsww {
        match self.bits {
            0 => Fdsww::B000,
            1 => Fdsww::B001,
            2 => Fdsww::B010,
            3 => Fdsww::B011,
            4 => Fdsww::B100,
            5 => Fdsww::B101,
            6 => Fdsww::B110,
            7 => Fdsww::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Sample window width is 4 sample clock cycles."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Fdsww::B000
    }
    #[doc = "Sample window width is 8 sample clock cycles."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Fdsww::B001
    }
    #[doc = "Sample window width is 16 sample clock cycles."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Fdsww::B010
    }
    #[doc = "Sample window width is 32 sample clock cycles."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Fdsww::B011
    }
    #[doc = "Sample window width is 64 sample clock cycles."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Fdsww::B100
    }
    #[doc = "Sample window width is 128 sample clock cycles."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Fdsww::B101
    }
    #[doc = "Sample window width is 256 sample clock cycles."]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Fdsww::B110
    }
    #[doc = "Sample window width is 512 sample clock cycles."]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Fdsww::B111
    }
}
#[doc = "Field `FDSWW` writer - Fault Detect Sample Window Width"]
pub type FdswwW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fdsww, crate::Safe>;
impl<'a, REG> FdswwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sample window width is 4 sample clock cycles."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Fdsww::B000)
    }
    #[doc = "Sample window width is 8 sample clock cycles."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Fdsww::B001)
    }
    #[doc = "Sample window width is 16 sample clock cycles."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Fdsww::B010)
    }
    #[doc = "Sample window width is 32 sample clock cycles."]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Fdsww::B011)
    }
    #[doc = "Sample window width is 64 sample clock cycles."]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Fdsww::B100)
    }
    #[doc = "Sample window width is 128 sample clock cycles."]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Fdsww::B101)
    }
    #[doc = "Sample window width is 256 sample clock cycles."]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Fdsww::B110)
    }
    #[doc = "Sample window width is 512 sample clock cycles."]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Fdsww::B111)
    }
}
#[doc = "Fault Detect Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fdprs {
    #[doc = "0: 1/1 bus clock."]
    B000 = 0,
    #[doc = "1: 1/2 bus clock."]
    B001 = 1,
    #[doc = "2: 1/4 bus clock."]
    B010 = 2,
    #[doc = "3: 1/8 bus clock."]
    B011 = 3,
    #[doc = "4: 1/16 bus clock."]
    B100 = 4,
    #[doc = "5: 1/32 bus clock."]
    B101 = 5,
    #[doc = "6: 1/64 bus clock."]
    B110 = 6,
    #[doc = "7: 1/128 bus clock."]
    B111 = 7,
}
impl From<Fdprs> for u8 {
    #[inline(always)]
    fn from(variant: Fdprs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fdprs {
    type Ux = u8;
}
impl crate::IsEnum for Fdprs {}
#[doc = "Field `FDPRS` reader - Fault Detect Clock Prescaler"]
pub type FdprsR = crate::FieldReader<Fdprs>;
impl FdprsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdprs {
        match self.bits {
            0 => Fdprs::B000,
            1 => Fdprs::B001,
            2 => Fdprs::B010,
            3 => Fdprs::B011,
            4 => Fdprs::B100,
            5 => Fdprs::B101,
            6 => Fdprs::B110,
            7 => Fdprs::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "1/1 bus clock."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Fdprs::B000
    }
    #[doc = "1/2 bus clock."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Fdprs::B001
    }
    #[doc = "1/4 bus clock."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Fdprs::B010
    }
    #[doc = "1/8 bus clock."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Fdprs::B011
    }
    #[doc = "1/16 bus clock."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Fdprs::B100
    }
    #[doc = "1/32 bus clock."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Fdprs::B101
    }
    #[doc = "1/64 bus clock."]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Fdprs::B110
    }
    #[doc = "1/128 bus clock."]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Fdprs::B111
    }
}
#[doc = "Field `FDPRS` writer - Fault Detect Clock Prescaler"]
pub type FdprsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fdprs, crate::Safe>;
impl<'a, REG> FdprsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1/1 bus clock."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Fdprs::B000)
    }
    #[doc = "1/2 bus clock."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Fdprs::B001)
    }
    #[doc = "1/4 bus clock."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Fdprs::B010)
    }
    #[doc = "1/8 bus clock."]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Fdprs::B011)
    }
    #[doc = "1/16 bus clock."]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Fdprs::B100)
    }
    #[doc = "1/32 bus clock."]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Fdprs::B101)
    }
    #[doc = "1/64 bus clock."]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Fdprs::B110)
    }
    #[doc = "1/128 bus clock."]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Fdprs::B111)
    }
}
impl R {
    #[doc = "Bits 0:5 - Fault Detect Pin ID"]
    #[inline(always)]
    pub fn fdpinid(&self) -> FdpinidR {
        FdpinidR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Fault Detect Back Plane Enable"]
    #[inline(always)]
    pub fn fdbpen(&self) -> FdbpenR {
        FdbpenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fault Detect Enable"]
    #[inline(always)]
    pub fn fden(&self) -> FdenR {
        FdenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Fault Detect Sample Window Width"]
    #[inline(always)]
    pub fn fdsww(&self) -> FdswwR {
        FdswwR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Fault Detect Clock Prescaler"]
    #[inline(always)]
    pub fn fdprs(&self) -> FdprsR {
        FdprsR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Fault Detect Pin ID"]
    #[inline(always)]
    #[must_use]
    pub fn fdpinid(&mut self) -> FdpinidW<FdcrSpec> {
        FdpinidW::new(self, 0)
    }
    #[doc = "Bit 6 - Fault Detect Back Plane Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdbpen(&mut self) -> FdbpenW<FdcrSpec> {
        FdbpenW::new(self, 6)
    }
    #[doc = "Bit 7 - Fault Detect Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fden(&mut self) -> FdenW<FdcrSpec> {
        FdenW::new(self, 7)
    }
    #[doc = "Bits 9:11 - Fault Detect Sample Window Width"]
    #[inline(always)]
    #[must_use]
    pub fn fdsww(&mut self) -> FdswwW<FdcrSpec> {
        FdswwW::new(self, 9)
    }
    #[doc = "Bits 12:14 - Fault Detect Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn fdprs(&mut self) -> FdprsW<FdcrSpec> {
        FdprsW::new(self, 12)
    }
}
#[doc = "LCD Fault Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcrSpec;
impl crate::RegisterSpec for FdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcr::R`](R) reader structure"]
impl crate::Readable for FdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcr::W`](W) writer structure"]
impl crate::Writable for FdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCR to value 0"]
impl crate::Resettable for FdcrSpec {
    const RESET_VALUE: u32 = 0;
}
