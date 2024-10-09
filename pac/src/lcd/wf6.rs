#[doc = "Register `WF6` reader"]
pub type R = crate::R<Wf6Spec>;
#[doc = "Register `WF6` writer"]
pub type W = crate::W<Wf6Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd6 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd6> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD6` reader - no description available"]
pub type Bpalcd6R = crate::BitReader<Bpalcd6>;
impl Bpalcd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd6 {
        match self.bits {
            false => Bpalcd6::B0,
            true => Bpalcd6::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd6::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd6::B1
    }
}
#[doc = "Field `BPALCD6` writer - no description available"]
pub type Bpalcd6W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd6>;
impl<'a, REG> Bpalcd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd6::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd6::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd6 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd6> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD6` reader - no description available"]
pub type Bpblcd6R = crate::BitReader<Bpblcd6>;
impl Bpblcd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd6 {
        match self.bits {
            false => Bpblcd6::B0,
            true => Bpblcd6::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd6::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd6::B1
    }
}
#[doc = "Field `BPBLCD6` writer - no description available"]
pub type Bpblcd6W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd6>;
impl<'a, REG> Bpblcd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd6::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd6::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd6 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd6> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD6` reader - no description available"]
pub type Bpclcd6R = crate::BitReader<Bpclcd6>;
impl Bpclcd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd6 {
        match self.bits {
            false => Bpclcd6::B0,
            true => Bpclcd6::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd6::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd6::B1
    }
}
#[doc = "Field `BPCLCD6` writer - no description available"]
pub type Bpclcd6W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd6>;
impl<'a, REG> Bpclcd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd6::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd6::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd6 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd6> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD6` reader - no description available"]
pub type Bpdlcd6R = crate::BitReader<Bpdlcd6>;
impl Bpdlcd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd6 {
        match self.bits {
            false => Bpdlcd6::B0,
            true => Bpdlcd6::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd6::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd6::B1
    }
}
#[doc = "Field `BPDLCD6` writer - no description available"]
pub type Bpdlcd6W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd6>;
impl<'a, REG> Bpdlcd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd6::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd6::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd6 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd6> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD6` reader - no description available"]
pub type Bpelcd6R = crate::BitReader<Bpelcd6>;
impl Bpelcd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd6 {
        match self.bits {
            false => Bpelcd6::B0,
            true => Bpelcd6::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd6::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd6::B1
    }
}
#[doc = "Field `BPELCD6` writer - no description available"]
pub type Bpelcd6W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd6>;
impl<'a, REG> Bpelcd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd6::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd6::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd6 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd6> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD6` reader - no description available"]
pub type Bpflcd6R = crate::BitReader<Bpflcd6>;
impl Bpflcd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd6 {
        match self.bits {
            false => Bpflcd6::B0,
            true => Bpflcd6::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd6::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd6::B1
    }
}
#[doc = "Field `BPFLCD6` writer - no description available"]
pub type Bpflcd6W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd6>;
impl<'a, REG> Bpflcd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd6::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd6::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd6 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd6> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD6` reader - no description available"]
pub type Bpglcd6R = crate::BitReader<Bpglcd6>;
impl Bpglcd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd6 {
        match self.bits {
            false => Bpglcd6::B0,
            true => Bpglcd6::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd6::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd6::B1
    }
}
#[doc = "Field `BPGLCD6` writer - no description available"]
pub type Bpglcd6W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd6>;
impl<'a, REG> Bpglcd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd6::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd6::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd6 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd6> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD6` reader - no description available"]
pub type Bphlcd6R = crate::BitReader<Bphlcd6>;
impl Bphlcd6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd6 {
        match self.bits {
            false => Bphlcd6::B0,
            true => Bphlcd6::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd6::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd6::B1
    }
}
#[doc = "Field `BPHLCD6` writer - no description available"]
pub type Bphlcd6W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd6>;
impl<'a, REG> Bphlcd6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd6::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd6::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd6(&self) -> Bpalcd6R {
        Bpalcd6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd6(&self) -> Bpblcd6R {
        Bpblcd6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd6(&self) -> Bpclcd6R {
        Bpclcd6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd6(&self) -> Bpdlcd6R {
        Bpdlcd6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd6(&self) -> Bpelcd6R {
        Bpelcd6R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd6(&self) -> Bpflcd6R {
        Bpflcd6R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd6(&self) -> Bpglcd6R {
        Bpglcd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd6(&self) -> Bphlcd6R {
        Bphlcd6R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd6(&mut self) -> Bpalcd6W<Wf6Spec> {
        Bpalcd6W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd6(&mut self) -> Bpblcd6W<Wf6Spec> {
        Bpblcd6W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd6(&mut self) -> Bpclcd6W<Wf6Spec> {
        Bpclcd6W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd6(&mut self) -> Bpdlcd6W<Wf6Spec> {
        Bpdlcd6W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd6(&mut self) -> Bpelcd6W<Wf6Spec> {
        Bpelcd6W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd6(&mut self) -> Bpflcd6W<Wf6Spec> {
        Bpflcd6W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd6(&mut self) -> Bpglcd6W<Wf6Spec> {
        Bpglcd6W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd6(&mut self) -> Bphlcd6W<Wf6Spec> {
        Bphlcd6W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf6Spec;
impl crate::RegisterSpec for Wf6Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf6::R`](R) reader structure"]
impl crate::Readable for Wf6Spec {}
#[doc = "`write(|w| ..)` method takes [`wf6::W`](W) writer structure"]
impl crate::Writable for Wf6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF6 to value 0"]
impl crate::Resettable for Wf6Spec {
    const RESET_VALUE: u8 = 0;
}
