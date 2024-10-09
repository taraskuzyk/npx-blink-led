#[doc = "Register `WF40` reader"]
pub type R = crate::R<LcdWf40Spec>;
#[doc = "Register `WF40` writer"]
pub type W = crate::W<LcdWf40Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd40 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd40> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD40` reader - no description available"]
pub type Bpalcd40R = crate::BitReader<Bpalcd40>;
impl Bpalcd40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd40 {
        match self.bits {
            false => Bpalcd40::B0,
            true => Bpalcd40::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd40::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd40::B1
    }
}
#[doc = "Field `BPALCD40` writer - no description available"]
pub type Bpalcd40W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd40>;
impl<'a, REG> Bpalcd40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd40::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd40::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd40 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd40> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD40` reader - no description available"]
pub type Bpblcd40R = crate::BitReader<Bpblcd40>;
impl Bpblcd40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd40 {
        match self.bits {
            false => Bpblcd40::B0,
            true => Bpblcd40::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd40::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd40::B1
    }
}
#[doc = "Field `BPBLCD40` writer - no description available"]
pub type Bpblcd40W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd40>;
impl<'a, REG> Bpblcd40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd40::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd40::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd40 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd40> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD40` reader - no description available"]
pub type Bpclcd40R = crate::BitReader<Bpclcd40>;
impl Bpclcd40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd40 {
        match self.bits {
            false => Bpclcd40::B0,
            true => Bpclcd40::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd40::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd40::B1
    }
}
#[doc = "Field `BPCLCD40` writer - no description available"]
pub type Bpclcd40W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd40>;
impl<'a, REG> Bpclcd40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd40::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd40::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd40 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd40> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD40` reader - no description available"]
pub type Bpdlcd40R = crate::BitReader<Bpdlcd40>;
impl Bpdlcd40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd40 {
        match self.bits {
            false => Bpdlcd40::B0,
            true => Bpdlcd40::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd40::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd40::B1
    }
}
#[doc = "Field `BPDLCD40` writer - no description available"]
pub type Bpdlcd40W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd40>;
impl<'a, REG> Bpdlcd40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd40::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd40::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd40 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd40> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD40` reader - no description available"]
pub type Bpelcd40R = crate::BitReader<Bpelcd40>;
impl Bpelcd40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd40 {
        match self.bits {
            false => Bpelcd40::B0,
            true => Bpelcd40::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd40::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd40::B1
    }
}
#[doc = "Field `BPELCD40` writer - no description available"]
pub type Bpelcd40W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd40>;
impl<'a, REG> Bpelcd40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd40::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd40::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd40 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd40> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD40` reader - no description available"]
pub type Bpflcd40R = crate::BitReader<Bpflcd40>;
impl Bpflcd40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd40 {
        match self.bits {
            false => Bpflcd40::B0,
            true => Bpflcd40::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd40::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd40::B1
    }
}
#[doc = "Field `BPFLCD40` writer - no description available"]
pub type Bpflcd40W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd40>;
impl<'a, REG> Bpflcd40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd40::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd40::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd40 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd40> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD40` reader - no description available"]
pub type Bpglcd40R = crate::BitReader<Bpglcd40>;
impl Bpglcd40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd40 {
        match self.bits {
            false => Bpglcd40::B0,
            true => Bpglcd40::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd40::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd40::B1
    }
}
#[doc = "Field `BPGLCD40` writer - no description available"]
pub type Bpglcd40W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd40>;
impl<'a, REG> Bpglcd40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd40::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd40::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd40 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd40> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd40) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD40` reader - no description available"]
pub type Bphlcd40R = crate::BitReader<Bphlcd40>;
impl Bphlcd40R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd40 {
        match self.bits {
            false => Bphlcd40::B0,
            true => Bphlcd40::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd40::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd40::B1
    }
}
#[doc = "Field `BPHLCD40` writer - no description available"]
pub type Bphlcd40W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd40>;
impl<'a, REG> Bphlcd40W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd40::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd40::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd40(&self) -> Bpalcd40R {
        Bpalcd40R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd40(&self) -> Bpblcd40R {
        Bpblcd40R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd40(&self) -> Bpclcd40R {
        Bpclcd40R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd40(&self) -> Bpdlcd40R {
        Bpdlcd40R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd40(&self) -> Bpelcd40R {
        Bpelcd40R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd40(&self) -> Bpflcd40R {
        Bpflcd40R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd40(&self) -> Bpglcd40R {
        Bpglcd40R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd40(&self) -> Bphlcd40R {
        Bphlcd40R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd40(&mut self) -> Bpalcd40W<LcdWf40Spec> {
        Bpalcd40W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd40(&mut self) -> Bpblcd40W<LcdWf40Spec> {
        Bpblcd40W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd40(&mut self) -> Bpclcd40W<LcdWf40Spec> {
        Bpclcd40W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd40(&mut self) -> Bpdlcd40W<LcdWf40Spec> {
        Bpdlcd40W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd40(&mut self) -> Bpelcd40W<LcdWf40Spec> {
        Bpelcd40W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd40(&mut self) -> Bpflcd40W<LcdWf40Spec> {
        Bpflcd40W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd40(&mut self) -> Bpglcd40W<LcdWf40Spec> {
        Bpglcd40W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd40(&mut self) -> Bphlcd40W<LcdWf40Spec> {
        Bphlcd40W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 40.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf40Spec;
impl crate::RegisterSpec for LcdWf40Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf40::R`](R) reader structure"]
impl crate::Readable for LcdWf40Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf40::W`](W) writer structure"]
impl crate::Writable for LcdWf40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF40 to value 0"]
impl crate::Resettable for LcdWf40Spec {
    const RESET_VALUE: u8 = 0;
}
