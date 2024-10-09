#[doc = "Register `WF54` reader"]
pub type R = crate::R<Wf54Spec>;
#[doc = "Register `WF54` writer"]
pub type W = crate::W<Wf54Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd54 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd54> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD54` reader - no description available"]
pub type Bpalcd54R = crate::BitReader<Bpalcd54>;
impl Bpalcd54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd54 {
        match self.bits {
            false => Bpalcd54::B0,
            true => Bpalcd54::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd54::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd54::B1
    }
}
#[doc = "Field `BPALCD54` writer - no description available"]
pub type Bpalcd54W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd54>;
impl<'a, REG> Bpalcd54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd54::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd54::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd54 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd54> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD54` reader - no description available"]
pub type Bpblcd54R = crate::BitReader<Bpblcd54>;
impl Bpblcd54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd54 {
        match self.bits {
            false => Bpblcd54::B0,
            true => Bpblcd54::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd54::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd54::B1
    }
}
#[doc = "Field `BPBLCD54` writer - no description available"]
pub type Bpblcd54W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd54>;
impl<'a, REG> Bpblcd54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd54::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd54::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd54 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd54> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD54` reader - no description available"]
pub type Bpclcd54R = crate::BitReader<Bpclcd54>;
impl Bpclcd54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd54 {
        match self.bits {
            false => Bpclcd54::B0,
            true => Bpclcd54::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd54::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd54::B1
    }
}
#[doc = "Field `BPCLCD54` writer - no description available"]
pub type Bpclcd54W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd54>;
impl<'a, REG> Bpclcd54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd54::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd54::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd54 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd54> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD54` reader - no description available"]
pub type Bpdlcd54R = crate::BitReader<Bpdlcd54>;
impl Bpdlcd54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd54 {
        match self.bits {
            false => Bpdlcd54::B0,
            true => Bpdlcd54::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd54::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd54::B1
    }
}
#[doc = "Field `BPDLCD54` writer - no description available"]
pub type Bpdlcd54W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd54>;
impl<'a, REG> Bpdlcd54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd54::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd54::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd54 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd54> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD54` reader - no description available"]
pub type Bpelcd54R = crate::BitReader<Bpelcd54>;
impl Bpelcd54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd54 {
        match self.bits {
            false => Bpelcd54::B0,
            true => Bpelcd54::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd54::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd54::B1
    }
}
#[doc = "Field `BPELCD54` writer - no description available"]
pub type Bpelcd54W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd54>;
impl<'a, REG> Bpelcd54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd54::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd54::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd54 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd54> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD54` reader - no description available"]
pub type Bpflcd54R = crate::BitReader<Bpflcd54>;
impl Bpflcd54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd54 {
        match self.bits {
            false => Bpflcd54::B0,
            true => Bpflcd54::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd54::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd54::B1
    }
}
#[doc = "Field `BPFLCD54` writer - no description available"]
pub type Bpflcd54W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd54>;
impl<'a, REG> Bpflcd54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd54::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd54::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd54 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd54> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD54` reader - no description available"]
pub type Bpglcd54R = crate::BitReader<Bpglcd54>;
impl Bpglcd54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd54 {
        match self.bits {
            false => Bpglcd54::B0,
            true => Bpglcd54::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd54::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd54::B1
    }
}
#[doc = "Field `BPGLCD54` writer - no description available"]
pub type Bpglcd54W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd54>;
impl<'a, REG> Bpglcd54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd54::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd54::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd54 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd54> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd54) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD54` reader - no description available"]
pub type Bphlcd54R = crate::BitReader<Bphlcd54>;
impl Bphlcd54R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd54 {
        match self.bits {
            false => Bphlcd54::B0,
            true => Bphlcd54::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd54::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd54::B1
    }
}
#[doc = "Field `BPHLCD54` writer - no description available"]
pub type Bphlcd54W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd54>;
impl<'a, REG> Bphlcd54W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd54::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd54::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd54(&self) -> Bpalcd54R {
        Bpalcd54R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd54(&self) -> Bpblcd54R {
        Bpblcd54R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd54(&self) -> Bpclcd54R {
        Bpclcd54R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd54(&self) -> Bpdlcd54R {
        Bpdlcd54R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd54(&self) -> Bpelcd54R {
        Bpelcd54R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd54(&self) -> Bpflcd54R {
        Bpflcd54R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd54(&self) -> Bpglcd54R {
        Bpglcd54R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd54(&self) -> Bphlcd54R {
        Bphlcd54R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd54(&mut self) -> Bpalcd54W<Wf54Spec> {
        Bpalcd54W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd54(&mut self) -> Bpblcd54W<Wf54Spec> {
        Bpblcd54W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd54(&mut self) -> Bpclcd54W<Wf54Spec> {
        Bpclcd54W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd54(&mut self) -> Bpdlcd54W<Wf54Spec> {
        Bpdlcd54W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd54(&mut self) -> Bpelcd54W<Wf54Spec> {
        Bpelcd54W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd54(&mut self) -> Bpflcd54W<Wf54Spec> {
        Bpflcd54W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd54(&mut self) -> Bpglcd54W<Wf54Spec> {
        Bpglcd54W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd54(&mut self) -> Bphlcd54W<Wf54Spec> {
        Bphlcd54W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 54.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf54Spec;
impl crate::RegisterSpec for Wf54Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf54::R`](R) reader structure"]
impl crate::Readable for Wf54Spec {}
#[doc = "`write(|w| ..)` method takes [`wf54::W`](W) writer structure"]
impl crate::Writable for Wf54Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF54 to value 0"]
impl crate::Resettable for Wf54Spec {
    const RESET_VALUE: u8 = 0;
}
