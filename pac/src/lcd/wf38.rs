#[doc = "Register `WF38` reader"]
pub type R = crate::R<Wf38Spec>;
#[doc = "Register `WF38` writer"]
pub type W = crate::W<Wf38Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd38 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd38> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD38` reader - no description available"]
pub type Bpalcd38R = crate::BitReader<Bpalcd38>;
impl Bpalcd38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd38 {
        match self.bits {
            false => Bpalcd38::B0,
            true => Bpalcd38::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd38::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd38::B1
    }
}
#[doc = "Field `BPALCD38` writer - no description available"]
pub type Bpalcd38W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd38>;
impl<'a, REG> Bpalcd38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd38::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd38::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd38 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd38> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD38` reader - no description available"]
pub type Bpblcd38R = crate::BitReader<Bpblcd38>;
impl Bpblcd38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd38 {
        match self.bits {
            false => Bpblcd38::B0,
            true => Bpblcd38::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd38::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd38::B1
    }
}
#[doc = "Field `BPBLCD38` writer - no description available"]
pub type Bpblcd38W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd38>;
impl<'a, REG> Bpblcd38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd38::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd38::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd38 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd38> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD38` reader - no description available"]
pub type Bpclcd38R = crate::BitReader<Bpclcd38>;
impl Bpclcd38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd38 {
        match self.bits {
            false => Bpclcd38::B0,
            true => Bpclcd38::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd38::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd38::B1
    }
}
#[doc = "Field `BPCLCD38` writer - no description available"]
pub type Bpclcd38W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd38>;
impl<'a, REG> Bpclcd38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd38::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd38::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd38 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd38> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD38` reader - no description available"]
pub type Bpdlcd38R = crate::BitReader<Bpdlcd38>;
impl Bpdlcd38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd38 {
        match self.bits {
            false => Bpdlcd38::B0,
            true => Bpdlcd38::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd38::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd38::B1
    }
}
#[doc = "Field `BPDLCD38` writer - no description available"]
pub type Bpdlcd38W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd38>;
impl<'a, REG> Bpdlcd38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd38::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd38::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd38 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd38> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD38` reader - no description available"]
pub type Bpelcd38R = crate::BitReader<Bpelcd38>;
impl Bpelcd38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd38 {
        match self.bits {
            false => Bpelcd38::B0,
            true => Bpelcd38::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd38::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd38::B1
    }
}
#[doc = "Field `BPELCD38` writer - no description available"]
pub type Bpelcd38W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd38>;
impl<'a, REG> Bpelcd38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd38::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd38::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd38 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd38> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD38` reader - no description available"]
pub type Bpflcd38R = crate::BitReader<Bpflcd38>;
impl Bpflcd38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd38 {
        match self.bits {
            false => Bpflcd38::B0,
            true => Bpflcd38::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd38::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd38::B1
    }
}
#[doc = "Field `BPFLCD38` writer - no description available"]
pub type Bpflcd38W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd38>;
impl<'a, REG> Bpflcd38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd38::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd38::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd38 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd38> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD38` reader - no description available"]
pub type Bpglcd38R = crate::BitReader<Bpglcd38>;
impl Bpglcd38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd38 {
        match self.bits {
            false => Bpglcd38::B0,
            true => Bpglcd38::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd38::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd38::B1
    }
}
#[doc = "Field `BPGLCD38` writer - no description available"]
pub type Bpglcd38W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd38>;
impl<'a, REG> Bpglcd38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd38::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd38::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd38 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd38> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd38) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD38` reader - no description available"]
pub type Bphlcd38R = crate::BitReader<Bphlcd38>;
impl Bphlcd38R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd38 {
        match self.bits {
            false => Bphlcd38::B0,
            true => Bphlcd38::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd38::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd38::B1
    }
}
#[doc = "Field `BPHLCD38` writer - no description available"]
pub type Bphlcd38W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd38>;
impl<'a, REG> Bphlcd38W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd38::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd38::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd38(&self) -> Bpalcd38R {
        Bpalcd38R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd38(&self) -> Bpblcd38R {
        Bpblcd38R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd38(&self) -> Bpclcd38R {
        Bpclcd38R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd38(&self) -> Bpdlcd38R {
        Bpdlcd38R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd38(&self) -> Bpelcd38R {
        Bpelcd38R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd38(&self) -> Bpflcd38R {
        Bpflcd38R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd38(&self) -> Bpglcd38R {
        Bpglcd38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd38(&self) -> Bphlcd38R {
        Bphlcd38R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd38(&mut self) -> Bpalcd38W<Wf38Spec> {
        Bpalcd38W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd38(&mut self) -> Bpblcd38W<Wf38Spec> {
        Bpblcd38W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd38(&mut self) -> Bpclcd38W<Wf38Spec> {
        Bpclcd38W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd38(&mut self) -> Bpdlcd38W<Wf38Spec> {
        Bpdlcd38W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd38(&mut self) -> Bpelcd38W<Wf38Spec> {
        Bpelcd38W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd38(&mut self) -> Bpflcd38W<Wf38Spec> {
        Bpflcd38W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd38(&mut self) -> Bpglcd38W<Wf38Spec> {
        Bpglcd38W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd38(&mut self) -> Bphlcd38W<Wf38Spec> {
        Bphlcd38W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 38.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf38Spec;
impl crate::RegisterSpec for Wf38Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf38::R`](R) reader structure"]
impl crate::Readable for Wf38Spec {}
#[doc = "`write(|w| ..)` method takes [`wf38::W`](W) writer structure"]
impl crate::Writable for Wf38Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF38 to value 0"]
impl crate::Resettable for Wf38Spec {
    const RESET_VALUE: u8 = 0;
}
