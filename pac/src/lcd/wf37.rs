#[doc = "Register `WF37` reader"]
pub type R = crate::R<Wf37Spec>;
#[doc = "Register `WF37` writer"]
pub type W = crate::W<Wf37Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd37 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd37> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD37` reader - no description available"]
pub type Bpalcd37R = crate::BitReader<Bpalcd37>;
impl Bpalcd37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd37 {
        match self.bits {
            false => Bpalcd37::B0,
            true => Bpalcd37::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd37::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd37::B1
    }
}
#[doc = "Field `BPALCD37` writer - no description available"]
pub type Bpalcd37W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd37>;
impl<'a, REG> Bpalcd37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd37::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd37::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd37 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd37> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD37` reader - no description available"]
pub type Bpblcd37R = crate::BitReader<Bpblcd37>;
impl Bpblcd37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd37 {
        match self.bits {
            false => Bpblcd37::B0,
            true => Bpblcd37::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd37::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd37::B1
    }
}
#[doc = "Field `BPBLCD37` writer - no description available"]
pub type Bpblcd37W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd37>;
impl<'a, REG> Bpblcd37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd37::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd37::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd37 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd37> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD37` reader - no description available"]
pub type Bpclcd37R = crate::BitReader<Bpclcd37>;
impl Bpclcd37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd37 {
        match self.bits {
            false => Bpclcd37::B0,
            true => Bpclcd37::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd37::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd37::B1
    }
}
#[doc = "Field `BPCLCD37` writer - no description available"]
pub type Bpclcd37W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd37>;
impl<'a, REG> Bpclcd37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd37::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd37::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd37 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd37> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD37` reader - no description available"]
pub type Bpdlcd37R = crate::BitReader<Bpdlcd37>;
impl Bpdlcd37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd37 {
        match self.bits {
            false => Bpdlcd37::B0,
            true => Bpdlcd37::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd37::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd37::B1
    }
}
#[doc = "Field `BPDLCD37` writer - no description available"]
pub type Bpdlcd37W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd37>;
impl<'a, REG> Bpdlcd37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd37::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd37::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd37 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd37> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD37` reader - no description available"]
pub type Bpelcd37R = crate::BitReader<Bpelcd37>;
impl Bpelcd37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd37 {
        match self.bits {
            false => Bpelcd37::B0,
            true => Bpelcd37::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd37::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd37::B1
    }
}
#[doc = "Field `BPELCD37` writer - no description available"]
pub type Bpelcd37W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd37>;
impl<'a, REG> Bpelcd37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd37::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd37::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd37 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd37> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD37` reader - no description available"]
pub type Bpflcd37R = crate::BitReader<Bpflcd37>;
impl Bpflcd37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd37 {
        match self.bits {
            false => Bpflcd37::B0,
            true => Bpflcd37::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd37::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd37::B1
    }
}
#[doc = "Field `BPFLCD37` writer - no description available"]
pub type Bpflcd37W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd37>;
impl<'a, REG> Bpflcd37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd37::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd37::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd37 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd37> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD37` reader - no description available"]
pub type Bpglcd37R = crate::BitReader<Bpglcd37>;
impl Bpglcd37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd37 {
        match self.bits {
            false => Bpglcd37::B0,
            true => Bpglcd37::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd37::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd37::B1
    }
}
#[doc = "Field `BPGLCD37` writer - no description available"]
pub type Bpglcd37W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd37>;
impl<'a, REG> Bpglcd37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd37::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd37::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd37 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd37> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd37) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD37` reader - no description available"]
pub type Bphlcd37R = crate::BitReader<Bphlcd37>;
impl Bphlcd37R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd37 {
        match self.bits {
            false => Bphlcd37::B0,
            true => Bphlcd37::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd37::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd37::B1
    }
}
#[doc = "Field `BPHLCD37` writer - no description available"]
pub type Bphlcd37W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd37>;
impl<'a, REG> Bphlcd37W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd37::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd37::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd37(&self) -> Bpalcd37R {
        Bpalcd37R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd37(&self) -> Bpblcd37R {
        Bpblcd37R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd37(&self) -> Bpclcd37R {
        Bpclcd37R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd37(&self) -> Bpdlcd37R {
        Bpdlcd37R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd37(&self) -> Bpelcd37R {
        Bpelcd37R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd37(&self) -> Bpflcd37R {
        Bpflcd37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd37(&self) -> Bpglcd37R {
        Bpglcd37R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd37(&self) -> Bphlcd37R {
        Bphlcd37R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd37(&mut self) -> Bpalcd37W<Wf37Spec> {
        Bpalcd37W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd37(&mut self) -> Bpblcd37W<Wf37Spec> {
        Bpblcd37W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd37(&mut self) -> Bpclcd37W<Wf37Spec> {
        Bpclcd37W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd37(&mut self) -> Bpdlcd37W<Wf37Spec> {
        Bpdlcd37W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd37(&mut self) -> Bpelcd37W<Wf37Spec> {
        Bpelcd37W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd37(&mut self) -> Bpflcd37W<Wf37Spec> {
        Bpflcd37W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd37(&mut self) -> Bpglcd37W<Wf37Spec> {
        Bpglcd37W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd37(&mut self) -> Bphlcd37W<Wf37Spec> {
        Bphlcd37W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 37.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf37::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf37::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf37Spec;
impl crate::RegisterSpec for Wf37Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf37::R`](R) reader structure"]
impl crate::Readable for Wf37Spec {}
#[doc = "`write(|w| ..)` method takes [`wf37::W`](W) writer structure"]
impl crate::Writable for Wf37Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF37 to value 0"]
impl crate::Resettable for Wf37Spec {
    const RESET_VALUE: u8 = 0;
}
