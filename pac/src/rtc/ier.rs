#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Time Invalid Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tiie {
    #[doc = "0: Time invalid flag does not generate an interrupt."]
    B0 = 0,
    #[doc = "1: Time invalid flag does generate an interrupt."]
    B1 = 1,
}
impl From<Tiie> for bool {
    #[inline(always)]
    fn from(variant: Tiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIIE` reader - Time Invalid Interrupt Enable"]
pub type TiieR = crate::BitReader<Tiie>;
impl TiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tiie {
        match self.bits {
            false => Tiie::B0,
            true => Tiie::B1,
        }
    }
    #[doc = "Time invalid flag does not generate an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tiie::B0
    }
    #[doc = "Time invalid flag does generate an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tiie::B1
    }
}
#[doc = "Field `TIIE` writer - Time Invalid Interrupt Enable"]
pub type TiieW<'a, REG> = crate::BitWriter<'a, REG, Tiie>;
impl<'a, REG> TiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time invalid flag does not generate an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tiie::B0)
    }
    #[doc = "Time invalid flag does generate an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tiie::B1)
    }
}
#[doc = "Time Overflow Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Toie {
    #[doc = "0: Time overflow flag does not generate an interrupt."]
    B0 = 0,
    #[doc = "1: Time overflow flag does generate an interrupt."]
    B1 = 1,
}
impl From<Toie> for bool {
    #[inline(always)]
    fn from(variant: Toie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOIE` reader - Time Overflow Interrupt Enable"]
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
    #[doc = "Time overflow flag does not generate an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Toie::B0
    }
    #[doc = "Time overflow flag does generate an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Toie::B1
    }
}
#[doc = "Field `TOIE` writer - Time Overflow Interrupt Enable"]
pub type ToieW<'a, REG> = crate::BitWriter<'a, REG, Toie>;
impl<'a, REG> ToieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time overflow flag does not generate an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Toie::B0)
    }
    #[doc = "Time overflow flag does generate an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Toie::B1)
    }
}
#[doc = "Time Alarm Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taie {
    #[doc = "0: Time alarm flag does not generate an interrupt."]
    B0 = 0,
    #[doc = "1: Time alarm flag does generate an interrupt."]
    B1 = 1,
}
impl From<Taie> for bool {
    #[inline(always)]
    fn from(variant: Taie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAIE` reader - Time Alarm Interrupt Enable"]
pub type TaieR = crate::BitReader<Taie>;
impl TaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taie {
        match self.bits {
            false => Taie::B0,
            true => Taie::B1,
        }
    }
    #[doc = "Time alarm flag does not generate an interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Taie::B0
    }
    #[doc = "Time alarm flag does generate an interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Taie::B1
    }
}
#[doc = "Field `TAIE` writer - Time Alarm Interrupt Enable"]
pub type TaieW<'a, REG> = crate::BitWriter<'a, REG, Taie>;
impl<'a, REG> TaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time alarm flag does not generate an interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Taie::B0)
    }
    #[doc = "Time alarm flag does generate an interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Taie::B1)
    }
}
#[doc = "Time Seconds Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsie {
    #[doc = "0: Seconds interrupt is disabled."]
    B0 = 0,
    #[doc = "1: Seconds interrupt is enabled."]
    B1 = 1,
}
impl From<Tsie> for bool {
    #[inline(always)]
    fn from(variant: Tsie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSIE` reader - Time Seconds Interrupt Enable"]
pub type TsieR = crate::BitReader<Tsie>;
impl TsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsie {
        match self.bits {
            false => Tsie::B0,
            true => Tsie::B1,
        }
    }
    #[doc = "Seconds interrupt is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tsie::B0
    }
    #[doc = "Seconds interrupt is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tsie::B1
    }
}
#[doc = "Field `TSIE` writer - Time Seconds Interrupt Enable"]
pub type TsieW<'a, REG> = crate::BitWriter<'a, REG, Tsie>;
impl<'a, REG> TsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Seconds interrupt is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tsie::B0)
    }
    #[doc = "Seconds interrupt is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tsie::B1)
    }
}
#[doc = "Wakeup Pin On\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wpon {
    #[doc = "0: No effect."]
    B0 = 0,
    #[doc = "1: If the wakeup pin is enabled, then the wakeup pin will assert."]
    B1 = 1,
}
impl From<Wpon> for bool {
    #[inline(always)]
    fn from(variant: Wpon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WPON` reader - Wakeup Pin On"]
pub type WponR = crate::BitReader<Wpon>;
impl WponR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wpon {
        match self.bits {
            false => Wpon::B0,
            true => Wpon::B1,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Wpon::B0
    }
    #[doc = "If the wakeup pin is enabled, then the wakeup pin will assert."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Wpon::B1
    }
}
#[doc = "Field `WPON` writer - Wakeup Pin On"]
pub type WponW<'a, REG> = crate::BitWriter<'a, REG, Wpon>;
impl<'a, REG> WponW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Wpon::B0)
    }
    #[doc = "If the wakeup pin is enabled, then the wakeup pin will assert."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Wpon::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
    #[inline(always)]
    pub fn tiie(&self) -> TiieR {
        TiieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&self) -> ToieR {
        ToieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
    #[inline(always)]
    pub fn taie(&self) -> TaieR {
        TaieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TsieR {
        TsieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Pin On"]
    #[inline(always)]
    pub fn wpon(&self) -> WponR {
        WponR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Invalid Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tiie(&mut self) -> TiieW<IerSpec> {
        TiieW::new(self, 0)
    }
    #[doc = "Bit 1 - Time Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> ToieW<IerSpec> {
        ToieW::new(self, 1)
    }
    #[doc = "Bit 2 - Time Alarm Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn taie(&mut self) -> TaieW<IerSpec> {
        TaieW::new(self, 2)
    }
    #[doc = "Bit 4 - Time Seconds Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TsieW<IerSpec> {
        TsieW::new(self, 4)
    }
    #[doc = "Bit 7 - Wakeup Pin On"]
    #[inline(always)]
    #[must_use]
    pub fn wpon(&mut self) -> WponW<IerSpec> {
        WponW::new(self, 7)
    }
}
#[doc = "RTC Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0x07"]
impl crate::Resettable for IerSpec {
    const RESET_VALUE: u32 = 0x07;
}
