#[doc = "Register `POL` reader"]
pub type R = crate::R<PolSpec>;
#[doc = "Register `POL` writer"]
pub type W = crate::W<PolSpec>;
#[doc = "Channel 0 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol0 {
    #[doc = "0: The channel polarity is active high."]
    B0 = 0,
    #[doc = "1: The channel polarity is active low."]
    B1 = 1,
}
impl From<Pol0> for bool {
    #[inline(always)]
    fn from(variant: Pol0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL0` reader - Channel 0 Polarity"]
pub type Pol0R = crate::BitReader<Pol0>;
impl Pol0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol0 {
        match self.bits {
            false => Pol0::B0,
            true => Pol0::B1,
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pol0::B0
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pol0::B1
    }
}
#[doc = "Field `POL0` writer - Channel 0 Polarity"]
pub type Pol0W<'a, REG> = crate::BitWriter<'a, REG, Pol0>;
impl<'a, REG> Pol0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pol0::B0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pol0::B1)
    }
}
#[doc = "Channel 1 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol1 {
    #[doc = "0: The channel polarity is active high."]
    B0 = 0,
    #[doc = "1: The channel polarity is active low."]
    B1 = 1,
}
impl From<Pol1> for bool {
    #[inline(always)]
    fn from(variant: Pol1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL1` reader - Channel 1 Polarity"]
pub type Pol1R = crate::BitReader<Pol1>;
impl Pol1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol1 {
        match self.bits {
            false => Pol1::B0,
            true => Pol1::B1,
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pol1::B0
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pol1::B1
    }
}
#[doc = "Field `POL1` writer - Channel 1 Polarity"]
pub type Pol1W<'a, REG> = crate::BitWriter<'a, REG, Pol1>;
impl<'a, REG> Pol1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pol1::B0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pol1::B1)
    }
}
#[doc = "Channel 2 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol2 {
    #[doc = "0: The channel polarity is active high."]
    B0 = 0,
    #[doc = "1: The channel polarity is active low."]
    B1 = 1,
}
impl From<Pol2> for bool {
    #[inline(always)]
    fn from(variant: Pol2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL2` reader - Channel 2 Polarity"]
pub type Pol2R = crate::BitReader<Pol2>;
impl Pol2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol2 {
        match self.bits {
            false => Pol2::B0,
            true => Pol2::B1,
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pol2::B0
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pol2::B1
    }
}
#[doc = "Field `POL2` writer - Channel 2 Polarity"]
pub type Pol2W<'a, REG> = crate::BitWriter<'a, REG, Pol2>;
impl<'a, REG> Pol2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pol2::B0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pol2::B1)
    }
}
#[doc = "Channel 3 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol3 {
    #[doc = "0: The channel polarity is active high."]
    B0 = 0,
    #[doc = "1: The channel polarity is active low."]
    B1 = 1,
}
impl From<Pol3> for bool {
    #[inline(always)]
    fn from(variant: Pol3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL3` reader - Channel 3 Polarity"]
pub type Pol3R = crate::BitReader<Pol3>;
impl Pol3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol3 {
        match self.bits {
            false => Pol3::B0,
            true => Pol3::B1,
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pol3::B0
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pol3::B1
    }
}
#[doc = "Field `POL3` writer - Channel 3 Polarity"]
pub type Pol3W<'a, REG> = crate::BitWriter<'a, REG, Pol3>;
impl<'a, REG> Pol3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pol3::B0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pol3::B1)
    }
}
#[doc = "Channel Polarity 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol4 {
    #[doc = "0: The channel polarity is active high"]
    B0 = 0,
    #[doc = "1: The channel polarity is active low."]
    B1 = 1,
}
impl From<Pol4> for bool {
    #[inline(always)]
    fn from(variant: Pol4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL4` reader - Channel Polarity 4"]
pub type Pol4R = crate::BitReader<Pol4>;
impl Pol4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol4 {
        match self.bits {
            false => Pol4::B0,
            true => Pol4::B1,
        }
    }
    #[doc = "The channel polarity is active high"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pol4::B0
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pol4::B1
    }
}
#[doc = "Field `POL4` writer - Channel Polarity 4"]
pub type Pol4W<'a, REG> = crate::BitWriter<'a, REG, Pol4>;
impl<'a, REG> Pol4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel polarity is active high"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pol4::B0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pol4::B1)
    }
}
#[doc = "Channel 5 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pol5 {
    #[doc = "0: The channel polarity is active high."]
    B0 = 0,
    #[doc = "1: The channel polarity is active low."]
    B1 = 1,
}
impl From<Pol5> for bool {
    #[inline(always)]
    fn from(variant: Pol5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL5` reader - Channel 5 Polarity"]
pub type Pol5R = crate::BitReader<Pol5>;
impl Pol5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pol5 {
        match self.bits {
            false => Pol5::B0,
            true => Pol5::B1,
        }
    }
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pol5::B0
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pol5::B1
    }
}
#[doc = "Field `POL5` writer - Channel 5 Polarity"]
pub type Pol5W<'a, REG> = crate::BitWriter<'a, REG, Pol5>;
impl<'a, REG> Pol5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pol5::B0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pol5::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&self) -> Pol0R {
        Pol0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> Pol1R {
        Pol1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> Pol2R {
        Pol2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&self) -> Pol3R {
        Pol3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Polarity 4"]
    #[inline(always)]
    pub fn pol4(&self) -> Pol4R {
        Pol4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Polarity"]
    #[inline(always)]
    pub fn pol5(&self) -> Pol5R {
        Pol5R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol0(&mut self) -> Pol0W<PolSpec> {
        Pol0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol1(&mut self) -> Pol1W<PolSpec> {
        Pol1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol2(&mut self) -> Pol2W<PolSpec> {
        Pol2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol3(&mut self) -> Pol3W<PolSpec> {
        Pol3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel Polarity 4"]
    #[inline(always)]
    #[must_use]
    pub fn pol4(&mut self) -> Pol4W<PolSpec> {
        Pol4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol5(&mut self) -> Pol5W<PolSpec> {
        Pol5W::new(self, 5)
    }
}
#[doc = "Channel Polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PolSpec;
impl crate::RegisterSpec for PolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pol::R`](R) reader structure"]
impl crate::Readable for PolSpec {}
#[doc = "`write(|w| ..)` method takes [`pol::W`](W) writer structure"]
impl crate::Writable for PolSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POL to value 0"]
impl crate::Resettable for PolSpec {
    const RESET_VALUE: u32 = 0;
}
