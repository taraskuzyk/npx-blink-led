#[doc = "Register `WF60` reader"]
pub type R = crate::R<LcdWf60Spec>;
#[doc = "Register `WF60` writer"]
pub type W = crate::W<LcdWf60Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd60 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd60> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD60` reader - no description available"]
pub type Bpalcd60R = crate::BitReader<Bpalcd60>;
impl Bpalcd60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd60 {
        match self.bits {
            false => Bpalcd60::B0,
            true => Bpalcd60::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd60::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd60::B1
    }
}
#[doc = "Field `BPALCD60` writer - no description available"]
pub type Bpalcd60W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd60>;
impl<'a, REG> Bpalcd60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd60::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd60::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd60 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd60> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD60` reader - no description available"]
pub type Bpblcd60R = crate::BitReader<Bpblcd60>;
impl Bpblcd60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd60 {
        match self.bits {
            false => Bpblcd60::B0,
            true => Bpblcd60::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd60::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd60::B1
    }
}
#[doc = "Field `BPBLCD60` writer - no description available"]
pub type Bpblcd60W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd60>;
impl<'a, REG> Bpblcd60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd60::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd60::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd60 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd60> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD60` reader - no description available"]
pub type Bpclcd60R = crate::BitReader<Bpclcd60>;
impl Bpclcd60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd60 {
        match self.bits {
            false => Bpclcd60::B0,
            true => Bpclcd60::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd60::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd60::B1
    }
}
#[doc = "Field `BPCLCD60` writer - no description available"]
pub type Bpclcd60W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd60>;
impl<'a, REG> Bpclcd60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd60::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd60::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd60 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd60> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD60` reader - no description available"]
pub type Bpdlcd60R = crate::BitReader<Bpdlcd60>;
impl Bpdlcd60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd60 {
        match self.bits {
            false => Bpdlcd60::B0,
            true => Bpdlcd60::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd60::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd60::B1
    }
}
#[doc = "Field `BPDLCD60` writer - no description available"]
pub type Bpdlcd60W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd60>;
impl<'a, REG> Bpdlcd60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd60::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd60::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd60 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd60> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD60` reader - no description available"]
pub type Bpelcd60R = crate::BitReader<Bpelcd60>;
impl Bpelcd60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd60 {
        match self.bits {
            false => Bpelcd60::B0,
            true => Bpelcd60::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd60::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd60::B1
    }
}
#[doc = "Field `BPELCD60` writer - no description available"]
pub type Bpelcd60W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd60>;
impl<'a, REG> Bpelcd60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd60::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd60::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd60 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd60> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD60` reader - no description available"]
pub type Bpflcd60R = crate::BitReader<Bpflcd60>;
impl Bpflcd60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd60 {
        match self.bits {
            false => Bpflcd60::B0,
            true => Bpflcd60::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd60::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd60::B1
    }
}
#[doc = "Field `BPFLCD60` writer - no description available"]
pub type Bpflcd60W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd60>;
impl<'a, REG> Bpflcd60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd60::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd60::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd60 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd60> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD60` reader - no description available"]
pub type Bpglcd60R = crate::BitReader<Bpglcd60>;
impl Bpglcd60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd60 {
        match self.bits {
            false => Bpglcd60::B0,
            true => Bpglcd60::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd60::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd60::B1
    }
}
#[doc = "Field `BPGLCD60` writer - no description available"]
pub type Bpglcd60W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd60>;
impl<'a, REG> Bpglcd60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd60::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd60::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd60 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd60> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd60) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD60` reader - no description available"]
pub type Bphlcd60R = crate::BitReader<Bphlcd60>;
impl Bphlcd60R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd60 {
        match self.bits {
            false => Bphlcd60::B0,
            true => Bphlcd60::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd60::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd60::B1
    }
}
#[doc = "Field `BPHLCD60` writer - no description available"]
pub type Bphlcd60W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd60>;
impl<'a, REG> Bphlcd60W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd60::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd60::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd60(&self) -> Bpalcd60R {
        Bpalcd60R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd60(&self) -> Bpblcd60R {
        Bpblcd60R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd60(&self) -> Bpclcd60R {
        Bpclcd60R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd60(&self) -> Bpdlcd60R {
        Bpdlcd60R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd60(&self) -> Bpelcd60R {
        Bpelcd60R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd60(&self) -> Bpflcd60R {
        Bpflcd60R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd60(&self) -> Bpglcd60R {
        Bpglcd60R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd60(&self) -> Bphlcd60R {
        Bphlcd60R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd60(&mut self) -> Bpalcd60W<LcdWf60Spec> {
        Bpalcd60W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd60(&mut self) -> Bpblcd60W<LcdWf60Spec> {
        Bpblcd60W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd60(&mut self) -> Bpclcd60W<LcdWf60Spec> {
        Bpclcd60W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd60(&mut self) -> Bpdlcd60W<LcdWf60Spec> {
        Bpdlcd60W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd60(&mut self) -> Bpelcd60W<LcdWf60Spec> {
        Bpelcd60W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd60(&mut self) -> Bpflcd60W<LcdWf60Spec> {
        Bpflcd60W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd60(&mut self) -> Bpglcd60W<LcdWf60Spec> {
        Bpglcd60W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd60(&mut self) -> Bphlcd60W<LcdWf60Spec> {
        Bphlcd60W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 60.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf60Spec;
impl crate::RegisterSpec for LcdWf60Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf60::R`](R) reader structure"]
impl crate::Readable for LcdWf60Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf60::W`](W) writer structure"]
impl crate::Writable for LcdWf60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF60 to value 0"]
impl crate::Resettable for LcdWf60Spec {
    const RESET_VALUE: u8 = 0;
}
