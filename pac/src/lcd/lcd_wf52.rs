#[doc = "Register `WF52` reader"]
pub type R = crate::R<LcdWf52Spec>;
#[doc = "Register `WF52` writer"]
pub type W = crate::W<LcdWf52Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd52 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd52> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD52` reader - no description available"]
pub type Bpalcd52R = crate::BitReader<Bpalcd52>;
impl Bpalcd52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd52 {
        match self.bits {
            false => Bpalcd52::B0,
            true => Bpalcd52::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd52::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd52::B1
    }
}
#[doc = "Field `BPALCD52` writer - no description available"]
pub type Bpalcd52W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd52>;
impl<'a, REG> Bpalcd52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd52::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd52::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd52 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd52> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD52` reader - no description available"]
pub type Bpblcd52R = crate::BitReader<Bpblcd52>;
impl Bpblcd52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd52 {
        match self.bits {
            false => Bpblcd52::B0,
            true => Bpblcd52::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd52::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd52::B1
    }
}
#[doc = "Field `BPBLCD52` writer - no description available"]
pub type Bpblcd52W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd52>;
impl<'a, REG> Bpblcd52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd52::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd52::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd52 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd52> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD52` reader - no description available"]
pub type Bpclcd52R = crate::BitReader<Bpclcd52>;
impl Bpclcd52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd52 {
        match self.bits {
            false => Bpclcd52::B0,
            true => Bpclcd52::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd52::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd52::B1
    }
}
#[doc = "Field `BPCLCD52` writer - no description available"]
pub type Bpclcd52W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd52>;
impl<'a, REG> Bpclcd52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd52::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd52::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd52 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd52> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD52` reader - no description available"]
pub type Bpdlcd52R = crate::BitReader<Bpdlcd52>;
impl Bpdlcd52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd52 {
        match self.bits {
            false => Bpdlcd52::B0,
            true => Bpdlcd52::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd52::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd52::B1
    }
}
#[doc = "Field `BPDLCD52` writer - no description available"]
pub type Bpdlcd52W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd52>;
impl<'a, REG> Bpdlcd52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd52::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd52::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd52 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd52> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD52` reader - no description available"]
pub type Bpelcd52R = crate::BitReader<Bpelcd52>;
impl Bpelcd52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd52 {
        match self.bits {
            false => Bpelcd52::B0,
            true => Bpelcd52::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd52::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd52::B1
    }
}
#[doc = "Field `BPELCD52` writer - no description available"]
pub type Bpelcd52W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd52>;
impl<'a, REG> Bpelcd52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd52::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd52::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd52 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd52> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD52` reader - no description available"]
pub type Bpflcd52R = crate::BitReader<Bpflcd52>;
impl Bpflcd52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd52 {
        match self.bits {
            false => Bpflcd52::B0,
            true => Bpflcd52::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd52::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd52::B1
    }
}
#[doc = "Field `BPFLCD52` writer - no description available"]
pub type Bpflcd52W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd52>;
impl<'a, REG> Bpflcd52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd52::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd52::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd52 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd52> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD52` reader - no description available"]
pub type Bpglcd52R = crate::BitReader<Bpglcd52>;
impl Bpglcd52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd52 {
        match self.bits {
            false => Bpglcd52::B0,
            true => Bpglcd52::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd52::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd52::B1
    }
}
#[doc = "Field `BPGLCD52` writer - no description available"]
pub type Bpglcd52W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd52>;
impl<'a, REG> Bpglcd52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd52::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd52::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd52 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd52> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd52) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD52` reader - no description available"]
pub type Bphlcd52R = crate::BitReader<Bphlcd52>;
impl Bphlcd52R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd52 {
        match self.bits {
            false => Bphlcd52::B0,
            true => Bphlcd52::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd52::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd52::B1
    }
}
#[doc = "Field `BPHLCD52` writer - no description available"]
pub type Bphlcd52W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd52>;
impl<'a, REG> Bphlcd52W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd52::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd52::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd52(&self) -> Bpalcd52R {
        Bpalcd52R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd52(&self) -> Bpblcd52R {
        Bpblcd52R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd52(&self) -> Bpclcd52R {
        Bpclcd52R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd52(&self) -> Bpdlcd52R {
        Bpdlcd52R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd52(&self) -> Bpelcd52R {
        Bpelcd52R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd52(&self) -> Bpflcd52R {
        Bpflcd52R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd52(&self) -> Bpglcd52R {
        Bpglcd52R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd52(&self) -> Bphlcd52R {
        Bphlcd52R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd52(&mut self) -> Bpalcd52W<LcdWf52Spec> {
        Bpalcd52W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd52(&mut self) -> Bpblcd52W<LcdWf52Spec> {
        Bpblcd52W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd52(&mut self) -> Bpclcd52W<LcdWf52Spec> {
        Bpclcd52W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd52(&mut self) -> Bpdlcd52W<LcdWf52Spec> {
        Bpdlcd52W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd52(&mut self) -> Bpelcd52W<LcdWf52Spec> {
        Bpelcd52W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd52(&mut self) -> Bpflcd52W<LcdWf52Spec> {
        Bpflcd52W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd52(&mut self) -> Bpglcd52W<LcdWf52Spec> {
        Bpglcd52W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd52(&mut self) -> Bphlcd52W<LcdWf52Spec> {
        Bphlcd52W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 52.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf52Spec;
impl crate::RegisterSpec for LcdWf52Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf52::R`](R) reader structure"]
impl crate::Readable for LcdWf52Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf52::W`](W) writer structure"]
impl crate::Writable for LcdWf52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF52 to value 0"]
impl crate::Resettable for LcdWf52Spec {
    const RESET_VALUE: u8 = 0;
}
