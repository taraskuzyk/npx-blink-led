#[doc = "Register `WF46` reader"]
pub type R = crate::R<Wf46Spec>;
#[doc = "Register `WF46` writer"]
pub type W = crate::W<Wf46Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd46 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd46> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD46` reader - no description available"]
pub type Bpalcd46R = crate::BitReader<Bpalcd46>;
impl Bpalcd46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd46 {
        match self.bits {
            false => Bpalcd46::B0,
            true => Bpalcd46::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd46::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd46::B1
    }
}
#[doc = "Field `BPALCD46` writer - no description available"]
pub type Bpalcd46W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd46>;
impl<'a, REG> Bpalcd46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd46::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd46::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd46 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd46> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD46` reader - no description available"]
pub type Bpblcd46R = crate::BitReader<Bpblcd46>;
impl Bpblcd46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd46 {
        match self.bits {
            false => Bpblcd46::B0,
            true => Bpblcd46::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd46::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd46::B1
    }
}
#[doc = "Field `BPBLCD46` writer - no description available"]
pub type Bpblcd46W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd46>;
impl<'a, REG> Bpblcd46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd46::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd46::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd46 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd46> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD46` reader - no description available"]
pub type Bpclcd46R = crate::BitReader<Bpclcd46>;
impl Bpclcd46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd46 {
        match self.bits {
            false => Bpclcd46::B0,
            true => Bpclcd46::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd46::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd46::B1
    }
}
#[doc = "Field `BPCLCD46` writer - no description available"]
pub type Bpclcd46W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd46>;
impl<'a, REG> Bpclcd46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd46::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd46::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd46 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd46> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD46` reader - no description available"]
pub type Bpdlcd46R = crate::BitReader<Bpdlcd46>;
impl Bpdlcd46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd46 {
        match self.bits {
            false => Bpdlcd46::B0,
            true => Bpdlcd46::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd46::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd46::B1
    }
}
#[doc = "Field `BPDLCD46` writer - no description available"]
pub type Bpdlcd46W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd46>;
impl<'a, REG> Bpdlcd46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd46::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd46::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd46 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd46> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD46` reader - no description available"]
pub type Bpelcd46R = crate::BitReader<Bpelcd46>;
impl Bpelcd46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd46 {
        match self.bits {
            false => Bpelcd46::B0,
            true => Bpelcd46::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd46::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd46::B1
    }
}
#[doc = "Field `BPELCD46` writer - no description available"]
pub type Bpelcd46W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd46>;
impl<'a, REG> Bpelcd46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd46::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd46::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd46 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd46> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD46` reader - no description available"]
pub type Bpflcd46R = crate::BitReader<Bpflcd46>;
impl Bpflcd46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd46 {
        match self.bits {
            false => Bpflcd46::B0,
            true => Bpflcd46::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd46::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd46::B1
    }
}
#[doc = "Field `BPFLCD46` writer - no description available"]
pub type Bpflcd46W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd46>;
impl<'a, REG> Bpflcd46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd46::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd46::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd46 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd46> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD46` reader - no description available"]
pub type Bpglcd46R = crate::BitReader<Bpglcd46>;
impl Bpglcd46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd46 {
        match self.bits {
            false => Bpglcd46::B0,
            true => Bpglcd46::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd46::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd46::B1
    }
}
#[doc = "Field `BPGLCD46` writer - no description available"]
pub type Bpglcd46W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd46>;
impl<'a, REG> Bpglcd46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd46::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd46::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd46 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd46> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd46) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD46` reader - no description available"]
pub type Bphlcd46R = crate::BitReader<Bphlcd46>;
impl Bphlcd46R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd46 {
        match self.bits {
            false => Bphlcd46::B0,
            true => Bphlcd46::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd46::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd46::B1
    }
}
#[doc = "Field `BPHLCD46` writer - no description available"]
pub type Bphlcd46W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd46>;
impl<'a, REG> Bphlcd46W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd46::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd46::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd46(&self) -> Bpalcd46R {
        Bpalcd46R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd46(&self) -> Bpblcd46R {
        Bpblcd46R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd46(&self) -> Bpclcd46R {
        Bpclcd46R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd46(&self) -> Bpdlcd46R {
        Bpdlcd46R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd46(&self) -> Bpelcd46R {
        Bpelcd46R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd46(&self) -> Bpflcd46R {
        Bpflcd46R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd46(&self) -> Bpglcd46R {
        Bpglcd46R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd46(&self) -> Bphlcd46R {
        Bphlcd46R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd46(&mut self) -> Bpalcd46W<Wf46Spec> {
        Bpalcd46W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd46(&mut self) -> Bpblcd46W<Wf46Spec> {
        Bpblcd46W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd46(&mut self) -> Bpclcd46W<Wf46Spec> {
        Bpclcd46W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd46(&mut self) -> Bpdlcd46W<Wf46Spec> {
        Bpdlcd46W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd46(&mut self) -> Bpelcd46W<Wf46Spec> {
        Bpelcd46W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd46(&mut self) -> Bpflcd46W<Wf46Spec> {
        Bpflcd46W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd46(&mut self) -> Bpglcd46W<Wf46Spec> {
        Bpglcd46W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd46(&mut self) -> Bphlcd46W<Wf46Spec> {
        Bphlcd46W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 46.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf46Spec;
impl crate::RegisterSpec for Wf46Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf46::R`](R) reader structure"]
impl crate::Readable for Wf46Spec {}
#[doc = "`write(|w| ..)` method takes [`wf46::W`](W) writer structure"]
impl crate::Writable for Wf46Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF46 to value 0"]
impl crate::Resettable for Wf46Spec {
    const RESET_VALUE: u8 = 0;
}
