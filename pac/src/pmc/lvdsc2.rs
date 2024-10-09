#[doc = "Register `LVDSC2` reader"]
pub type R = crate::R<Lvdsc2Spec>;
#[doc = "Register `LVDSC2` writer"]
pub type W = crate::W<Lvdsc2Spec>;
#[doc = "Low-Voltage Warning Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lvwv {
    #[doc = "0: Low trip point selected (VLVW = VLVW1)"]
    B00 = 0,
    #[doc = "1: Mid 1 trip point selected (VLVW = VLVW2)"]
    B01 = 1,
    #[doc = "2: Mid 2 trip point selected (VLVW = VLVW3)"]
    B10 = 2,
    #[doc = "3: High trip point selected (VLVW = VLVW4)"]
    B11 = 3,
}
impl From<Lvwv> for u8 {
    #[inline(always)]
    fn from(variant: Lvwv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lvwv {
    type Ux = u8;
}
impl crate::IsEnum for Lvwv {}
#[doc = "Field `LVWV` reader - Low-Voltage Warning Voltage Select"]
pub type LvwvR = crate::FieldReader<Lvwv>;
impl LvwvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvwv {
        match self.bits {
            0 => Lvwv::B00,
            1 => Lvwv::B01,
            2 => Lvwv::B10,
            3 => Lvwv::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Low trip point selected (VLVW = VLVW1)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lvwv::B00
    }
    #[doc = "Mid 1 trip point selected (VLVW = VLVW2)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lvwv::B01
    }
    #[doc = "Mid 2 trip point selected (VLVW = VLVW3)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lvwv::B10
    }
    #[doc = "High trip point selected (VLVW = VLVW4)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Lvwv::B11
    }
}
#[doc = "Field `LVWV` writer - Low-Voltage Warning Voltage Select"]
pub type LvwvW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lvwv, crate::Safe>;
impl<'a, REG> LvwvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low trip point selected (VLVW = VLVW1)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lvwv::B00)
    }
    #[doc = "Mid 1 trip point selected (VLVW = VLVW2)"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lvwv::B01)
    }
    #[doc = "Mid 2 trip point selected (VLVW = VLVW3)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lvwv::B10)
    }
    #[doc = "High trip point selected (VLVW = VLVW4)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Lvwv::B11)
    }
}
#[doc = "Low-Voltage Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvwie {
    #[doc = "0: Hardware interrupt disabled (use polling)"]
    B0 = 0,
    #[doc = "1: Request a hardware interrupt when LVWF = 1"]
    B1 = 1,
}
impl From<Lvwie> for bool {
    #[inline(always)]
    fn from(variant: Lvwie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVWIE` reader - Low-Voltage Warning Interrupt Enable"]
pub type LvwieR = crate::BitReader<Lvwie>;
impl LvwieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvwie {
        match self.bits {
            false => Lvwie::B0,
            true => Lvwie::B1,
        }
    }
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lvwie::B0
    }
    #[doc = "Request a hardware interrupt when LVWF = 1"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lvwie::B1
    }
}
#[doc = "Field `LVWIE` writer - Low-Voltage Warning Interrupt Enable"]
pub type LvwieW<'a, REG> = crate::BitWriter<'a, REG, Lvwie>;
impl<'a, REG> LvwieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lvwie::B0)
    }
    #[doc = "Request a hardware interrupt when LVWF = 1"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lvwie::B1)
    }
}
#[doc = "Field `LVWACK` reader - Low-Voltage Warning Acknowledge"]
pub type LvwackR = crate::BitReader;
#[doc = "Field `LVWACK` writer - Low-Voltage Warning Acknowledge"]
pub type LvwackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Low-Voltage Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lvwf {
    #[doc = "0: Low-voltage warning event not detected"]
    B0 = 0,
    #[doc = "1: Low-voltage warning event detected"]
    B1 = 1,
}
impl From<Lvwf> for bool {
    #[inline(always)]
    fn from(variant: Lvwf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVWF` reader - Low-Voltage Warning Flag"]
pub type LvwfR = crate::BitReader<Lvwf>;
impl LvwfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lvwf {
        match self.bits {
            false => Lvwf::B0,
            true => Lvwf::B1,
        }
    }
    #[doc = "Low-voltage warning event not detected"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lvwf::B0
    }
    #[doc = "Low-voltage warning event detected"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lvwf::B1
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-Voltage Warning Voltage Select"]
    #[inline(always)]
    pub fn lvwv(&self) -> LvwvR {
        LvwvR::new(self.bits & 3)
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    pub fn lvwie(&self) -> LvwieR {
        LvwieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-Voltage Warning Acknowledge"]
    #[inline(always)]
    pub fn lvwack(&self) -> LvwackR {
        LvwackR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-Voltage Warning Flag"]
    #[inline(always)]
    pub fn lvwf(&self) -> LvwfR {
        LvwfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-Voltage Warning Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn lvwv(&mut self) -> LvwvW<Lvdsc2Spec> {
        LvwvW::new(self, 0)
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvwie(&mut self) -> LvwieW<Lvdsc2Spec> {
        LvwieW::new(self, 5)
    }
    #[doc = "Bit 6 - Low-Voltage Warning Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn lvwack(&mut self) -> LvwackW<Lvdsc2Spec> {
        LvwackW::new(self, 6)
    }
}
#[doc = "Low Voltage Detect Status And Control 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdsc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdsc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lvdsc2Spec;
impl crate::RegisterSpec for Lvdsc2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvdsc2::R`](R) reader structure"]
impl crate::Readable for Lvdsc2Spec {}
#[doc = "`write(|w| ..)` method takes [`lvdsc2::W`](W) writer structure"]
impl crate::Writable for Lvdsc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets LVDSC2 to value 0"]
impl crate::Resettable for Lvdsc2Spec {
    const RESET_VALUE: u8 = 0;
}
