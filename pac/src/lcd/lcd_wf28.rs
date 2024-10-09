#[doc = "Register `WF28` reader"]
pub type R = crate::R<LcdWf28Spec>;
#[doc = "Register `WF28` writer"]
pub type W = crate::W<LcdWf28Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd28 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd28> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD28` reader - no description available"]
pub type Bpalcd28R = crate::BitReader<Bpalcd28>;
impl Bpalcd28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd28 {
        match self.bits {
            false => Bpalcd28::B0,
            true => Bpalcd28::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd28::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd28::B1
    }
}
#[doc = "Field `BPALCD28` writer - no description available"]
pub type Bpalcd28W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd28>;
impl<'a, REG> Bpalcd28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd28::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd28::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd28 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd28> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD28` reader - no description available"]
pub type Bpblcd28R = crate::BitReader<Bpblcd28>;
impl Bpblcd28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd28 {
        match self.bits {
            false => Bpblcd28::B0,
            true => Bpblcd28::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd28::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd28::B1
    }
}
#[doc = "Field `BPBLCD28` writer - no description available"]
pub type Bpblcd28W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd28>;
impl<'a, REG> Bpblcd28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd28::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd28::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd28 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd28> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD28` reader - no description available"]
pub type Bpclcd28R = crate::BitReader<Bpclcd28>;
impl Bpclcd28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd28 {
        match self.bits {
            false => Bpclcd28::B0,
            true => Bpclcd28::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd28::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd28::B1
    }
}
#[doc = "Field `BPCLCD28` writer - no description available"]
pub type Bpclcd28W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd28>;
impl<'a, REG> Bpclcd28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd28::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd28::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd28 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd28> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD28` reader - no description available"]
pub type Bpdlcd28R = crate::BitReader<Bpdlcd28>;
impl Bpdlcd28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd28 {
        match self.bits {
            false => Bpdlcd28::B0,
            true => Bpdlcd28::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd28::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd28::B1
    }
}
#[doc = "Field `BPDLCD28` writer - no description available"]
pub type Bpdlcd28W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd28>;
impl<'a, REG> Bpdlcd28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd28::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd28::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd28 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd28> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD28` reader - no description available"]
pub type Bpelcd28R = crate::BitReader<Bpelcd28>;
impl Bpelcd28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd28 {
        match self.bits {
            false => Bpelcd28::B0,
            true => Bpelcd28::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd28::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd28::B1
    }
}
#[doc = "Field `BPELCD28` writer - no description available"]
pub type Bpelcd28W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd28>;
impl<'a, REG> Bpelcd28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd28::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd28::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd28 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd28> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD28` reader - no description available"]
pub type Bpflcd28R = crate::BitReader<Bpflcd28>;
impl Bpflcd28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd28 {
        match self.bits {
            false => Bpflcd28::B0,
            true => Bpflcd28::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd28::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd28::B1
    }
}
#[doc = "Field `BPFLCD28` writer - no description available"]
pub type Bpflcd28W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd28>;
impl<'a, REG> Bpflcd28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd28::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd28::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd28 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd28> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD28` reader - no description available"]
pub type Bpglcd28R = crate::BitReader<Bpglcd28>;
impl Bpglcd28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd28 {
        match self.bits {
            false => Bpglcd28::B0,
            true => Bpglcd28::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd28::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd28::B1
    }
}
#[doc = "Field `BPGLCD28` writer - no description available"]
pub type Bpglcd28W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd28>;
impl<'a, REG> Bpglcd28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd28::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd28::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd28 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd28> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD28` reader - no description available"]
pub type Bphlcd28R = crate::BitReader<Bphlcd28>;
impl Bphlcd28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd28 {
        match self.bits {
            false => Bphlcd28::B0,
            true => Bphlcd28::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd28::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd28::B1
    }
}
#[doc = "Field `BPHLCD28` writer - no description available"]
pub type Bphlcd28W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd28>;
impl<'a, REG> Bphlcd28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd28::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd28::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd28(&self) -> Bpalcd28R {
        Bpalcd28R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd28(&self) -> Bpblcd28R {
        Bpblcd28R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd28(&self) -> Bpclcd28R {
        Bpclcd28R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd28(&self) -> Bpdlcd28R {
        Bpdlcd28R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd28(&self) -> Bpelcd28R {
        Bpelcd28R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd28(&self) -> Bpflcd28R {
        Bpflcd28R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd28(&self) -> Bpglcd28R {
        Bpglcd28R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd28(&self) -> Bphlcd28R {
        Bphlcd28R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd28(&mut self) -> Bpalcd28W<LcdWf28Spec> {
        Bpalcd28W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd28(&mut self) -> Bpblcd28W<LcdWf28Spec> {
        Bpblcd28W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd28(&mut self) -> Bpclcd28W<LcdWf28Spec> {
        Bpclcd28W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd28(&mut self) -> Bpdlcd28W<LcdWf28Spec> {
        Bpdlcd28W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd28(&mut self) -> Bpelcd28W<LcdWf28Spec> {
        Bpelcd28W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd28(&mut self) -> Bpflcd28W<LcdWf28Spec> {
        Bpflcd28W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd28(&mut self) -> Bpglcd28W<LcdWf28Spec> {
        Bpglcd28W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd28(&mut self) -> Bphlcd28W<LcdWf28Spec> {
        Bphlcd28W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 28.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf28Spec;
impl crate::RegisterSpec for LcdWf28Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf28::R`](R) reader structure"]
impl crate::Readable for LcdWf28Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf28::W`](W) writer structure"]
impl crate::Writable for LcdWf28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF28 to value 0"]
impl crate::Resettable for LcdWf28Spec {
    const RESET_VALUE: u8 = 0;
}
