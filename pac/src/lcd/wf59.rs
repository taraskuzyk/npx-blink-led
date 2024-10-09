#[doc = "Register `WF59` reader"]
pub type R = crate::R<Wf59Spec>;
#[doc = "Register `WF59` writer"]
pub type W = crate::W<Wf59Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd59 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd59> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD59` reader - no description available"]
pub type Bpalcd59R = crate::BitReader<Bpalcd59>;
impl Bpalcd59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd59 {
        match self.bits {
            false => Bpalcd59::B0,
            true => Bpalcd59::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd59::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd59::B1
    }
}
#[doc = "Field `BPALCD59` writer - no description available"]
pub type Bpalcd59W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd59>;
impl<'a, REG> Bpalcd59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd59::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd59::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd59 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd59> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD59` reader - no description available"]
pub type Bpblcd59R = crate::BitReader<Bpblcd59>;
impl Bpblcd59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd59 {
        match self.bits {
            false => Bpblcd59::B0,
            true => Bpblcd59::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd59::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd59::B1
    }
}
#[doc = "Field `BPBLCD59` writer - no description available"]
pub type Bpblcd59W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd59>;
impl<'a, REG> Bpblcd59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd59::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd59::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd59 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd59> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD59` reader - no description available"]
pub type Bpclcd59R = crate::BitReader<Bpclcd59>;
impl Bpclcd59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd59 {
        match self.bits {
            false => Bpclcd59::B0,
            true => Bpclcd59::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd59::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd59::B1
    }
}
#[doc = "Field `BPCLCD59` writer - no description available"]
pub type Bpclcd59W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd59>;
impl<'a, REG> Bpclcd59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd59::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd59::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd59 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd59> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD59` reader - no description available"]
pub type Bpdlcd59R = crate::BitReader<Bpdlcd59>;
impl Bpdlcd59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd59 {
        match self.bits {
            false => Bpdlcd59::B0,
            true => Bpdlcd59::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd59::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd59::B1
    }
}
#[doc = "Field `BPDLCD59` writer - no description available"]
pub type Bpdlcd59W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd59>;
impl<'a, REG> Bpdlcd59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd59::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd59::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd59 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd59> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD59` reader - no description available"]
pub type Bpelcd59R = crate::BitReader<Bpelcd59>;
impl Bpelcd59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd59 {
        match self.bits {
            false => Bpelcd59::B0,
            true => Bpelcd59::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd59::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd59::B1
    }
}
#[doc = "Field `BPELCD59` writer - no description available"]
pub type Bpelcd59W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd59>;
impl<'a, REG> Bpelcd59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd59::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd59::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd59 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd59> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD59` reader - no description available"]
pub type Bpflcd59R = crate::BitReader<Bpflcd59>;
impl Bpflcd59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd59 {
        match self.bits {
            false => Bpflcd59::B0,
            true => Bpflcd59::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd59::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd59::B1
    }
}
#[doc = "Field `BPFLCD59` writer - no description available"]
pub type Bpflcd59W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd59>;
impl<'a, REG> Bpflcd59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd59::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd59::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd59 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd59> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD59` reader - no description available"]
pub type Bpglcd59R = crate::BitReader<Bpglcd59>;
impl Bpglcd59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd59 {
        match self.bits {
            false => Bpglcd59::B0,
            true => Bpglcd59::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd59::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd59::B1
    }
}
#[doc = "Field `BPGLCD59` writer - no description available"]
pub type Bpglcd59W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd59>;
impl<'a, REG> Bpglcd59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd59::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd59::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd59 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd59> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd59) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD59` reader - no description available"]
pub type Bphlcd59R = crate::BitReader<Bphlcd59>;
impl Bphlcd59R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd59 {
        match self.bits {
            false => Bphlcd59::B0,
            true => Bphlcd59::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd59::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd59::B1
    }
}
#[doc = "Field `BPHLCD59` writer - no description available"]
pub type Bphlcd59W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd59>;
impl<'a, REG> Bphlcd59W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd59::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd59::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd59(&self) -> Bpalcd59R {
        Bpalcd59R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd59(&self) -> Bpblcd59R {
        Bpblcd59R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd59(&self) -> Bpclcd59R {
        Bpclcd59R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd59(&self) -> Bpdlcd59R {
        Bpdlcd59R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd59(&self) -> Bpelcd59R {
        Bpelcd59R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd59(&self) -> Bpflcd59R {
        Bpflcd59R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd59(&self) -> Bpglcd59R {
        Bpglcd59R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd59(&self) -> Bphlcd59R {
        Bphlcd59R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd59(&mut self) -> Bpalcd59W<Wf59Spec> {
        Bpalcd59W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd59(&mut self) -> Bpblcd59W<Wf59Spec> {
        Bpblcd59W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd59(&mut self) -> Bpclcd59W<Wf59Spec> {
        Bpclcd59W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd59(&mut self) -> Bpdlcd59W<Wf59Spec> {
        Bpdlcd59W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd59(&mut self) -> Bpelcd59W<Wf59Spec> {
        Bpelcd59W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd59(&mut self) -> Bpflcd59W<Wf59Spec> {
        Bpflcd59W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd59(&mut self) -> Bpglcd59W<Wf59Spec> {
        Bpglcd59W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd59(&mut self) -> Bphlcd59W<Wf59Spec> {
        Bphlcd59W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 59.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf59::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf59::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf59Spec;
impl crate::RegisterSpec for Wf59Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf59::R`](R) reader structure"]
impl crate::Readable for Wf59Spec {}
#[doc = "`write(|w| ..)` method takes [`wf59::W`](W) writer structure"]
impl crate::Writable for Wf59Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF59 to value 0"]
impl crate::Resettable for Wf59Spec {
    const RESET_VALUE: u8 = 0;
}
