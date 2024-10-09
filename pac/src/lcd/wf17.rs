#[doc = "Register `WF17` reader"]
pub type R = crate::R<Wf17Spec>;
#[doc = "Register `WF17` writer"]
pub type W = crate::W<Wf17Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd17 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd17> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD17` reader - no description available"]
pub type Bpalcd17R = crate::BitReader<Bpalcd17>;
impl Bpalcd17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd17 {
        match self.bits {
            false => Bpalcd17::B0,
            true => Bpalcd17::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd17::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd17::B1
    }
}
#[doc = "Field `BPALCD17` writer - no description available"]
pub type Bpalcd17W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd17>;
impl<'a, REG> Bpalcd17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd17::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd17::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd17 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd17> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD17` reader - no description available"]
pub type Bpblcd17R = crate::BitReader<Bpblcd17>;
impl Bpblcd17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd17 {
        match self.bits {
            false => Bpblcd17::B0,
            true => Bpblcd17::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd17::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd17::B1
    }
}
#[doc = "Field `BPBLCD17` writer - no description available"]
pub type Bpblcd17W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd17>;
impl<'a, REG> Bpblcd17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd17::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd17::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd17 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd17> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD17` reader - no description available"]
pub type Bpclcd17R = crate::BitReader<Bpclcd17>;
impl Bpclcd17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd17 {
        match self.bits {
            false => Bpclcd17::B0,
            true => Bpclcd17::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd17::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd17::B1
    }
}
#[doc = "Field `BPCLCD17` writer - no description available"]
pub type Bpclcd17W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd17>;
impl<'a, REG> Bpclcd17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd17::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd17::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd17 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd17> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD17` reader - no description available"]
pub type Bpdlcd17R = crate::BitReader<Bpdlcd17>;
impl Bpdlcd17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd17 {
        match self.bits {
            false => Bpdlcd17::B0,
            true => Bpdlcd17::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd17::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd17::B1
    }
}
#[doc = "Field `BPDLCD17` writer - no description available"]
pub type Bpdlcd17W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd17>;
impl<'a, REG> Bpdlcd17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd17::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd17::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd17 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd17> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD17` reader - no description available"]
pub type Bpelcd17R = crate::BitReader<Bpelcd17>;
impl Bpelcd17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd17 {
        match self.bits {
            false => Bpelcd17::B0,
            true => Bpelcd17::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd17::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd17::B1
    }
}
#[doc = "Field `BPELCD17` writer - no description available"]
pub type Bpelcd17W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd17>;
impl<'a, REG> Bpelcd17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd17::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd17::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd17 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd17> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD17` reader - no description available"]
pub type Bpflcd17R = crate::BitReader<Bpflcd17>;
impl Bpflcd17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd17 {
        match self.bits {
            false => Bpflcd17::B0,
            true => Bpflcd17::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd17::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd17::B1
    }
}
#[doc = "Field `BPFLCD17` writer - no description available"]
pub type Bpflcd17W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd17>;
impl<'a, REG> Bpflcd17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd17::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd17::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd17 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd17> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD17` reader - no description available"]
pub type Bpglcd17R = crate::BitReader<Bpglcd17>;
impl Bpglcd17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd17 {
        match self.bits {
            false => Bpglcd17::B0,
            true => Bpglcd17::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd17::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd17::B1
    }
}
#[doc = "Field `BPGLCD17` writer - no description available"]
pub type Bpglcd17W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd17>;
impl<'a, REG> Bpglcd17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd17::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd17::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd17 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd17> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD17` reader - no description available"]
pub type Bphlcd17R = crate::BitReader<Bphlcd17>;
impl Bphlcd17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd17 {
        match self.bits {
            false => Bphlcd17::B0,
            true => Bphlcd17::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd17::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd17::B1
    }
}
#[doc = "Field `BPHLCD17` writer - no description available"]
pub type Bphlcd17W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd17>;
impl<'a, REG> Bphlcd17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd17::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd17::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd17(&self) -> Bpalcd17R {
        Bpalcd17R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd17(&self) -> Bpblcd17R {
        Bpblcd17R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd17(&self) -> Bpclcd17R {
        Bpclcd17R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd17(&self) -> Bpdlcd17R {
        Bpdlcd17R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd17(&self) -> Bpelcd17R {
        Bpelcd17R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd17(&self) -> Bpflcd17R {
        Bpflcd17R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd17(&self) -> Bpglcd17R {
        Bpglcd17R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd17(&self) -> Bphlcd17R {
        Bphlcd17R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd17(&mut self) -> Bpalcd17W<Wf17Spec> {
        Bpalcd17W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd17(&mut self) -> Bpblcd17W<Wf17Spec> {
        Bpblcd17W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd17(&mut self) -> Bpclcd17W<Wf17Spec> {
        Bpclcd17W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd17(&mut self) -> Bpdlcd17W<Wf17Spec> {
        Bpdlcd17W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd17(&mut self) -> Bpelcd17W<Wf17Spec> {
        Bpelcd17W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd17(&mut self) -> Bpflcd17W<Wf17Spec> {
        Bpflcd17W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd17(&mut self) -> Bpglcd17W<Wf17Spec> {
        Bpglcd17W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd17(&mut self) -> Bphlcd17W<Wf17Spec> {
        Bphlcd17W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 17.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf17Spec;
impl crate::RegisterSpec for Wf17Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf17::R`](R) reader structure"]
impl crate::Readable for Wf17Spec {}
#[doc = "`write(|w| ..)` method takes [`wf17::W`](W) writer structure"]
impl crate::Writable for Wf17Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF17 to value 0"]
impl crate::Resettable for Wf17Spec {
    const RESET_VALUE: u8 = 0;
}
