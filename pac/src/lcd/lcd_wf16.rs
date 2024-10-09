#[doc = "Register `WF16` reader"]
pub type R = crate::R<LcdWf16Spec>;
#[doc = "Register `WF16` writer"]
pub type W = crate::W<LcdWf16Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd16 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd16> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD16` reader - no description available"]
pub type Bpalcd16R = crate::BitReader<Bpalcd16>;
impl Bpalcd16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd16 {
        match self.bits {
            false => Bpalcd16::B0,
            true => Bpalcd16::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd16::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd16::B1
    }
}
#[doc = "Field `BPALCD16` writer - no description available"]
pub type Bpalcd16W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd16>;
impl<'a, REG> Bpalcd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd16::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd16::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd16 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd16> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD16` reader - no description available"]
pub type Bpblcd16R = crate::BitReader<Bpblcd16>;
impl Bpblcd16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd16 {
        match self.bits {
            false => Bpblcd16::B0,
            true => Bpblcd16::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd16::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd16::B1
    }
}
#[doc = "Field `BPBLCD16` writer - no description available"]
pub type Bpblcd16W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd16>;
impl<'a, REG> Bpblcd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd16::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd16::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd16 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd16> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD16` reader - no description available"]
pub type Bpclcd16R = crate::BitReader<Bpclcd16>;
impl Bpclcd16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd16 {
        match self.bits {
            false => Bpclcd16::B0,
            true => Bpclcd16::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd16::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd16::B1
    }
}
#[doc = "Field `BPCLCD16` writer - no description available"]
pub type Bpclcd16W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd16>;
impl<'a, REG> Bpclcd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd16::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd16::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd16 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd16> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD16` reader - no description available"]
pub type Bpdlcd16R = crate::BitReader<Bpdlcd16>;
impl Bpdlcd16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd16 {
        match self.bits {
            false => Bpdlcd16::B0,
            true => Bpdlcd16::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd16::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd16::B1
    }
}
#[doc = "Field `BPDLCD16` writer - no description available"]
pub type Bpdlcd16W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd16>;
impl<'a, REG> Bpdlcd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd16::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd16::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd16 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd16> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD16` reader - no description available"]
pub type Bpelcd16R = crate::BitReader<Bpelcd16>;
impl Bpelcd16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd16 {
        match self.bits {
            false => Bpelcd16::B0,
            true => Bpelcd16::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd16::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd16::B1
    }
}
#[doc = "Field `BPELCD16` writer - no description available"]
pub type Bpelcd16W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd16>;
impl<'a, REG> Bpelcd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd16::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd16::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd16 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd16> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD16` reader - no description available"]
pub type Bpflcd16R = crate::BitReader<Bpflcd16>;
impl Bpflcd16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd16 {
        match self.bits {
            false => Bpflcd16::B0,
            true => Bpflcd16::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd16::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd16::B1
    }
}
#[doc = "Field `BPFLCD16` writer - no description available"]
pub type Bpflcd16W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd16>;
impl<'a, REG> Bpflcd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd16::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd16::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd16 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd16> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD16` reader - no description available"]
pub type Bpglcd16R = crate::BitReader<Bpglcd16>;
impl Bpglcd16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd16 {
        match self.bits {
            false => Bpglcd16::B0,
            true => Bpglcd16::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd16::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd16::B1
    }
}
#[doc = "Field `BPGLCD16` writer - no description available"]
pub type Bpglcd16W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd16>;
impl<'a, REG> Bpglcd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd16::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd16::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd16 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd16> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD16` reader - no description available"]
pub type Bphlcd16R = crate::BitReader<Bphlcd16>;
impl Bphlcd16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd16 {
        match self.bits {
            false => Bphlcd16::B0,
            true => Bphlcd16::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd16::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd16::B1
    }
}
#[doc = "Field `BPHLCD16` writer - no description available"]
pub type Bphlcd16W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd16>;
impl<'a, REG> Bphlcd16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd16::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd16::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd16(&self) -> Bpalcd16R {
        Bpalcd16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd16(&self) -> Bpblcd16R {
        Bpblcd16R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd16(&self) -> Bpclcd16R {
        Bpclcd16R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd16(&self) -> Bpdlcd16R {
        Bpdlcd16R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd16(&self) -> Bpelcd16R {
        Bpelcd16R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd16(&self) -> Bpflcd16R {
        Bpflcd16R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd16(&self) -> Bpglcd16R {
        Bpglcd16R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd16(&self) -> Bphlcd16R {
        Bphlcd16R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd16(&mut self) -> Bpalcd16W<LcdWf16Spec> {
        Bpalcd16W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd16(&mut self) -> Bpblcd16W<LcdWf16Spec> {
        Bpblcd16W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd16(&mut self) -> Bpclcd16W<LcdWf16Spec> {
        Bpclcd16W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd16(&mut self) -> Bpdlcd16W<LcdWf16Spec> {
        Bpdlcd16W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd16(&mut self) -> Bpelcd16W<LcdWf16Spec> {
        Bpelcd16W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd16(&mut self) -> Bpflcd16W<LcdWf16Spec> {
        Bpflcd16W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd16(&mut self) -> Bpglcd16W<LcdWf16Spec> {
        Bpglcd16W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd16(&mut self) -> Bphlcd16W<LcdWf16Spec> {
        Bphlcd16W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 16.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf16Spec;
impl crate::RegisterSpec for LcdWf16Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf16::R`](R) reader structure"]
impl crate::Readable for LcdWf16Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf16::W`](W) writer structure"]
impl crate::Writable for LcdWf16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF16 to value 0"]
impl crate::Resettable for LcdWf16Spec {
    const RESET_VALUE: u8 = 0;
}
