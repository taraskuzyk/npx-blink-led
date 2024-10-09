#[doc = "Register `WF44` reader"]
pub type R = crate::R<LcdWf44Spec>;
#[doc = "Register `WF44` writer"]
pub type W = crate::W<LcdWf44Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd44 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd44> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD44` reader - no description available"]
pub type Bpalcd44R = crate::BitReader<Bpalcd44>;
impl Bpalcd44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd44 {
        match self.bits {
            false => Bpalcd44::B0,
            true => Bpalcd44::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd44::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd44::B1
    }
}
#[doc = "Field `BPALCD44` writer - no description available"]
pub type Bpalcd44W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd44>;
impl<'a, REG> Bpalcd44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd44::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd44::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd44 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd44> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD44` reader - no description available"]
pub type Bpblcd44R = crate::BitReader<Bpblcd44>;
impl Bpblcd44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd44 {
        match self.bits {
            false => Bpblcd44::B0,
            true => Bpblcd44::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd44::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd44::B1
    }
}
#[doc = "Field `BPBLCD44` writer - no description available"]
pub type Bpblcd44W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd44>;
impl<'a, REG> Bpblcd44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd44::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd44::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd44 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd44> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD44` reader - no description available"]
pub type Bpclcd44R = crate::BitReader<Bpclcd44>;
impl Bpclcd44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd44 {
        match self.bits {
            false => Bpclcd44::B0,
            true => Bpclcd44::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd44::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd44::B1
    }
}
#[doc = "Field `BPCLCD44` writer - no description available"]
pub type Bpclcd44W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd44>;
impl<'a, REG> Bpclcd44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd44::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd44::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd44 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd44> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD44` reader - no description available"]
pub type Bpdlcd44R = crate::BitReader<Bpdlcd44>;
impl Bpdlcd44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd44 {
        match self.bits {
            false => Bpdlcd44::B0,
            true => Bpdlcd44::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd44::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd44::B1
    }
}
#[doc = "Field `BPDLCD44` writer - no description available"]
pub type Bpdlcd44W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd44>;
impl<'a, REG> Bpdlcd44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd44::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd44::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd44 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd44> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD44` reader - no description available"]
pub type Bpelcd44R = crate::BitReader<Bpelcd44>;
impl Bpelcd44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd44 {
        match self.bits {
            false => Bpelcd44::B0,
            true => Bpelcd44::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd44::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd44::B1
    }
}
#[doc = "Field `BPELCD44` writer - no description available"]
pub type Bpelcd44W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd44>;
impl<'a, REG> Bpelcd44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd44::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd44::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd44 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd44> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD44` reader - no description available"]
pub type Bpflcd44R = crate::BitReader<Bpflcd44>;
impl Bpflcd44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd44 {
        match self.bits {
            false => Bpflcd44::B0,
            true => Bpflcd44::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd44::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd44::B1
    }
}
#[doc = "Field `BPFLCD44` writer - no description available"]
pub type Bpflcd44W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd44>;
impl<'a, REG> Bpflcd44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd44::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd44::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd44 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd44> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD44` reader - no description available"]
pub type Bpglcd44R = crate::BitReader<Bpglcd44>;
impl Bpglcd44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd44 {
        match self.bits {
            false => Bpglcd44::B0,
            true => Bpglcd44::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd44::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd44::B1
    }
}
#[doc = "Field `BPGLCD44` writer - no description available"]
pub type Bpglcd44W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd44>;
impl<'a, REG> Bpglcd44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd44::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd44::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd44 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd44> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd44) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD44` reader - no description available"]
pub type Bphlcd44R = crate::BitReader<Bphlcd44>;
impl Bphlcd44R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd44 {
        match self.bits {
            false => Bphlcd44::B0,
            true => Bphlcd44::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd44::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd44::B1
    }
}
#[doc = "Field `BPHLCD44` writer - no description available"]
pub type Bphlcd44W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd44>;
impl<'a, REG> Bphlcd44W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd44::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd44::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd44(&self) -> Bpalcd44R {
        Bpalcd44R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd44(&self) -> Bpblcd44R {
        Bpblcd44R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd44(&self) -> Bpclcd44R {
        Bpclcd44R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd44(&self) -> Bpdlcd44R {
        Bpdlcd44R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd44(&self) -> Bpelcd44R {
        Bpelcd44R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd44(&self) -> Bpflcd44R {
        Bpflcd44R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd44(&self) -> Bpglcd44R {
        Bpglcd44R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd44(&self) -> Bphlcd44R {
        Bphlcd44R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd44(&mut self) -> Bpalcd44W<LcdWf44Spec> {
        Bpalcd44W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd44(&mut self) -> Bpblcd44W<LcdWf44Spec> {
        Bpblcd44W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd44(&mut self) -> Bpclcd44W<LcdWf44Spec> {
        Bpclcd44W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd44(&mut self) -> Bpdlcd44W<LcdWf44Spec> {
        Bpdlcd44W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd44(&mut self) -> Bpelcd44W<LcdWf44Spec> {
        Bpelcd44W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd44(&mut self) -> Bpflcd44W<LcdWf44Spec> {
        Bpflcd44W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd44(&mut self) -> Bpglcd44W<LcdWf44Spec> {
        Bpglcd44W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd44(&mut self) -> Bphlcd44W<LcdWf44Spec> {
        Bphlcd44W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 44.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf44Spec;
impl crate::RegisterSpec for LcdWf44Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf44::R`](R) reader structure"]
impl crate::Readable for LcdWf44Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf44::W`](W) writer structure"]
impl crate::Writable for LcdWf44Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF44 to value 0"]
impl crate::Resettable for LcdWf44Spec {
    const RESET_VALUE: u8 = 0;
}
