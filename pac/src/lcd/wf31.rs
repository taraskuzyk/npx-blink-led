#[doc = "Register `WF31` reader"]
pub type R = crate::R<Wf31Spec>;
#[doc = "Register `WF31` writer"]
pub type W = crate::W<Wf31Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd31 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd31> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD31` reader - no description available"]
pub type Bpalcd31R = crate::BitReader<Bpalcd31>;
impl Bpalcd31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd31 {
        match self.bits {
            false => Bpalcd31::B0,
            true => Bpalcd31::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd31::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd31::B1
    }
}
#[doc = "Field `BPALCD31` writer - no description available"]
pub type Bpalcd31W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd31>;
impl<'a, REG> Bpalcd31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd31::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd31::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd31 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd31> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD31` reader - no description available"]
pub type Bpblcd31R = crate::BitReader<Bpblcd31>;
impl Bpblcd31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd31 {
        match self.bits {
            false => Bpblcd31::B0,
            true => Bpblcd31::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd31::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd31::B1
    }
}
#[doc = "Field `BPBLCD31` writer - no description available"]
pub type Bpblcd31W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd31>;
impl<'a, REG> Bpblcd31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd31::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd31::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd31 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd31> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD31` reader - no description available"]
pub type Bpclcd31R = crate::BitReader<Bpclcd31>;
impl Bpclcd31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd31 {
        match self.bits {
            false => Bpclcd31::B0,
            true => Bpclcd31::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd31::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd31::B1
    }
}
#[doc = "Field `BPCLCD31` writer - no description available"]
pub type Bpclcd31W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd31>;
impl<'a, REG> Bpclcd31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd31::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd31::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd31 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd31> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD31` reader - no description available"]
pub type Bpdlcd31R = crate::BitReader<Bpdlcd31>;
impl Bpdlcd31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd31 {
        match self.bits {
            false => Bpdlcd31::B0,
            true => Bpdlcd31::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd31::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd31::B1
    }
}
#[doc = "Field `BPDLCD31` writer - no description available"]
pub type Bpdlcd31W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd31>;
impl<'a, REG> Bpdlcd31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd31::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd31::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd31 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd31> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD31` reader - no description available"]
pub type Bpelcd31R = crate::BitReader<Bpelcd31>;
impl Bpelcd31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd31 {
        match self.bits {
            false => Bpelcd31::B0,
            true => Bpelcd31::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd31::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd31::B1
    }
}
#[doc = "Field `BPELCD31` writer - no description available"]
pub type Bpelcd31W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd31>;
impl<'a, REG> Bpelcd31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd31::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd31::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd31 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd31> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD31` reader - no description available"]
pub type Bpflcd31R = crate::BitReader<Bpflcd31>;
impl Bpflcd31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd31 {
        match self.bits {
            false => Bpflcd31::B0,
            true => Bpflcd31::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd31::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd31::B1
    }
}
#[doc = "Field `BPFLCD31` writer - no description available"]
pub type Bpflcd31W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd31>;
impl<'a, REG> Bpflcd31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd31::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd31::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd31 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd31> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD31` reader - no description available"]
pub type Bpglcd31R = crate::BitReader<Bpglcd31>;
impl Bpglcd31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd31 {
        match self.bits {
            false => Bpglcd31::B0,
            true => Bpglcd31::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd31::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd31::B1
    }
}
#[doc = "Field `BPGLCD31` writer - no description available"]
pub type Bpglcd31W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd31>;
impl<'a, REG> Bpglcd31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd31::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd31::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd31 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd31> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD31` reader - no description available"]
pub type Bphlcd31R = crate::BitReader<Bphlcd31>;
impl Bphlcd31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd31 {
        match self.bits {
            false => Bphlcd31::B0,
            true => Bphlcd31::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd31::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd31::B1
    }
}
#[doc = "Field `BPHLCD31` writer - no description available"]
pub type Bphlcd31W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd31>;
impl<'a, REG> Bphlcd31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd31::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd31::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd31(&self) -> Bpalcd31R {
        Bpalcd31R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd31(&self) -> Bpblcd31R {
        Bpblcd31R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd31(&self) -> Bpclcd31R {
        Bpclcd31R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd31(&self) -> Bpdlcd31R {
        Bpdlcd31R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd31(&self) -> Bpelcd31R {
        Bpelcd31R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd31(&self) -> Bpflcd31R {
        Bpflcd31R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd31(&self) -> Bpglcd31R {
        Bpglcd31R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd31(&self) -> Bphlcd31R {
        Bphlcd31R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd31(&mut self) -> Bpalcd31W<Wf31Spec> {
        Bpalcd31W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd31(&mut self) -> Bpblcd31W<Wf31Spec> {
        Bpblcd31W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd31(&mut self) -> Bpclcd31W<Wf31Spec> {
        Bpclcd31W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd31(&mut self) -> Bpdlcd31W<Wf31Spec> {
        Bpdlcd31W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd31(&mut self) -> Bpelcd31W<Wf31Spec> {
        Bpelcd31W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd31(&mut self) -> Bpflcd31W<Wf31Spec> {
        Bpflcd31W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd31(&mut self) -> Bpglcd31W<Wf31Spec> {
        Bpglcd31W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd31(&mut self) -> Bphlcd31W<Wf31Spec> {
        Bphlcd31W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 31.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf31Spec;
impl crate::RegisterSpec for Wf31Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf31::R`](R) reader structure"]
impl crate::Readable for Wf31Spec {}
#[doc = "`write(|w| ..)` method takes [`wf31::W`](W) writer structure"]
impl crate::Writable for Wf31Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF31 to value 0"]
impl crate::Resettable for Wf31Spec {
    const RESET_VALUE: u8 = 0;
}
