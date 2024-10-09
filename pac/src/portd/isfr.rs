#[doc = "Register `ISFR` reader"]
pub type R = crate::R<IsfrSpec>;
#[doc = "Register `ISFR` writer"]
pub type W = crate::W<IsfrSpec>;
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf0 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf0> for bool {
    #[inline(always)]
    fn from(variant: Isf0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF0` reader - Interrupt Status Flag"]
pub type Isf0R = crate::BitReader<Isf0>;
impl Isf0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf0 {
        match self.bits {
            false => Isf0::B00000000000000000000000000000000,
            true => Isf0::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf0::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf0::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF0` writer - Interrupt Status Flag"]
pub type Isf0W<'a, REG> = crate::BitWriter<'a, REG, Isf0>;
impl<'a, REG> Isf0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf0::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf0::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf1 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf1> for bool {
    #[inline(always)]
    fn from(variant: Isf1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF1` reader - Interrupt Status Flag"]
pub type Isf1R = crate::BitReader<Isf1>;
impl Isf1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf1 {
        match self.bits {
            false => Isf1::B00000000000000000000000000000000,
            true => Isf1::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf1::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf1::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF1` writer - Interrupt Status Flag"]
pub type Isf1W<'a, REG> = crate::BitWriter<'a, REG, Isf1>;
impl<'a, REG> Isf1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf1::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf1::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf2 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf2> for bool {
    #[inline(always)]
    fn from(variant: Isf2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF2` reader - Interrupt Status Flag"]
pub type Isf2R = crate::BitReader<Isf2>;
impl Isf2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf2 {
        match self.bits {
            false => Isf2::B00000000000000000000000000000000,
            true => Isf2::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf2::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf2::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF2` writer - Interrupt Status Flag"]
pub type Isf2W<'a, REG> = crate::BitWriter<'a, REG, Isf2>;
impl<'a, REG> Isf2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf2::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf2::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf3 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf3> for bool {
    #[inline(always)]
    fn from(variant: Isf3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF3` reader - Interrupt Status Flag"]
pub type Isf3R = crate::BitReader<Isf3>;
impl Isf3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf3 {
        match self.bits {
            false => Isf3::B00000000000000000000000000000000,
            true => Isf3::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf3::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf3::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF3` writer - Interrupt Status Flag"]
pub type Isf3W<'a, REG> = crate::BitWriter<'a, REG, Isf3>;
impl<'a, REG> Isf3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf3::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf3::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf4 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf4> for bool {
    #[inline(always)]
    fn from(variant: Isf4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF4` reader - Interrupt Status Flag"]
pub type Isf4R = crate::BitReader<Isf4>;
impl Isf4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf4 {
        match self.bits {
            false => Isf4::B00000000000000000000000000000000,
            true => Isf4::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf4::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf4::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF4` writer - Interrupt Status Flag"]
pub type Isf4W<'a, REG> = crate::BitWriter<'a, REG, Isf4>;
impl<'a, REG> Isf4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf4::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf4::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf5 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf5> for bool {
    #[inline(always)]
    fn from(variant: Isf5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF5` reader - Interrupt Status Flag"]
pub type Isf5R = crate::BitReader<Isf5>;
impl Isf5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf5 {
        match self.bits {
            false => Isf5::B00000000000000000000000000000000,
            true => Isf5::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf5::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf5::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF5` writer - Interrupt Status Flag"]
pub type Isf5W<'a, REG> = crate::BitWriter<'a, REG, Isf5>;
impl<'a, REG> Isf5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf5::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf5::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf6 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf6> for bool {
    #[inline(always)]
    fn from(variant: Isf6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF6` reader - Interrupt Status Flag"]
pub type Isf6R = crate::BitReader<Isf6>;
impl Isf6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf6 {
        match self.bits {
            false => Isf6::B00000000000000000000000000000000,
            true => Isf6::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf6::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf6::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF6` writer - Interrupt Status Flag"]
pub type Isf6W<'a, REG> = crate::BitWriter<'a, REG, Isf6>;
impl<'a, REG> Isf6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf6::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf6::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf7 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf7> for bool {
    #[inline(always)]
    fn from(variant: Isf7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF7` reader - Interrupt Status Flag"]
pub type Isf7R = crate::BitReader<Isf7>;
impl Isf7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf7 {
        match self.bits {
            false => Isf7::B00000000000000000000000000000000,
            true => Isf7::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf7::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf7::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF7` writer - Interrupt Status Flag"]
pub type Isf7W<'a, REG> = crate::BitWriter<'a, REG, Isf7>;
impl<'a, REG> Isf7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf7::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf7::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf8 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf8> for bool {
    #[inline(always)]
    fn from(variant: Isf8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF8` reader - Interrupt Status Flag"]
pub type Isf8R = crate::BitReader<Isf8>;
impl Isf8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf8 {
        match self.bits {
            false => Isf8::B00000000000000000000000000000000,
            true => Isf8::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf8::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf8::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF8` writer - Interrupt Status Flag"]
pub type Isf8W<'a, REG> = crate::BitWriter<'a, REG, Isf8>;
impl<'a, REG> Isf8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf8::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf8::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf9 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf9> for bool {
    #[inline(always)]
    fn from(variant: Isf9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF9` reader - Interrupt Status Flag"]
pub type Isf9R = crate::BitReader<Isf9>;
impl Isf9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf9 {
        match self.bits {
            false => Isf9::B00000000000000000000000000000000,
            true => Isf9::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf9::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf9::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF9` writer - Interrupt Status Flag"]
pub type Isf9W<'a, REG> = crate::BitWriter<'a, REG, Isf9>;
impl<'a, REG> Isf9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf9::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf9::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf10 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf10> for bool {
    #[inline(always)]
    fn from(variant: Isf10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF10` reader - Interrupt Status Flag"]
pub type Isf10R = crate::BitReader<Isf10>;
impl Isf10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf10 {
        match self.bits {
            false => Isf10::B00000000000000000000000000000000,
            true => Isf10::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf10::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf10::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF10` writer - Interrupt Status Flag"]
pub type Isf10W<'a, REG> = crate::BitWriter<'a, REG, Isf10>;
impl<'a, REG> Isf10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf10::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf10::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf11 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf11> for bool {
    #[inline(always)]
    fn from(variant: Isf11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF11` reader - Interrupt Status Flag"]
pub type Isf11R = crate::BitReader<Isf11>;
impl Isf11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf11 {
        match self.bits {
            false => Isf11::B00000000000000000000000000000000,
            true => Isf11::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf11::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf11::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF11` writer - Interrupt Status Flag"]
pub type Isf11W<'a, REG> = crate::BitWriter<'a, REG, Isf11>;
impl<'a, REG> Isf11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf11::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf11::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf12 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf12> for bool {
    #[inline(always)]
    fn from(variant: Isf12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF12` reader - Interrupt Status Flag"]
pub type Isf12R = crate::BitReader<Isf12>;
impl Isf12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf12 {
        match self.bits {
            false => Isf12::B00000000000000000000000000000000,
            true => Isf12::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf12::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf12::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF12` writer - Interrupt Status Flag"]
pub type Isf12W<'a, REG> = crate::BitWriter<'a, REG, Isf12>;
impl<'a, REG> Isf12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf12::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf12::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf13 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf13> for bool {
    #[inline(always)]
    fn from(variant: Isf13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF13` reader - Interrupt Status Flag"]
pub type Isf13R = crate::BitReader<Isf13>;
impl Isf13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf13 {
        match self.bits {
            false => Isf13::B00000000000000000000000000000000,
            true => Isf13::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf13::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf13::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF13` writer - Interrupt Status Flag"]
pub type Isf13W<'a, REG> = crate::BitWriter<'a, REG, Isf13>;
impl<'a, REG> Isf13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf13::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf13::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf14 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf14> for bool {
    #[inline(always)]
    fn from(variant: Isf14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF14` reader - Interrupt Status Flag"]
pub type Isf14R = crate::BitReader<Isf14>;
impl Isf14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf14 {
        match self.bits {
            false => Isf14::B00000000000000000000000000000000,
            true => Isf14::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf14::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf14::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF14` writer - Interrupt Status Flag"]
pub type Isf14W<'a, REG> = crate::BitWriter<'a, REG, Isf14>;
impl<'a, REG> Isf14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf14::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf14::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf15 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf15> for bool {
    #[inline(always)]
    fn from(variant: Isf15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF15` reader - Interrupt Status Flag"]
pub type Isf15R = crate::BitReader<Isf15>;
impl Isf15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf15 {
        match self.bits {
            false => Isf15::B00000000000000000000000000000000,
            true => Isf15::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf15::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf15::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF15` writer - Interrupt Status Flag"]
pub type Isf15W<'a, REG> = crate::BitWriter<'a, REG, Isf15>;
impl<'a, REG> Isf15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf15::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf15::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf16 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf16> for bool {
    #[inline(always)]
    fn from(variant: Isf16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF16` reader - Interrupt Status Flag"]
pub type Isf16R = crate::BitReader<Isf16>;
impl Isf16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf16 {
        match self.bits {
            false => Isf16::B00000000000000000000000000000000,
            true => Isf16::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf16::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf16::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF16` writer - Interrupt Status Flag"]
pub type Isf16W<'a, REG> = crate::BitWriter<'a, REG, Isf16>;
impl<'a, REG> Isf16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf16::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf16::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf17 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf17> for bool {
    #[inline(always)]
    fn from(variant: Isf17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF17` reader - Interrupt Status Flag"]
pub type Isf17R = crate::BitReader<Isf17>;
impl Isf17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf17 {
        match self.bits {
            false => Isf17::B00000000000000000000000000000000,
            true => Isf17::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf17::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf17::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF17` writer - Interrupt Status Flag"]
pub type Isf17W<'a, REG> = crate::BitWriter<'a, REG, Isf17>;
impl<'a, REG> Isf17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf17::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf17::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf18 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf18> for bool {
    #[inline(always)]
    fn from(variant: Isf18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF18` reader - Interrupt Status Flag"]
pub type Isf18R = crate::BitReader<Isf18>;
impl Isf18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf18 {
        match self.bits {
            false => Isf18::B00000000000000000000000000000000,
            true => Isf18::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf18::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf18::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF18` writer - Interrupt Status Flag"]
pub type Isf18W<'a, REG> = crate::BitWriter<'a, REG, Isf18>;
impl<'a, REG> Isf18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf18::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf18::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf19 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf19> for bool {
    #[inline(always)]
    fn from(variant: Isf19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF19` reader - Interrupt Status Flag"]
pub type Isf19R = crate::BitReader<Isf19>;
impl Isf19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf19 {
        match self.bits {
            false => Isf19::B00000000000000000000000000000000,
            true => Isf19::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf19::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf19::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF19` writer - Interrupt Status Flag"]
pub type Isf19W<'a, REG> = crate::BitWriter<'a, REG, Isf19>;
impl<'a, REG> Isf19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf19::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf19::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf20 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf20> for bool {
    #[inline(always)]
    fn from(variant: Isf20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF20` reader - Interrupt Status Flag"]
pub type Isf20R = crate::BitReader<Isf20>;
impl Isf20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf20 {
        match self.bits {
            false => Isf20::B00000000000000000000000000000000,
            true => Isf20::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf20::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf20::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF20` writer - Interrupt Status Flag"]
pub type Isf20W<'a, REG> = crate::BitWriter<'a, REG, Isf20>;
impl<'a, REG> Isf20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf20::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf20::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf21 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf21> for bool {
    #[inline(always)]
    fn from(variant: Isf21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF21` reader - Interrupt Status Flag"]
pub type Isf21R = crate::BitReader<Isf21>;
impl Isf21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf21 {
        match self.bits {
            false => Isf21::B00000000000000000000000000000000,
            true => Isf21::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf21::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf21::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF21` writer - Interrupt Status Flag"]
pub type Isf21W<'a, REG> = crate::BitWriter<'a, REG, Isf21>;
impl<'a, REG> Isf21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf21::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf21::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf22 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf22> for bool {
    #[inline(always)]
    fn from(variant: Isf22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF22` reader - Interrupt Status Flag"]
pub type Isf22R = crate::BitReader<Isf22>;
impl Isf22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf22 {
        match self.bits {
            false => Isf22::B00000000000000000000000000000000,
            true => Isf22::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf22::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf22::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF22` writer - Interrupt Status Flag"]
pub type Isf22W<'a, REG> = crate::BitWriter<'a, REG, Isf22>;
impl<'a, REG> Isf22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf22::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf22::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf23 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf23> for bool {
    #[inline(always)]
    fn from(variant: Isf23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF23` reader - Interrupt Status Flag"]
pub type Isf23R = crate::BitReader<Isf23>;
impl Isf23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf23 {
        match self.bits {
            false => Isf23::B00000000000000000000000000000000,
            true => Isf23::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf23::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf23::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF23` writer - Interrupt Status Flag"]
pub type Isf23W<'a, REG> = crate::BitWriter<'a, REG, Isf23>;
impl<'a, REG> Isf23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf23::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf23::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf24 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf24> for bool {
    #[inline(always)]
    fn from(variant: Isf24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF24` reader - Interrupt Status Flag"]
pub type Isf24R = crate::BitReader<Isf24>;
impl Isf24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf24 {
        match self.bits {
            false => Isf24::B00000000000000000000000000000000,
            true => Isf24::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf24::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf24::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF24` writer - Interrupt Status Flag"]
pub type Isf24W<'a, REG> = crate::BitWriter<'a, REG, Isf24>;
impl<'a, REG> Isf24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf24::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf24::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf25 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf25> for bool {
    #[inline(always)]
    fn from(variant: Isf25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF25` reader - Interrupt Status Flag"]
pub type Isf25R = crate::BitReader<Isf25>;
impl Isf25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf25 {
        match self.bits {
            false => Isf25::B00000000000000000000000000000000,
            true => Isf25::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf25::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf25::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF25` writer - Interrupt Status Flag"]
pub type Isf25W<'a, REG> = crate::BitWriter<'a, REG, Isf25>;
impl<'a, REG> Isf25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf25::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf25::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf26 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf26> for bool {
    #[inline(always)]
    fn from(variant: Isf26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF26` reader - Interrupt Status Flag"]
pub type Isf26R = crate::BitReader<Isf26>;
impl Isf26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf26 {
        match self.bits {
            false => Isf26::B00000000000000000000000000000000,
            true => Isf26::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf26::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf26::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF26` writer - Interrupt Status Flag"]
pub type Isf26W<'a, REG> = crate::BitWriter<'a, REG, Isf26>;
impl<'a, REG> Isf26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf26::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf26::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf27 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf27> for bool {
    #[inline(always)]
    fn from(variant: Isf27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF27` reader - Interrupt Status Flag"]
pub type Isf27R = crate::BitReader<Isf27>;
impl Isf27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf27 {
        match self.bits {
            false => Isf27::B00000000000000000000000000000000,
            true => Isf27::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf27::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf27::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF27` writer - Interrupt Status Flag"]
pub type Isf27W<'a, REG> = crate::BitWriter<'a, REG, Isf27>;
impl<'a, REG> Isf27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf27::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf27::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf28 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf28> for bool {
    #[inline(always)]
    fn from(variant: Isf28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF28` reader - Interrupt Status Flag"]
pub type Isf28R = crate::BitReader<Isf28>;
impl Isf28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf28 {
        match self.bits {
            false => Isf28::B00000000000000000000000000000000,
            true => Isf28::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf28::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf28::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF28` writer - Interrupt Status Flag"]
pub type Isf28W<'a, REG> = crate::BitWriter<'a, REG, Isf28>;
impl<'a, REG> Isf28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf28::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf28::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf29 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf29> for bool {
    #[inline(always)]
    fn from(variant: Isf29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF29` reader - Interrupt Status Flag"]
pub type Isf29R = crate::BitReader<Isf29>;
impl Isf29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf29 {
        match self.bits {
            false => Isf29::B00000000000000000000000000000000,
            true => Isf29::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf29::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf29::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF29` writer - Interrupt Status Flag"]
pub type Isf29W<'a, REG> = crate::BitWriter<'a, REG, Isf29>;
impl<'a, REG> Isf29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf29::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf29::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf30 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf30> for bool {
    #[inline(always)]
    fn from(variant: Isf30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF30` reader - Interrupt Status Flag"]
pub type Isf30R = crate::BitReader<Isf30>;
impl Isf30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf30 {
        match self.bits {
            false => Isf30::B00000000000000000000000000000000,
            true => Isf30::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf30::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf30::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF30` writer - Interrupt Status Flag"]
pub type Isf30W<'a, REG> = crate::BitWriter<'a, REG, Isf30>;
impl<'a, REG> Isf30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf30::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf30::B00000000000000000000000000000001)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf31 {
    #[doc = "0: Configured interrupt is not detected."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B00000000000000000000000000000001 = 1,
}
impl From<Isf31> for bool {
    #[inline(always)]
    fn from(variant: Isf31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF31` reader - Interrupt Status Flag"]
pub type Isf31R = crate::BitReader<Isf31>;
impl Isf31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf31 {
        match self.bits {
            false => Isf31::B00000000000000000000000000000000,
            true => Isf31::B00000000000000000000000000000001,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Isf31::B00000000000000000000000000000000
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Isf31::B00000000000000000000000000000001
    }
}
#[doc = "Field `ISF31` writer - Interrupt Status Flag"]
pub type Isf31W<'a, REG> = crate::BitWriter<'a, REG, Isf31>;
impl<'a, REG> Isf31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Isf31::B00000000000000000000000000000000)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Isf31::B00000000000000000000000000000001)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf0(&self) -> Isf0R {
        Isf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf1(&self) -> Isf1R {
        Isf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf2(&self) -> Isf2R {
        Isf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf3(&self) -> Isf3R {
        Isf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf4(&self) -> Isf4R {
        Isf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf5(&self) -> Isf5R {
        Isf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf6(&self) -> Isf6R {
        Isf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf7(&self) -> Isf7R {
        Isf7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf8(&self) -> Isf8R {
        Isf8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf9(&self) -> Isf9R {
        Isf9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf10(&self) -> Isf10R {
        Isf10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf11(&self) -> Isf11R {
        Isf11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf12(&self) -> Isf12R {
        Isf12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf13(&self) -> Isf13R {
        Isf13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf14(&self) -> Isf14R {
        Isf14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf15(&self) -> Isf15R {
        Isf15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf16(&self) -> Isf16R {
        Isf16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf17(&self) -> Isf17R {
        Isf17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf18(&self) -> Isf18R {
        Isf18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf19(&self) -> Isf19R {
        Isf19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf20(&self) -> Isf20R {
        Isf20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf21(&self) -> Isf21R {
        Isf21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf22(&self) -> Isf22R {
        Isf22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf23(&self) -> Isf23R {
        Isf23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf24(&self) -> Isf24R {
        Isf24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf25(&self) -> Isf25R {
        Isf25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf26(&self) -> Isf26R {
        Isf26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf27(&self) -> Isf27R {
        Isf27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf28(&self) -> Isf28R {
        Isf28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf29(&self) -> Isf29R {
        Isf29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf30(&self) -> Isf30R {
        Isf30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf31(&self) -> Isf31R {
        Isf31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf0(&mut self) -> Isf0W<IsfrSpec> {
        Isf0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf1(&mut self) -> Isf1W<IsfrSpec> {
        Isf1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf2(&mut self) -> Isf2W<IsfrSpec> {
        Isf2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf3(&mut self) -> Isf3W<IsfrSpec> {
        Isf3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf4(&mut self) -> Isf4W<IsfrSpec> {
        Isf4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf5(&mut self) -> Isf5W<IsfrSpec> {
        Isf5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf6(&mut self) -> Isf6W<IsfrSpec> {
        Isf6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf7(&mut self) -> Isf7W<IsfrSpec> {
        Isf7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf8(&mut self) -> Isf8W<IsfrSpec> {
        Isf8W::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf9(&mut self) -> Isf9W<IsfrSpec> {
        Isf9W::new(self, 9)
    }
    #[doc = "Bit 10 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf10(&mut self) -> Isf10W<IsfrSpec> {
        Isf10W::new(self, 10)
    }
    #[doc = "Bit 11 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf11(&mut self) -> Isf11W<IsfrSpec> {
        Isf11W::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf12(&mut self) -> Isf12W<IsfrSpec> {
        Isf12W::new(self, 12)
    }
    #[doc = "Bit 13 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf13(&mut self) -> Isf13W<IsfrSpec> {
        Isf13W::new(self, 13)
    }
    #[doc = "Bit 14 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf14(&mut self) -> Isf14W<IsfrSpec> {
        Isf14W::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf15(&mut self) -> Isf15W<IsfrSpec> {
        Isf15W::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf16(&mut self) -> Isf16W<IsfrSpec> {
        Isf16W::new(self, 16)
    }
    #[doc = "Bit 17 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf17(&mut self) -> Isf17W<IsfrSpec> {
        Isf17W::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf18(&mut self) -> Isf18W<IsfrSpec> {
        Isf18W::new(self, 18)
    }
    #[doc = "Bit 19 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf19(&mut self) -> Isf19W<IsfrSpec> {
        Isf19W::new(self, 19)
    }
    #[doc = "Bit 20 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf20(&mut self) -> Isf20W<IsfrSpec> {
        Isf20W::new(self, 20)
    }
    #[doc = "Bit 21 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf21(&mut self) -> Isf21W<IsfrSpec> {
        Isf21W::new(self, 21)
    }
    #[doc = "Bit 22 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf22(&mut self) -> Isf22W<IsfrSpec> {
        Isf22W::new(self, 22)
    }
    #[doc = "Bit 23 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf23(&mut self) -> Isf23W<IsfrSpec> {
        Isf23W::new(self, 23)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf24(&mut self) -> Isf24W<IsfrSpec> {
        Isf24W::new(self, 24)
    }
    #[doc = "Bit 25 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf25(&mut self) -> Isf25W<IsfrSpec> {
        Isf25W::new(self, 25)
    }
    #[doc = "Bit 26 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf26(&mut self) -> Isf26W<IsfrSpec> {
        Isf26W::new(self, 26)
    }
    #[doc = "Bit 27 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf27(&mut self) -> Isf27W<IsfrSpec> {
        Isf27W::new(self, 27)
    }
    #[doc = "Bit 28 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf28(&mut self) -> Isf28W<IsfrSpec> {
        Isf28W::new(self, 28)
    }
    #[doc = "Bit 29 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf29(&mut self) -> Isf29W<IsfrSpec> {
        Isf29W::new(self, 29)
    }
    #[doc = "Bit 30 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf30(&mut self) -> Isf30W<IsfrSpec> {
        Isf30W::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf31(&mut self) -> Isf31W<IsfrSpec> {
        Isf31W::new(self, 31)
    }
}
#[doc = "Interrupt Status Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsfrSpec;
impl crate::RegisterSpec for IsfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isfr::R`](R) reader structure"]
impl crate::Readable for IsfrSpec {}
#[doc = "`write(|w| ..)` method takes [`isfr::W`](W) writer structure"]
impl crate::Writable for IsfrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISFR to value 0"]
impl crate::Resettable for IsfrSpec {
    const RESET_VALUE: u32 = 0;
}
