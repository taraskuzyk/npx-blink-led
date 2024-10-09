#[doc = "Register `AR` reader"]
pub type R = crate::R<ArSpec>;
#[doc = "Register `AR` writer"]
pub type W = crate::W<ArSpec>;
#[doc = "Field `BRATE` reader - Blink-rate configuration"]
pub type BrateR = crate::FieldReader;
#[doc = "Field `BRATE` writer - Blink-rate configuration"]
pub type BrateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Blink mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bmode {
    #[doc = "0: Display blank during the blink period."]
    B0 = 0,
    #[doc = "1: Display alternate display during blink period (Ignored if duty is 5 or greater)."]
    B1 = 1,
}
impl From<Bmode> for bool {
    #[inline(always)]
    fn from(variant: Bmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BMODE` reader - Blink mode"]
pub type BmodeR = crate::BitReader<Bmode>;
impl BmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bmode {
        match self.bits {
            false => Bmode::B0,
            true => Bmode::B1,
        }
    }
    #[doc = "Display blank during the blink period."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bmode::B0
    }
    #[doc = "Display alternate display during blink period (Ignored if duty is 5 or greater)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bmode::B1
    }
}
#[doc = "Field `BMODE` writer - Blink mode"]
pub type BmodeW<'a, REG> = crate::BitWriter<'a, REG, Bmode>;
impl<'a, REG> BmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Display blank during the blink period."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bmode::B0)
    }
    #[doc = "Display alternate display during blink period (Ignored if duty is 5 or greater)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bmode::B1)
    }
}
#[doc = "Blank display mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blank {
    #[doc = "0: Normal or alternate display mode."]
    B0 = 0,
    #[doc = "1: Blank display mode."]
    B1 = 1,
}
impl From<Blank> for bool {
    #[inline(always)]
    fn from(variant: Blank) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLANK` reader - Blank display mode"]
pub type BlankR = crate::BitReader<Blank>;
impl BlankR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blank {
        match self.bits {
            false => Blank::B0,
            true => Blank::B1,
        }
    }
    #[doc = "Normal or alternate display mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Blank::B0
    }
    #[doc = "Blank display mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Blank::B1
    }
}
#[doc = "Field `BLANK` writer - Blank display mode"]
pub type BlankW<'a, REG> = crate::BitWriter<'a, REG, Blank>;
impl<'a, REG> BlankW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal or alternate display mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Blank::B0)
    }
    #[doc = "Blank display mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Blank::B1)
    }
}
#[doc = "Alternate display mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alt {
    #[doc = "0: Normal display mode."]
    B0 = 0,
    #[doc = "1: Alternate display mode."]
    B1 = 1,
}
impl From<Alt> for bool {
    #[inline(always)]
    fn from(variant: Alt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALT` reader - Alternate display mode"]
pub type AltR = crate::BitReader<Alt>;
impl AltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alt {
        match self.bits {
            false => Alt::B0,
            true => Alt::B1,
        }
    }
    #[doc = "Normal display mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Alt::B0
    }
    #[doc = "Alternate display mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Alt::B1
    }
}
#[doc = "Field `ALT` writer - Alternate display mode"]
pub type AltW<'a, REG> = crate::BitWriter<'a, REG, Alt>;
impl<'a, REG> AltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal display mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Alt::B0)
    }
    #[doc = "Alternate display mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Alt::B1)
    }
}
#[doc = "Blink command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blink {
    #[doc = "0: Disables blinking."]
    B0 = 0,
    #[doc = "1: Starts blinking at blinking frequency specified by LCD blink rate calculation."]
    B1 = 1,
}
impl From<Blink> for bool {
    #[inline(always)]
    fn from(variant: Blink) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLINK` reader - Blink command"]
pub type BlinkR = crate::BitReader<Blink>;
impl BlinkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blink {
        match self.bits {
            false => Blink::B0,
            true => Blink::B1,
        }
    }
    #[doc = "Disables blinking."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Blink::B0
    }
    #[doc = "Starts blinking at blinking frequency specified by LCD blink rate calculation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Blink::B1
    }
}
#[doc = "Field `BLINK` writer - Blink command"]
pub type BlinkW<'a, REG> = crate::BitWriter<'a, REG, Blink>;
impl<'a, REG> BlinkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables blinking."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Blink::B0)
    }
    #[doc = "Starts blinking at blinking frequency specified by LCD blink rate calculation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Blink::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Blink-rate configuration"]
    #[inline(always)]
    pub fn brate(&self) -> BrateR {
        BrateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Blink mode"]
    #[inline(always)]
    pub fn bmode(&self) -> BmodeR {
        BmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Blank display mode"]
    #[inline(always)]
    pub fn blank(&self) -> BlankR {
        BlankR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alternate display mode"]
    #[inline(always)]
    pub fn alt(&self) -> AltR {
        AltR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Blink command"]
    #[inline(always)]
    pub fn blink(&self) -> BlinkR {
        BlinkR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Blink-rate configuration"]
    #[inline(always)]
    #[must_use]
    pub fn brate(&mut self) -> BrateW<ArSpec> {
        BrateW::new(self, 0)
    }
    #[doc = "Bit 3 - Blink mode"]
    #[inline(always)]
    #[must_use]
    pub fn bmode(&mut self) -> BmodeW<ArSpec> {
        BmodeW::new(self, 3)
    }
    #[doc = "Bit 5 - Blank display mode"]
    #[inline(always)]
    #[must_use]
    pub fn blank(&mut self) -> BlankW<ArSpec> {
        BlankW::new(self, 5)
    }
    #[doc = "Bit 6 - Alternate display mode"]
    #[inline(always)]
    #[must_use]
    pub fn alt(&mut self) -> AltW<ArSpec> {
        AltW::new(self, 6)
    }
    #[doc = "Bit 7 - Blink command"]
    #[inline(always)]
    #[must_use]
    pub fn blink(&mut self) -> BlinkW<ArSpec> {
        BlinkW::new(self, 7)
    }
}
#[doc = "LCD Auxiliary Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArSpec;
impl crate::RegisterSpec for ArSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ar::R`](R) reader structure"]
impl crate::Readable for ArSpec {}
#[doc = "`write(|w| ..)` method takes [`ar::W`](W) writer structure"]
impl crate::Writable for ArSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AR to value 0"]
impl crate::Resettable for ArSpec {
    const RESET_VALUE: u32 = 0;
}
