#[doc = "Register `FILT2` reader"]
pub type R = crate::R<Filt2Spec>;
#[doc = "Register `FILT2` writer"]
pub type W = crate::W<Filt2Spec>;
#[doc = "Filter Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filtsel {
    #[doc = "0: Select LLWU_P0 for filter"]
    B0000 = 0,
    #[doc = "15: Select LLWU_P15 for filter"]
    B1111 = 15,
}
impl From<Filtsel> for u8 {
    #[inline(always)]
    fn from(variant: Filtsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filtsel {
    type Ux = u8;
}
impl crate::IsEnum for Filtsel {}
#[doc = "Field `FILTSEL` reader - Filter Pin Select"]
pub type FiltselR = crate::FieldReader<Filtsel>;
impl FiltselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Filtsel> {
        match self.bits {
            0 => Some(Filtsel::B0000),
            15 => Some(Filtsel::B1111),
            _ => None,
        }
    }
    #[doc = "Select LLWU_P0 for filter"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Filtsel::B0000
    }
    #[doc = "Select LLWU_P15 for filter"]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == Filtsel::B1111
    }
}
#[doc = "Field `FILTSEL` writer - Filter Pin Select"]
pub type FiltselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Filtsel>;
impl<'a, REG> FiltselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select LLWU_P0 for filter"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Filtsel::B0000)
    }
    #[doc = "Select LLWU_P15 for filter"]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(Filtsel::B1111)
    }
}
#[doc = "Digital Filter On External Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filte {
    #[doc = "0: Filter disabled"]
    B00 = 0,
    #[doc = "1: Filter posedge detect enabled"]
    B01 = 1,
    #[doc = "2: Filter negedge detect enabled"]
    B10 = 2,
    #[doc = "3: Filter any edge detect enabled"]
    B11 = 3,
}
impl From<Filte> for u8 {
    #[inline(always)]
    fn from(variant: Filte) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filte {
    type Ux = u8;
}
impl crate::IsEnum for Filte {}
#[doc = "Field `FILTE` reader - Digital Filter On External Pin"]
pub type FilteR = crate::FieldReader<Filte>;
impl FilteR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filte {
        match self.bits {
            0 => Filte::B00,
            1 => Filte::B01,
            2 => Filte::B10,
            3 => Filte::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Filte::B00
    }
    #[doc = "Filter posedge detect enabled"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Filte::B01
    }
    #[doc = "Filter negedge detect enabled"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Filte::B10
    }
    #[doc = "Filter any edge detect enabled"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Filte::B11
    }
}
#[doc = "Field `FILTE` writer - Digital Filter On External Pin"]
pub type FilteW<'a, REG> = crate::FieldWriter<'a, REG, 2, Filte, crate::Safe>;
impl<'a, REG> FilteW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Filte::B00)
    }
    #[doc = "Filter posedge detect enabled"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Filte::B01)
    }
    #[doc = "Filter negedge detect enabled"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Filte::B10)
    }
    #[doc = "Filter any edge detect enabled"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Filte::B11)
    }
}
#[doc = "Filter Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filtf {
    #[doc = "0: Pin Filter 2 was not a wakeup source"]
    B0 = 0,
    #[doc = "1: Pin Filter 2 was a wakeup source"]
    B1 = 1,
}
impl From<Filtf> for bool {
    #[inline(always)]
    fn from(variant: Filtf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTF` reader - Filter Detect Flag"]
pub type FiltfR = crate::BitReader<Filtf>;
impl FiltfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filtf {
        match self.bits {
            false => Filtf::B0,
            true => Filtf::B1,
        }
    }
    #[doc = "Pin Filter 2 was not a wakeup source"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Filtf::B0
    }
    #[doc = "Pin Filter 2 was a wakeup source"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Filtf::B1
    }
}
#[doc = "Field `FILTF` writer - Filter Detect Flag"]
pub type FiltfW<'a, REG> = crate::BitWriter<'a, REG, Filtf>;
impl<'a, REG> FiltfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin Filter 2 was not a wakeup source"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Filtf::B0)
    }
    #[doc = "Pin Filter 2 was a wakeup source"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Filtf::B1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Filter Pin Select"]
    #[inline(always)]
    pub fn filtsel(&self) -> FiltselR {
        FiltselR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 5:6 - Digital Filter On External Pin"]
    #[inline(always)]
    pub fn filte(&self) -> FilteR {
        FilteR::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7 - Filter Detect Flag"]
    #[inline(always)]
    pub fn filtf(&self) -> FiltfR {
        FiltfR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn filtsel(&mut self) -> FiltselW<Filt2Spec> {
        FiltselW::new(self, 0)
    }
    #[doc = "Bits 5:6 - Digital Filter On External Pin"]
    #[inline(always)]
    #[must_use]
    pub fn filte(&mut self) -> FilteW<Filt2Spec> {
        FilteW::new(self, 5)
    }
    #[doc = "Bit 7 - Filter Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn filtf(&mut self) -> FiltfW<Filt2Spec> {
        FiltfW::new(self, 7)
    }
}
#[doc = "LLWU Pin Filter 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`filt2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filt2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Filt2Spec;
impl crate::RegisterSpec for Filt2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`filt2::R`](R) reader structure"]
impl crate::Readable for Filt2Spec {}
#[doc = "`write(|w| ..)` method takes [`filt2::W`](W) writer structure"]
impl crate::Writable for Filt2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FILT2 to value 0"]
impl crate::Resettable for Filt2Spec {
    const RESET_VALUE: u8 = 0;
}
