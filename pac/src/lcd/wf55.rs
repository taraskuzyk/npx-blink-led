#[doc = "Register `WF55` reader"]
pub type R = crate::R<Wf55Spec>;
#[doc = "Register `WF55` writer"]
pub type W = crate::W<Wf55Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd55 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd55> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD55` reader - no description available"]
pub type Bpalcd55R = crate::BitReader<Bpalcd55>;
impl Bpalcd55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd55 {
        match self.bits {
            false => Bpalcd55::B0,
            true => Bpalcd55::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd55::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd55::B1
    }
}
#[doc = "Field `BPALCD55` writer - no description available"]
pub type Bpalcd55W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd55>;
impl<'a, REG> Bpalcd55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd55::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd55::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd55 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd55> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD55` reader - no description available"]
pub type Bpblcd55R = crate::BitReader<Bpblcd55>;
impl Bpblcd55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd55 {
        match self.bits {
            false => Bpblcd55::B0,
            true => Bpblcd55::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd55::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd55::B1
    }
}
#[doc = "Field `BPBLCD55` writer - no description available"]
pub type Bpblcd55W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd55>;
impl<'a, REG> Bpblcd55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd55::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd55::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd55 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd55> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD55` reader - no description available"]
pub type Bpclcd55R = crate::BitReader<Bpclcd55>;
impl Bpclcd55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd55 {
        match self.bits {
            false => Bpclcd55::B0,
            true => Bpclcd55::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd55::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd55::B1
    }
}
#[doc = "Field `BPCLCD55` writer - no description available"]
pub type Bpclcd55W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd55>;
impl<'a, REG> Bpclcd55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd55::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd55::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd55 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd55> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD55` reader - no description available"]
pub type Bpdlcd55R = crate::BitReader<Bpdlcd55>;
impl Bpdlcd55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd55 {
        match self.bits {
            false => Bpdlcd55::B0,
            true => Bpdlcd55::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd55::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd55::B1
    }
}
#[doc = "Field `BPDLCD55` writer - no description available"]
pub type Bpdlcd55W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd55>;
impl<'a, REG> Bpdlcd55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd55::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd55::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd55 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd55> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD55` reader - no description available"]
pub type Bpelcd55R = crate::BitReader<Bpelcd55>;
impl Bpelcd55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd55 {
        match self.bits {
            false => Bpelcd55::B0,
            true => Bpelcd55::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd55::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd55::B1
    }
}
#[doc = "Field `BPELCD55` writer - no description available"]
pub type Bpelcd55W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd55>;
impl<'a, REG> Bpelcd55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd55::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd55::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd55 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd55> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD55` reader - no description available"]
pub type Bpflcd55R = crate::BitReader<Bpflcd55>;
impl Bpflcd55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd55 {
        match self.bits {
            false => Bpflcd55::B0,
            true => Bpflcd55::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd55::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd55::B1
    }
}
#[doc = "Field `BPFLCD55` writer - no description available"]
pub type Bpflcd55W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd55>;
impl<'a, REG> Bpflcd55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd55::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd55::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd55 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd55> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD55` reader - no description available"]
pub type Bpglcd55R = crate::BitReader<Bpglcd55>;
impl Bpglcd55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd55 {
        match self.bits {
            false => Bpglcd55::B0,
            true => Bpglcd55::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd55::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd55::B1
    }
}
#[doc = "Field `BPGLCD55` writer - no description available"]
pub type Bpglcd55W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd55>;
impl<'a, REG> Bpglcd55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd55::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd55::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd55 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd55> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd55) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD55` reader - no description available"]
pub type Bphlcd55R = crate::BitReader<Bphlcd55>;
impl Bphlcd55R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd55 {
        match self.bits {
            false => Bphlcd55::B0,
            true => Bphlcd55::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd55::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd55::B1
    }
}
#[doc = "Field `BPHLCD55` writer - no description available"]
pub type Bphlcd55W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd55>;
impl<'a, REG> Bphlcd55W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd55::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd55::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd55(&self) -> Bpalcd55R {
        Bpalcd55R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd55(&self) -> Bpblcd55R {
        Bpblcd55R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd55(&self) -> Bpclcd55R {
        Bpclcd55R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd55(&self) -> Bpdlcd55R {
        Bpdlcd55R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd55(&self) -> Bpelcd55R {
        Bpelcd55R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd55(&self) -> Bpflcd55R {
        Bpflcd55R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd55(&self) -> Bpglcd55R {
        Bpglcd55R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd55(&self) -> Bphlcd55R {
        Bphlcd55R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd55(&mut self) -> Bpalcd55W<Wf55Spec> {
        Bpalcd55W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd55(&mut self) -> Bpblcd55W<Wf55Spec> {
        Bpblcd55W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd55(&mut self) -> Bpclcd55W<Wf55Spec> {
        Bpclcd55W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd55(&mut self) -> Bpdlcd55W<Wf55Spec> {
        Bpdlcd55W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd55(&mut self) -> Bpelcd55W<Wf55Spec> {
        Bpelcd55W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd55(&mut self) -> Bpflcd55W<Wf55Spec> {
        Bpflcd55W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd55(&mut self) -> Bpglcd55W<Wf55Spec> {
        Bpglcd55W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd55(&mut self) -> Bphlcd55W<Wf55Spec> {
        Bphlcd55W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 55.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf55::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf55::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf55Spec;
impl crate::RegisterSpec for Wf55Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf55::R`](R) reader structure"]
impl crate::Readable for Wf55Spec {}
#[doc = "`write(|w| ..)` method takes [`wf55::W`](W) writer structure"]
impl crate::Writable for Wf55Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF55 to value 0"]
impl crate::Resettable for Wf55Spec {
    const RESET_VALUE: u8 = 0;
}
