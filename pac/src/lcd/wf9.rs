#[doc = "Register `WF9` reader"]
pub type R = crate::R<Wf9Spec>;
#[doc = "Register `WF9` writer"]
pub type W = crate::W<Wf9Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd9 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd9> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD9` reader - no description available"]
pub type Bpalcd9R = crate::BitReader<Bpalcd9>;
impl Bpalcd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd9 {
        match self.bits {
            false => Bpalcd9::B0,
            true => Bpalcd9::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd9::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd9::B1
    }
}
#[doc = "Field `BPALCD9` writer - no description available"]
pub type Bpalcd9W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd9>;
impl<'a, REG> Bpalcd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd9::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd9::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd9 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd9> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD9` reader - no description available"]
pub type Bpblcd9R = crate::BitReader<Bpblcd9>;
impl Bpblcd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd9 {
        match self.bits {
            false => Bpblcd9::B0,
            true => Bpblcd9::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd9::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd9::B1
    }
}
#[doc = "Field `BPBLCD9` writer - no description available"]
pub type Bpblcd9W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd9>;
impl<'a, REG> Bpblcd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd9::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd9::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd9 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd9> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD9` reader - no description available"]
pub type Bpclcd9R = crate::BitReader<Bpclcd9>;
impl Bpclcd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd9 {
        match self.bits {
            false => Bpclcd9::B0,
            true => Bpclcd9::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd9::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd9::B1
    }
}
#[doc = "Field `BPCLCD9` writer - no description available"]
pub type Bpclcd9W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd9>;
impl<'a, REG> Bpclcd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd9::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd9::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd9 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd9> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD9` reader - no description available"]
pub type Bpdlcd9R = crate::BitReader<Bpdlcd9>;
impl Bpdlcd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd9 {
        match self.bits {
            false => Bpdlcd9::B0,
            true => Bpdlcd9::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd9::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd9::B1
    }
}
#[doc = "Field `BPDLCD9` writer - no description available"]
pub type Bpdlcd9W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd9>;
impl<'a, REG> Bpdlcd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd9::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd9::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd9 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd9> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD9` reader - no description available"]
pub type Bpelcd9R = crate::BitReader<Bpelcd9>;
impl Bpelcd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd9 {
        match self.bits {
            false => Bpelcd9::B0,
            true => Bpelcd9::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd9::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd9::B1
    }
}
#[doc = "Field `BPELCD9` writer - no description available"]
pub type Bpelcd9W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd9>;
impl<'a, REG> Bpelcd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd9::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd9::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd9 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd9> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD9` reader - no description available"]
pub type Bpflcd9R = crate::BitReader<Bpflcd9>;
impl Bpflcd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd9 {
        match self.bits {
            false => Bpflcd9::B0,
            true => Bpflcd9::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd9::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd9::B1
    }
}
#[doc = "Field `BPFLCD9` writer - no description available"]
pub type Bpflcd9W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd9>;
impl<'a, REG> Bpflcd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd9::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd9::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd9 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd9> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD9` reader - no description available"]
pub type Bpglcd9R = crate::BitReader<Bpglcd9>;
impl Bpglcd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd9 {
        match self.bits {
            false => Bpglcd9::B0,
            true => Bpglcd9::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd9::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd9::B1
    }
}
#[doc = "Field `BPGLCD9` writer - no description available"]
pub type Bpglcd9W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd9>;
impl<'a, REG> Bpglcd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd9::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd9::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd9 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd9> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD9` reader - no description available"]
pub type Bphlcd9R = crate::BitReader<Bphlcd9>;
impl Bphlcd9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd9 {
        match self.bits {
            false => Bphlcd9::B0,
            true => Bphlcd9::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd9::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd9::B1
    }
}
#[doc = "Field `BPHLCD9` writer - no description available"]
pub type Bphlcd9W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd9>;
impl<'a, REG> Bphlcd9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd9::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd9::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd9(&self) -> Bpalcd9R {
        Bpalcd9R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd9(&self) -> Bpblcd9R {
        Bpblcd9R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd9(&self) -> Bpclcd9R {
        Bpclcd9R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd9(&self) -> Bpdlcd9R {
        Bpdlcd9R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd9(&self) -> Bpelcd9R {
        Bpelcd9R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd9(&self) -> Bpflcd9R {
        Bpflcd9R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd9(&self) -> Bpglcd9R {
        Bpglcd9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd9(&self) -> Bphlcd9R {
        Bphlcd9R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd9(&mut self) -> Bpalcd9W<Wf9Spec> {
        Bpalcd9W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd9(&mut self) -> Bpblcd9W<Wf9Spec> {
        Bpblcd9W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd9(&mut self) -> Bpclcd9W<Wf9Spec> {
        Bpclcd9W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd9(&mut self) -> Bpdlcd9W<Wf9Spec> {
        Bpdlcd9W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd9(&mut self) -> Bpelcd9W<Wf9Spec> {
        Bpelcd9W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd9(&mut self) -> Bpflcd9W<Wf9Spec> {
        Bpflcd9W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd9(&mut self) -> Bpglcd9W<Wf9Spec> {
        Bpglcd9W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd9(&mut self) -> Bphlcd9W<Wf9Spec> {
        Bphlcd9W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 9.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf9Spec;
impl crate::RegisterSpec for Wf9Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf9::R`](R) reader structure"]
impl crate::Readable for Wf9Spec {}
#[doc = "`write(|w| ..)` method takes [`wf9::W`](W) writer structure"]
impl crate::Writable for Wf9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF9 to value 0"]
impl crate::Resettable for Wf9Spec {
    const RESET_VALUE: u8 = 0;
}
