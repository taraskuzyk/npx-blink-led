#[doc = "Register `WF23` reader"]
pub type R = crate::R<Wf23Spec>;
#[doc = "Register `WF23` writer"]
pub type W = crate::W<Wf23Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd23 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd23> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD23` reader - no description available"]
pub type Bpalcd23R = crate::BitReader<Bpalcd23>;
impl Bpalcd23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd23 {
        match self.bits {
            false => Bpalcd23::B0,
            true => Bpalcd23::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd23::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd23::B1
    }
}
#[doc = "Field `BPALCD23` writer - no description available"]
pub type Bpalcd23W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd23>;
impl<'a, REG> Bpalcd23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd23::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd23::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd23 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd23> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD23` reader - no description available"]
pub type Bpblcd23R = crate::BitReader<Bpblcd23>;
impl Bpblcd23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd23 {
        match self.bits {
            false => Bpblcd23::B0,
            true => Bpblcd23::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd23::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd23::B1
    }
}
#[doc = "Field `BPBLCD23` writer - no description available"]
pub type Bpblcd23W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd23>;
impl<'a, REG> Bpblcd23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd23::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd23::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd23 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd23> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD23` reader - no description available"]
pub type Bpclcd23R = crate::BitReader<Bpclcd23>;
impl Bpclcd23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd23 {
        match self.bits {
            false => Bpclcd23::B0,
            true => Bpclcd23::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd23::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd23::B1
    }
}
#[doc = "Field `BPCLCD23` writer - no description available"]
pub type Bpclcd23W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd23>;
impl<'a, REG> Bpclcd23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd23::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd23::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd23 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd23> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD23` reader - no description available"]
pub type Bpdlcd23R = crate::BitReader<Bpdlcd23>;
impl Bpdlcd23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd23 {
        match self.bits {
            false => Bpdlcd23::B0,
            true => Bpdlcd23::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd23::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd23::B1
    }
}
#[doc = "Field `BPDLCD23` writer - no description available"]
pub type Bpdlcd23W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd23>;
impl<'a, REG> Bpdlcd23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd23::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd23::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd23 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd23> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD23` reader - no description available"]
pub type Bpelcd23R = crate::BitReader<Bpelcd23>;
impl Bpelcd23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd23 {
        match self.bits {
            false => Bpelcd23::B0,
            true => Bpelcd23::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd23::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd23::B1
    }
}
#[doc = "Field `BPELCD23` writer - no description available"]
pub type Bpelcd23W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd23>;
impl<'a, REG> Bpelcd23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd23::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd23::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd23 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd23> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD23` reader - no description available"]
pub type Bpflcd23R = crate::BitReader<Bpflcd23>;
impl Bpflcd23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd23 {
        match self.bits {
            false => Bpflcd23::B0,
            true => Bpflcd23::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd23::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd23::B1
    }
}
#[doc = "Field `BPFLCD23` writer - no description available"]
pub type Bpflcd23W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd23>;
impl<'a, REG> Bpflcd23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd23::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd23::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd23 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd23> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD23` reader - no description available"]
pub type Bpglcd23R = crate::BitReader<Bpglcd23>;
impl Bpglcd23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd23 {
        match self.bits {
            false => Bpglcd23::B0,
            true => Bpglcd23::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd23::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd23::B1
    }
}
#[doc = "Field `BPGLCD23` writer - no description available"]
pub type Bpglcd23W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd23>;
impl<'a, REG> Bpglcd23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd23::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd23::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd23 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd23> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD23` reader - no description available"]
pub type Bphlcd23R = crate::BitReader<Bphlcd23>;
impl Bphlcd23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd23 {
        match self.bits {
            false => Bphlcd23::B0,
            true => Bphlcd23::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd23::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd23::B1
    }
}
#[doc = "Field `BPHLCD23` writer - no description available"]
pub type Bphlcd23W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd23>;
impl<'a, REG> Bphlcd23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd23::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd23::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd23(&self) -> Bpalcd23R {
        Bpalcd23R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd23(&self) -> Bpblcd23R {
        Bpblcd23R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd23(&self) -> Bpclcd23R {
        Bpclcd23R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd23(&self) -> Bpdlcd23R {
        Bpdlcd23R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd23(&self) -> Bpelcd23R {
        Bpelcd23R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd23(&self) -> Bpflcd23R {
        Bpflcd23R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd23(&self) -> Bpglcd23R {
        Bpglcd23R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd23(&self) -> Bphlcd23R {
        Bphlcd23R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd23(&mut self) -> Bpalcd23W<Wf23Spec> {
        Bpalcd23W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd23(&mut self) -> Bpblcd23W<Wf23Spec> {
        Bpblcd23W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd23(&mut self) -> Bpclcd23W<Wf23Spec> {
        Bpclcd23W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd23(&mut self) -> Bpdlcd23W<Wf23Spec> {
        Bpdlcd23W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd23(&mut self) -> Bpelcd23W<Wf23Spec> {
        Bpelcd23W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd23(&mut self) -> Bpflcd23W<Wf23Spec> {
        Bpflcd23W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd23(&mut self) -> Bpglcd23W<Wf23Spec> {
        Bpglcd23W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd23(&mut self) -> Bphlcd23W<Wf23Spec> {
        Bphlcd23W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 23.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf23Spec;
impl crate::RegisterSpec for Wf23Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf23::R`](R) reader structure"]
impl crate::Readable for Wf23Spec {}
#[doc = "`write(|w| ..)` method takes [`wf23::W`](W) writer structure"]
impl crate::Writable for Wf23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF23 to value 0"]
impl crate::Resettable for Wf23Spec {
    const RESET_VALUE: u8 = 0;
}
