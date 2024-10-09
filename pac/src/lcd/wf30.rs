#[doc = "Register `WF30` reader"]
pub type R = crate::R<Wf30Spec>;
#[doc = "Register `WF30` writer"]
pub type W = crate::W<Wf30Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd30 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd30> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD30` reader - no description available"]
pub type Bpalcd30R = crate::BitReader<Bpalcd30>;
impl Bpalcd30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd30 {
        match self.bits {
            false => Bpalcd30::B0,
            true => Bpalcd30::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd30::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd30::B1
    }
}
#[doc = "Field `BPALCD30` writer - no description available"]
pub type Bpalcd30W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd30>;
impl<'a, REG> Bpalcd30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd30::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd30::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd30 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd30> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD30` reader - no description available"]
pub type Bpblcd30R = crate::BitReader<Bpblcd30>;
impl Bpblcd30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd30 {
        match self.bits {
            false => Bpblcd30::B0,
            true => Bpblcd30::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd30::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd30::B1
    }
}
#[doc = "Field `BPBLCD30` writer - no description available"]
pub type Bpblcd30W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd30>;
impl<'a, REG> Bpblcd30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd30::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd30::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd30 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd30> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD30` reader - no description available"]
pub type Bpclcd30R = crate::BitReader<Bpclcd30>;
impl Bpclcd30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd30 {
        match self.bits {
            false => Bpclcd30::B0,
            true => Bpclcd30::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd30::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd30::B1
    }
}
#[doc = "Field `BPCLCD30` writer - no description available"]
pub type Bpclcd30W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd30>;
impl<'a, REG> Bpclcd30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd30::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd30::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd30 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd30> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD30` reader - no description available"]
pub type Bpdlcd30R = crate::BitReader<Bpdlcd30>;
impl Bpdlcd30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd30 {
        match self.bits {
            false => Bpdlcd30::B0,
            true => Bpdlcd30::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd30::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd30::B1
    }
}
#[doc = "Field `BPDLCD30` writer - no description available"]
pub type Bpdlcd30W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd30>;
impl<'a, REG> Bpdlcd30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd30::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd30::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd30 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd30> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD30` reader - no description available"]
pub type Bpelcd30R = crate::BitReader<Bpelcd30>;
impl Bpelcd30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd30 {
        match self.bits {
            false => Bpelcd30::B0,
            true => Bpelcd30::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd30::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd30::B1
    }
}
#[doc = "Field `BPELCD30` writer - no description available"]
pub type Bpelcd30W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd30>;
impl<'a, REG> Bpelcd30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd30::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd30::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd30 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd30> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD30` reader - no description available"]
pub type Bpflcd30R = crate::BitReader<Bpflcd30>;
impl Bpflcd30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd30 {
        match self.bits {
            false => Bpflcd30::B0,
            true => Bpflcd30::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd30::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd30::B1
    }
}
#[doc = "Field `BPFLCD30` writer - no description available"]
pub type Bpflcd30W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd30>;
impl<'a, REG> Bpflcd30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd30::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd30::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd30 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd30> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD30` reader - no description available"]
pub type Bpglcd30R = crate::BitReader<Bpglcd30>;
impl Bpglcd30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd30 {
        match self.bits {
            false => Bpglcd30::B0,
            true => Bpglcd30::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd30::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd30::B1
    }
}
#[doc = "Field `BPGLCD30` writer - no description available"]
pub type Bpglcd30W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd30>;
impl<'a, REG> Bpglcd30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd30::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd30::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd30 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd30> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD30` reader - no description available"]
pub type Bphlcd30R = crate::BitReader<Bphlcd30>;
impl Bphlcd30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd30 {
        match self.bits {
            false => Bphlcd30::B0,
            true => Bphlcd30::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd30::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd30::B1
    }
}
#[doc = "Field `BPHLCD30` writer - no description available"]
pub type Bphlcd30W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd30>;
impl<'a, REG> Bphlcd30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd30::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd30::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd30(&self) -> Bpalcd30R {
        Bpalcd30R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd30(&self) -> Bpblcd30R {
        Bpblcd30R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd30(&self) -> Bpclcd30R {
        Bpclcd30R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd30(&self) -> Bpdlcd30R {
        Bpdlcd30R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd30(&self) -> Bpelcd30R {
        Bpelcd30R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd30(&self) -> Bpflcd30R {
        Bpflcd30R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd30(&self) -> Bpglcd30R {
        Bpglcd30R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd30(&self) -> Bphlcd30R {
        Bphlcd30R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd30(&mut self) -> Bpalcd30W<Wf30Spec> {
        Bpalcd30W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd30(&mut self) -> Bpblcd30W<Wf30Spec> {
        Bpblcd30W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd30(&mut self) -> Bpclcd30W<Wf30Spec> {
        Bpclcd30W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd30(&mut self) -> Bpdlcd30W<Wf30Spec> {
        Bpdlcd30W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd30(&mut self) -> Bpelcd30W<Wf30Spec> {
        Bpelcd30W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd30(&mut self) -> Bpflcd30W<Wf30Spec> {
        Bpflcd30W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd30(&mut self) -> Bpglcd30W<Wf30Spec> {
        Bpglcd30W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd30(&mut self) -> Bphlcd30W<Wf30Spec> {
        Bphlcd30W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 30.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf30Spec;
impl crate::RegisterSpec for Wf30Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf30::R`](R) reader structure"]
impl crate::Readable for Wf30Spec {}
#[doc = "`write(|w| ..)` method takes [`wf30::W`](W) writer structure"]
impl crate::Writable for Wf30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF30 to value 0"]
impl crate::Resettable for Wf30Spec {
    const RESET_VALUE: u8 = 0;
}
