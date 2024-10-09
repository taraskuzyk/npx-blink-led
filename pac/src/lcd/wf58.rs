#[doc = "Register `WF58` reader"]
pub type R = crate::R<Wf58Spec>;
#[doc = "Register `WF58` writer"]
pub type W = crate::W<Wf58Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd58 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd58> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD58` reader - no description available"]
pub type Bpalcd58R = crate::BitReader<Bpalcd58>;
impl Bpalcd58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd58 {
        match self.bits {
            false => Bpalcd58::B0,
            true => Bpalcd58::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd58::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd58::B1
    }
}
#[doc = "Field `BPALCD58` writer - no description available"]
pub type Bpalcd58W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd58>;
impl<'a, REG> Bpalcd58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd58::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd58::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd58 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd58> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD58` reader - no description available"]
pub type Bpblcd58R = crate::BitReader<Bpblcd58>;
impl Bpblcd58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd58 {
        match self.bits {
            false => Bpblcd58::B0,
            true => Bpblcd58::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd58::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd58::B1
    }
}
#[doc = "Field `BPBLCD58` writer - no description available"]
pub type Bpblcd58W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd58>;
impl<'a, REG> Bpblcd58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd58::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd58::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd58 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd58> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD58` reader - no description available"]
pub type Bpclcd58R = crate::BitReader<Bpclcd58>;
impl Bpclcd58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd58 {
        match self.bits {
            false => Bpclcd58::B0,
            true => Bpclcd58::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd58::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd58::B1
    }
}
#[doc = "Field `BPCLCD58` writer - no description available"]
pub type Bpclcd58W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd58>;
impl<'a, REG> Bpclcd58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd58::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd58::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd58 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd58> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD58` reader - no description available"]
pub type Bpdlcd58R = crate::BitReader<Bpdlcd58>;
impl Bpdlcd58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd58 {
        match self.bits {
            false => Bpdlcd58::B0,
            true => Bpdlcd58::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd58::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd58::B1
    }
}
#[doc = "Field `BPDLCD58` writer - no description available"]
pub type Bpdlcd58W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd58>;
impl<'a, REG> Bpdlcd58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd58::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd58::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd58 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd58> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD58` reader - no description available"]
pub type Bpelcd58R = crate::BitReader<Bpelcd58>;
impl Bpelcd58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd58 {
        match self.bits {
            false => Bpelcd58::B0,
            true => Bpelcd58::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd58::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd58::B1
    }
}
#[doc = "Field `BPELCD58` writer - no description available"]
pub type Bpelcd58W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd58>;
impl<'a, REG> Bpelcd58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd58::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd58::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd58 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd58> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD58` reader - no description available"]
pub type Bpflcd58R = crate::BitReader<Bpflcd58>;
impl Bpflcd58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd58 {
        match self.bits {
            false => Bpflcd58::B0,
            true => Bpflcd58::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd58::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd58::B1
    }
}
#[doc = "Field `BPFLCD58` writer - no description available"]
pub type Bpflcd58W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd58>;
impl<'a, REG> Bpflcd58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd58::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd58::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd58 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd58> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD58` reader - no description available"]
pub type Bpglcd58R = crate::BitReader<Bpglcd58>;
impl Bpglcd58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd58 {
        match self.bits {
            false => Bpglcd58::B0,
            true => Bpglcd58::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd58::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd58::B1
    }
}
#[doc = "Field `BPGLCD58` writer - no description available"]
pub type Bpglcd58W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd58>;
impl<'a, REG> Bpglcd58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd58::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd58::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd58 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd58> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd58) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD58` reader - no description available"]
pub type Bphlcd58R = crate::BitReader<Bphlcd58>;
impl Bphlcd58R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd58 {
        match self.bits {
            false => Bphlcd58::B0,
            true => Bphlcd58::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd58::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd58::B1
    }
}
#[doc = "Field `BPHLCD58` writer - no description available"]
pub type Bphlcd58W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd58>;
impl<'a, REG> Bphlcd58W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd58::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd58::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd58(&self) -> Bpalcd58R {
        Bpalcd58R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd58(&self) -> Bpblcd58R {
        Bpblcd58R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd58(&self) -> Bpclcd58R {
        Bpclcd58R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd58(&self) -> Bpdlcd58R {
        Bpdlcd58R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd58(&self) -> Bpelcd58R {
        Bpelcd58R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd58(&self) -> Bpflcd58R {
        Bpflcd58R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd58(&self) -> Bpglcd58R {
        Bpglcd58R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd58(&self) -> Bphlcd58R {
        Bphlcd58R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd58(&mut self) -> Bpalcd58W<Wf58Spec> {
        Bpalcd58W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd58(&mut self) -> Bpblcd58W<Wf58Spec> {
        Bpblcd58W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd58(&mut self) -> Bpclcd58W<Wf58Spec> {
        Bpclcd58W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd58(&mut self) -> Bpdlcd58W<Wf58Spec> {
        Bpdlcd58W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd58(&mut self) -> Bpelcd58W<Wf58Spec> {
        Bpelcd58W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd58(&mut self) -> Bpflcd58W<Wf58Spec> {
        Bpflcd58W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd58(&mut self) -> Bpglcd58W<Wf58Spec> {
        Bpglcd58W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd58(&mut self) -> Bphlcd58W<Wf58Spec> {
        Bphlcd58W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 58.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf58::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf58::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf58Spec;
impl crate::RegisterSpec for Wf58Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf58::R`](R) reader structure"]
impl crate::Readable for Wf58Spec {}
#[doc = "`write(|w| ..)` method takes [`wf58::W`](W) writer structure"]
impl crate::Writable for Wf58Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF58 to value 0"]
impl crate::Resettable for Wf58Spec {
    const RESET_VALUE: u8 = 0;
}
