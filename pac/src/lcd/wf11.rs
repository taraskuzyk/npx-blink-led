#[doc = "Register `WF11` reader"]
pub type R = crate::R<Wf11Spec>;
#[doc = "Register `WF11` writer"]
pub type W = crate::W<Wf11Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd11 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd11> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD11` reader - no description available"]
pub type Bpalcd11R = crate::BitReader<Bpalcd11>;
impl Bpalcd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd11 {
        match self.bits {
            false => Bpalcd11::B0,
            true => Bpalcd11::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd11::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd11::B1
    }
}
#[doc = "Field `BPALCD11` writer - no description available"]
pub type Bpalcd11W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd11>;
impl<'a, REG> Bpalcd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd11::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd11::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd11 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd11> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD11` reader - no description available"]
pub type Bpblcd11R = crate::BitReader<Bpblcd11>;
impl Bpblcd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd11 {
        match self.bits {
            false => Bpblcd11::B0,
            true => Bpblcd11::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd11::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd11::B1
    }
}
#[doc = "Field `BPBLCD11` writer - no description available"]
pub type Bpblcd11W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd11>;
impl<'a, REG> Bpblcd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd11::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd11::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd11 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd11> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD11` reader - no description available"]
pub type Bpclcd11R = crate::BitReader<Bpclcd11>;
impl Bpclcd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd11 {
        match self.bits {
            false => Bpclcd11::B0,
            true => Bpclcd11::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd11::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd11::B1
    }
}
#[doc = "Field `BPCLCD11` writer - no description available"]
pub type Bpclcd11W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd11>;
impl<'a, REG> Bpclcd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd11::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd11::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd11 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd11> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD11` reader - no description available"]
pub type Bpdlcd11R = crate::BitReader<Bpdlcd11>;
impl Bpdlcd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd11 {
        match self.bits {
            false => Bpdlcd11::B0,
            true => Bpdlcd11::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd11::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd11::B1
    }
}
#[doc = "Field `BPDLCD11` writer - no description available"]
pub type Bpdlcd11W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd11>;
impl<'a, REG> Bpdlcd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd11::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd11::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd11 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd11> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD11` reader - no description available"]
pub type Bpelcd11R = crate::BitReader<Bpelcd11>;
impl Bpelcd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd11 {
        match self.bits {
            false => Bpelcd11::B0,
            true => Bpelcd11::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd11::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd11::B1
    }
}
#[doc = "Field `BPELCD11` writer - no description available"]
pub type Bpelcd11W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd11>;
impl<'a, REG> Bpelcd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd11::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd11::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd11 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd11> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD11` reader - no description available"]
pub type Bpflcd11R = crate::BitReader<Bpflcd11>;
impl Bpflcd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd11 {
        match self.bits {
            false => Bpflcd11::B0,
            true => Bpflcd11::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd11::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd11::B1
    }
}
#[doc = "Field `BPFLCD11` writer - no description available"]
pub type Bpflcd11W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd11>;
impl<'a, REG> Bpflcd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd11::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd11::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd11 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd11> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD11` reader - no description available"]
pub type Bpglcd11R = crate::BitReader<Bpglcd11>;
impl Bpglcd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd11 {
        match self.bits {
            false => Bpglcd11::B0,
            true => Bpglcd11::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd11::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd11::B1
    }
}
#[doc = "Field `BPGLCD11` writer - no description available"]
pub type Bpglcd11W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd11>;
impl<'a, REG> Bpglcd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd11::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd11::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd11 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd11> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD11` reader - no description available"]
pub type Bphlcd11R = crate::BitReader<Bphlcd11>;
impl Bphlcd11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd11 {
        match self.bits {
            false => Bphlcd11::B0,
            true => Bphlcd11::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd11::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd11::B1
    }
}
#[doc = "Field `BPHLCD11` writer - no description available"]
pub type Bphlcd11W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd11>;
impl<'a, REG> Bphlcd11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd11::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd11::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd11(&self) -> Bpalcd11R {
        Bpalcd11R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd11(&self) -> Bpblcd11R {
        Bpblcd11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd11(&self) -> Bpclcd11R {
        Bpclcd11R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd11(&self) -> Bpdlcd11R {
        Bpdlcd11R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd11(&self) -> Bpelcd11R {
        Bpelcd11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd11(&self) -> Bpflcd11R {
        Bpflcd11R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd11(&self) -> Bpglcd11R {
        Bpglcd11R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd11(&self) -> Bphlcd11R {
        Bphlcd11R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd11(&mut self) -> Bpalcd11W<Wf11Spec> {
        Bpalcd11W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd11(&mut self) -> Bpblcd11W<Wf11Spec> {
        Bpblcd11W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd11(&mut self) -> Bpclcd11W<Wf11Spec> {
        Bpclcd11W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd11(&mut self) -> Bpdlcd11W<Wf11Spec> {
        Bpdlcd11W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd11(&mut self) -> Bpelcd11W<Wf11Spec> {
        Bpelcd11W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd11(&mut self) -> Bpflcd11W<Wf11Spec> {
        Bpflcd11W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd11(&mut self) -> Bpglcd11W<Wf11Spec> {
        Bpglcd11W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd11(&mut self) -> Bphlcd11W<Wf11Spec> {
        Bphlcd11W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 11.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf11Spec;
impl crate::RegisterSpec for Wf11Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf11::R`](R) reader structure"]
impl crate::Readable for Wf11Spec {}
#[doc = "`write(|w| ..)` method takes [`wf11::W`](W) writer structure"]
impl crate::Writable for Wf11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF11 to value 0"]
impl crate::Resettable for Wf11Spec {
    const RESET_VALUE: u8 = 0;
}
