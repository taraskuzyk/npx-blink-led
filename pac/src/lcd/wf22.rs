#[doc = "Register `WF22` reader"]
pub type R = crate::R<Wf22Spec>;
#[doc = "Register `WF22` writer"]
pub type W = crate::W<Wf22Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd22 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd22> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD22` reader - no description available"]
pub type Bpalcd22R = crate::BitReader<Bpalcd22>;
impl Bpalcd22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd22 {
        match self.bits {
            false => Bpalcd22::B0,
            true => Bpalcd22::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd22::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd22::B1
    }
}
#[doc = "Field `BPALCD22` writer - no description available"]
pub type Bpalcd22W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd22>;
impl<'a, REG> Bpalcd22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd22::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd22::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd22 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd22> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD22` reader - no description available"]
pub type Bpblcd22R = crate::BitReader<Bpblcd22>;
impl Bpblcd22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd22 {
        match self.bits {
            false => Bpblcd22::B0,
            true => Bpblcd22::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd22::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd22::B1
    }
}
#[doc = "Field `BPBLCD22` writer - no description available"]
pub type Bpblcd22W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd22>;
impl<'a, REG> Bpblcd22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd22::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd22::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd22 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd22> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD22` reader - no description available"]
pub type Bpclcd22R = crate::BitReader<Bpclcd22>;
impl Bpclcd22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd22 {
        match self.bits {
            false => Bpclcd22::B0,
            true => Bpclcd22::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd22::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd22::B1
    }
}
#[doc = "Field `BPCLCD22` writer - no description available"]
pub type Bpclcd22W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd22>;
impl<'a, REG> Bpclcd22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd22::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd22::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd22 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd22> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD22` reader - no description available"]
pub type Bpdlcd22R = crate::BitReader<Bpdlcd22>;
impl Bpdlcd22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd22 {
        match self.bits {
            false => Bpdlcd22::B0,
            true => Bpdlcd22::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd22::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd22::B1
    }
}
#[doc = "Field `BPDLCD22` writer - no description available"]
pub type Bpdlcd22W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd22>;
impl<'a, REG> Bpdlcd22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd22::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd22::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd22 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd22> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD22` reader - no description available"]
pub type Bpelcd22R = crate::BitReader<Bpelcd22>;
impl Bpelcd22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd22 {
        match self.bits {
            false => Bpelcd22::B0,
            true => Bpelcd22::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd22::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd22::B1
    }
}
#[doc = "Field `BPELCD22` writer - no description available"]
pub type Bpelcd22W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd22>;
impl<'a, REG> Bpelcd22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd22::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd22::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd22 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd22> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD22` reader - no description available"]
pub type Bpflcd22R = crate::BitReader<Bpflcd22>;
impl Bpflcd22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd22 {
        match self.bits {
            false => Bpflcd22::B0,
            true => Bpflcd22::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd22::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd22::B1
    }
}
#[doc = "Field `BPFLCD22` writer - no description available"]
pub type Bpflcd22W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd22>;
impl<'a, REG> Bpflcd22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd22::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd22::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd22 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd22> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD22` reader - no description available"]
pub type Bpglcd22R = crate::BitReader<Bpglcd22>;
impl Bpglcd22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd22 {
        match self.bits {
            false => Bpglcd22::B0,
            true => Bpglcd22::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd22::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd22::B1
    }
}
#[doc = "Field `BPGLCD22` writer - no description available"]
pub type Bpglcd22W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd22>;
impl<'a, REG> Bpglcd22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd22::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd22::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd22 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd22> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD22` reader - no description available"]
pub type Bphlcd22R = crate::BitReader<Bphlcd22>;
impl Bphlcd22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd22 {
        match self.bits {
            false => Bphlcd22::B0,
            true => Bphlcd22::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd22::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd22::B1
    }
}
#[doc = "Field `BPHLCD22` writer - no description available"]
pub type Bphlcd22W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd22>;
impl<'a, REG> Bphlcd22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd22::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd22::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd22(&self) -> Bpalcd22R {
        Bpalcd22R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd22(&self) -> Bpblcd22R {
        Bpblcd22R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd22(&self) -> Bpclcd22R {
        Bpclcd22R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd22(&self) -> Bpdlcd22R {
        Bpdlcd22R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd22(&self) -> Bpelcd22R {
        Bpelcd22R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd22(&self) -> Bpflcd22R {
        Bpflcd22R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd22(&self) -> Bpglcd22R {
        Bpglcd22R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd22(&self) -> Bphlcd22R {
        Bphlcd22R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd22(&mut self) -> Bpalcd22W<Wf22Spec> {
        Bpalcd22W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd22(&mut self) -> Bpblcd22W<Wf22Spec> {
        Bpblcd22W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd22(&mut self) -> Bpclcd22W<Wf22Spec> {
        Bpclcd22W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd22(&mut self) -> Bpdlcd22W<Wf22Spec> {
        Bpdlcd22W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd22(&mut self) -> Bpelcd22W<Wf22Spec> {
        Bpelcd22W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd22(&mut self) -> Bpflcd22W<Wf22Spec> {
        Bpflcd22W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd22(&mut self) -> Bpglcd22W<Wf22Spec> {
        Bpglcd22W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd22(&mut self) -> Bphlcd22W<Wf22Spec> {
        Bphlcd22W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 22.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf22Spec;
impl crate::RegisterSpec for Wf22Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf22::R`](R) reader structure"]
impl crate::Readable for Wf22Spec {}
#[doc = "`write(|w| ..)` method takes [`wf22::W`](W) writer structure"]
impl crate::Writable for Wf22Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF22 to value 0"]
impl crate::Resettable for Wf22Spec {
    const RESET_VALUE: u8 = 0;
}
