#[doc = "Register `WF0` reader"]
pub type R = crate::R<LcdWf0Spec>;
#[doc = "Register `WF0` writer"]
pub type W = crate::W<LcdWf0Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd0 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd0> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD0` reader - no description available"]
pub type Bpalcd0R = crate::BitReader<Bpalcd0>;
impl Bpalcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd0 {
        match self.bits {
            false => Bpalcd0::B0,
            true => Bpalcd0::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd0::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd0::B1
    }
}
#[doc = "Field `BPALCD0` writer - no description available"]
pub type Bpalcd0W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd0>;
impl<'a, REG> Bpalcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd0::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd0::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd0 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd0> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD0` reader - no description available"]
pub type Bpblcd0R = crate::BitReader<Bpblcd0>;
impl Bpblcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd0 {
        match self.bits {
            false => Bpblcd0::B0,
            true => Bpblcd0::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd0::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd0::B1
    }
}
#[doc = "Field `BPBLCD0` writer - no description available"]
pub type Bpblcd0W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd0>;
impl<'a, REG> Bpblcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd0::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd0::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd0 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd0> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD0` reader - no description available"]
pub type Bpclcd0R = crate::BitReader<Bpclcd0>;
impl Bpclcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd0 {
        match self.bits {
            false => Bpclcd0::B0,
            true => Bpclcd0::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd0::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd0::B1
    }
}
#[doc = "Field `BPCLCD0` writer - no description available"]
pub type Bpclcd0W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd0>;
impl<'a, REG> Bpclcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd0::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd0::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd0 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd0> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD0` reader - no description available"]
pub type Bpdlcd0R = crate::BitReader<Bpdlcd0>;
impl Bpdlcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd0 {
        match self.bits {
            false => Bpdlcd0::B0,
            true => Bpdlcd0::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd0::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd0::B1
    }
}
#[doc = "Field `BPDLCD0` writer - no description available"]
pub type Bpdlcd0W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd0>;
impl<'a, REG> Bpdlcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd0::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd0::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd0 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd0> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD0` reader - no description available"]
pub type Bpelcd0R = crate::BitReader<Bpelcd0>;
impl Bpelcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd0 {
        match self.bits {
            false => Bpelcd0::B0,
            true => Bpelcd0::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd0::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd0::B1
    }
}
#[doc = "Field `BPELCD0` writer - no description available"]
pub type Bpelcd0W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd0>;
impl<'a, REG> Bpelcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd0::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd0::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd0 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd0> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD0` reader - no description available"]
pub type Bpflcd0R = crate::BitReader<Bpflcd0>;
impl Bpflcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd0 {
        match self.bits {
            false => Bpflcd0::B0,
            true => Bpflcd0::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd0::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd0::B1
    }
}
#[doc = "Field `BPFLCD0` writer - no description available"]
pub type Bpflcd0W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd0>;
impl<'a, REG> Bpflcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd0::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd0::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd0 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd0> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD0` reader - no description available"]
pub type Bpglcd0R = crate::BitReader<Bpglcd0>;
impl Bpglcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd0 {
        match self.bits {
            false => Bpglcd0::B0,
            true => Bpglcd0::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd0::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd0::B1
    }
}
#[doc = "Field `BPGLCD0` writer - no description available"]
pub type Bpglcd0W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd0>;
impl<'a, REG> Bpglcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd0::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd0::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd0 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd0> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD0` reader - no description available"]
pub type Bphlcd0R = crate::BitReader<Bphlcd0>;
impl Bphlcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd0 {
        match self.bits {
            false => Bphlcd0::B0,
            true => Bphlcd0::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd0::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd0::B1
    }
}
#[doc = "Field `BPHLCD0` writer - no description available"]
pub type Bphlcd0W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd0>;
impl<'a, REG> Bphlcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd0::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd0::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd0(&self) -> Bpalcd0R {
        Bpalcd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd0(&self) -> Bpblcd0R {
        Bpblcd0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd0(&self) -> Bpclcd0R {
        Bpclcd0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd0(&self) -> Bpdlcd0R {
        Bpdlcd0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd0(&self) -> Bpelcd0R {
        Bpelcd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd0(&self) -> Bpflcd0R {
        Bpflcd0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd0(&self) -> Bpglcd0R {
        Bpglcd0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd0(&self) -> Bphlcd0R {
        Bphlcd0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd0(&mut self) -> Bpalcd0W<LcdWf0Spec> {
        Bpalcd0W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd0(&mut self) -> Bpblcd0W<LcdWf0Spec> {
        Bpblcd0W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd0(&mut self) -> Bpclcd0W<LcdWf0Spec> {
        Bpclcd0W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd0(&mut self) -> Bpdlcd0W<LcdWf0Spec> {
        Bpdlcd0W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd0(&mut self) -> Bpelcd0W<LcdWf0Spec> {
        Bpelcd0W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd0(&mut self) -> Bpflcd0W<LcdWf0Spec> {
        Bpflcd0W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd0(&mut self) -> Bpglcd0W<LcdWf0Spec> {
        Bpglcd0W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd0(&mut self) -> Bphlcd0W<LcdWf0Spec> {
        Bphlcd0W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf0Spec;
impl crate::RegisterSpec for LcdWf0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf0::R`](R) reader structure"]
impl crate::Readable for LcdWf0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf0::W`](W) writer structure"]
impl crate::Writable for LcdWf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF0 to value 0"]
impl crate::Resettable for LcdWf0Spec {
    const RESET_VALUE: u8 = 0;
}
