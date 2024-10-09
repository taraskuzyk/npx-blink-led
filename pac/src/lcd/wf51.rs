#[doc = "Register `WF51` reader"]
pub type R = crate::R<Wf51Spec>;
#[doc = "Register `WF51` writer"]
pub type W = crate::W<Wf51Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd51 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd51> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD51` reader - no description available"]
pub type Bpalcd51R = crate::BitReader<Bpalcd51>;
impl Bpalcd51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd51 {
        match self.bits {
            false => Bpalcd51::B0,
            true => Bpalcd51::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd51::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd51::B1
    }
}
#[doc = "Field `BPALCD51` writer - no description available"]
pub type Bpalcd51W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd51>;
impl<'a, REG> Bpalcd51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd51::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd51::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd51 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd51> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD51` reader - no description available"]
pub type Bpblcd51R = crate::BitReader<Bpblcd51>;
impl Bpblcd51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd51 {
        match self.bits {
            false => Bpblcd51::B0,
            true => Bpblcd51::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd51::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd51::B1
    }
}
#[doc = "Field `BPBLCD51` writer - no description available"]
pub type Bpblcd51W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd51>;
impl<'a, REG> Bpblcd51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd51::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd51::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd51 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd51> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD51` reader - no description available"]
pub type Bpclcd51R = crate::BitReader<Bpclcd51>;
impl Bpclcd51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd51 {
        match self.bits {
            false => Bpclcd51::B0,
            true => Bpclcd51::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd51::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd51::B1
    }
}
#[doc = "Field `BPCLCD51` writer - no description available"]
pub type Bpclcd51W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd51>;
impl<'a, REG> Bpclcd51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd51::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd51::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd51 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd51> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD51` reader - no description available"]
pub type Bpdlcd51R = crate::BitReader<Bpdlcd51>;
impl Bpdlcd51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd51 {
        match self.bits {
            false => Bpdlcd51::B0,
            true => Bpdlcd51::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd51::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd51::B1
    }
}
#[doc = "Field `BPDLCD51` writer - no description available"]
pub type Bpdlcd51W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd51>;
impl<'a, REG> Bpdlcd51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd51::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd51::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd51 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd51> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD51` reader - no description available"]
pub type Bpelcd51R = crate::BitReader<Bpelcd51>;
impl Bpelcd51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd51 {
        match self.bits {
            false => Bpelcd51::B0,
            true => Bpelcd51::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd51::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd51::B1
    }
}
#[doc = "Field `BPELCD51` writer - no description available"]
pub type Bpelcd51W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd51>;
impl<'a, REG> Bpelcd51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd51::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd51::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd51 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd51> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD51` reader - no description available"]
pub type Bpflcd51R = crate::BitReader<Bpflcd51>;
impl Bpflcd51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd51 {
        match self.bits {
            false => Bpflcd51::B0,
            true => Bpflcd51::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd51::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd51::B1
    }
}
#[doc = "Field `BPFLCD51` writer - no description available"]
pub type Bpflcd51W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd51>;
impl<'a, REG> Bpflcd51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd51::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd51::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd51 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd51> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD51` reader - no description available"]
pub type Bpglcd51R = crate::BitReader<Bpglcd51>;
impl Bpglcd51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd51 {
        match self.bits {
            false => Bpglcd51::B0,
            true => Bpglcd51::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd51::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd51::B1
    }
}
#[doc = "Field `BPGLCD51` writer - no description available"]
pub type Bpglcd51W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd51>;
impl<'a, REG> Bpglcd51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd51::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd51::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd51 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd51> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd51) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD51` reader - no description available"]
pub type Bphlcd51R = crate::BitReader<Bphlcd51>;
impl Bphlcd51R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd51 {
        match self.bits {
            false => Bphlcd51::B0,
            true => Bphlcd51::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd51::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd51::B1
    }
}
#[doc = "Field `BPHLCD51` writer - no description available"]
pub type Bphlcd51W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd51>;
impl<'a, REG> Bphlcd51W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd51::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd51::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd51(&self) -> Bpalcd51R {
        Bpalcd51R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd51(&self) -> Bpblcd51R {
        Bpblcd51R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd51(&self) -> Bpclcd51R {
        Bpclcd51R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd51(&self) -> Bpdlcd51R {
        Bpdlcd51R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd51(&self) -> Bpelcd51R {
        Bpelcd51R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd51(&self) -> Bpflcd51R {
        Bpflcd51R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd51(&self) -> Bpglcd51R {
        Bpglcd51R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd51(&self) -> Bphlcd51R {
        Bphlcd51R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd51(&mut self) -> Bpalcd51W<Wf51Spec> {
        Bpalcd51W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd51(&mut self) -> Bpblcd51W<Wf51Spec> {
        Bpblcd51W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd51(&mut self) -> Bpclcd51W<Wf51Spec> {
        Bpclcd51W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd51(&mut self) -> Bpdlcd51W<Wf51Spec> {
        Bpdlcd51W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd51(&mut self) -> Bpelcd51W<Wf51Spec> {
        Bpelcd51W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd51(&mut self) -> Bpflcd51W<Wf51Spec> {
        Bpflcd51W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd51(&mut self) -> Bpglcd51W<Wf51Spec> {
        Bpglcd51W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd51(&mut self) -> Bphlcd51W<Wf51Spec> {
        Bphlcd51W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 51.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf51Spec;
impl crate::RegisterSpec for Wf51Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf51::R`](R) reader structure"]
impl crate::Readable for Wf51Spec {}
#[doc = "`write(|w| ..)` method takes [`wf51::W`](W) writer structure"]
impl crate::Writable for Wf51Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF51 to value 0"]
impl crate::Resettable for Wf51Spec {
    const RESET_VALUE: u8 = 0;
}
