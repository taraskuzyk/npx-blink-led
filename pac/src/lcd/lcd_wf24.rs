#[doc = "Register `WF24` reader"]
pub type R = crate::R<LcdWf24Spec>;
#[doc = "Register `WF24` writer"]
pub type W = crate::W<LcdWf24Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd24 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd24> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD24` reader - no description available"]
pub type Bpalcd24R = crate::BitReader<Bpalcd24>;
impl Bpalcd24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd24 {
        match self.bits {
            false => Bpalcd24::B0,
            true => Bpalcd24::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd24::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd24::B1
    }
}
#[doc = "Field `BPALCD24` writer - no description available"]
pub type Bpalcd24W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd24>;
impl<'a, REG> Bpalcd24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd24::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd24::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd24 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd24> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD24` reader - no description available"]
pub type Bpblcd24R = crate::BitReader<Bpblcd24>;
impl Bpblcd24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd24 {
        match self.bits {
            false => Bpblcd24::B0,
            true => Bpblcd24::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd24::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd24::B1
    }
}
#[doc = "Field `BPBLCD24` writer - no description available"]
pub type Bpblcd24W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd24>;
impl<'a, REG> Bpblcd24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd24::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd24::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd24 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd24> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD24` reader - no description available"]
pub type Bpclcd24R = crate::BitReader<Bpclcd24>;
impl Bpclcd24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd24 {
        match self.bits {
            false => Bpclcd24::B0,
            true => Bpclcd24::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd24::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd24::B1
    }
}
#[doc = "Field `BPCLCD24` writer - no description available"]
pub type Bpclcd24W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd24>;
impl<'a, REG> Bpclcd24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd24::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd24::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd24 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd24> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD24` reader - no description available"]
pub type Bpdlcd24R = crate::BitReader<Bpdlcd24>;
impl Bpdlcd24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd24 {
        match self.bits {
            false => Bpdlcd24::B0,
            true => Bpdlcd24::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd24::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd24::B1
    }
}
#[doc = "Field `BPDLCD24` writer - no description available"]
pub type Bpdlcd24W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd24>;
impl<'a, REG> Bpdlcd24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd24::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd24::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd24 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd24> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD24` reader - no description available"]
pub type Bpelcd24R = crate::BitReader<Bpelcd24>;
impl Bpelcd24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd24 {
        match self.bits {
            false => Bpelcd24::B0,
            true => Bpelcd24::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd24::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd24::B1
    }
}
#[doc = "Field `BPELCD24` writer - no description available"]
pub type Bpelcd24W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd24>;
impl<'a, REG> Bpelcd24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd24::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd24::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd24 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd24> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD24` reader - no description available"]
pub type Bpflcd24R = crate::BitReader<Bpflcd24>;
impl Bpflcd24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd24 {
        match self.bits {
            false => Bpflcd24::B0,
            true => Bpflcd24::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd24::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd24::B1
    }
}
#[doc = "Field `BPFLCD24` writer - no description available"]
pub type Bpflcd24W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd24>;
impl<'a, REG> Bpflcd24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd24::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd24::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd24 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd24> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD24` reader - no description available"]
pub type Bpglcd24R = crate::BitReader<Bpglcd24>;
impl Bpglcd24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd24 {
        match self.bits {
            false => Bpglcd24::B0,
            true => Bpglcd24::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd24::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd24::B1
    }
}
#[doc = "Field `BPGLCD24` writer - no description available"]
pub type Bpglcd24W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd24>;
impl<'a, REG> Bpglcd24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd24::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd24::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd24 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd24> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD24` reader - no description available"]
pub type Bphlcd24R = crate::BitReader<Bphlcd24>;
impl Bphlcd24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd24 {
        match self.bits {
            false => Bphlcd24::B0,
            true => Bphlcd24::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd24::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd24::B1
    }
}
#[doc = "Field `BPHLCD24` writer - no description available"]
pub type Bphlcd24W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd24>;
impl<'a, REG> Bphlcd24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd24::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd24::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd24(&self) -> Bpalcd24R {
        Bpalcd24R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd24(&self) -> Bpblcd24R {
        Bpblcd24R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd24(&self) -> Bpclcd24R {
        Bpclcd24R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd24(&self) -> Bpdlcd24R {
        Bpdlcd24R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd24(&self) -> Bpelcd24R {
        Bpelcd24R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd24(&self) -> Bpflcd24R {
        Bpflcd24R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd24(&self) -> Bpglcd24R {
        Bpglcd24R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd24(&self) -> Bphlcd24R {
        Bphlcd24R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd24(&mut self) -> Bpalcd24W<LcdWf24Spec> {
        Bpalcd24W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd24(&mut self) -> Bpblcd24W<LcdWf24Spec> {
        Bpblcd24W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd24(&mut self) -> Bpclcd24W<LcdWf24Spec> {
        Bpclcd24W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd24(&mut self) -> Bpdlcd24W<LcdWf24Spec> {
        Bpdlcd24W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd24(&mut self) -> Bpelcd24W<LcdWf24Spec> {
        Bpelcd24W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd24(&mut self) -> Bpflcd24W<LcdWf24Spec> {
        Bpflcd24W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd24(&mut self) -> Bpglcd24W<LcdWf24Spec> {
        Bpglcd24W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd24(&mut self) -> Bphlcd24W<LcdWf24Spec> {
        Bphlcd24W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 24.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf24Spec;
impl crate::RegisterSpec for LcdWf24Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf24::R`](R) reader structure"]
impl crate::Readable for LcdWf24Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf24::W`](W) writer structure"]
impl crate::Writable for LcdWf24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF24 to value 0"]
impl crate::Resettable for LcdWf24Spec {
    const RESET_VALUE: u8 = 0;
}
