#[doc = "Register `WF3` reader"]
pub type R = crate::R<Wf3Spec>;
#[doc = "Register `WF3` writer"]
pub type W = crate::W<Wf3Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd3 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd3> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD3` reader - no description available"]
pub type Bpalcd3R = crate::BitReader<Bpalcd3>;
impl Bpalcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd3 {
        match self.bits {
            false => Bpalcd3::B0,
            true => Bpalcd3::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd3::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd3::B1
    }
}
#[doc = "Field `BPALCD3` writer - no description available"]
pub type Bpalcd3W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd3>;
impl<'a, REG> Bpalcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd3::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd3::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd3 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd3> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD3` reader - no description available"]
pub type Bpblcd3R = crate::BitReader<Bpblcd3>;
impl Bpblcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd3 {
        match self.bits {
            false => Bpblcd3::B0,
            true => Bpblcd3::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd3::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd3::B1
    }
}
#[doc = "Field `BPBLCD3` writer - no description available"]
pub type Bpblcd3W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd3>;
impl<'a, REG> Bpblcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd3::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd3::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd3 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd3> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD3` reader - no description available"]
pub type Bpclcd3R = crate::BitReader<Bpclcd3>;
impl Bpclcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd3 {
        match self.bits {
            false => Bpclcd3::B0,
            true => Bpclcd3::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd3::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd3::B1
    }
}
#[doc = "Field `BPCLCD3` writer - no description available"]
pub type Bpclcd3W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd3>;
impl<'a, REG> Bpclcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd3::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd3::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd3 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd3> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD3` reader - no description available"]
pub type Bpdlcd3R = crate::BitReader<Bpdlcd3>;
impl Bpdlcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd3 {
        match self.bits {
            false => Bpdlcd3::B0,
            true => Bpdlcd3::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd3::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd3::B1
    }
}
#[doc = "Field `BPDLCD3` writer - no description available"]
pub type Bpdlcd3W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd3>;
impl<'a, REG> Bpdlcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd3::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd3::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd3 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd3> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD3` reader - no description available"]
pub type Bpelcd3R = crate::BitReader<Bpelcd3>;
impl Bpelcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd3 {
        match self.bits {
            false => Bpelcd3::B0,
            true => Bpelcd3::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd3::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd3::B1
    }
}
#[doc = "Field `BPELCD3` writer - no description available"]
pub type Bpelcd3W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd3>;
impl<'a, REG> Bpelcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd3::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd3::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd3 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd3> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD3` reader - no description available"]
pub type Bpflcd3R = crate::BitReader<Bpflcd3>;
impl Bpflcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd3 {
        match self.bits {
            false => Bpflcd3::B0,
            true => Bpflcd3::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd3::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd3::B1
    }
}
#[doc = "Field `BPFLCD3` writer - no description available"]
pub type Bpflcd3W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd3>;
impl<'a, REG> Bpflcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd3::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd3::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd3 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd3> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD3` reader - no description available"]
pub type Bpglcd3R = crate::BitReader<Bpglcd3>;
impl Bpglcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd3 {
        match self.bits {
            false => Bpglcd3::B0,
            true => Bpglcd3::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd3::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd3::B1
    }
}
#[doc = "Field `BPGLCD3` writer - no description available"]
pub type Bpglcd3W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd3>;
impl<'a, REG> Bpglcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd3::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd3::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd3 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd3> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD3` reader - no description available"]
pub type Bphlcd3R = crate::BitReader<Bphlcd3>;
impl Bphlcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd3 {
        match self.bits {
            false => Bphlcd3::B0,
            true => Bphlcd3::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd3::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd3::B1
    }
}
#[doc = "Field `BPHLCD3` writer - no description available"]
pub type Bphlcd3W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd3>;
impl<'a, REG> Bphlcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd3::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd3::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd3(&self) -> Bpalcd3R {
        Bpalcd3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd3(&self) -> Bpblcd3R {
        Bpblcd3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd3(&self) -> Bpclcd3R {
        Bpclcd3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd3(&self) -> Bpdlcd3R {
        Bpdlcd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd3(&self) -> Bpelcd3R {
        Bpelcd3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd3(&self) -> Bpflcd3R {
        Bpflcd3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd3(&self) -> Bpglcd3R {
        Bpglcd3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd3(&self) -> Bphlcd3R {
        Bphlcd3R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd3(&mut self) -> Bpalcd3W<Wf3Spec> {
        Bpalcd3W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd3(&mut self) -> Bpblcd3W<Wf3Spec> {
        Bpblcd3W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd3(&mut self) -> Bpclcd3W<Wf3Spec> {
        Bpclcd3W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd3(&mut self) -> Bpdlcd3W<Wf3Spec> {
        Bpdlcd3W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd3(&mut self) -> Bpelcd3W<Wf3Spec> {
        Bpelcd3W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd3(&mut self) -> Bpflcd3W<Wf3Spec> {
        Bpflcd3W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd3(&mut self) -> Bpglcd3W<Wf3Spec> {
        Bpglcd3W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd3(&mut self) -> Bphlcd3W<Wf3Spec> {
        Bphlcd3W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf3Spec;
impl crate::RegisterSpec for Wf3Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf3::R`](R) reader structure"]
impl crate::Readable for Wf3Spec {}
#[doc = "`write(|w| ..)` method takes [`wf3::W`](W) writer structure"]
impl crate::Writable for Wf3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF3 to value 0"]
impl crate::Resettable for Wf3Spec {
    const RESET_VALUE: u8 = 0;
}
