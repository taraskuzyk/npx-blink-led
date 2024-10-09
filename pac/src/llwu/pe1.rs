#[doc = "Register `PE1` reader"]
pub type R = crate::R<Pe1Spec>;
#[doc = "Register `PE1` writer"]
pub type W = crate::W<Pe1Spec>;
#[doc = "Wakeup Pin Enable For LLWU_P0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe0 {
    #[doc = "0: External input pin disabled as wakeup input"]
    B00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    B01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    B10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    B11 = 3,
}
impl From<Wupe0> for u8 {
    #[inline(always)]
    fn from(variant: Wupe0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe0 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe0 {}
#[doc = "Field `WUPE0` reader - Wakeup Pin Enable For LLWU_P0"]
pub type Wupe0R = crate::FieldReader<Wupe0>;
impl Wupe0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe0 {
        match self.bits {
            0 => Wupe0::B00,
            1 => Wupe0::B01,
            2 => Wupe0::B10,
            3 => Wupe0::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Wupe0::B00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Wupe0::B01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Wupe0::B10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Wupe0::B11
    }
}
#[doc = "Field `WUPE0` writer - Wakeup Pin Enable For LLWU_P0"]
pub type Wupe0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe0, crate::Safe>;
impl<'a, REG> Wupe0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe0::B00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe0::B01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe0::B10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe0::B11)
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe1 {
    #[doc = "0: External input pin disabled as wakeup input"]
    B00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    B01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    B10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    B11 = 3,
}
impl From<Wupe1> for u8 {
    #[inline(always)]
    fn from(variant: Wupe1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe1 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe1 {}
#[doc = "Field `WUPE1` reader - Wakeup Pin Enable For LLWU_P1"]
pub type Wupe1R = crate::FieldReader<Wupe1>;
impl Wupe1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe1 {
        match self.bits {
            0 => Wupe1::B00,
            1 => Wupe1::B01,
            2 => Wupe1::B10,
            3 => Wupe1::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Wupe1::B00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Wupe1::B01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Wupe1::B10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Wupe1::B11
    }
}
#[doc = "Field `WUPE1` writer - Wakeup Pin Enable For LLWU_P1"]
pub type Wupe1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe1, crate::Safe>;
impl<'a, REG> Wupe1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe1::B00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe1::B01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe1::B10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe1::B11)
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe2 {
    #[doc = "0: External input pin disabled as wakeup input"]
    B00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    B01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    B10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    B11 = 3,
}
impl From<Wupe2> for u8 {
    #[inline(always)]
    fn from(variant: Wupe2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe2 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe2 {}
#[doc = "Field `WUPE2` reader - Wakeup Pin Enable For LLWU_P2"]
pub type Wupe2R = crate::FieldReader<Wupe2>;
impl Wupe2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe2 {
        match self.bits {
            0 => Wupe2::B00,
            1 => Wupe2::B01,
            2 => Wupe2::B10,
            3 => Wupe2::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Wupe2::B00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Wupe2::B01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Wupe2::B10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Wupe2::B11
    }
}
#[doc = "Field `WUPE2` writer - Wakeup Pin Enable For LLWU_P2"]
pub type Wupe2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe2, crate::Safe>;
impl<'a, REG> Wupe2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe2::B00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe2::B01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe2::B10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe2::B11)
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe3 {
    #[doc = "0: External input pin disabled as wakeup input"]
    B00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    B01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    B10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    B11 = 3,
}
impl From<Wupe3> for u8 {
    #[inline(always)]
    fn from(variant: Wupe3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe3 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe3 {}
#[doc = "Field `WUPE3` reader - Wakeup Pin Enable For LLWU_P3"]
pub type Wupe3R = crate::FieldReader<Wupe3>;
impl Wupe3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe3 {
        match self.bits {
            0 => Wupe3::B00,
            1 => Wupe3::B01,
            2 => Wupe3::B10,
            3 => Wupe3::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Wupe3::B00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Wupe3::B01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Wupe3::B10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Wupe3::B11
    }
}
#[doc = "Field `WUPE3` writer - Wakeup Pin Enable For LLWU_P3"]
pub type Wupe3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe3, crate::Safe>;
impl<'a, REG> Wupe3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe3::B00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe3::B01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe3::B10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe3::B11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P0"]
    #[inline(always)]
    pub fn wupe0(&self) -> Wupe0R {
        Wupe0R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P1"]
    #[inline(always)]
    pub fn wupe1(&self) -> Wupe1R {
        Wupe1R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P2"]
    #[inline(always)]
    pub fn wupe2(&self) -> Wupe2R {
        Wupe2R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P3"]
    #[inline(always)]
    pub fn wupe3(&self) -> Wupe3R {
        Wupe3R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P0"]
    #[inline(always)]
    #[must_use]
    pub fn wupe0(&mut self) -> Wupe0W<Pe1Spec> {
        Wupe0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P1"]
    #[inline(always)]
    #[must_use]
    pub fn wupe1(&mut self) -> Wupe1W<Pe1Spec> {
        Wupe1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P2"]
    #[inline(always)]
    #[must_use]
    pub fn wupe2(&mut self) -> Wupe2W<Pe1Spec> {
        Wupe2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P3"]
    #[inline(always)]
    #[must_use]
    pub fn wupe3(&mut self) -> Wupe3W<Pe1Spec> {
        Wupe3W::new(self, 6)
    }
}
#[doc = "LLWU Pin Enable 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pe1Spec;
impl crate::RegisterSpec for Pe1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pe1::R`](R) reader structure"]
impl crate::Readable for Pe1Spec {}
#[doc = "`write(|w| ..)` method takes [`pe1::W`](W) writer structure"]
impl crate::Writable for Pe1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PE1 to value 0"]
impl crate::Resettable for Pe1Spec {
    const RESET_VALUE: u8 = 0;
}
