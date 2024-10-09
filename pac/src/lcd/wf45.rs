#[doc = "Register `WF45` reader"]
pub type R = crate::R<Wf45Spec>;
#[doc = "Register `WF45` writer"]
pub type W = crate::W<Wf45Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd45 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd45> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD45` reader - no description available"]
pub type Bpalcd45R = crate::BitReader<Bpalcd45>;
impl Bpalcd45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd45 {
        match self.bits {
            false => Bpalcd45::B0,
            true => Bpalcd45::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd45::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd45::B1
    }
}
#[doc = "Field `BPALCD45` writer - no description available"]
pub type Bpalcd45W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd45>;
impl<'a, REG> Bpalcd45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd45::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd45::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd45 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd45> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD45` reader - no description available"]
pub type Bpblcd45R = crate::BitReader<Bpblcd45>;
impl Bpblcd45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd45 {
        match self.bits {
            false => Bpblcd45::B0,
            true => Bpblcd45::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd45::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd45::B1
    }
}
#[doc = "Field `BPBLCD45` writer - no description available"]
pub type Bpblcd45W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd45>;
impl<'a, REG> Bpblcd45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd45::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd45::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd45 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd45> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD45` reader - no description available"]
pub type Bpclcd45R = crate::BitReader<Bpclcd45>;
impl Bpclcd45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd45 {
        match self.bits {
            false => Bpclcd45::B0,
            true => Bpclcd45::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd45::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd45::B1
    }
}
#[doc = "Field `BPCLCD45` writer - no description available"]
pub type Bpclcd45W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd45>;
impl<'a, REG> Bpclcd45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd45::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd45::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd45 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd45> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD45` reader - no description available"]
pub type Bpdlcd45R = crate::BitReader<Bpdlcd45>;
impl Bpdlcd45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd45 {
        match self.bits {
            false => Bpdlcd45::B0,
            true => Bpdlcd45::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd45::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd45::B1
    }
}
#[doc = "Field `BPDLCD45` writer - no description available"]
pub type Bpdlcd45W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd45>;
impl<'a, REG> Bpdlcd45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd45::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd45::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd45 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd45> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD45` reader - no description available"]
pub type Bpelcd45R = crate::BitReader<Bpelcd45>;
impl Bpelcd45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd45 {
        match self.bits {
            false => Bpelcd45::B0,
            true => Bpelcd45::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd45::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd45::B1
    }
}
#[doc = "Field `BPELCD45` writer - no description available"]
pub type Bpelcd45W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd45>;
impl<'a, REG> Bpelcd45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd45::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd45::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd45 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd45> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD45` reader - no description available"]
pub type Bpflcd45R = crate::BitReader<Bpflcd45>;
impl Bpflcd45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd45 {
        match self.bits {
            false => Bpflcd45::B0,
            true => Bpflcd45::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd45::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd45::B1
    }
}
#[doc = "Field `BPFLCD45` writer - no description available"]
pub type Bpflcd45W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd45>;
impl<'a, REG> Bpflcd45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd45::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd45::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd45 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd45> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD45` reader - no description available"]
pub type Bpglcd45R = crate::BitReader<Bpglcd45>;
impl Bpglcd45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd45 {
        match self.bits {
            false => Bpglcd45::B0,
            true => Bpglcd45::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd45::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd45::B1
    }
}
#[doc = "Field `BPGLCD45` writer - no description available"]
pub type Bpglcd45W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd45>;
impl<'a, REG> Bpglcd45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd45::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd45::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd45 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd45> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd45) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD45` reader - no description available"]
pub type Bphlcd45R = crate::BitReader<Bphlcd45>;
impl Bphlcd45R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd45 {
        match self.bits {
            false => Bphlcd45::B0,
            true => Bphlcd45::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd45::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd45::B1
    }
}
#[doc = "Field `BPHLCD45` writer - no description available"]
pub type Bphlcd45W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd45>;
impl<'a, REG> Bphlcd45W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd45::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd45::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd45(&self) -> Bpalcd45R {
        Bpalcd45R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd45(&self) -> Bpblcd45R {
        Bpblcd45R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd45(&self) -> Bpclcd45R {
        Bpclcd45R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd45(&self) -> Bpdlcd45R {
        Bpdlcd45R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd45(&self) -> Bpelcd45R {
        Bpelcd45R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd45(&self) -> Bpflcd45R {
        Bpflcd45R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd45(&self) -> Bpglcd45R {
        Bpglcd45R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd45(&self) -> Bphlcd45R {
        Bphlcd45R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd45(&mut self) -> Bpalcd45W<Wf45Spec> {
        Bpalcd45W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd45(&mut self) -> Bpblcd45W<Wf45Spec> {
        Bpblcd45W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd45(&mut self) -> Bpclcd45W<Wf45Spec> {
        Bpclcd45W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd45(&mut self) -> Bpdlcd45W<Wf45Spec> {
        Bpdlcd45W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd45(&mut self) -> Bpelcd45W<Wf45Spec> {
        Bpelcd45W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd45(&mut self) -> Bpflcd45W<Wf45Spec> {
        Bpflcd45W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd45(&mut self) -> Bpglcd45W<Wf45Spec> {
        Bpglcd45W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd45(&mut self) -> Bphlcd45W<Wf45Spec> {
        Bphlcd45W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 45.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf45Spec;
impl crate::RegisterSpec for Wf45Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf45::R`](R) reader structure"]
impl crate::Readable for Wf45Spec {}
#[doc = "`write(|w| ..)` method takes [`wf45::W`](W) writer structure"]
impl crate::Writable for Wf45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF45 to value 0"]
impl crate::Resettable for Wf45Spec {
    const RESET_VALUE: u8 = 0;
}
