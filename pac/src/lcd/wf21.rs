#[doc = "Register `WF21` reader"]
pub type R = crate::R<Wf21Spec>;
#[doc = "Register `WF21` writer"]
pub type W = crate::W<Wf21Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd21 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd21> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD21` reader - no description available"]
pub type Bpalcd21R = crate::BitReader<Bpalcd21>;
impl Bpalcd21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd21 {
        match self.bits {
            false => Bpalcd21::B0,
            true => Bpalcd21::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd21::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd21::B1
    }
}
#[doc = "Field `BPALCD21` writer - no description available"]
pub type Bpalcd21W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd21>;
impl<'a, REG> Bpalcd21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd21::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd21::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd21 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd21> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD21` reader - no description available"]
pub type Bpblcd21R = crate::BitReader<Bpblcd21>;
impl Bpblcd21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd21 {
        match self.bits {
            false => Bpblcd21::B0,
            true => Bpblcd21::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd21::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd21::B1
    }
}
#[doc = "Field `BPBLCD21` writer - no description available"]
pub type Bpblcd21W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd21>;
impl<'a, REG> Bpblcd21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd21::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd21::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd21 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd21> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD21` reader - no description available"]
pub type Bpclcd21R = crate::BitReader<Bpclcd21>;
impl Bpclcd21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd21 {
        match self.bits {
            false => Bpclcd21::B0,
            true => Bpclcd21::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd21::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd21::B1
    }
}
#[doc = "Field `BPCLCD21` writer - no description available"]
pub type Bpclcd21W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd21>;
impl<'a, REG> Bpclcd21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd21::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd21::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd21 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd21> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD21` reader - no description available"]
pub type Bpdlcd21R = crate::BitReader<Bpdlcd21>;
impl Bpdlcd21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd21 {
        match self.bits {
            false => Bpdlcd21::B0,
            true => Bpdlcd21::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd21::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd21::B1
    }
}
#[doc = "Field `BPDLCD21` writer - no description available"]
pub type Bpdlcd21W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd21>;
impl<'a, REG> Bpdlcd21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd21::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd21::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd21 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd21> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD21` reader - no description available"]
pub type Bpelcd21R = crate::BitReader<Bpelcd21>;
impl Bpelcd21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd21 {
        match self.bits {
            false => Bpelcd21::B0,
            true => Bpelcd21::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd21::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd21::B1
    }
}
#[doc = "Field `BPELCD21` writer - no description available"]
pub type Bpelcd21W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd21>;
impl<'a, REG> Bpelcd21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd21::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd21::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd21 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd21> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD21` reader - no description available"]
pub type Bpflcd21R = crate::BitReader<Bpflcd21>;
impl Bpflcd21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd21 {
        match self.bits {
            false => Bpflcd21::B0,
            true => Bpflcd21::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd21::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd21::B1
    }
}
#[doc = "Field `BPFLCD21` writer - no description available"]
pub type Bpflcd21W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd21>;
impl<'a, REG> Bpflcd21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd21::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd21::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd21 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd21> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD21` reader - no description available"]
pub type Bpglcd21R = crate::BitReader<Bpglcd21>;
impl Bpglcd21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd21 {
        match self.bits {
            false => Bpglcd21::B0,
            true => Bpglcd21::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd21::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd21::B1
    }
}
#[doc = "Field `BPGLCD21` writer - no description available"]
pub type Bpglcd21W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd21>;
impl<'a, REG> Bpglcd21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd21::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd21::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd21 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd21> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD21` reader - no description available"]
pub type Bphlcd21R = crate::BitReader<Bphlcd21>;
impl Bphlcd21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd21 {
        match self.bits {
            false => Bphlcd21::B0,
            true => Bphlcd21::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd21::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd21::B1
    }
}
#[doc = "Field `BPHLCD21` writer - no description available"]
pub type Bphlcd21W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd21>;
impl<'a, REG> Bphlcd21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd21::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd21::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd21(&self) -> Bpalcd21R {
        Bpalcd21R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd21(&self) -> Bpblcd21R {
        Bpblcd21R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd21(&self) -> Bpclcd21R {
        Bpclcd21R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd21(&self) -> Bpdlcd21R {
        Bpdlcd21R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd21(&self) -> Bpelcd21R {
        Bpelcd21R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd21(&self) -> Bpflcd21R {
        Bpflcd21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd21(&self) -> Bpglcd21R {
        Bpglcd21R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd21(&self) -> Bphlcd21R {
        Bphlcd21R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd21(&mut self) -> Bpalcd21W<Wf21Spec> {
        Bpalcd21W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd21(&mut self) -> Bpblcd21W<Wf21Spec> {
        Bpblcd21W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd21(&mut self) -> Bpclcd21W<Wf21Spec> {
        Bpclcd21W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd21(&mut self) -> Bpdlcd21W<Wf21Spec> {
        Bpdlcd21W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd21(&mut self) -> Bpelcd21W<Wf21Spec> {
        Bpelcd21W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd21(&mut self) -> Bpflcd21W<Wf21Spec> {
        Bpflcd21W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd21(&mut self) -> Bpglcd21W<Wf21Spec> {
        Bpglcd21W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd21(&mut self) -> Bphlcd21W<Wf21Spec> {
        Bphlcd21W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 21.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf21Spec;
impl crate::RegisterSpec for Wf21Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf21::R`](R) reader structure"]
impl crate::Readable for Wf21Spec {}
#[doc = "`write(|w| ..)` method takes [`wf21::W`](W) writer structure"]
impl crate::Writable for Wf21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF21 to value 0"]
impl crate::Resettable for Wf21Spec {
    const RESET_VALUE: u8 = 0;
}
