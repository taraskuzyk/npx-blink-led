#[doc = "Register `WF35` reader"]
pub type R = crate::R<Wf35Spec>;
#[doc = "Register `WF35` writer"]
pub type W = crate::W<Wf35Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd35 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd35> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD35` reader - no description available"]
pub type Bpalcd35R = crate::BitReader<Bpalcd35>;
impl Bpalcd35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd35 {
        match self.bits {
            false => Bpalcd35::B0,
            true => Bpalcd35::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd35::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd35::B1
    }
}
#[doc = "Field `BPALCD35` writer - no description available"]
pub type Bpalcd35W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd35>;
impl<'a, REG> Bpalcd35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd35::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd35::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd35 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd35> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD35` reader - no description available"]
pub type Bpblcd35R = crate::BitReader<Bpblcd35>;
impl Bpblcd35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd35 {
        match self.bits {
            false => Bpblcd35::B0,
            true => Bpblcd35::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd35::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd35::B1
    }
}
#[doc = "Field `BPBLCD35` writer - no description available"]
pub type Bpblcd35W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd35>;
impl<'a, REG> Bpblcd35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd35::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd35::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd35 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd35> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD35` reader - no description available"]
pub type Bpclcd35R = crate::BitReader<Bpclcd35>;
impl Bpclcd35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd35 {
        match self.bits {
            false => Bpclcd35::B0,
            true => Bpclcd35::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd35::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd35::B1
    }
}
#[doc = "Field `BPCLCD35` writer - no description available"]
pub type Bpclcd35W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd35>;
impl<'a, REG> Bpclcd35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd35::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd35::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd35 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd35> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD35` reader - no description available"]
pub type Bpdlcd35R = crate::BitReader<Bpdlcd35>;
impl Bpdlcd35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd35 {
        match self.bits {
            false => Bpdlcd35::B0,
            true => Bpdlcd35::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd35::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd35::B1
    }
}
#[doc = "Field `BPDLCD35` writer - no description available"]
pub type Bpdlcd35W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd35>;
impl<'a, REG> Bpdlcd35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd35::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd35::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd35 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd35> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD35` reader - no description available"]
pub type Bpelcd35R = crate::BitReader<Bpelcd35>;
impl Bpelcd35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd35 {
        match self.bits {
            false => Bpelcd35::B0,
            true => Bpelcd35::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd35::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd35::B1
    }
}
#[doc = "Field `BPELCD35` writer - no description available"]
pub type Bpelcd35W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd35>;
impl<'a, REG> Bpelcd35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd35::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd35::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd35 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd35> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD35` reader - no description available"]
pub type Bpflcd35R = crate::BitReader<Bpflcd35>;
impl Bpflcd35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd35 {
        match self.bits {
            false => Bpflcd35::B0,
            true => Bpflcd35::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd35::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd35::B1
    }
}
#[doc = "Field `BPFLCD35` writer - no description available"]
pub type Bpflcd35W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd35>;
impl<'a, REG> Bpflcd35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd35::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd35::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd35 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd35> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD35` reader - no description available"]
pub type Bpglcd35R = crate::BitReader<Bpglcd35>;
impl Bpglcd35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd35 {
        match self.bits {
            false => Bpglcd35::B0,
            true => Bpglcd35::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd35::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd35::B1
    }
}
#[doc = "Field `BPGLCD35` writer - no description available"]
pub type Bpglcd35W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd35>;
impl<'a, REG> Bpglcd35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd35::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd35::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd35 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd35> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd35) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD35` reader - no description available"]
pub type Bphlcd35R = crate::BitReader<Bphlcd35>;
impl Bphlcd35R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd35 {
        match self.bits {
            false => Bphlcd35::B0,
            true => Bphlcd35::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd35::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd35::B1
    }
}
#[doc = "Field `BPHLCD35` writer - no description available"]
pub type Bphlcd35W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd35>;
impl<'a, REG> Bphlcd35W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd35::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd35::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd35(&self) -> Bpalcd35R {
        Bpalcd35R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd35(&self) -> Bpblcd35R {
        Bpblcd35R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd35(&self) -> Bpclcd35R {
        Bpclcd35R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd35(&self) -> Bpdlcd35R {
        Bpdlcd35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd35(&self) -> Bpelcd35R {
        Bpelcd35R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd35(&self) -> Bpflcd35R {
        Bpflcd35R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd35(&self) -> Bpglcd35R {
        Bpglcd35R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd35(&self) -> Bphlcd35R {
        Bphlcd35R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd35(&mut self) -> Bpalcd35W<Wf35Spec> {
        Bpalcd35W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd35(&mut self) -> Bpblcd35W<Wf35Spec> {
        Bpblcd35W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd35(&mut self) -> Bpclcd35W<Wf35Spec> {
        Bpclcd35W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd35(&mut self) -> Bpdlcd35W<Wf35Spec> {
        Bpdlcd35W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd35(&mut self) -> Bpelcd35W<Wf35Spec> {
        Bpelcd35W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd35(&mut self) -> Bpflcd35W<Wf35Spec> {
        Bpflcd35W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd35(&mut self) -> Bpglcd35W<Wf35Spec> {
        Bpglcd35W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd35(&mut self) -> Bphlcd35W<Wf35Spec> {
        Bphlcd35W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 35.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf35Spec;
impl crate::RegisterSpec for Wf35Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf35::R`](R) reader structure"]
impl crate::Readable for Wf35Spec {}
#[doc = "`write(|w| ..)` method takes [`wf35::W`](W) writer structure"]
impl crate::Writable for Wf35Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF35 to value 0"]
impl crate::Resettable for Wf35Spec {
    const RESET_VALUE: u8 = 0;
}
