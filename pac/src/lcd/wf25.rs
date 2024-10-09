#[doc = "Register `WF25` reader"]
pub type R = crate::R<Wf25Spec>;
#[doc = "Register `WF25` writer"]
pub type W = crate::W<Wf25Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd25 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd25> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD25` reader - no description available"]
pub type Bpalcd25R = crate::BitReader<Bpalcd25>;
impl Bpalcd25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd25 {
        match self.bits {
            false => Bpalcd25::B0,
            true => Bpalcd25::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd25::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd25::B1
    }
}
#[doc = "Field `BPALCD25` writer - no description available"]
pub type Bpalcd25W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd25>;
impl<'a, REG> Bpalcd25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd25::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd25::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd25 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd25> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD25` reader - no description available"]
pub type Bpblcd25R = crate::BitReader<Bpblcd25>;
impl Bpblcd25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd25 {
        match self.bits {
            false => Bpblcd25::B0,
            true => Bpblcd25::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd25::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd25::B1
    }
}
#[doc = "Field `BPBLCD25` writer - no description available"]
pub type Bpblcd25W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd25>;
impl<'a, REG> Bpblcd25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd25::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd25::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd25 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd25> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD25` reader - no description available"]
pub type Bpclcd25R = crate::BitReader<Bpclcd25>;
impl Bpclcd25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd25 {
        match self.bits {
            false => Bpclcd25::B0,
            true => Bpclcd25::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd25::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd25::B1
    }
}
#[doc = "Field `BPCLCD25` writer - no description available"]
pub type Bpclcd25W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd25>;
impl<'a, REG> Bpclcd25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd25::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd25::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd25 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd25> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD25` reader - no description available"]
pub type Bpdlcd25R = crate::BitReader<Bpdlcd25>;
impl Bpdlcd25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd25 {
        match self.bits {
            false => Bpdlcd25::B0,
            true => Bpdlcd25::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd25::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd25::B1
    }
}
#[doc = "Field `BPDLCD25` writer - no description available"]
pub type Bpdlcd25W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd25>;
impl<'a, REG> Bpdlcd25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd25::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd25::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd25 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd25> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD25` reader - no description available"]
pub type Bpelcd25R = crate::BitReader<Bpelcd25>;
impl Bpelcd25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd25 {
        match self.bits {
            false => Bpelcd25::B0,
            true => Bpelcd25::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd25::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd25::B1
    }
}
#[doc = "Field `BPELCD25` writer - no description available"]
pub type Bpelcd25W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd25>;
impl<'a, REG> Bpelcd25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd25::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd25::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd25 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd25> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD25` reader - no description available"]
pub type Bpflcd25R = crate::BitReader<Bpflcd25>;
impl Bpflcd25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd25 {
        match self.bits {
            false => Bpflcd25::B0,
            true => Bpflcd25::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd25::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd25::B1
    }
}
#[doc = "Field `BPFLCD25` writer - no description available"]
pub type Bpflcd25W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd25>;
impl<'a, REG> Bpflcd25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd25::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd25::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd25 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd25> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD25` reader - no description available"]
pub type Bpglcd25R = crate::BitReader<Bpglcd25>;
impl Bpglcd25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd25 {
        match self.bits {
            false => Bpglcd25::B0,
            true => Bpglcd25::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd25::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd25::B1
    }
}
#[doc = "Field `BPGLCD25` writer - no description available"]
pub type Bpglcd25W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd25>;
impl<'a, REG> Bpglcd25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd25::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd25::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd25 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd25> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD25` reader - no description available"]
pub type Bphlcd25R = crate::BitReader<Bphlcd25>;
impl Bphlcd25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd25 {
        match self.bits {
            false => Bphlcd25::B0,
            true => Bphlcd25::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd25::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd25::B1
    }
}
#[doc = "Field `BPHLCD25` writer - no description available"]
pub type Bphlcd25W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd25>;
impl<'a, REG> Bphlcd25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd25::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd25::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd25(&self) -> Bpalcd25R {
        Bpalcd25R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd25(&self) -> Bpblcd25R {
        Bpblcd25R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd25(&self) -> Bpclcd25R {
        Bpclcd25R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd25(&self) -> Bpdlcd25R {
        Bpdlcd25R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd25(&self) -> Bpelcd25R {
        Bpelcd25R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd25(&self) -> Bpflcd25R {
        Bpflcd25R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd25(&self) -> Bpglcd25R {
        Bpglcd25R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd25(&self) -> Bphlcd25R {
        Bphlcd25R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd25(&mut self) -> Bpalcd25W<Wf25Spec> {
        Bpalcd25W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd25(&mut self) -> Bpblcd25W<Wf25Spec> {
        Bpblcd25W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd25(&mut self) -> Bpclcd25W<Wf25Spec> {
        Bpclcd25W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd25(&mut self) -> Bpdlcd25W<Wf25Spec> {
        Bpdlcd25W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd25(&mut self) -> Bpelcd25W<Wf25Spec> {
        Bpelcd25W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd25(&mut self) -> Bpflcd25W<Wf25Spec> {
        Bpflcd25W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd25(&mut self) -> Bpglcd25W<Wf25Spec> {
        Bpglcd25W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd25(&mut self) -> Bphlcd25W<Wf25Spec> {
        Bphlcd25W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 25.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf25Spec;
impl crate::RegisterSpec for Wf25Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf25::R`](R) reader structure"]
impl crate::Readable for Wf25Spec {}
#[doc = "`write(|w| ..)` method takes [`wf25::W`](W) writer structure"]
impl crate::Writable for Wf25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF25 to value 0"]
impl crate::Resettable for Wf25Spec {
    const RESET_VALUE: u8 = 0;
}
