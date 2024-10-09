#[doc = "Register `WF32` reader"]
pub type R = crate::R<LcdWf32Spec>;
#[doc = "Register `WF32` writer"]
pub type W = crate::W<LcdWf32Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd32 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd32> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD32` reader - no description available"]
pub type Bpalcd32R = crate::BitReader<Bpalcd32>;
impl Bpalcd32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd32 {
        match self.bits {
            false => Bpalcd32::B0,
            true => Bpalcd32::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd32::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd32::B1
    }
}
#[doc = "Field `BPALCD32` writer - no description available"]
pub type Bpalcd32W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd32>;
impl<'a, REG> Bpalcd32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd32::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd32::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd32 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd32> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD32` reader - no description available"]
pub type Bpblcd32R = crate::BitReader<Bpblcd32>;
impl Bpblcd32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd32 {
        match self.bits {
            false => Bpblcd32::B0,
            true => Bpblcd32::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd32::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd32::B1
    }
}
#[doc = "Field `BPBLCD32` writer - no description available"]
pub type Bpblcd32W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd32>;
impl<'a, REG> Bpblcd32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd32::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd32::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd32 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd32> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD32` reader - no description available"]
pub type Bpclcd32R = crate::BitReader<Bpclcd32>;
impl Bpclcd32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd32 {
        match self.bits {
            false => Bpclcd32::B0,
            true => Bpclcd32::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd32::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd32::B1
    }
}
#[doc = "Field `BPCLCD32` writer - no description available"]
pub type Bpclcd32W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd32>;
impl<'a, REG> Bpclcd32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd32::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd32::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd32 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd32> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD32` reader - no description available"]
pub type Bpdlcd32R = crate::BitReader<Bpdlcd32>;
impl Bpdlcd32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd32 {
        match self.bits {
            false => Bpdlcd32::B0,
            true => Bpdlcd32::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd32::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd32::B1
    }
}
#[doc = "Field `BPDLCD32` writer - no description available"]
pub type Bpdlcd32W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd32>;
impl<'a, REG> Bpdlcd32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd32::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd32::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd32 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd32> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD32` reader - no description available"]
pub type Bpelcd32R = crate::BitReader<Bpelcd32>;
impl Bpelcd32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd32 {
        match self.bits {
            false => Bpelcd32::B0,
            true => Bpelcd32::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd32::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd32::B1
    }
}
#[doc = "Field `BPELCD32` writer - no description available"]
pub type Bpelcd32W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd32>;
impl<'a, REG> Bpelcd32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd32::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd32::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd32 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd32> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD32` reader - no description available"]
pub type Bpflcd32R = crate::BitReader<Bpflcd32>;
impl Bpflcd32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd32 {
        match self.bits {
            false => Bpflcd32::B0,
            true => Bpflcd32::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd32::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd32::B1
    }
}
#[doc = "Field `BPFLCD32` writer - no description available"]
pub type Bpflcd32W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd32>;
impl<'a, REG> Bpflcd32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd32::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd32::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd32 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd32> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD32` reader - no description available"]
pub type Bpglcd32R = crate::BitReader<Bpglcd32>;
impl Bpglcd32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd32 {
        match self.bits {
            false => Bpglcd32::B0,
            true => Bpglcd32::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd32::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd32::B1
    }
}
#[doc = "Field `BPGLCD32` writer - no description available"]
pub type Bpglcd32W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd32>;
impl<'a, REG> Bpglcd32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd32::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd32::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd32 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd32> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD32` reader - no description available"]
pub type Bphlcd32R = crate::BitReader<Bphlcd32>;
impl Bphlcd32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd32 {
        match self.bits {
            false => Bphlcd32::B0,
            true => Bphlcd32::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd32::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd32::B1
    }
}
#[doc = "Field `BPHLCD32` writer - no description available"]
pub type Bphlcd32W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd32>;
impl<'a, REG> Bphlcd32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd32::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd32::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd32(&self) -> Bpalcd32R {
        Bpalcd32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd32(&self) -> Bpblcd32R {
        Bpblcd32R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd32(&self) -> Bpclcd32R {
        Bpclcd32R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd32(&self) -> Bpdlcd32R {
        Bpdlcd32R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd32(&self) -> Bpelcd32R {
        Bpelcd32R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd32(&self) -> Bpflcd32R {
        Bpflcd32R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd32(&self) -> Bpglcd32R {
        Bpglcd32R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd32(&self) -> Bphlcd32R {
        Bphlcd32R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd32(&mut self) -> Bpalcd32W<LcdWf32Spec> {
        Bpalcd32W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd32(&mut self) -> Bpblcd32W<LcdWf32Spec> {
        Bpblcd32W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd32(&mut self) -> Bpclcd32W<LcdWf32Spec> {
        Bpclcd32W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd32(&mut self) -> Bpdlcd32W<LcdWf32Spec> {
        Bpdlcd32W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd32(&mut self) -> Bpelcd32W<LcdWf32Spec> {
        Bpelcd32W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd32(&mut self) -> Bpflcd32W<LcdWf32Spec> {
        Bpflcd32W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd32(&mut self) -> Bpglcd32W<LcdWf32Spec> {
        Bpglcd32W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd32(&mut self) -> Bphlcd32W<LcdWf32Spec> {
        Bphlcd32W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 32.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf32Spec;
impl crate::RegisterSpec for LcdWf32Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf32::R`](R) reader structure"]
impl crate::Readable for LcdWf32Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf32::W`](W) writer structure"]
impl crate::Writable for LcdWf32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF32 to value 0"]
impl crate::Resettable for LcdWf32Spec {
    const RESET_VALUE: u8 = 0;
}
