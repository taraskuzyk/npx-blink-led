#[doc = "Register `WF5` reader"]
pub type R = crate::R<Wf5Spec>;
#[doc = "Register `WF5` writer"]
pub type W = crate::W<Wf5Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd5 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd5> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD5` reader - no description available"]
pub type Bpalcd5R = crate::BitReader<Bpalcd5>;
impl Bpalcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd5 {
        match self.bits {
            false => Bpalcd5::B0,
            true => Bpalcd5::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd5::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd5::B1
    }
}
#[doc = "Field `BPALCD5` writer - no description available"]
pub type Bpalcd5W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd5>;
impl<'a, REG> Bpalcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd5::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd5::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd5 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd5> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD5` reader - no description available"]
pub type Bpblcd5R = crate::BitReader<Bpblcd5>;
impl Bpblcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd5 {
        match self.bits {
            false => Bpblcd5::B0,
            true => Bpblcd5::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd5::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd5::B1
    }
}
#[doc = "Field `BPBLCD5` writer - no description available"]
pub type Bpblcd5W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd5>;
impl<'a, REG> Bpblcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd5::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd5::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd5 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd5> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD5` reader - no description available"]
pub type Bpclcd5R = crate::BitReader<Bpclcd5>;
impl Bpclcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd5 {
        match self.bits {
            false => Bpclcd5::B0,
            true => Bpclcd5::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd5::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd5::B1
    }
}
#[doc = "Field `BPCLCD5` writer - no description available"]
pub type Bpclcd5W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd5>;
impl<'a, REG> Bpclcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd5::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd5::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd5 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd5> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD5` reader - no description available"]
pub type Bpdlcd5R = crate::BitReader<Bpdlcd5>;
impl Bpdlcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd5 {
        match self.bits {
            false => Bpdlcd5::B0,
            true => Bpdlcd5::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd5::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd5::B1
    }
}
#[doc = "Field `BPDLCD5` writer - no description available"]
pub type Bpdlcd5W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd5>;
impl<'a, REG> Bpdlcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd5::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd5::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd5 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd5> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD5` reader - no description available"]
pub type Bpelcd5R = crate::BitReader<Bpelcd5>;
impl Bpelcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd5 {
        match self.bits {
            false => Bpelcd5::B0,
            true => Bpelcd5::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd5::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd5::B1
    }
}
#[doc = "Field `BPELCD5` writer - no description available"]
pub type Bpelcd5W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd5>;
impl<'a, REG> Bpelcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd5::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd5::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd5 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd5> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD5` reader - no description available"]
pub type Bpflcd5R = crate::BitReader<Bpflcd5>;
impl Bpflcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd5 {
        match self.bits {
            false => Bpflcd5::B0,
            true => Bpflcd5::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd5::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd5::B1
    }
}
#[doc = "Field `BPFLCD5` writer - no description available"]
pub type Bpflcd5W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd5>;
impl<'a, REG> Bpflcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd5::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd5::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd5 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd5> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD5` reader - no description available"]
pub type Bpglcd5R = crate::BitReader<Bpglcd5>;
impl Bpglcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd5 {
        match self.bits {
            false => Bpglcd5::B0,
            true => Bpglcd5::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd5::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd5::B1
    }
}
#[doc = "Field `BPGLCD5` writer - no description available"]
pub type Bpglcd5W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd5>;
impl<'a, REG> Bpglcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd5::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd5::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd5 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd5> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD5` reader - no description available"]
pub type Bphlcd5R = crate::BitReader<Bphlcd5>;
impl Bphlcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd5 {
        match self.bits {
            false => Bphlcd5::B0,
            true => Bphlcd5::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd5::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd5::B1
    }
}
#[doc = "Field `BPHLCD5` writer - no description available"]
pub type Bphlcd5W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd5>;
impl<'a, REG> Bphlcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd5::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd5::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd5(&self) -> Bpalcd5R {
        Bpalcd5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd5(&self) -> Bpblcd5R {
        Bpblcd5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd5(&self) -> Bpclcd5R {
        Bpclcd5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd5(&self) -> Bpdlcd5R {
        Bpdlcd5R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd5(&self) -> Bpelcd5R {
        Bpelcd5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd5(&self) -> Bpflcd5R {
        Bpflcd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd5(&self) -> Bpglcd5R {
        Bpglcd5R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd5(&self) -> Bphlcd5R {
        Bphlcd5R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd5(&mut self) -> Bpalcd5W<Wf5Spec> {
        Bpalcd5W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd5(&mut self) -> Bpblcd5W<Wf5Spec> {
        Bpblcd5W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd5(&mut self) -> Bpclcd5W<Wf5Spec> {
        Bpclcd5W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd5(&mut self) -> Bpdlcd5W<Wf5Spec> {
        Bpdlcd5W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd5(&mut self) -> Bpelcd5W<Wf5Spec> {
        Bpelcd5W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd5(&mut self) -> Bpflcd5W<Wf5Spec> {
        Bpflcd5W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd5(&mut self) -> Bpglcd5W<Wf5Spec> {
        Bpglcd5W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd5(&mut self) -> Bphlcd5W<Wf5Spec> {
        Bphlcd5W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf5Spec;
impl crate::RegisterSpec for Wf5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf5::R`](R) reader structure"]
impl crate::Readable for Wf5Spec {}
#[doc = "`write(|w| ..)` method takes [`wf5::W`](W) writer structure"]
impl crate::Writable for Wf5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF5 to value 0"]
impl crate::Resettable for Wf5Spec {
    const RESET_VALUE: u8 = 0;
}
