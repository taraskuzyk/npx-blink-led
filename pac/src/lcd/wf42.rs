#[doc = "Register `WF42` reader"]
pub type R = crate::R<Wf42Spec>;
#[doc = "Register `WF42` writer"]
pub type W = crate::W<Wf42Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd42 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd42> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD42` reader - no description available"]
pub type Bpalcd42R = crate::BitReader<Bpalcd42>;
impl Bpalcd42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd42 {
        match self.bits {
            false => Bpalcd42::B0,
            true => Bpalcd42::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd42::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd42::B1
    }
}
#[doc = "Field `BPALCD42` writer - no description available"]
pub type Bpalcd42W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd42>;
impl<'a, REG> Bpalcd42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd42::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd42::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd42 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd42> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD42` reader - no description available"]
pub type Bpblcd42R = crate::BitReader<Bpblcd42>;
impl Bpblcd42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd42 {
        match self.bits {
            false => Bpblcd42::B0,
            true => Bpblcd42::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd42::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd42::B1
    }
}
#[doc = "Field `BPBLCD42` writer - no description available"]
pub type Bpblcd42W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd42>;
impl<'a, REG> Bpblcd42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd42::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd42::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd42 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd42> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD42` reader - no description available"]
pub type Bpclcd42R = crate::BitReader<Bpclcd42>;
impl Bpclcd42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd42 {
        match self.bits {
            false => Bpclcd42::B0,
            true => Bpclcd42::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd42::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd42::B1
    }
}
#[doc = "Field `BPCLCD42` writer - no description available"]
pub type Bpclcd42W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd42>;
impl<'a, REG> Bpclcd42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd42::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd42::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd42 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd42> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD42` reader - no description available"]
pub type Bpdlcd42R = crate::BitReader<Bpdlcd42>;
impl Bpdlcd42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd42 {
        match self.bits {
            false => Bpdlcd42::B0,
            true => Bpdlcd42::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd42::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd42::B1
    }
}
#[doc = "Field `BPDLCD42` writer - no description available"]
pub type Bpdlcd42W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd42>;
impl<'a, REG> Bpdlcd42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd42::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd42::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd42 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd42> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD42` reader - no description available"]
pub type Bpelcd42R = crate::BitReader<Bpelcd42>;
impl Bpelcd42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd42 {
        match self.bits {
            false => Bpelcd42::B0,
            true => Bpelcd42::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd42::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd42::B1
    }
}
#[doc = "Field `BPELCD42` writer - no description available"]
pub type Bpelcd42W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd42>;
impl<'a, REG> Bpelcd42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd42::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd42::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd42 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd42> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD42` reader - no description available"]
pub type Bpflcd42R = crate::BitReader<Bpflcd42>;
impl Bpflcd42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd42 {
        match self.bits {
            false => Bpflcd42::B0,
            true => Bpflcd42::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd42::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd42::B1
    }
}
#[doc = "Field `BPFLCD42` writer - no description available"]
pub type Bpflcd42W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd42>;
impl<'a, REG> Bpflcd42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd42::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd42::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd42 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd42> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD42` reader - no description available"]
pub type Bpglcd42R = crate::BitReader<Bpglcd42>;
impl Bpglcd42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd42 {
        match self.bits {
            false => Bpglcd42::B0,
            true => Bpglcd42::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd42::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd42::B1
    }
}
#[doc = "Field `BPGLCD42` writer - no description available"]
pub type Bpglcd42W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd42>;
impl<'a, REG> Bpglcd42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd42::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd42::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd42 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd42> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd42) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD42` reader - no description available"]
pub type Bphlcd42R = crate::BitReader<Bphlcd42>;
impl Bphlcd42R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd42 {
        match self.bits {
            false => Bphlcd42::B0,
            true => Bphlcd42::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd42::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd42::B1
    }
}
#[doc = "Field `BPHLCD42` writer - no description available"]
pub type Bphlcd42W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd42>;
impl<'a, REG> Bphlcd42W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd42::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd42::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd42(&self) -> Bpalcd42R {
        Bpalcd42R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd42(&self) -> Bpblcd42R {
        Bpblcd42R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd42(&self) -> Bpclcd42R {
        Bpclcd42R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd42(&self) -> Bpdlcd42R {
        Bpdlcd42R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd42(&self) -> Bpelcd42R {
        Bpelcd42R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd42(&self) -> Bpflcd42R {
        Bpflcd42R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd42(&self) -> Bpglcd42R {
        Bpglcd42R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd42(&self) -> Bphlcd42R {
        Bphlcd42R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd42(&mut self) -> Bpalcd42W<Wf42Spec> {
        Bpalcd42W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd42(&mut self) -> Bpblcd42W<Wf42Spec> {
        Bpblcd42W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd42(&mut self) -> Bpclcd42W<Wf42Spec> {
        Bpclcd42W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd42(&mut self) -> Bpdlcd42W<Wf42Spec> {
        Bpdlcd42W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd42(&mut self) -> Bpelcd42W<Wf42Spec> {
        Bpelcd42W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd42(&mut self) -> Bpflcd42W<Wf42Spec> {
        Bpflcd42W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd42(&mut self) -> Bpglcd42W<Wf42Spec> {
        Bpglcd42W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd42(&mut self) -> Bphlcd42W<Wf42Spec> {
        Bphlcd42W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 42.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf42::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf42::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf42Spec;
impl crate::RegisterSpec for Wf42Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf42::R`](R) reader structure"]
impl crate::Readable for Wf42Spec {}
#[doc = "`write(|w| ..)` method takes [`wf42::W`](W) writer structure"]
impl crate::Writable for Wf42Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF42 to value 0"]
impl crate::Resettable for Wf42Spec {
    const RESET_VALUE: u8 = 0;
}
