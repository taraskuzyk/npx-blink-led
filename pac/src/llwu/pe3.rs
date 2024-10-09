#[doc = "Register `PE3` reader"]
pub type R = crate::R<Pe3Spec>;
#[doc = "Register `PE3` writer"]
pub type W = crate::W<Pe3Spec>;
#[doc = "Wakeup Pin Enable For LLWU_P8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe8 {
    #[doc = "0: External input pin disabled as wakeup input"]
    B00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    B01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    B10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    B11 = 3,
}
impl From<Wupe8> for u8 {
    #[inline(always)]
    fn from(variant: Wupe8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe8 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe8 {}
#[doc = "Field `WUPE8` reader - Wakeup Pin Enable For LLWU_P8"]
pub type Wupe8R = crate::FieldReader<Wupe8>;
impl Wupe8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe8 {
        match self.bits {
            0 => Wupe8::B00,
            1 => Wupe8::B01,
            2 => Wupe8::B10,
            3 => Wupe8::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Wupe8::B00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Wupe8::B01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Wupe8::B10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Wupe8::B11
    }
}
#[doc = "Field `WUPE8` writer - Wakeup Pin Enable For LLWU_P8"]
pub type Wupe8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe8, crate::Safe>;
impl<'a, REG> Wupe8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe8::B00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe8::B01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe8::B10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe8::B11)
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe9 {
    #[doc = "0: External input pin disabled as wakeup input"]
    B00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    B01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    B10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    B11 = 3,
}
impl From<Wupe9> for u8 {
    #[inline(always)]
    fn from(variant: Wupe9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe9 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe9 {}
#[doc = "Field `WUPE9` reader - Wakeup Pin Enable For LLWU_P9"]
pub type Wupe9R = crate::FieldReader<Wupe9>;
impl Wupe9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe9 {
        match self.bits {
            0 => Wupe9::B00,
            1 => Wupe9::B01,
            2 => Wupe9::B10,
            3 => Wupe9::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Wupe9::B00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Wupe9::B01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Wupe9::B10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Wupe9::B11
    }
}
#[doc = "Field `WUPE9` writer - Wakeup Pin Enable For LLWU_P9"]
pub type Wupe9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe9, crate::Safe>;
impl<'a, REG> Wupe9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe9::B00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe9::B01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe9::B10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe9::B11)
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe10 {
    #[doc = "0: External input pin disabled as wakeup input"]
    B00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    B01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    B10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    B11 = 3,
}
impl From<Wupe10> for u8 {
    #[inline(always)]
    fn from(variant: Wupe10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe10 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe10 {}
#[doc = "Field `WUPE10` reader - Wakeup Pin Enable For LLWU_P10"]
pub type Wupe10R = crate::FieldReader<Wupe10>;
impl Wupe10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe10 {
        match self.bits {
            0 => Wupe10::B00,
            1 => Wupe10::B01,
            2 => Wupe10::B10,
            3 => Wupe10::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Wupe10::B00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Wupe10::B01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Wupe10::B10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Wupe10::B11
    }
}
#[doc = "Field `WUPE10` writer - Wakeup Pin Enable For LLWU_P10"]
pub type Wupe10W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe10, crate::Safe>;
impl<'a, REG> Wupe10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe10::B00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe10::B01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe10::B10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe10::B11)
    }
}
#[doc = "Wakeup Pin Enable For LLWU_P11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wupe11 {
    #[doc = "0: External input pin disabled as wakeup input"]
    B00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    B01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    B10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    B11 = 3,
}
impl From<Wupe11> for u8 {
    #[inline(always)]
    fn from(variant: Wupe11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wupe11 {
    type Ux = u8;
}
impl crate::IsEnum for Wupe11 {}
#[doc = "Field `WUPE11` reader - Wakeup Pin Enable For LLWU_P11"]
pub type Wupe11R = crate::FieldReader<Wupe11>;
impl Wupe11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wupe11 {
        match self.bits {
            0 => Wupe11::B00,
            1 => Wupe11::B01,
            2 => Wupe11::B10,
            3 => Wupe11::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Wupe11::B00
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Wupe11::B01
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Wupe11::B10
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Wupe11::B11
    }
}
#[doc = "Field `WUPE11` writer - Wakeup Pin Enable For LLWU_P11"]
pub type Wupe11W<'a, REG> = crate::FieldWriter<'a, REG, 2, Wupe11, crate::Safe>;
impl<'a, REG> Wupe11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe11::B00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe11::B01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe11::B10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Wupe11::B11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P8"]
    #[inline(always)]
    pub fn wupe8(&self) -> Wupe8R {
        Wupe8R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P9"]
    #[inline(always)]
    pub fn wupe9(&self) -> Wupe9R {
        Wupe9R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P10"]
    #[inline(always)]
    pub fn wupe10(&self) -> Wupe10R {
        Wupe10R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P11"]
    #[inline(always)]
    pub fn wupe11(&self) -> Wupe11R {
        Wupe11R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P8"]
    #[inline(always)]
    #[must_use]
    pub fn wupe8(&mut self) -> Wupe8W<Pe3Spec> {
        Wupe8W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P9"]
    #[inline(always)]
    #[must_use]
    pub fn wupe9(&mut self) -> Wupe9W<Pe3Spec> {
        Wupe9W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P10"]
    #[inline(always)]
    #[must_use]
    pub fn wupe10(&mut self) -> Wupe10W<Pe3Spec> {
        Wupe10W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P11"]
    #[inline(always)]
    #[must_use]
    pub fn wupe11(&mut self) -> Wupe11W<Pe3Spec> {
        Wupe11W::new(self, 6)
    }
}
#[doc = "LLWU Pin Enable 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`pe3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pe3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pe3Spec;
impl crate::RegisterSpec for Pe3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pe3::R`](R) reader structure"]
impl crate::Readable for Pe3Spec {}
#[doc = "`write(|w| ..)` method takes [`pe3::W`](W) writer structure"]
impl crate::Writable for Pe3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PE3 to value 0"]
impl crate::Resettable for Pe3Spec {
    const RESET_VALUE: u8 = 0;
}
