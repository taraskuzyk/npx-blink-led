#[doc = "Register `WF8` reader"]
pub type R = crate::R<LcdWf8Spec>;
#[doc = "Register `WF8` writer"]
pub type W = crate::W<LcdWf8Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd8 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd8> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD8` reader - no description available"]
pub type Bpalcd8R = crate::BitReader<Bpalcd8>;
impl Bpalcd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd8 {
        match self.bits {
            false => Bpalcd8::B0,
            true => Bpalcd8::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd8::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd8::B1
    }
}
#[doc = "Field `BPALCD8` writer - no description available"]
pub type Bpalcd8W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd8>;
impl<'a, REG> Bpalcd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd8::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd8::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd8 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd8> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD8` reader - no description available"]
pub type Bpblcd8R = crate::BitReader<Bpblcd8>;
impl Bpblcd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd8 {
        match self.bits {
            false => Bpblcd8::B0,
            true => Bpblcd8::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd8::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd8::B1
    }
}
#[doc = "Field `BPBLCD8` writer - no description available"]
pub type Bpblcd8W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd8>;
impl<'a, REG> Bpblcd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd8::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd8::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd8 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd8> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD8` reader - no description available"]
pub type Bpclcd8R = crate::BitReader<Bpclcd8>;
impl Bpclcd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd8 {
        match self.bits {
            false => Bpclcd8::B0,
            true => Bpclcd8::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd8::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd8::B1
    }
}
#[doc = "Field `BPCLCD8` writer - no description available"]
pub type Bpclcd8W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd8>;
impl<'a, REG> Bpclcd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd8::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd8::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd8 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd8> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD8` reader - no description available"]
pub type Bpdlcd8R = crate::BitReader<Bpdlcd8>;
impl Bpdlcd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd8 {
        match self.bits {
            false => Bpdlcd8::B0,
            true => Bpdlcd8::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd8::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd8::B1
    }
}
#[doc = "Field `BPDLCD8` writer - no description available"]
pub type Bpdlcd8W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd8>;
impl<'a, REG> Bpdlcd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd8::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd8::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd8 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd8> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD8` reader - no description available"]
pub type Bpelcd8R = crate::BitReader<Bpelcd8>;
impl Bpelcd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd8 {
        match self.bits {
            false => Bpelcd8::B0,
            true => Bpelcd8::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd8::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd8::B1
    }
}
#[doc = "Field `BPELCD8` writer - no description available"]
pub type Bpelcd8W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd8>;
impl<'a, REG> Bpelcd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd8::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd8::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd8 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd8> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD8` reader - no description available"]
pub type Bpflcd8R = crate::BitReader<Bpflcd8>;
impl Bpflcd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd8 {
        match self.bits {
            false => Bpflcd8::B0,
            true => Bpflcd8::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd8::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd8::B1
    }
}
#[doc = "Field `BPFLCD8` writer - no description available"]
pub type Bpflcd8W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd8>;
impl<'a, REG> Bpflcd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd8::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd8::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd8 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd8> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD8` reader - no description available"]
pub type Bpglcd8R = crate::BitReader<Bpglcd8>;
impl Bpglcd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd8 {
        match self.bits {
            false => Bpglcd8::B0,
            true => Bpglcd8::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd8::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd8::B1
    }
}
#[doc = "Field `BPGLCD8` writer - no description available"]
pub type Bpglcd8W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd8>;
impl<'a, REG> Bpglcd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd8::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd8::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd8 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd8> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD8` reader - no description available"]
pub type Bphlcd8R = crate::BitReader<Bphlcd8>;
impl Bphlcd8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd8 {
        match self.bits {
            false => Bphlcd8::B0,
            true => Bphlcd8::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd8::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd8::B1
    }
}
#[doc = "Field `BPHLCD8` writer - no description available"]
pub type Bphlcd8W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd8>;
impl<'a, REG> Bphlcd8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd8::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd8::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd8(&self) -> Bpalcd8R {
        Bpalcd8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd8(&self) -> Bpblcd8R {
        Bpblcd8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd8(&self) -> Bpclcd8R {
        Bpclcd8R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd8(&self) -> Bpdlcd8R {
        Bpdlcd8R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd8(&self) -> Bpelcd8R {
        Bpelcd8R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd8(&self) -> Bpflcd8R {
        Bpflcd8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd8(&self) -> Bpglcd8R {
        Bpglcd8R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd8(&self) -> Bphlcd8R {
        Bphlcd8R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd8(&mut self) -> Bpalcd8W<LcdWf8Spec> {
        Bpalcd8W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd8(&mut self) -> Bpblcd8W<LcdWf8Spec> {
        Bpblcd8W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd8(&mut self) -> Bpclcd8W<LcdWf8Spec> {
        Bpclcd8W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd8(&mut self) -> Bpdlcd8W<LcdWf8Spec> {
        Bpdlcd8W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd8(&mut self) -> Bpelcd8W<LcdWf8Spec> {
        Bpelcd8W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd8(&mut self) -> Bpflcd8W<LcdWf8Spec> {
        Bpflcd8W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd8(&mut self) -> Bpglcd8W<LcdWf8Spec> {
        Bpglcd8W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd8(&mut self) -> Bphlcd8W<LcdWf8Spec> {
        Bphlcd8W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 8.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf8Spec;
impl crate::RegisterSpec for LcdWf8Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf8::R`](R) reader structure"]
impl crate::Readable for LcdWf8Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf8::W`](W) writer structure"]
impl crate::Writable for LcdWf8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF8 to value 0"]
impl crate::Resettable for LcdWf8Spec {
    const RESET_VALUE: u8 = 0;
}
