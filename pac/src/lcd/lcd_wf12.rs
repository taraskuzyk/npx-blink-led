#[doc = "Register `WF12` reader"]
pub type R = crate::R<LcdWf12Spec>;
#[doc = "Register `WF12` writer"]
pub type W = crate::W<LcdWf12Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd12 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd12> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD12` reader - no description available"]
pub type Bpalcd12R = crate::BitReader<Bpalcd12>;
impl Bpalcd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd12 {
        match self.bits {
            false => Bpalcd12::B0,
            true => Bpalcd12::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd12::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd12::B1
    }
}
#[doc = "Field `BPALCD12` writer - no description available"]
pub type Bpalcd12W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd12>;
impl<'a, REG> Bpalcd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd12::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd12::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd12 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd12> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD12` reader - no description available"]
pub type Bpblcd12R = crate::BitReader<Bpblcd12>;
impl Bpblcd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd12 {
        match self.bits {
            false => Bpblcd12::B0,
            true => Bpblcd12::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd12::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd12::B1
    }
}
#[doc = "Field `BPBLCD12` writer - no description available"]
pub type Bpblcd12W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd12>;
impl<'a, REG> Bpblcd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd12::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd12::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd12 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd12> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD12` reader - no description available"]
pub type Bpclcd12R = crate::BitReader<Bpclcd12>;
impl Bpclcd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd12 {
        match self.bits {
            false => Bpclcd12::B0,
            true => Bpclcd12::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd12::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd12::B1
    }
}
#[doc = "Field `BPCLCD12` writer - no description available"]
pub type Bpclcd12W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd12>;
impl<'a, REG> Bpclcd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd12::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd12::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd12 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd12> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD12` reader - no description available"]
pub type Bpdlcd12R = crate::BitReader<Bpdlcd12>;
impl Bpdlcd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd12 {
        match self.bits {
            false => Bpdlcd12::B0,
            true => Bpdlcd12::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd12::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd12::B1
    }
}
#[doc = "Field `BPDLCD12` writer - no description available"]
pub type Bpdlcd12W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd12>;
impl<'a, REG> Bpdlcd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd12::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd12::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd12 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd12> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD12` reader - no description available"]
pub type Bpelcd12R = crate::BitReader<Bpelcd12>;
impl Bpelcd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd12 {
        match self.bits {
            false => Bpelcd12::B0,
            true => Bpelcd12::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd12::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd12::B1
    }
}
#[doc = "Field `BPELCD12` writer - no description available"]
pub type Bpelcd12W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd12>;
impl<'a, REG> Bpelcd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd12::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd12::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd12 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd12> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD12` reader - no description available"]
pub type Bpflcd12R = crate::BitReader<Bpflcd12>;
impl Bpflcd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd12 {
        match self.bits {
            false => Bpflcd12::B0,
            true => Bpflcd12::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd12::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd12::B1
    }
}
#[doc = "Field `BPFLCD12` writer - no description available"]
pub type Bpflcd12W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd12>;
impl<'a, REG> Bpflcd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd12::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd12::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd12 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd12> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD12` reader - no description available"]
pub type Bpglcd12R = crate::BitReader<Bpglcd12>;
impl Bpglcd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd12 {
        match self.bits {
            false => Bpglcd12::B0,
            true => Bpglcd12::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd12::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd12::B1
    }
}
#[doc = "Field `BPGLCD12` writer - no description available"]
pub type Bpglcd12W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd12>;
impl<'a, REG> Bpglcd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd12::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd12::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd12 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd12> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD12` reader - no description available"]
pub type Bphlcd12R = crate::BitReader<Bphlcd12>;
impl Bphlcd12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd12 {
        match self.bits {
            false => Bphlcd12::B0,
            true => Bphlcd12::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd12::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd12::B1
    }
}
#[doc = "Field `BPHLCD12` writer - no description available"]
pub type Bphlcd12W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd12>;
impl<'a, REG> Bphlcd12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd12::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd12::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd12(&self) -> Bpalcd12R {
        Bpalcd12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd12(&self) -> Bpblcd12R {
        Bpblcd12R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd12(&self) -> Bpclcd12R {
        Bpclcd12R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd12(&self) -> Bpdlcd12R {
        Bpdlcd12R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd12(&self) -> Bpelcd12R {
        Bpelcd12R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd12(&self) -> Bpflcd12R {
        Bpflcd12R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd12(&self) -> Bpglcd12R {
        Bpglcd12R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd12(&self) -> Bphlcd12R {
        Bphlcd12R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd12(&mut self) -> Bpalcd12W<LcdWf12Spec> {
        Bpalcd12W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd12(&mut self) -> Bpblcd12W<LcdWf12Spec> {
        Bpblcd12W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd12(&mut self) -> Bpclcd12W<LcdWf12Spec> {
        Bpclcd12W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd12(&mut self) -> Bpdlcd12W<LcdWf12Spec> {
        Bpdlcd12W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd12(&mut self) -> Bpelcd12W<LcdWf12Spec> {
        Bpelcd12W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd12(&mut self) -> Bpflcd12W<LcdWf12Spec> {
        Bpflcd12W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd12(&mut self) -> Bpglcd12W<LcdWf12Spec> {
        Bpglcd12W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd12(&mut self) -> Bphlcd12W<LcdWf12Spec> {
        Bphlcd12W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 12.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf12Spec;
impl crate::RegisterSpec for LcdWf12Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf12::R`](R) reader structure"]
impl crate::Readable for LcdWf12Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf12::W`](W) writer structure"]
impl crate::Writable for LcdWf12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF12 to value 0"]
impl crate::Resettable for LcdWf12Spec {
    const RESET_VALUE: u8 = 0;
}
