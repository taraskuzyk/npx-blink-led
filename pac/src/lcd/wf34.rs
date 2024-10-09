#[doc = "Register `WF34` reader"]
pub type R = crate::R<Wf34Spec>;
#[doc = "Register `WF34` writer"]
pub type W = crate::W<Wf34Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd34 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd34> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD34` reader - no description available"]
pub type Bpalcd34R = crate::BitReader<Bpalcd34>;
impl Bpalcd34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd34 {
        match self.bits {
            false => Bpalcd34::B0,
            true => Bpalcd34::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd34::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd34::B1
    }
}
#[doc = "Field `BPALCD34` writer - no description available"]
pub type Bpalcd34W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd34>;
impl<'a, REG> Bpalcd34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd34::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd34::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd34 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd34> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD34` reader - no description available"]
pub type Bpblcd34R = crate::BitReader<Bpblcd34>;
impl Bpblcd34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd34 {
        match self.bits {
            false => Bpblcd34::B0,
            true => Bpblcd34::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd34::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd34::B1
    }
}
#[doc = "Field `BPBLCD34` writer - no description available"]
pub type Bpblcd34W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd34>;
impl<'a, REG> Bpblcd34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd34::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd34::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd34 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd34> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD34` reader - no description available"]
pub type Bpclcd34R = crate::BitReader<Bpclcd34>;
impl Bpclcd34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd34 {
        match self.bits {
            false => Bpclcd34::B0,
            true => Bpclcd34::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd34::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd34::B1
    }
}
#[doc = "Field `BPCLCD34` writer - no description available"]
pub type Bpclcd34W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd34>;
impl<'a, REG> Bpclcd34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd34::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd34::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd34 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd34> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD34` reader - no description available"]
pub type Bpdlcd34R = crate::BitReader<Bpdlcd34>;
impl Bpdlcd34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd34 {
        match self.bits {
            false => Bpdlcd34::B0,
            true => Bpdlcd34::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd34::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd34::B1
    }
}
#[doc = "Field `BPDLCD34` writer - no description available"]
pub type Bpdlcd34W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd34>;
impl<'a, REG> Bpdlcd34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd34::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd34::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd34 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd34> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD34` reader - no description available"]
pub type Bpelcd34R = crate::BitReader<Bpelcd34>;
impl Bpelcd34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd34 {
        match self.bits {
            false => Bpelcd34::B0,
            true => Bpelcd34::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd34::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd34::B1
    }
}
#[doc = "Field `BPELCD34` writer - no description available"]
pub type Bpelcd34W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd34>;
impl<'a, REG> Bpelcd34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd34::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd34::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd34 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd34> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD34` reader - no description available"]
pub type Bpflcd34R = crate::BitReader<Bpflcd34>;
impl Bpflcd34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd34 {
        match self.bits {
            false => Bpflcd34::B0,
            true => Bpflcd34::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd34::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd34::B1
    }
}
#[doc = "Field `BPFLCD34` writer - no description available"]
pub type Bpflcd34W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd34>;
impl<'a, REG> Bpflcd34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd34::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd34::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd34 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd34> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD34` reader - no description available"]
pub type Bpglcd34R = crate::BitReader<Bpglcd34>;
impl Bpglcd34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd34 {
        match self.bits {
            false => Bpglcd34::B0,
            true => Bpglcd34::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd34::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd34::B1
    }
}
#[doc = "Field `BPGLCD34` writer - no description available"]
pub type Bpglcd34W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd34>;
impl<'a, REG> Bpglcd34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd34::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd34::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd34 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd34> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd34) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD34` reader - no description available"]
pub type Bphlcd34R = crate::BitReader<Bphlcd34>;
impl Bphlcd34R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd34 {
        match self.bits {
            false => Bphlcd34::B0,
            true => Bphlcd34::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd34::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd34::B1
    }
}
#[doc = "Field `BPHLCD34` writer - no description available"]
pub type Bphlcd34W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd34>;
impl<'a, REG> Bphlcd34W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd34::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd34::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd34(&self) -> Bpalcd34R {
        Bpalcd34R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd34(&self) -> Bpblcd34R {
        Bpblcd34R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd34(&self) -> Bpclcd34R {
        Bpclcd34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd34(&self) -> Bpdlcd34R {
        Bpdlcd34R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd34(&self) -> Bpelcd34R {
        Bpelcd34R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd34(&self) -> Bpflcd34R {
        Bpflcd34R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd34(&self) -> Bpglcd34R {
        Bpglcd34R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd34(&self) -> Bphlcd34R {
        Bphlcd34R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd34(&mut self) -> Bpalcd34W<Wf34Spec> {
        Bpalcd34W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd34(&mut self) -> Bpblcd34W<Wf34Spec> {
        Bpblcd34W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd34(&mut self) -> Bpclcd34W<Wf34Spec> {
        Bpclcd34W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd34(&mut self) -> Bpdlcd34W<Wf34Spec> {
        Bpdlcd34W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd34(&mut self) -> Bpelcd34W<Wf34Spec> {
        Bpelcd34W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd34(&mut self) -> Bpflcd34W<Wf34Spec> {
        Bpflcd34W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd34(&mut self) -> Bpglcd34W<Wf34Spec> {
        Bpglcd34W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd34(&mut self) -> Bphlcd34W<Wf34Spec> {
        Bphlcd34W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 34.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf34Spec;
impl crate::RegisterSpec for Wf34Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf34::R`](R) reader structure"]
impl crate::Readable for Wf34Spec {}
#[doc = "`write(|w| ..)` method takes [`wf34::W`](W) writer structure"]
impl crate::Writable for Wf34Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF34 to value 0"]
impl crate::Resettable for Wf34Spec {
    const RESET_VALUE: u8 = 0;
}
