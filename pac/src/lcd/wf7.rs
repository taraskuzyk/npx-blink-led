#[doc = "Register `WF7` reader"]
pub type R = crate::R<Wf7Spec>;
#[doc = "Register `WF7` writer"]
pub type W = crate::W<Wf7Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd7 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd7> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD7` reader - no description available"]
pub type Bpalcd7R = crate::BitReader<Bpalcd7>;
impl Bpalcd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd7 {
        match self.bits {
            false => Bpalcd7::B0,
            true => Bpalcd7::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd7::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd7::B1
    }
}
#[doc = "Field `BPALCD7` writer - no description available"]
pub type Bpalcd7W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd7>;
impl<'a, REG> Bpalcd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd7::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd7::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd7 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd7> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD7` reader - no description available"]
pub type Bpblcd7R = crate::BitReader<Bpblcd7>;
impl Bpblcd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd7 {
        match self.bits {
            false => Bpblcd7::B0,
            true => Bpblcd7::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd7::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd7::B1
    }
}
#[doc = "Field `BPBLCD7` writer - no description available"]
pub type Bpblcd7W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd7>;
impl<'a, REG> Bpblcd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd7::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd7::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd7 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd7> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD7` reader - no description available"]
pub type Bpclcd7R = crate::BitReader<Bpclcd7>;
impl Bpclcd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd7 {
        match self.bits {
            false => Bpclcd7::B0,
            true => Bpclcd7::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd7::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd7::B1
    }
}
#[doc = "Field `BPCLCD7` writer - no description available"]
pub type Bpclcd7W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd7>;
impl<'a, REG> Bpclcd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd7::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd7::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd7 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd7> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD7` reader - no description available"]
pub type Bpdlcd7R = crate::BitReader<Bpdlcd7>;
impl Bpdlcd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd7 {
        match self.bits {
            false => Bpdlcd7::B0,
            true => Bpdlcd7::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd7::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd7::B1
    }
}
#[doc = "Field `BPDLCD7` writer - no description available"]
pub type Bpdlcd7W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd7>;
impl<'a, REG> Bpdlcd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd7::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd7::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd7 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd7> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD7` reader - no description available"]
pub type Bpelcd7R = crate::BitReader<Bpelcd7>;
impl Bpelcd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd7 {
        match self.bits {
            false => Bpelcd7::B0,
            true => Bpelcd7::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd7::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd7::B1
    }
}
#[doc = "Field `BPELCD7` writer - no description available"]
pub type Bpelcd7W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd7>;
impl<'a, REG> Bpelcd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd7::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd7::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd7 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd7> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD7` reader - no description available"]
pub type Bpflcd7R = crate::BitReader<Bpflcd7>;
impl Bpflcd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd7 {
        match self.bits {
            false => Bpflcd7::B0,
            true => Bpflcd7::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd7::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd7::B1
    }
}
#[doc = "Field `BPFLCD7` writer - no description available"]
pub type Bpflcd7W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd7>;
impl<'a, REG> Bpflcd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd7::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd7::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd7 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd7> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD7` reader - no description available"]
pub type Bpglcd7R = crate::BitReader<Bpglcd7>;
impl Bpglcd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd7 {
        match self.bits {
            false => Bpglcd7::B0,
            true => Bpglcd7::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd7::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd7::B1
    }
}
#[doc = "Field `BPGLCD7` writer - no description available"]
pub type Bpglcd7W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd7>;
impl<'a, REG> Bpglcd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd7::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd7::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd7 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd7> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD7` reader - no description available"]
pub type Bphlcd7R = crate::BitReader<Bphlcd7>;
impl Bphlcd7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd7 {
        match self.bits {
            false => Bphlcd7::B0,
            true => Bphlcd7::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd7::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd7::B1
    }
}
#[doc = "Field `BPHLCD7` writer - no description available"]
pub type Bphlcd7W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd7>;
impl<'a, REG> Bphlcd7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd7::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd7::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd7(&self) -> Bpalcd7R {
        Bpalcd7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd7(&self) -> Bpblcd7R {
        Bpblcd7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd7(&self) -> Bpclcd7R {
        Bpclcd7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd7(&self) -> Bpdlcd7R {
        Bpdlcd7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd7(&self) -> Bpelcd7R {
        Bpelcd7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd7(&self) -> Bpflcd7R {
        Bpflcd7R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd7(&self) -> Bpglcd7R {
        Bpglcd7R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd7(&self) -> Bphlcd7R {
        Bphlcd7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd7(&mut self) -> Bpalcd7W<Wf7Spec> {
        Bpalcd7W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd7(&mut self) -> Bpblcd7W<Wf7Spec> {
        Bpblcd7W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd7(&mut self) -> Bpclcd7W<Wf7Spec> {
        Bpclcd7W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd7(&mut self) -> Bpdlcd7W<Wf7Spec> {
        Bpdlcd7W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd7(&mut self) -> Bpelcd7W<Wf7Spec> {
        Bpelcd7W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd7(&mut self) -> Bpflcd7W<Wf7Spec> {
        Bpflcd7W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd7(&mut self) -> Bpglcd7W<Wf7Spec> {
        Bpglcd7W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd7(&mut self) -> Bphlcd7W<Wf7Spec> {
        Bphlcd7W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf7Spec;
impl crate::RegisterSpec for Wf7Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf7::R`](R) reader structure"]
impl crate::Readable for Wf7Spec {}
#[doc = "`write(|w| ..)` method takes [`wf7::W`](W) writer structure"]
impl crate::Writable for Wf7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF7 to value 0"]
impl crate::Resettable for Wf7Spec {
    const RESET_VALUE: u8 = 0;
}
