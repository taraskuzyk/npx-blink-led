#[doc = "Register `WF36` reader"]
pub type R = crate::R<LcdWf36Spec>;
#[doc = "Register `WF36` writer"]
pub type W = crate::W<LcdWf36Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd36 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd36> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD36` reader - no description available"]
pub type Bpalcd36R = crate::BitReader<Bpalcd36>;
impl Bpalcd36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd36 {
        match self.bits {
            false => Bpalcd36::B0,
            true => Bpalcd36::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd36::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd36::B1
    }
}
#[doc = "Field `BPALCD36` writer - no description available"]
pub type Bpalcd36W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd36>;
impl<'a, REG> Bpalcd36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd36::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd36::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd36 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd36> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD36` reader - no description available"]
pub type Bpblcd36R = crate::BitReader<Bpblcd36>;
impl Bpblcd36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd36 {
        match self.bits {
            false => Bpblcd36::B0,
            true => Bpblcd36::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd36::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd36::B1
    }
}
#[doc = "Field `BPBLCD36` writer - no description available"]
pub type Bpblcd36W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd36>;
impl<'a, REG> Bpblcd36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd36::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd36::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd36 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd36> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD36` reader - no description available"]
pub type Bpclcd36R = crate::BitReader<Bpclcd36>;
impl Bpclcd36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd36 {
        match self.bits {
            false => Bpclcd36::B0,
            true => Bpclcd36::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd36::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd36::B1
    }
}
#[doc = "Field `BPCLCD36` writer - no description available"]
pub type Bpclcd36W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd36>;
impl<'a, REG> Bpclcd36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd36::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd36::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd36 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd36> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD36` reader - no description available"]
pub type Bpdlcd36R = crate::BitReader<Bpdlcd36>;
impl Bpdlcd36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd36 {
        match self.bits {
            false => Bpdlcd36::B0,
            true => Bpdlcd36::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd36::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd36::B1
    }
}
#[doc = "Field `BPDLCD36` writer - no description available"]
pub type Bpdlcd36W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd36>;
impl<'a, REG> Bpdlcd36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd36::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd36::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd36 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd36> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD36` reader - no description available"]
pub type Bpelcd36R = crate::BitReader<Bpelcd36>;
impl Bpelcd36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd36 {
        match self.bits {
            false => Bpelcd36::B0,
            true => Bpelcd36::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd36::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd36::B1
    }
}
#[doc = "Field `BPELCD36` writer - no description available"]
pub type Bpelcd36W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd36>;
impl<'a, REG> Bpelcd36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd36::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd36::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd36 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd36> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD36` reader - no description available"]
pub type Bpflcd36R = crate::BitReader<Bpflcd36>;
impl Bpflcd36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd36 {
        match self.bits {
            false => Bpflcd36::B0,
            true => Bpflcd36::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd36::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd36::B1
    }
}
#[doc = "Field `BPFLCD36` writer - no description available"]
pub type Bpflcd36W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd36>;
impl<'a, REG> Bpflcd36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd36::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd36::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd36 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd36> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD36` reader - no description available"]
pub type Bpglcd36R = crate::BitReader<Bpglcd36>;
impl Bpglcd36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd36 {
        match self.bits {
            false => Bpglcd36::B0,
            true => Bpglcd36::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd36::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd36::B1
    }
}
#[doc = "Field `BPGLCD36` writer - no description available"]
pub type Bpglcd36W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd36>;
impl<'a, REG> Bpglcd36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd36::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd36::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd36 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd36> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd36) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD36` reader - no description available"]
pub type Bphlcd36R = crate::BitReader<Bphlcd36>;
impl Bphlcd36R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd36 {
        match self.bits {
            false => Bphlcd36::B0,
            true => Bphlcd36::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd36::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd36::B1
    }
}
#[doc = "Field `BPHLCD36` writer - no description available"]
pub type Bphlcd36W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd36>;
impl<'a, REG> Bphlcd36W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd36::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd36::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd36(&self) -> Bpalcd36R {
        Bpalcd36R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd36(&self) -> Bpblcd36R {
        Bpblcd36R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd36(&self) -> Bpclcd36R {
        Bpclcd36R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd36(&self) -> Bpdlcd36R {
        Bpdlcd36R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd36(&self) -> Bpelcd36R {
        Bpelcd36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd36(&self) -> Bpflcd36R {
        Bpflcd36R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd36(&self) -> Bpglcd36R {
        Bpglcd36R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd36(&self) -> Bphlcd36R {
        Bphlcd36R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd36(&mut self) -> Bpalcd36W<LcdWf36Spec> {
        Bpalcd36W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd36(&mut self) -> Bpblcd36W<LcdWf36Spec> {
        Bpblcd36W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd36(&mut self) -> Bpclcd36W<LcdWf36Spec> {
        Bpclcd36W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd36(&mut self) -> Bpdlcd36W<LcdWf36Spec> {
        Bpdlcd36W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd36(&mut self) -> Bpelcd36W<LcdWf36Spec> {
        Bpelcd36W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd36(&mut self) -> Bpflcd36W<LcdWf36Spec> {
        Bpflcd36W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd36(&mut self) -> Bpglcd36W<LcdWf36Spec> {
        Bpglcd36W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd36(&mut self) -> Bphlcd36W<LcdWf36Spec> {
        Bphlcd36W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 36.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf36Spec;
impl crate::RegisterSpec for LcdWf36Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf36::R`](R) reader structure"]
impl crate::Readable for LcdWf36Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf36::W`](W) writer structure"]
impl crate::Writable for LcdWf36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF36 to value 0"]
impl crate::Resettable for LcdWf36Spec {
    const RESET_VALUE: u8 = 0;
}
