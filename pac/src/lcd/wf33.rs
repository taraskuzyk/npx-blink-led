#[doc = "Register `WF33` reader"]
pub type R = crate::R<Wf33Spec>;
#[doc = "Register `WF33` writer"]
pub type W = crate::W<Wf33Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd33 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd33> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD33` reader - no description available"]
pub type Bpalcd33R = crate::BitReader<Bpalcd33>;
impl Bpalcd33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd33 {
        match self.bits {
            false => Bpalcd33::B0,
            true => Bpalcd33::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd33::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd33::B1
    }
}
#[doc = "Field `BPALCD33` writer - no description available"]
pub type Bpalcd33W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd33>;
impl<'a, REG> Bpalcd33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd33::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd33::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd33 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd33> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD33` reader - no description available"]
pub type Bpblcd33R = crate::BitReader<Bpblcd33>;
impl Bpblcd33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd33 {
        match self.bits {
            false => Bpblcd33::B0,
            true => Bpblcd33::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd33::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd33::B1
    }
}
#[doc = "Field `BPBLCD33` writer - no description available"]
pub type Bpblcd33W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd33>;
impl<'a, REG> Bpblcd33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd33::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd33::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd33 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd33> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD33` reader - no description available"]
pub type Bpclcd33R = crate::BitReader<Bpclcd33>;
impl Bpclcd33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd33 {
        match self.bits {
            false => Bpclcd33::B0,
            true => Bpclcd33::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd33::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd33::B1
    }
}
#[doc = "Field `BPCLCD33` writer - no description available"]
pub type Bpclcd33W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd33>;
impl<'a, REG> Bpclcd33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd33::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd33::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd33 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd33> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD33` reader - no description available"]
pub type Bpdlcd33R = crate::BitReader<Bpdlcd33>;
impl Bpdlcd33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd33 {
        match self.bits {
            false => Bpdlcd33::B0,
            true => Bpdlcd33::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd33::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd33::B1
    }
}
#[doc = "Field `BPDLCD33` writer - no description available"]
pub type Bpdlcd33W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd33>;
impl<'a, REG> Bpdlcd33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd33::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd33::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd33 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd33> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD33` reader - no description available"]
pub type Bpelcd33R = crate::BitReader<Bpelcd33>;
impl Bpelcd33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd33 {
        match self.bits {
            false => Bpelcd33::B0,
            true => Bpelcd33::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd33::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd33::B1
    }
}
#[doc = "Field `BPELCD33` writer - no description available"]
pub type Bpelcd33W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd33>;
impl<'a, REG> Bpelcd33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd33::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd33::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd33 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd33> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD33` reader - no description available"]
pub type Bpflcd33R = crate::BitReader<Bpflcd33>;
impl Bpflcd33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd33 {
        match self.bits {
            false => Bpflcd33::B0,
            true => Bpflcd33::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd33::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd33::B1
    }
}
#[doc = "Field `BPFLCD33` writer - no description available"]
pub type Bpflcd33W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd33>;
impl<'a, REG> Bpflcd33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd33::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd33::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd33 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd33> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD33` reader - no description available"]
pub type Bpglcd33R = crate::BitReader<Bpglcd33>;
impl Bpglcd33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd33 {
        match self.bits {
            false => Bpglcd33::B0,
            true => Bpglcd33::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd33::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd33::B1
    }
}
#[doc = "Field `BPGLCD33` writer - no description available"]
pub type Bpglcd33W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd33>;
impl<'a, REG> Bpglcd33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd33::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd33::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd33 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd33> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd33) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD33` reader - no description available"]
pub type Bphlcd33R = crate::BitReader<Bphlcd33>;
impl Bphlcd33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd33 {
        match self.bits {
            false => Bphlcd33::B0,
            true => Bphlcd33::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd33::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd33::B1
    }
}
#[doc = "Field `BPHLCD33` writer - no description available"]
pub type Bphlcd33W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd33>;
impl<'a, REG> Bphlcd33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd33::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd33::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd33(&self) -> Bpalcd33R {
        Bpalcd33R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd33(&self) -> Bpblcd33R {
        Bpblcd33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd33(&self) -> Bpclcd33R {
        Bpclcd33R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd33(&self) -> Bpdlcd33R {
        Bpdlcd33R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd33(&self) -> Bpelcd33R {
        Bpelcd33R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd33(&self) -> Bpflcd33R {
        Bpflcd33R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd33(&self) -> Bpglcd33R {
        Bpglcd33R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd33(&self) -> Bphlcd33R {
        Bphlcd33R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd33(&mut self) -> Bpalcd33W<Wf33Spec> {
        Bpalcd33W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd33(&mut self) -> Bpblcd33W<Wf33Spec> {
        Bpblcd33W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd33(&mut self) -> Bpclcd33W<Wf33Spec> {
        Bpclcd33W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd33(&mut self) -> Bpdlcd33W<Wf33Spec> {
        Bpdlcd33W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd33(&mut self) -> Bpelcd33W<Wf33Spec> {
        Bpelcd33W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd33(&mut self) -> Bpflcd33W<Wf33Spec> {
        Bpflcd33W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd33(&mut self) -> Bpglcd33W<Wf33Spec> {
        Bpglcd33W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd33(&mut self) -> Bphlcd33W<Wf33Spec> {
        Bphlcd33W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 33.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf33Spec;
impl crate::RegisterSpec for Wf33Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf33::R`](R) reader structure"]
impl crate::Readable for Wf33Spec {}
#[doc = "`write(|w| ..)` method takes [`wf33::W`](W) writer structure"]
impl crate::Writable for Wf33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF33 to value 0"]
impl crate::Resettable for Wf33Spec {
    const RESET_VALUE: u8 = 0;
}
