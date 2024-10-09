#[doc = "Register `WF48` reader"]
pub type R = crate::R<LcdWf48Spec>;
#[doc = "Register `WF48` writer"]
pub type W = crate::W<LcdWf48Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd48 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd48> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD48` reader - no description available"]
pub type Bpalcd48R = crate::BitReader<Bpalcd48>;
impl Bpalcd48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd48 {
        match self.bits {
            false => Bpalcd48::B0,
            true => Bpalcd48::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd48::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd48::B1
    }
}
#[doc = "Field `BPALCD48` writer - no description available"]
pub type Bpalcd48W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd48>;
impl<'a, REG> Bpalcd48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd48::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd48::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd48 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd48> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD48` reader - no description available"]
pub type Bpblcd48R = crate::BitReader<Bpblcd48>;
impl Bpblcd48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd48 {
        match self.bits {
            false => Bpblcd48::B0,
            true => Bpblcd48::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd48::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd48::B1
    }
}
#[doc = "Field `BPBLCD48` writer - no description available"]
pub type Bpblcd48W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd48>;
impl<'a, REG> Bpblcd48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd48::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd48::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd48 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd48> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD48` reader - no description available"]
pub type Bpclcd48R = crate::BitReader<Bpclcd48>;
impl Bpclcd48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd48 {
        match self.bits {
            false => Bpclcd48::B0,
            true => Bpclcd48::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd48::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd48::B1
    }
}
#[doc = "Field `BPCLCD48` writer - no description available"]
pub type Bpclcd48W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd48>;
impl<'a, REG> Bpclcd48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd48::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd48::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd48 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd48> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD48` reader - no description available"]
pub type Bpdlcd48R = crate::BitReader<Bpdlcd48>;
impl Bpdlcd48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd48 {
        match self.bits {
            false => Bpdlcd48::B0,
            true => Bpdlcd48::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd48::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd48::B1
    }
}
#[doc = "Field `BPDLCD48` writer - no description available"]
pub type Bpdlcd48W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd48>;
impl<'a, REG> Bpdlcd48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd48::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd48::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd48 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd48> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD48` reader - no description available"]
pub type Bpelcd48R = crate::BitReader<Bpelcd48>;
impl Bpelcd48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd48 {
        match self.bits {
            false => Bpelcd48::B0,
            true => Bpelcd48::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd48::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd48::B1
    }
}
#[doc = "Field `BPELCD48` writer - no description available"]
pub type Bpelcd48W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd48>;
impl<'a, REG> Bpelcd48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd48::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd48::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd48 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd48> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD48` reader - no description available"]
pub type Bpflcd48R = crate::BitReader<Bpflcd48>;
impl Bpflcd48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd48 {
        match self.bits {
            false => Bpflcd48::B0,
            true => Bpflcd48::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd48::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd48::B1
    }
}
#[doc = "Field `BPFLCD48` writer - no description available"]
pub type Bpflcd48W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd48>;
impl<'a, REG> Bpflcd48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd48::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd48::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd48 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd48> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD48` reader - no description available"]
pub type Bpglcd48R = crate::BitReader<Bpglcd48>;
impl Bpglcd48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd48 {
        match self.bits {
            false => Bpglcd48::B0,
            true => Bpglcd48::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd48::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd48::B1
    }
}
#[doc = "Field `BPGLCD48` writer - no description available"]
pub type Bpglcd48W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd48>;
impl<'a, REG> Bpglcd48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd48::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd48::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd48 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd48> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd48) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD48` reader - no description available"]
pub type Bphlcd48R = crate::BitReader<Bphlcd48>;
impl Bphlcd48R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd48 {
        match self.bits {
            false => Bphlcd48::B0,
            true => Bphlcd48::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd48::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd48::B1
    }
}
#[doc = "Field `BPHLCD48` writer - no description available"]
pub type Bphlcd48W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd48>;
impl<'a, REG> Bphlcd48W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd48::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd48::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd48(&self) -> Bpalcd48R {
        Bpalcd48R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd48(&self) -> Bpblcd48R {
        Bpblcd48R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd48(&self) -> Bpclcd48R {
        Bpclcd48R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd48(&self) -> Bpdlcd48R {
        Bpdlcd48R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd48(&self) -> Bpelcd48R {
        Bpelcd48R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd48(&self) -> Bpflcd48R {
        Bpflcd48R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd48(&self) -> Bpglcd48R {
        Bpglcd48R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd48(&self) -> Bphlcd48R {
        Bphlcd48R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd48(&mut self) -> Bpalcd48W<LcdWf48Spec> {
        Bpalcd48W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd48(&mut self) -> Bpblcd48W<LcdWf48Spec> {
        Bpblcd48W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd48(&mut self) -> Bpclcd48W<LcdWf48Spec> {
        Bpclcd48W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd48(&mut self) -> Bpdlcd48W<LcdWf48Spec> {
        Bpdlcd48W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd48(&mut self) -> Bpelcd48W<LcdWf48Spec> {
        Bpelcd48W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd48(&mut self) -> Bpflcd48W<LcdWf48Spec> {
        Bpflcd48W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd48(&mut self) -> Bpglcd48W<LcdWf48Spec> {
        Bpglcd48W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd48(&mut self) -> Bphlcd48W<LcdWf48Spec> {
        Bphlcd48W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 48.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf48Spec;
impl crate::RegisterSpec for LcdWf48Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf48::R`](R) reader structure"]
impl crate::Readable for LcdWf48Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf48::W`](W) writer structure"]
impl crate::Writable for LcdWf48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF48 to value 0"]
impl crate::Resettable for LcdWf48Spec {
    const RESET_VALUE: u8 = 0;
}
