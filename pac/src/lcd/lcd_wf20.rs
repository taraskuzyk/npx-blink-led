#[doc = "Register `WF20` reader"]
pub type R = crate::R<LcdWf20Spec>;
#[doc = "Register `WF20` writer"]
pub type W = crate::W<LcdWf20Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd20 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd20> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD20` reader - no description available"]
pub type Bpalcd20R = crate::BitReader<Bpalcd20>;
impl Bpalcd20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd20 {
        match self.bits {
            false => Bpalcd20::B0,
            true => Bpalcd20::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd20::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd20::B1
    }
}
#[doc = "Field `BPALCD20` writer - no description available"]
pub type Bpalcd20W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd20>;
impl<'a, REG> Bpalcd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd20::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd20::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd20 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd20> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD20` reader - no description available"]
pub type Bpblcd20R = crate::BitReader<Bpblcd20>;
impl Bpblcd20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd20 {
        match self.bits {
            false => Bpblcd20::B0,
            true => Bpblcd20::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd20::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd20::B1
    }
}
#[doc = "Field `BPBLCD20` writer - no description available"]
pub type Bpblcd20W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd20>;
impl<'a, REG> Bpblcd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd20::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd20::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd20 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd20> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD20` reader - no description available"]
pub type Bpclcd20R = crate::BitReader<Bpclcd20>;
impl Bpclcd20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd20 {
        match self.bits {
            false => Bpclcd20::B0,
            true => Bpclcd20::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd20::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd20::B1
    }
}
#[doc = "Field `BPCLCD20` writer - no description available"]
pub type Bpclcd20W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd20>;
impl<'a, REG> Bpclcd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd20::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd20::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd20 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd20> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD20` reader - no description available"]
pub type Bpdlcd20R = crate::BitReader<Bpdlcd20>;
impl Bpdlcd20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd20 {
        match self.bits {
            false => Bpdlcd20::B0,
            true => Bpdlcd20::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd20::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd20::B1
    }
}
#[doc = "Field `BPDLCD20` writer - no description available"]
pub type Bpdlcd20W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd20>;
impl<'a, REG> Bpdlcd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd20::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd20::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd20 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd20> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD20` reader - no description available"]
pub type Bpelcd20R = crate::BitReader<Bpelcd20>;
impl Bpelcd20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd20 {
        match self.bits {
            false => Bpelcd20::B0,
            true => Bpelcd20::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd20::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd20::B1
    }
}
#[doc = "Field `BPELCD20` writer - no description available"]
pub type Bpelcd20W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd20>;
impl<'a, REG> Bpelcd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd20::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd20::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd20 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd20> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD20` reader - no description available"]
pub type Bpflcd20R = crate::BitReader<Bpflcd20>;
impl Bpflcd20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd20 {
        match self.bits {
            false => Bpflcd20::B0,
            true => Bpflcd20::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd20::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd20::B1
    }
}
#[doc = "Field `BPFLCD20` writer - no description available"]
pub type Bpflcd20W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd20>;
impl<'a, REG> Bpflcd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd20::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd20::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd20 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd20> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD20` reader - no description available"]
pub type Bpglcd20R = crate::BitReader<Bpglcd20>;
impl Bpglcd20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd20 {
        match self.bits {
            false => Bpglcd20::B0,
            true => Bpglcd20::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd20::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd20::B1
    }
}
#[doc = "Field `BPGLCD20` writer - no description available"]
pub type Bpglcd20W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd20>;
impl<'a, REG> Bpglcd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd20::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd20::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd20 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd20> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD20` reader - no description available"]
pub type Bphlcd20R = crate::BitReader<Bphlcd20>;
impl Bphlcd20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd20 {
        match self.bits {
            false => Bphlcd20::B0,
            true => Bphlcd20::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd20::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd20::B1
    }
}
#[doc = "Field `BPHLCD20` writer - no description available"]
pub type Bphlcd20W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd20>;
impl<'a, REG> Bphlcd20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd20::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd20::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd20(&self) -> Bpalcd20R {
        Bpalcd20R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd20(&self) -> Bpblcd20R {
        Bpblcd20R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd20(&self) -> Bpclcd20R {
        Bpclcd20R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd20(&self) -> Bpdlcd20R {
        Bpdlcd20R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd20(&self) -> Bpelcd20R {
        Bpelcd20R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd20(&self) -> Bpflcd20R {
        Bpflcd20R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd20(&self) -> Bpglcd20R {
        Bpglcd20R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd20(&self) -> Bphlcd20R {
        Bphlcd20R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd20(&mut self) -> Bpalcd20W<LcdWf20Spec> {
        Bpalcd20W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd20(&mut self) -> Bpblcd20W<LcdWf20Spec> {
        Bpblcd20W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd20(&mut self) -> Bpclcd20W<LcdWf20Spec> {
        Bpclcd20W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd20(&mut self) -> Bpdlcd20W<LcdWf20Spec> {
        Bpdlcd20W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd20(&mut self) -> Bpelcd20W<LcdWf20Spec> {
        Bpelcd20W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd20(&mut self) -> Bpflcd20W<LcdWf20Spec> {
        Bpflcd20W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd20(&mut self) -> Bpglcd20W<LcdWf20Spec> {
        Bpglcd20W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd20(&mut self) -> Bphlcd20W<LcdWf20Spec> {
        Bphlcd20W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 20.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf20Spec;
impl crate::RegisterSpec for LcdWf20Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf20::R`](R) reader structure"]
impl crate::Readable for LcdWf20Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf20::W`](W) writer structure"]
impl crate::Writable for LcdWf20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF20 to value 0"]
impl crate::Resettable for LcdWf20Spec {
    const RESET_VALUE: u8 = 0;
}
