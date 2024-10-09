#[doc = "Register `C2` reader"]
pub type R = crate::R<C2Spec>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2Spec>;
#[doc = "Low-frequency Internal Reference Clock Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ircs {
    #[doc = "0: LIRC is in 2 MHz mode."]
    B0 = 0,
    #[doc = "1: LIRC is in 8 MHz mode."]
    B1 = 1,
}
impl From<Ircs> for bool {
    #[inline(always)]
    fn from(variant: Ircs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCS` reader - Low-frequency Internal Reference Clock Select"]
pub type IrcsR = crate::BitReader<Ircs>;
impl IrcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ircs {
        match self.bits {
            false => Ircs::B0,
            true => Ircs::B1,
        }
    }
    #[doc = "LIRC is in 2 MHz mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ircs::B0
    }
    #[doc = "LIRC is in 8 MHz mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ircs::B1
    }
}
#[doc = "Field `IRCS` writer - Low-frequency Internal Reference Clock Select"]
pub type IrcsW<'a, REG> = crate::BitWriter<'a, REG, Ircs>;
impl<'a, REG> IrcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIRC is in 2 MHz mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ircs::B0)
    }
    #[doc = "LIRC is in 8 MHz mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ircs::B1)
    }
}
#[doc = "External Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erefs0 {
    #[doc = "0: External clock requested."]
    B0 = 0,
    #[doc = "1: Oscillator requested."]
    B1 = 1,
}
impl From<Erefs0> for bool {
    #[inline(always)]
    fn from(variant: Erefs0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EREFS0` reader - External Clock Source Select"]
pub type Erefs0R = crate::BitReader<Erefs0>;
impl Erefs0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erefs0 {
        match self.bits {
            false => Erefs0::B0,
            true => Erefs0::B1,
        }
    }
    #[doc = "External clock requested."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Erefs0::B0
    }
    #[doc = "Oscillator requested."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Erefs0::B1
    }
}
#[doc = "Field `EREFS0` writer - External Clock Source Select"]
pub type Erefs0W<'a, REG> = crate::BitWriter<'a, REG, Erefs0>;
impl<'a, REG> Erefs0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "External clock requested."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Erefs0::B0)
    }
    #[doc = "Oscillator requested."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Erefs0::B1)
    }
}
#[doc = "Crystal Oscillator Operation Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hgo0 {
    #[doc = "0: Configure crystal oscillator for low-power operation."]
    B0 = 0,
    #[doc = "1: Configure crystal oscillator for high-gain operation."]
    B1 = 1,
}
impl From<Hgo0> for bool {
    #[inline(always)]
    fn from(variant: Hgo0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HGO0` reader - Crystal Oscillator Operation Mode Select"]
pub type Hgo0R = crate::BitReader<Hgo0>;
impl Hgo0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hgo0 {
        match self.bits {
            false => Hgo0::B0,
            true => Hgo0::B1,
        }
    }
    #[doc = "Configure crystal oscillator for low-power operation."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hgo0::B0
    }
    #[doc = "Configure crystal oscillator for high-gain operation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hgo0::B1
    }
}
#[doc = "Field `HGO0` writer - Crystal Oscillator Operation Mode Select"]
pub type Hgo0W<'a, REG> = crate::BitWriter<'a, REG, Hgo0>;
impl<'a, REG> Hgo0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configure crystal oscillator for low-power operation."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hgo0::B0)
    }
    #[doc = "Configure crystal oscillator for high-gain operation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hgo0::B1)
    }
}
#[doc = "External Clock Source Frequency Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Range0 {
    #[doc = "0: Low frequency range selected for the crystal oscillator or the external clock source."]
    B00 = 0,
    #[doc = "1: High frequency range selected for the crystal oscillator or the external clock source."]
    B01 = 1,
    #[doc = "2: Very high frequency range selected for the crystal oscillator or the external clock source."]
    B10 = 2,
    #[doc = "3: Very high frequency range selected for the crystal oscillator or the external clock source. Same effect as 10."]
    B11 = 3,
}
impl From<Range0> for u8 {
    #[inline(always)]
    fn from(variant: Range0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Range0 {
    type Ux = u8;
}
impl crate::IsEnum for Range0 {}
#[doc = "Field `RANGE0` reader - External Clock Source Frequency Range Select"]
pub type Range0R = crate::FieldReader<Range0>;
impl Range0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Range0 {
        match self.bits {
            0 => Range0::B00,
            1 => Range0::B01,
            2 => Range0::B10,
            3 => Range0::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Low frequency range selected for the crystal oscillator or the external clock source."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Range0::B00
    }
    #[doc = "High frequency range selected for the crystal oscillator or the external clock source."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Range0::B01
    }
    #[doc = "Very high frequency range selected for the crystal oscillator or the external clock source."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Range0::B10
    }
    #[doc = "Very high frequency range selected for the crystal oscillator or the external clock source. Same effect as 10."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Range0::B11
    }
}
#[doc = "Field `RANGE0` writer - External Clock Source Frequency Range Select"]
pub type Range0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Range0, crate::Safe>;
impl<'a, REG> Range0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low frequency range selected for the crystal oscillator or the external clock source."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::B00)
    }
    #[doc = "High frequency range selected for the crystal oscillator or the external clock source."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::B01)
    }
    #[doc = "Very high frequency range selected for the crystal oscillator or the external clock source."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::B10)
    }
    #[doc = "Very high frequency range selected for the crystal oscillator or the external clock source. Same effect as 10."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Range0::B11)
    }
}
impl R {
    #[doc = "Bit 0 - Low-frequency Internal Reference Clock Select"]
    #[inline(always)]
    pub fn ircs(&self) -> IrcsR {
        IrcsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - External Clock Source Select"]
    #[inline(always)]
    pub fn erefs0(&self) -> Erefs0R {
        Erefs0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Crystal Oscillator Operation Mode Select"]
    #[inline(always)]
    pub fn hgo0(&self) -> Hgo0R {
        Hgo0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - External Clock Source Frequency Range Select"]
    #[inline(always)]
    pub fn range0(&self) -> Range0R {
        Range0R::new((self.bits >> 4) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Low-frequency Internal Reference Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn ircs(&mut self) -> IrcsW<C2Spec> {
        IrcsW::new(self, 0)
    }
    #[doc = "Bit 2 - External Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn erefs0(&mut self) -> Erefs0W<C2Spec> {
        Erefs0W::new(self, 2)
    }
    #[doc = "Bit 3 - Crystal Oscillator Operation Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn hgo0(&mut self) -> Hgo0W<C2Spec> {
        Hgo0W::new(self, 3)
    }
    #[doc = "Bits 4:5 - External Clock Source Frequency Range Select"]
    #[inline(always)]
    #[must_use]
    pub fn range0(&mut self) -> Range0W<C2Spec> {
        Range0W::new(self, 4)
    }
}
#[doc = "MCG Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2Spec;
impl crate::RegisterSpec for C2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c2::R`](R) reader structure"]
impl crate::Readable for C2Spec {}
#[doc = "`write(|w| ..)` method takes [`c2::W`](W) writer structure"]
impl crate::Writable for C2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C2 to value 0x01"]
impl crate::Resettable for C2Spec {
    const RESET_VALUE: u8 = 0x01;
}
