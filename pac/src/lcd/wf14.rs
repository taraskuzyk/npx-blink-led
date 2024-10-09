#[doc = "Register `WF14` reader"]
pub type R = crate::R<Wf14Spec>;
#[doc = "Register `WF14` writer"]
pub type W = crate::W<Wf14Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd14 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd14> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD14` reader - no description available"]
pub type Bpalcd14R = crate::BitReader<Bpalcd14>;
impl Bpalcd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd14 {
        match self.bits {
            false => Bpalcd14::B0,
            true => Bpalcd14::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd14::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd14::B1
    }
}
#[doc = "Field `BPALCD14` writer - no description available"]
pub type Bpalcd14W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd14>;
impl<'a, REG> Bpalcd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd14::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd14::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd14 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd14> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD14` reader - no description available"]
pub type Bpblcd14R = crate::BitReader<Bpblcd14>;
impl Bpblcd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd14 {
        match self.bits {
            false => Bpblcd14::B0,
            true => Bpblcd14::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd14::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd14::B1
    }
}
#[doc = "Field `BPBLCD14` writer - no description available"]
pub type Bpblcd14W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd14>;
impl<'a, REG> Bpblcd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd14::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd14::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd14 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd14> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD14` reader - no description available"]
pub type Bpclcd14R = crate::BitReader<Bpclcd14>;
impl Bpclcd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd14 {
        match self.bits {
            false => Bpclcd14::B0,
            true => Bpclcd14::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd14::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd14::B1
    }
}
#[doc = "Field `BPCLCD14` writer - no description available"]
pub type Bpclcd14W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd14>;
impl<'a, REG> Bpclcd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd14::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd14::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd14 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd14> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD14` reader - no description available"]
pub type Bpdlcd14R = crate::BitReader<Bpdlcd14>;
impl Bpdlcd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd14 {
        match self.bits {
            false => Bpdlcd14::B0,
            true => Bpdlcd14::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd14::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd14::B1
    }
}
#[doc = "Field `BPDLCD14` writer - no description available"]
pub type Bpdlcd14W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd14>;
impl<'a, REG> Bpdlcd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd14::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd14::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd14 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd14> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD14` reader - no description available"]
pub type Bpelcd14R = crate::BitReader<Bpelcd14>;
impl Bpelcd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd14 {
        match self.bits {
            false => Bpelcd14::B0,
            true => Bpelcd14::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd14::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd14::B1
    }
}
#[doc = "Field `BPELCD14` writer - no description available"]
pub type Bpelcd14W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd14>;
impl<'a, REG> Bpelcd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd14::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd14::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd14 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd14> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD14` reader - no description available"]
pub type Bpflcd14R = crate::BitReader<Bpflcd14>;
impl Bpflcd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd14 {
        match self.bits {
            false => Bpflcd14::B0,
            true => Bpflcd14::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd14::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd14::B1
    }
}
#[doc = "Field `BPFLCD14` writer - no description available"]
pub type Bpflcd14W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd14>;
impl<'a, REG> Bpflcd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd14::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd14::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd14 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd14> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD14` reader - no description available"]
pub type Bpglcd14R = crate::BitReader<Bpglcd14>;
impl Bpglcd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd14 {
        match self.bits {
            false => Bpglcd14::B0,
            true => Bpglcd14::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd14::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd14::B1
    }
}
#[doc = "Field `BPGLCD14` writer - no description available"]
pub type Bpglcd14W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd14>;
impl<'a, REG> Bpglcd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd14::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd14::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd14 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd14> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD14` reader - no description available"]
pub type Bphlcd14R = crate::BitReader<Bphlcd14>;
impl Bphlcd14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd14 {
        match self.bits {
            false => Bphlcd14::B0,
            true => Bphlcd14::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd14::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd14::B1
    }
}
#[doc = "Field `BPHLCD14` writer - no description available"]
pub type Bphlcd14W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd14>;
impl<'a, REG> Bphlcd14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd14::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd14::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd14(&self) -> Bpalcd14R {
        Bpalcd14R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd14(&self) -> Bpblcd14R {
        Bpblcd14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd14(&self) -> Bpclcd14R {
        Bpclcd14R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd14(&self) -> Bpdlcd14R {
        Bpdlcd14R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd14(&self) -> Bpelcd14R {
        Bpelcd14R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd14(&self) -> Bpflcd14R {
        Bpflcd14R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd14(&self) -> Bpglcd14R {
        Bpglcd14R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd14(&self) -> Bphlcd14R {
        Bphlcd14R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd14(&mut self) -> Bpalcd14W<Wf14Spec> {
        Bpalcd14W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd14(&mut self) -> Bpblcd14W<Wf14Spec> {
        Bpblcd14W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd14(&mut self) -> Bpclcd14W<Wf14Spec> {
        Bpclcd14W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd14(&mut self) -> Bpdlcd14W<Wf14Spec> {
        Bpdlcd14W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd14(&mut self) -> Bpelcd14W<Wf14Spec> {
        Bpelcd14W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd14(&mut self) -> Bpflcd14W<Wf14Spec> {
        Bpflcd14W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd14(&mut self) -> Bpglcd14W<Wf14Spec> {
        Bpglcd14W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd14(&mut self) -> Bphlcd14W<Wf14Spec> {
        Bphlcd14W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 14.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf14Spec;
impl crate::RegisterSpec for Wf14Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf14::R`](R) reader structure"]
impl crate::Readable for Wf14Spec {}
#[doc = "`write(|w| ..)` method takes [`wf14::W`](W) writer structure"]
impl crate::Writable for Wf14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF14 to value 0"]
impl crate::Resettable for Wf14Spec {
    const RESET_VALUE: u8 = 0;
}
