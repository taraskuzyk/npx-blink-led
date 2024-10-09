#[doc = "Register `WF4` reader"]
pub type R = crate::R<LcdWf4Spec>;
#[doc = "Register `WF4` writer"]
pub type W = crate::W<LcdWf4Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd4 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd4> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD4` reader - no description available"]
pub type Bpalcd4R = crate::BitReader<Bpalcd4>;
impl Bpalcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd4 {
        match self.bits {
            false => Bpalcd4::B0,
            true => Bpalcd4::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd4::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd4::B1
    }
}
#[doc = "Field `BPALCD4` writer - no description available"]
pub type Bpalcd4W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd4>;
impl<'a, REG> Bpalcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd4::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd4::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd4 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd4> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD4` reader - no description available"]
pub type Bpblcd4R = crate::BitReader<Bpblcd4>;
impl Bpblcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd4 {
        match self.bits {
            false => Bpblcd4::B0,
            true => Bpblcd4::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd4::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd4::B1
    }
}
#[doc = "Field `BPBLCD4` writer - no description available"]
pub type Bpblcd4W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd4>;
impl<'a, REG> Bpblcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd4::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd4::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd4 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd4> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD4` reader - no description available"]
pub type Bpclcd4R = crate::BitReader<Bpclcd4>;
impl Bpclcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd4 {
        match self.bits {
            false => Bpclcd4::B0,
            true => Bpclcd4::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd4::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd4::B1
    }
}
#[doc = "Field `BPCLCD4` writer - no description available"]
pub type Bpclcd4W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd4>;
impl<'a, REG> Bpclcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd4::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd4::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd4 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd4> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD4` reader - no description available"]
pub type Bpdlcd4R = crate::BitReader<Bpdlcd4>;
impl Bpdlcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd4 {
        match self.bits {
            false => Bpdlcd4::B0,
            true => Bpdlcd4::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd4::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd4::B1
    }
}
#[doc = "Field `BPDLCD4` writer - no description available"]
pub type Bpdlcd4W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd4>;
impl<'a, REG> Bpdlcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd4::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd4::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd4 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd4> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD4` reader - no description available"]
pub type Bpelcd4R = crate::BitReader<Bpelcd4>;
impl Bpelcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd4 {
        match self.bits {
            false => Bpelcd4::B0,
            true => Bpelcd4::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd4::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd4::B1
    }
}
#[doc = "Field `BPELCD4` writer - no description available"]
pub type Bpelcd4W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd4>;
impl<'a, REG> Bpelcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd4::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd4::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd4 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd4> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD4` reader - no description available"]
pub type Bpflcd4R = crate::BitReader<Bpflcd4>;
impl Bpflcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd4 {
        match self.bits {
            false => Bpflcd4::B0,
            true => Bpflcd4::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd4::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd4::B1
    }
}
#[doc = "Field `BPFLCD4` writer - no description available"]
pub type Bpflcd4W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd4>;
impl<'a, REG> Bpflcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd4::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd4::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd4 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd4> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD4` reader - no description available"]
pub type Bpglcd4R = crate::BitReader<Bpglcd4>;
impl Bpglcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd4 {
        match self.bits {
            false => Bpglcd4::B0,
            true => Bpglcd4::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd4::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd4::B1
    }
}
#[doc = "Field `BPGLCD4` writer - no description available"]
pub type Bpglcd4W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd4>;
impl<'a, REG> Bpglcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd4::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd4::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd4 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd4> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD4` reader - no description available"]
pub type Bphlcd4R = crate::BitReader<Bphlcd4>;
impl Bphlcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd4 {
        match self.bits {
            false => Bphlcd4::B0,
            true => Bphlcd4::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd4::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd4::B1
    }
}
#[doc = "Field `BPHLCD4` writer - no description available"]
pub type Bphlcd4W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd4>;
impl<'a, REG> Bphlcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd4::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd4::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd4(&self) -> Bpalcd4R {
        Bpalcd4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd4(&self) -> Bpblcd4R {
        Bpblcd4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd4(&self) -> Bpclcd4R {
        Bpclcd4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd4(&self) -> Bpdlcd4R {
        Bpdlcd4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd4(&self) -> Bpelcd4R {
        Bpelcd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd4(&self) -> Bpflcd4R {
        Bpflcd4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd4(&self) -> Bpglcd4R {
        Bpglcd4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd4(&self) -> Bphlcd4R {
        Bphlcd4R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd4(&mut self) -> Bpalcd4W<LcdWf4Spec> {
        Bpalcd4W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd4(&mut self) -> Bpblcd4W<LcdWf4Spec> {
        Bpblcd4W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd4(&mut self) -> Bpclcd4W<LcdWf4Spec> {
        Bpclcd4W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd4(&mut self) -> Bpdlcd4W<LcdWf4Spec> {
        Bpdlcd4W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd4(&mut self) -> Bpelcd4W<LcdWf4Spec> {
        Bpelcd4W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd4(&mut self) -> Bpflcd4W<LcdWf4Spec> {
        Bpflcd4W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd4(&mut self) -> Bpglcd4W<LcdWf4Spec> {
        Bpglcd4W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd4(&mut self) -> Bphlcd4W<LcdWf4Spec> {
        Bphlcd4W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf4Spec;
impl crate::RegisterSpec for LcdWf4Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf4::R`](R) reader structure"]
impl crate::Readable for LcdWf4Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf4::W`](W) writer structure"]
impl crate::Writable for LcdWf4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF4 to value 0"]
impl crate::Resettable for LcdWf4Spec {
    const RESET_VALUE: u8 = 0;
}
