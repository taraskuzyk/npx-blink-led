#[doc = "Register `WF1` reader"]
pub type R = crate::R<Wf1Spec>;
#[doc = "Register `WF1` writer"]
pub type W = crate::W<Wf1Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd1 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd1> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD1` reader - no description available"]
pub type Bpalcd1R = crate::BitReader<Bpalcd1>;
impl Bpalcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd1 {
        match self.bits {
            false => Bpalcd1::B0,
            true => Bpalcd1::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd1::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd1::B1
    }
}
#[doc = "Field `BPALCD1` writer - no description available"]
pub type Bpalcd1W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd1>;
impl<'a, REG> Bpalcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd1::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd1::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd1 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd1> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD1` reader - no description available"]
pub type Bpblcd1R = crate::BitReader<Bpblcd1>;
impl Bpblcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd1 {
        match self.bits {
            false => Bpblcd1::B0,
            true => Bpblcd1::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd1::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd1::B1
    }
}
#[doc = "Field `BPBLCD1` writer - no description available"]
pub type Bpblcd1W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd1>;
impl<'a, REG> Bpblcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd1::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd1::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd1 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd1> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD1` reader - no description available"]
pub type Bpclcd1R = crate::BitReader<Bpclcd1>;
impl Bpclcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd1 {
        match self.bits {
            false => Bpclcd1::B0,
            true => Bpclcd1::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd1::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd1::B1
    }
}
#[doc = "Field `BPCLCD1` writer - no description available"]
pub type Bpclcd1W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd1>;
impl<'a, REG> Bpclcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd1::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd1::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd1 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd1> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD1` reader - no description available"]
pub type Bpdlcd1R = crate::BitReader<Bpdlcd1>;
impl Bpdlcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd1 {
        match self.bits {
            false => Bpdlcd1::B0,
            true => Bpdlcd1::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd1::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd1::B1
    }
}
#[doc = "Field `BPDLCD1` writer - no description available"]
pub type Bpdlcd1W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd1>;
impl<'a, REG> Bpdlcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd1::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd1::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd1 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd1> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD1` reader - no description available"]
pub type Bpelcd1R = crate::BitReader<Bpelcd1>;
impl Bpelcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd1 {
        match self.bits {
            false => Bpelcd1::B0,
            true => Bpelcd1::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd1::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd1::B1
    }
}
#[doc = "Field `BPELCD1` writer - no description available"]
pub type Bpelcd1W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd1>;
impl<'a, REG> Bpelcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd1::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd1::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd1 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd1> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD1` reader - no description available"]
pub type Bpflcd1R = crate::BitReader<Bpflcd1>;
impl Bpflcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd1 {
        match self.bits {
            false => Bpflcd1::B0,
            true => Bpflcd1::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd1::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd1::B1
    }
}
#[doc = "Field `BPFLCD1` writer - no description available"]
pub type Bpflcd1W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd1>;
impl<'a, REG> Bpflcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd1::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd1::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd1 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd1> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD1` reader - no description available"]
pub type Bpglcd1R = crate::BitReader<Bpglcd1>;
impl Bpglcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd1 {
        match self.bits {
            false => Bpglcd1::B0,
            true => Bpglcd1::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd1::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd1::B1
    }
}
#[doc = "Field `BPGLCD1` writer - no description available"]
pub type Bpglcd1W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd1>;
impl<'a, REG> Bpglcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd1::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd1::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd1 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd1> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD1` reader - no description available"]
pub type Bphlcd1R = crate::BitReader<Bphlcd1>;
impl Bphlcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd1 {
        match self.bits {
            false => Bphlcd1::B0,
            true => Bphlcd1::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd1::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd1::B1
    }
}
#[doc = "Field `BPHLCD1` writer - no description available"]
pub type Bphlcd1W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd1>;
impl<'a, REG> Bphlcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd1::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd1::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd1(&self) -> Bpalcd1R {
        Bpalcd1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd1(&self) -> Bpblcd1R {
        Bpblcd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd1(&self) -> Bpclcd1R {
        Bpclcd1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd1(&self) -> Bpdlcd1R {
        Bpdlcd1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd1(&self) -> Bpelcd1R {
        Bpelcd1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd1(&self) -> Bpflcd1R {
        Bpflcd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd1(&self) -> Bpglcd1R {
        Bpglcd1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd1(&self) -> Bphlcd1R {
        Bphlcd1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd1(&mut self) -> Bpalcd1W<Wf1Spec> {
        Bpalcd1W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd1(&mut self) -> Bpblcd1W<Wf1Spec> {
        Bpblcd1W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd1(&mut self) -> Bpclcd1W<Wf1Spec> {
        Bpclcd1W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd1(&mut self) -> Bpdlcd1W<Wf1Spec> {
        Bpdlcd1W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd1(&mut self) -> Bpelcd1W<Wf1Spec> {
        Bpelcd1W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd1(&mut self) -> Bpflcd1W<Wf1Spec> {
        Bpflcd1W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd1(&mut self) -> Bpglcd1W<Wf1Spec> {
        Bpglcd1W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd1(&mut self) -> Bphlcd1W<Wf1Spec> {
        Bphlcd1W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf1Spec;
impl crate::RegisterSpec for Wf1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf1::R`](R) reader structure"]
impl crate::Readable for Wf1Spec {}
#[doc = "`write(|w| ..)` method takes [`wf1::W`](W) writer structure"]
impl crate::Writable for Wf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF1 to value 0"]
impl crate::Resettable for Wf1Spec {
    const RESET_VALUE: u8 = 0;
}
