#[doc = "Register `WF50` reader"]
pub type R = crate::R<Wf50Spec>;
#[doc = "Register `WF50` writer"]
pub type W = crate::W<Wf50Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd50 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd50> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD50` reader - no description available"]
pub type Bpalcd50R = crate::BitReader<Bpalcd50>;
impl Bpalcd50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd50 {
        match self.bits {
            false => Bpalcd50::B0,
            true => Bpalcd50::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd50::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd50::B1
    }
}
#[doc = "Field `BPALCD50` writer - no description available"]
pub type Bpalcd50W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd50>;
impl<'a, REG> Bpalcd50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd50::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd50::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd50 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd50> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD50` reader - no description available"]
pub type Bpblcd50R = crate::BitReader<Bpblcd50>;
impl Bpblcd50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd50 {
        match self.bits {
            false => Bpblcd50::B0,
            true => Bpblcd50::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd50::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd50::B1
    }
}
#[doc = "Field `BPBLCD50` writer - no description available"]
pub type Bpblcd50W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd50>;
impl<'a, REG> Bpblcd50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd50::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd50::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd50 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd50> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD50` reader - no description available"]
pub type Bpclcd50R = crate::BitReader<Bpclcd50>;
impl Bpclcd50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd50 {
        match self.bits {
            false => Bpclcd50::B0,
            true => Bpclcd50::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd50::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd50::B1
    }
}
#[doc = "Field `BPCLCD50` writer - no description available"]
pub type Bpclcd50W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd50>;
impl<'a, REG> Bpclcd50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd50::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd50::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd50 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd50> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD50` reader - no description available"]
pub type Bpdlcd50R = crate::BitReader<Bpdlcd50>;
impl Bpdlcd50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd50 {
        match self.bits {
            false => Bpdlcd50::B0,
            true => Bpdlcd50::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd50::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd50::B1
    }
}
#[doc = "Field `BPDLCD50` writer - no description available"]
pub type Bpdlcd50W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd50>;
impl<'a, REG> Bpdlcd50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd50::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd50::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd50 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd50> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD50` reader - no description available"]
pub type Bpelcd50R = crate::BitReader<Bpelcd50>;
impl Bpelcd50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd50 {
        match self.bits {
            false => Bpelcd50::B0,
            true => Bpelcd50::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd50::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd50::B1
    }
}
#[doc = "Field `BPELCD50` writer - no description available"]
pub type Bpelcd50W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd50>;
impl<'a, REG> Bpelcd50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd50::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd50::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd50 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd50> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD50` reader - no description available"]
pub type Bpflcd50R = crate::BitReader<Bpflcd50>;
impl Bpflcd50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd50 {
        match self.bits {
            false => Bpflcd50::B0,
            true => Bpflcd50::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd50::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd50::B1
    }
}
#[doc = "Field `BPFLCD50` writer - no description available"]
pub type Bpflcd50W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd50>;
impl<'a, REG> Bpflcd50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd50::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd50::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd50 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd50> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD50` reader - no description available"]
pub type Bpglcd50R = crate::BitReader<Bpglcd50>;
impl Bpglcd50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd50 {
        match self.bits {
            false => Bpglcd50::B0,
            true => Bpglcd50::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd50::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd50::B1
    }
}
#[doc = "Field `BPGLCD50` writer - no description available"]
pub type Bpglcd50W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd50>;
impl<'a, REG> Bpglcd50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd50::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd50::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd50 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd50> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd50) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD50` reader - no description available"]
pub type Bphlcd50R = crate::BitReader<Bphlcd50>;
impl Bphlcd50R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd50 {
        match self.bits {
            false => Bphlcd50::B0,
            true => Bphlcd50::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd50::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd50::B1
    }
}
#[doc = "Field `BPHLCD50` writer - no description available"]
pub type Bphlcd50W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd50>;
impl<'a, REG> Bphlcd50W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd50::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd50::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd50(&self) -> Bpalcd50R {
        Bpalcd50R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd50(&self) -> Bpblcd50R {
        Bpblcd50R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd50(&self) -> Bpclcd50R {
        Bpclcd50R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd50(&self) -> Bpdlcd50R {
        Bpdlcd50R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd50(&self) -> Bpelcd50R {
        Bpelcd50R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd50(&self) -> Bpflcd50R {
        Bpflcd50R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd50(&self) -> Bpglcd50R {
        Bpglcd50R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd50(&self) -> Bphlcd50R {
        Bphlcd50R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd50(&mut self) -> Bpalcd50W<Wf50Spec> {
        Bpalcd50W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd50(&mut self) -> Bpblcd50W<Wf50Spec> {
        Bpblcd50W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd50(&mut self) -> Bpclcd50W<Wf50Spec> {
        Bpclcd50W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd50(&mut self) -> Bpdlcd50W<Wf50Spec> {
        Bpdlcd50W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd50(&mut self) -> Bpelcd50W<Wf50Spec> {
        Bpelcd50W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd50(&mut self) -> Bpflcd50W<Wf50Spec> {
        Bpflcd50W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd50(&mut self) -> Bpglcd50W<Wf50Spec> {
        Bpglcd50W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd50(&mut self) -> Bphlcd50W<Wf50Spec> {
        Bphlcd50W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 50.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf50Spec;
impl crate::RegisterSpec for Wf50Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf50::R`](R) reader structure"]
impl crate::Readable for Wf50Spec {}
#[doc = "`write(|w| ..)` method takes [`wf50::W`](W) writer structure"]
impl crate::Writable for Wf50Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF50 to value 0"]
impl crate::Resettable for Wf50Spec {
    const RESET_VALUE: u8 = 0;
}
