#[doc = "Register `FLT` reader"]
pub type R = crate::R<FltSpec>;
#[doc = "Register `FLT` writer"]
pub type W = crate::W<FltSpec>;
#[doc = "I2C Programmable Filter Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flt {
    #[doc = "0: No filter/bypass"]
    B0000 = 0,
}
impl From<Flt> for u8 {
    #[inline(always)]
    fn from(variant: Flt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flt {
    type Ux = u8;
}
impl crate::IsEnum for Flt {}
#[doc = "Field `FLT` reader - I2C Programmable Filter Factor"]
pub type FltR = crate::FieldReader<Flt>;
impl FltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flt> {
        match self.bits {
            0 => Some(Flt::B0000),
            _ => None,
        }
    }
    #[doc = "No filter/bypass"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Flt::B0000
    }
}
#[doc = "Field `FLT` writer - I2C Programmable Filter Factor"]
pub type FltW<'a, REG> = crate::FieldWriter<'a, REG, 4, Flt>;
impl<'a, REG> FltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filter/bypass"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Flt::B0000)
    }
}
#[doc = "I2C Bus Start Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Startf {
    #[doc = "0: No start happens on I2C bus"]
    B0 = 0,
    #[doc = "1: Start detected on I2C bus"]
    B1 = 1,
}
impl From<Startf> for bool {
    #[inline(always)]
    fn from(variant: Startf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTF` reader - I2C Bus Start Detect Flag"]
pub type StartfR = crate::BitReader<Startf>;
impl StartfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startf {
        match self.bits {
            false => Startf::B0,
            true => Startf::B1,
        }
    }
    #[doc = "No start happens on I2C bus"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Startf::B0
    }
    #[doc = "Start detected on I2C bus"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Startf::B1
    }
}
#[doc = "Field `STARTF` writer - I2C Bus Start Detect Flag"]
pub type StartfW<'a, REG> = crate::BitWriter<'a, REG, Startf>;
impl<'a, REG> StartfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No start happens on I2C bus"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Startf::B0)
    }
    #[doc = "Start detected on I2C bus"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Startf::B1)
    }
}
#[doc = "I2C Bus Stop or Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ssie {
    #[doc = "0: Stop or start detection interrupt is disabled"]
    B0 = 0,
    #[doc = "1: Stop or start detection interrupt is enabled"]
    B1 = 1,
}
impl From<Ssie> for bool {
    #[inline(always)]
    fn from(variant: Ssie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSIE` reader - I2C Bus Stop or Start Interrupt Enable"]
pub type SsieR = crate::BitReader<Ssie>;
impl SsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssie {
        match self.bits {
            false => Ssie::B0,
            true => Ssie::B1,
        }
    }
    #[doc = "Stop or start detection interrupt is disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ssie::B0
    }
    #[doc = "Stop or start detection interrupt is enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ssie::B1
    }
}
#[doc = "Field `SSIE` writer - I2C Bus Stop or Start Interrupt Enable"]
pub type SsieW<'a, REG> = crate::BitWriter<'a, REG, Ssie>;
impl<'a, REG> SsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop or start detection interrupt is disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ssie::B0)
    }
    #[doc = "Stop or start detection interrupt is enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ssie::B1)
    }
}
#[doc = "I2C Bus Stop Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopf {
    #[doc = "0: No stop happens on I2C bus"]
    B0 = 0,
    #[doc = "1: Stop detected on I2C bus"]
    B1 = 1,
}
impl From<Stopf> for bool {
    #[inline(always)]
    fn from(variant: Stopf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPF` reader - I2C Bus Stop Detect Flag"]
pub type StopfR = crate::BitReader<Stopf>;
impl StopfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopf {
        match self.bits {
            false => Stopf::B0,
            true => Stopf::B1,
        }
    }
    #[doc = "No stop happens on I2C bus"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stopf::B0
    }
    #[doc = "Stop detected on I2C bus"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stopf::B1
    }
}
#[doc = "Field `STOPF` writer - I2C Bus Stop Detect Flag"]
pub type StopfW<'a, REG> = crate::BitWriter<'a, REG, Stopf>;
impl<'a, REG> StopfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No stop happens on I2C bus"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stopf::B0)
    }
    #[doc = "Stop detected on I2C bus"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stopf::B1)
    }
}
#[doc = "Stop Hold Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shen {
    #[doc = "0: Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    B0 = 0,
    #[doc = "1: Stop holdoff is enabled."]
    B1 = 1,
}
impl From<Shen> for bool {
    #[inline(always)]
    fn from(variant: Shen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SHEN` reader - Stop Hold Enable"]
pub type ShenR = crate::BitReader<Shen>;
impl ShenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Shen {
        match self.bits {
            false => Shen::B0,
            true => Shen::B1,
        }
    }
    #[doc = "Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Shen::B0
    }
    #[doc = "Stop holdoff is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Shen::B1
    }
}
#[doc = "Field `SHEN` writer - Stop Hold Enable"]
pub type ShenW<'a, REG> = crate::BitWriter<'a, REG, Shen>;
impl<'a, REG> ShenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stop holdoff is disabled. The MCU's entry to stop mode is not gated."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Shen::B0)
    }
    #[doc = "Stop holdoff is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Shen::B1)
    }
}
impl R {
    #[doc = "Bits 0:3 - I2C Programmable Filter Factor"]
    #[inline(always)]
    pub fn flt(&self) -> FltR {
        FltR::new(self.bits & 0x0f)
    }
    #[doc = "Bit 4 - I2C Bus Start Detect Flag"]
    #[inline(always)]
    pub fn startf(&self) -> StartfR {
        StartfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Bus Stop or Start Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SsieR {
        SsieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Bus Stop Detect Flag"]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stop Hold Enable"]
    #[inline(always)]
    pub fn shen(&self) -> ShenR {
        ShenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - I2C Programmable Filter Factor"]
    #[inline(always)]
    #[must_use]
    pub fn flt(&mut self) -> FltW<FltSpec> {
        FltW::new(self, 0)
    }
    #[doc = "Bit 4 - I2C Bus Start Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn startf(&mut self) -> StartfW<FltSpec> {
        StartfW::new(self, 4)
    }
    #[doc = "Bit 5 - I2C Bus Stop or Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssie(&mut self) -> SsieW<FltSpec> {
        SsieW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C Bus Stop Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn stopf(&mut self) -> StopfW<FltSpec> {
        StopfW::new(self, 6)
    }
    #[doc = "Bit 7 - Stop Hold Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shen(&mut self) -> ShenW<FltSpec> {
        ShenW::new(self, 7)
    }
}
#[doc = "I2C Programmable Input Glitch Filter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltSpec;
impl crate::RegisterSpec for FltSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`flt::R`](R) reader structure"]
impl crate::Readable for FltSpec {}
#[doc = "`write(|w| ..)` method takes [`flt::W`](W) writer structure"]
impl crate::Writable for FltSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FLT to value 0"]
impl crate::Resettable for FltSpec {
    const RESET_VALUE: u8 = 0;
}
