#[doc = "Register `WF39` reader"]
pub type R = crate::R<Wf39Spec>;
#[doc = "Register `WF39` writer"]
pub type W = crate::W<Wf39Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd39 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd39> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD39` reader - no description available"]
pub type Bpalcd39R = crate::BitReader<Bpalcd39>;
impl Bpalcd39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd39 {
        match self.bits {
            false => Bpalcd39::B0,
            true => Bpalcd39::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd39::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd39::B1
    }
}
#[doc = "Field `BPALCD39` writer - no description available"]
pub type Bpalcd39W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd39>;
impl<'a, REG> Bpalcd39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd39::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd39::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd39 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd39> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD39` reader - no description available"]
pub type Bpblcd39R = crate::BitReader<Bpblcd39>;
impl Bpblcd39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd39 {
        match self.bits {
            false => Bpblcd39::B0,
            true => Bpblcd39::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd39::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd39::B1
    }
}
#[doc = "Field `BPBLCD39` writer - no description available"]
pub type Bpblcd39W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd39>;
impl<'a, REG> Bpblcd39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd39::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd39::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd39 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd39> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD39` reader - no description available"]
pub type Bpclcd39R = crate::BitReader<Bpclcd39>;
impl Bpclcd39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd39 {
        match self.bits {
            false => Bpclcd39::B0,
            true => Bpclcd39::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd39::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd39::B1
    }
}
#[doc = "Field `BPCLCD39` writer - no description available"]
pub type Bpclcd39W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd39>;
impl<'a, REG> Bpclcd39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd39::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd39::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd39 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd39> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD39` reader - no description available"]
pub type Bpdlcd39R = crate::BitReader<Bpdlcd39>;
impl Bpdlcd39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd39 {
        match self.bits {
            false => Bpdlcd39::B0,
            true => Bpdlcd39::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd39::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd39::B1
    }
}
#[doc = "Field `BPDLCD39` writer - no description available"]
pub type Bpdlcd39W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd39>;
impl<'a, REG> Bpdlcd39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd39::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd39::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd39 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd39> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD39` reader - no description available"]
pub type Bpelcd39R = crate::BitReader<Bpelcd39>;
impl Bpelcd39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd39 {
        match self.bits {
            false => Bpelcd39::B0,
            true => Bpelcd39::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd39::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd39::B1
    }
}
#[doc = "Field `BPELCD39` writer - no description available"]
pub type Bpelcd39W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd39>;
impl<'a, REG> Bpelcd39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd39::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd39::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd39 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd39> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD39` reader - no description available"]
pub type Bpflcd39R = crate::BitReader<Bpflcd39>;
impl Bpflcd39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd39 {
        match self.bits {
            false => Bpflcd39::B0,
            true => Bpflcd39::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd39::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd39::B1
    }
}
#[doc = "Field `BPFLCD39` writer - no description available"]
pub type Bpflcd39W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd39>;
impl<'a, REG> Bpflcd39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd39::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd39::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd39 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd39> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD39` reader - no description available"]
pub type Bpglcd39R = crate::BitReader<Bpglcd39>;
impl Bpglcd39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd39 {
        match self.bits {
            false => Bpglcd39::B0,
            true => Bpglcd39::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd39::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd39::B1
    }
}
#[doc = "Field `BPGLCD39` writer - no description available"]
pub type Bpglcd39W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd39>;
impl<'a, REG> Bpglcd39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd39::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd39::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd39 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd39> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd39) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD39` reader - no description available"]
pub type Bphlcd39R = crate::BitReader<Bphlcd39>;
impl Bphlcd39R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd39 {
        match self.bits {
            false => Bphlcd39::B0,
            true => Bphlcd39::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd39::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd39::B1
    }
}
#[doc = "Field `BPHLCD39` writer - no description available"]
pub type Bphlcd39W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd39>;
impl<'a, REG> Bphlcd39W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd39::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd39::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd39(&self) -> Bpalcd39R {
        Bpalcd39R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd39(&self) -> Bpblcd39R {
        Bpblcd39R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd39(&self) -> Bpclcd39R {
        Bpclcd39R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd39(&self) -> Bpdlcd39R {
        Bpdlcd39R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd39(&self) -> Bpelcd39R {
        Bpelcd39R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd39(&self) -> Bpflcd39R {
        Bpflcd39R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd39(&self) -> Bpglcd39R {
        Bpglcd39R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd39(&self) -> Bphlcd39R {
        Bphlcd39R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd39(&mut self) -> Bpalcd39W<Wf39Spec> {
        Bpalcd39W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd39(&mut self) -> Bpblcd39W<Wf39Spec> {
        Bpblcd39W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd39(&mut self) -> Bpclcd39W<Wf39Spec> {
        Bpclcd39W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd39(&mut self) -> Bpdlcd39W<Wf39Spec> {
        Bpdlcd39W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd39(&mut self) -> Bpelcd39W<Wf39Spec> {
        Bpelcd39W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd39(&mut self) -> Bpflcd39W<Wf39Spec> {
        Bpflcd39W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd39(&mut self) -> Bpglcd39W<Wf39Spec> {
        Bpglcd39W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd39(&mut self) -> Bphlcd39W<Wf39Spec> {
        Bphlcd39W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 39.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf39::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf39::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf39Spec;
impl crate::RegisterSpec for Wf39Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf39::R`](R) reader structure"]
impl crate::Readable for Wf39Spec {}
#[doc = "`write(|w| ..)` method takes [`wf39::W`](W) writer structure"]
impl crate::Writable for Wf39Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF39 to value 0"]
impl crate::Resettable for Wf39Spec {
    const RESET_VALUE: u8 = 0;
}
