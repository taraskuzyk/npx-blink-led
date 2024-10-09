#[doc = "Register `WF2` reader"]
pub type R = crate::R<Wf2Spec>;
#[doc = "Register `WF2` writer"]
pub type W = crate::W<Wf2Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd2 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd2> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD2` reader - no description available"]
pub type Bpalcd2R = crate::BitReader<Bpalcd2>;
impl Bpalcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd2 {
        match self.bits {
            false => Bpalcd2::B0,
            true => Bpalcd2::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd2::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd2::B1
    }
}
#[doc = "Field `BPALCD2` writer - no description available"]
pub type Bpalcd2W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd2>;
impl<'a, REG> Bpalcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd2::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd2::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd2 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd2> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD2` reader - no description available"]
pub type Bpblcd2R = crate::BitReader<Bpblcd2>;
impl Bpblcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd2 {
        match self.bits {
            false => Bpblcd2::B0,
            true => Bpblcd2::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd2::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd2::B1
    }
}
#[doc = "Field `BPBLCD2` writer - no description available"]
pub type Bpblcd2W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd2>;
impl<'a, REG> Bpblcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd2::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd2::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd2 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd2> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD2` reader - no description available"]
pub type Bpclcd2R = crate::BitReader<Bpclcd2>;
impl Bpclcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd2 {
        match self.bits {
            false => Bpclcd2::B0,
            true => Bpclcd2::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd2::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd2::B1
    }
}
#[doc = "Field `BPCLCD2` writer - no description available"]
pub type Bpclcd2W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd2>;
impl<'a, REG> Bpclcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd2::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd2::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd2 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd2> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD2` reader - no description available"]
pub type Bpdlcd2R = crate::BitReader<Bpdlcd2>;
impl Bpdlcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd2 {
        match self.bits {
            false => Bpdlcd2::B0,
            true => Bpdlcd2::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd2::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd2::B1
    }
}
#[doc = "Field `BPDLCD2` writer - no description available"]
pub type Bpdlcd2W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd2>;
impl<'a, REG> Bpdlcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd2::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd2::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd2 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd2> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD2` reader - no description available"]
pub type Bpelcd2R = crate::BitReader<Bpelcd2>;
impl Bpelcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd2 {
        match self.bits {
            false => Bpelcd2::B0,
            true => Bpelcd2::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd2::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd2::B1
    }
}
#[doc = "Field `BPELCD2` writer - no description available"]
pub type Bpelcd2W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd2>;
impl<'a, REG> Bpelcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd2::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd2::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd2 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd2> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD2` reader - no description available"]
pub type Bpflcd2R = crate::BitReader<Bpflcd2>;
impl Bpflcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd2 {
        match self.bits {
            false => Bpflcd2::B0,
            true => Bpflcd2::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd2::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd2::B1
    }
}
#[doc = "Field `BPFLCD2` writer - no description available"]
pub type Bpflcd2W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd2>;
impl<'a, REG> Bpflcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd2::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd2::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd2 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd2> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD2` reader - no description available"]
pub type Bpglcd2R = crate::BitReader<Bpglcd2>;
impl Bpglcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd2 {
        match self.bits {
            false => Bpglcd2::B0,
            true => Bpglcd2::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd2::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd2::B1
    }
}
#[doc = "Field `BPGLCD2` writer - no description available"]
pub type Bpglcd2W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd2>;
impl<'a, REG> Bpglcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd2::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd2::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd2 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd2> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD2` reader - no description available"]
pub type Bphlcd2R = crate::BitReader<Bphlcd2>;
impl Bphlcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd2 {
        match self.bits {
            false => Bphlcd2::B0,
            true => Bphlcd2::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd2::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd2::B1
    }
}
#[doc = "Field `BPHLCD2` writer - no description available"]
pub type Bphlcd2W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd2>;
impl<'a, REG> Bphlcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd2::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd2::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd2(&self) -> Bpalcd2R {
        Bpalcd2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd2(&self) -> Bpblcd2R {
        Bpblcd2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd2(&self) -> Bpclcd2R {
        Bpclcd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd2(&self) -> Bpdlcd2R {
        Bpdlcd2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd2(&self) -> Bpelcd2R {
        Bpelcd2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd2(&self) -> Bpflcd2R {
        Bpflcd2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd2(&self) -> Bpglcd2R {
        Bpglcd2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd2(&self) -> Bphlcd2R {
        Bphlcd2R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd2(&mut self) -> Bpalcd2W<Wf2Spec> {
        Bpalcd2W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd2(&mut self) -> Bpblcd2W<Wf2Spec> {
        Bpblcd2W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd2(&mut self) -> Bpclcd2W<Wf2Spec> {
        Bpclcd2W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd2(&mut self) -> Bpdlcd2W<Wf2Spec> {
        Bpdlcd2W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd2(&mut self) -> Bpelcd2W<Wf2Spec> {
        Bpelcd2W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd2(&mut self) -> Bpflcd2W<Wf2Spec> {
        Bpflcd2W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd2(&mut self) -> Bpglcd2W<Wf2Spec> {
        Bpglcd2W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd2(&mut self) -> Bphlcd2W<Wf2Spec> {
        Bphlcd2W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf2Spec;
impl crate::RegisterSpec for Wf2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf2::R`](R) reader structure"]
impl crate::Readable for Wf2Spec {}
#[doc = "`write(|w| ..)` method takes [`wf2::W`](W) writer structure"]
impl crate::Writable for Wf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF2 to value 0"]
impl crate::Resettable for Wf2Spec {
    const RESET_VALUE: u8 = 0;
}
