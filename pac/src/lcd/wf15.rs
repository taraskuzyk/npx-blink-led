#[doc = "Register `WF15` reader"]
pub type R = crate::R<Wf15Spec>;
#[doc = "Register `WF15` writer"]
pub type W = crate::W<Wf15Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd15 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd15> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD15` reader - no description available"]
pub type Bpalcd15R = crate::BitReader<Bpalcd15>;
impl Bpalcd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd15 {
        match self.bits {
            false => Bpalcd15::B0,
            true => Bpalcd15::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd15::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd15::B1
    }
}
#[doc = "Field `BPALCD15` writer - no description available"]
pub type Bpalcd15W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd15>;
impl<'a, REG> Bpalcd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd15::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd15::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd15 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd15> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD15` reader - no description available"]
pub type Bpblcd15R = crate::BitReader<Bpblcd15>;
impl Bpblcd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd15 {
        match self.bits {
            false => Bpblcd15::B0,
            true => Bpblcd15::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd15::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd15::B1
    }
}
#[doc = "Field `BPBLCD15` writer - no description available"]
pub type Bpblcd15W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd15>;
impl<'a, REG> Bpblcd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd15::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd15::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd15 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd15> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD15` reader - no description available"]
pub type Bpclcd15R = crate::BitReader<Bpclcd15>;
impl Bpclcd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd15 {
        match self.bits {
            false => Bpclcd15::B0,
            true => Bpclcd15::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd15::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd15::B1
    }
}
#[doc = "Field `BPCLCD15` writer - no description available"]
pub type Bpclcd15W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd15>;
impl<'a, REG> Bpclcd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd15::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd15::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd15 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd15> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD15` reader - no description available"]
pub type Bpdlcd15R = crate::BitReader<Bpdlcd15>;
impl Bpdlcd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd15 {
        match self.bits {
            false => Bpdlcd15::B0,
            true => Bpdlcd15::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd15::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd15::B1
    }
}
#[doc = "Field `BPDLCD15` writer - no description available"]
pub type Bpdlcd15W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd15>;
impl<'a, REG> Bpdlcd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd15::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd15::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd15 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd15> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD15` reader - no description available"]
pub type Bpelcd15R = crate::BitReader<Bpelcd15>;
impl Bpelcd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd15 {
        match self.bits {
            false => Bpelcd15::B0,
            true => Bpelcd15::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd15::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd15::B1
    }
}
#[doc = "Field `BPELCD15` writer - no description available"]
pub type Bpelcd15W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd15>;
impl<'a, REG> Bpelcd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd15::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd15::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd15 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd15> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD15` reader - no description available"]
pub type Bpflcd15R = crate::BitReader<Bpflcd15>;
impl Bpflcd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd15 {
        match self.bits {
            false => Bpflcd15::B0,
            true => Bpflcd15::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd15::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd15::B1
    }
}
#[doc = "Field `BPFLCD15` writer - no description available"]
pub type Bpflcd15W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd15>;
impl<'a, REG> Bpflcd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd15::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd15::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd15 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd15> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD15` reader - no description available"]
pub type Bpglcd15R = crate::BitReader<Bpglcd15>;
impl Bpglcd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd15 {
        match self.bits {
            false => Bpglcd15::B0,
            true => Bpglcd15::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd15::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd15::B1
    }
}
#[doc = "Field `BPGLCD15` writer - no description available"]
pub type Bpglcd15W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd15>;
impl<'a, REG> Bpglcd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd15::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd15::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd15 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd15> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD15` reader - no description available"]
pub type Bphlcd15R = crate::BitReader<Bphlcd15>;
impl Bphlcd15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd15 {
        match self.bits {
            false => Bphlcd15::B0,
            true => Bphlcd15::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd15::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd15::B1
    }
}
#[doc = "Field `BPHLCD15` writer - no description available"]
pub type Bphlcd15W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd15>;
impl<'a, REG> Bphlcd15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd15::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd15::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd15(&self) -> Bpalcd15R {
        Bpalcd15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd15(&self) -> Bpblcd15R {
        Bpblcd15R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd15(&self) -> Bpclcd15R {
        Bpclcd15R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd15(&self) -> Bpdlcd15R {
        Bpdlcd15R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd15(&self) -> Bpelcd15R {
        Bpelcd15R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd15(&self) -> Bpflcd15R {
        Bpflcd15R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd15(&self) -> Bpglcd15R {
        Bpglcd15R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd15(&self) -> Bphlcd15R {
        Bphlcd15R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd15(&mut self) -> Bpalcd15W<Wf15Spec> {
        Bpalcd15W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd15(&mut self) -> Bpblcd15W<Wf15Spec> {
        Bpblcd15W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd15(&mut self) -> Bpclcd15W<Wf15Spec> {
        Bpclcd15W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd15(&mut self) -> Bpdlcd15W<Wf15Spec> {
        Bpdlcd15W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd15(&mut self) -> Bpelcd15W<Wf15Spec> {
        Bpelcd15W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd15(&mut self) -> Bpflcd15W<Wf15Spec> {
        Bpflcd15W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd15(&mut self) -> Bpglcd15W<Wf15Spec> {
        Bpglcd15W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd15(&mut self) -> Bphlcd15W<Wf15Spec> {
        Bphlcd15W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf15Spec;
impl crate::RegisterSpec for Wf15Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf15::R`](R) reader structure"]
impl crate::Readable for Wf15Spec {}
#[doc = "`write(|w| ..)` method takes [`wf15::W`](W) writer structure"]
impl crate::Writable for Wf15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF15 to value 0"]
impl crate::Resettable for Wf15Spec {
    const RESET_VALUE: u8 = 0;
}
