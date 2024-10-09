#[doc = "Register `WF63` reader"]
pub type R = crate::R<Wf63Spec>;
#[doc = "Register `WF63` writer"]
pub type W = crate::W<Wf63Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd63 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd63> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD63` reader - no description available"]
pub type Bpalcd63R = crate::BitReader<Bpalcd63>;
impl Bpalcd63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd63 {
        match self.bits {
            false => Bpalcd63::B0,
            true => Bpalcd63::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd63::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd63::B1
    }
}
#[doc = "Field `BPALCD63` writer - no description available"]
pub type Bpalcd63W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd63>;
impl<'a, REG> Bpalcd63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd63::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd63::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd63 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd63> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD63` reader - no description available"]
pub type Bpblcd63R = crate::BitReader<Bpblcd63>;
impl Bpblcd63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd63 {
        match self.bits {
            false => Bpblcd63::B0,
            true => Bpblcd63::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd63::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd63::B1
    }
}
#[doc = "Field `BPBLCD63` writer - no description available"]
pub type Bpblcd63W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd63>;
impl<'a, REG> Bpblcd63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd63::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd63::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd63 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd63> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD63` reader - no description available"]
pub type Bpclcd63R = crate::BitReader<Bpclcd63>;
impl Bpclcd63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd63 {
        match self.bits {
            false => Bpclcd63::B0,
            true => Bpclcd63::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd63::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd63::B1
    }
}
#[doc = "Field `BPCLCD63` writer - no description available"]
pub type Bpclcd63W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd63>;
impl<'a, REG> Bpclcd63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd63::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd63::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd63 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd63> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD63` reader - no description available"]
pub type Bpdlcd63R = crate::BitReader<Bpdlcd63>;
impl Bpdlcd63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd63 {
        match self.bits {
            false => Bpdlcd63::B0,
            true => Bpdlcd63::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd63::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd63::B1
    }
}
#[doc = "Field `BPDLCD63` writer - no description available"]
pub type Bpdlcd63W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd63>;
impl<'a, REG> Bpdlcd63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd63::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd63::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd63 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd63> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD63` reader - no description available"]
pub type Bpelcd63R = crate::BitReader<Bpelcd63>;
impl Bpelcd63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd63 {
        match self.bits {
            false => Bpelcd63::B0,
            true => Bpelcd63::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd63::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd63::B1
    }
}
#[doc = "Field `BPELCD63` writer - no description available"]
pub type Bpelcd63W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd63>;
impl<'a, REG> Bpelcd63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd63::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd63::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd63 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd63> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD63` reader - no description available"]
pub type Bpflcd63R = crate::BitReader<Bpflcd63>;
impl Bpflcd63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd63 {
        match self.bits {
            false => Bpflcd63::B0,
            true => Bpflcd63::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd63::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd63::B1
    }
}
#[doc = "Field `BPFLCD63` writer - no description available"]
pub type Bpflcd63W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd63>;
impl<'a, REG> Bpflcd63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd63::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd63::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd63 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd63> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD63` reader - no description available"]
pub type Bpglcd63R = crate::BitReader<Bpglcd63>;
impl Bpglcd63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd63 {
        match self.bits {
            false => Bpglcd63::B0,
            true => Bpglcd63::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd63::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd63::B1
    }
}
#[doc = "Field `BPGLCD63` writer - no description available"]
pub type Bpglcd63W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd63>;
impl<'a, REG> Bpglcd63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd63::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd63::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd63 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd63> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd63) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD63` reader - no description available"]
pub type Bphlcd63R = crate::BitReader<Bphlcd63>;
impl Bphlcd63R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd63 {
        match self.bits {
            false => Bphlcd63::B0,
            true => Bphlcd63::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd63::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd63::B1
    }
}
#[doc = "Field `BPHLCD63` writer - no description available"]
pub type Bphlcd63W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd63>;
impl<'a, REG> Bphlcd63W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd63::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd63::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd63(&self) -> Bpalcd63R {
        Bpalcd63R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd63(&self) -> Bpblcd63R {
        Bpblcd63R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd63(&self) -> Bpclcd63R {
        Bpclcd63R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd63(&self) -> Bpdlcd63R {
        Bpdlcd63R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd63(&self) -> Bpelcd63R {
        Bpelcd63R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd63(&self) -> Bpflcd63R {
        Bpflcd63R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd63(&self) -> Bpglcd63R {
        Bpglcd63R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd63(&self) -> Bphlcd63R {
        Bphlcd63R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd63(&mut self) -> Bpalcd63W<Wf63Spec> {
        Bpalcd63W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd63(&mut self) -> Bpblcd63W<Wf63Spec> {
        Bpblcd63W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd63(&mut self) -> Bpclcd63W<Wf63Spec> {
        Bpclcd63W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd63(&mut self) -> Bpdlcd63W<Wf63Spec> {
        Bpdlcd63W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd63(&mut self) -> Bpelcd63W<Wf63Spec> {
        Bpelcd63W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd63(&mut self) -> Bpflcd63W<Wf63Spec> {
        Bpflcd63W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd63(&mut self) -> Bpglcd63W<Wf63Spec> {
        Bpglcd63W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd63(&mut self) -> Bphlcd63W<Wf63Spec> {
        Bphlcd63W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 63.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf63::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf63::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf63Spec;
impl crate::RegisterSpec for Wf63Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf63::R`](R) reader structure"]
impl crate::Readable for Wf63Spec {}
#[doc = "`write(|w| ..)` method takes [`wf63::W`](W) writer structure"]
impl crate::Writable for Wf63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF63 to value 0"]
impl crate::Resettable for Wf63Spec {
    const RESET_VALUE: u8 = 0;
}
