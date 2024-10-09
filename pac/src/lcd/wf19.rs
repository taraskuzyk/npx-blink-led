#[doc = "Register `WF19` reader"]
pub type R = crate::R<Wf19Spec>;
#[doc = "Register `WF19` writer"]
pub type W = crate::W<Wf19Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd19 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd19> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD19` reader - no description available"]
pub type Bpalcd19R = crate::BitReader<Bpalcd19>;
impl Bpalcd19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd19 {
        match self.bits {
            false => Bpalcd19::B0,
            true => Bpalcd19::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd19::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd19::B1
    }
}
#[doc = "Field `BPALCD19` writer - no description available"]
pub type Bpalcd19W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd19>;
impl<'a, REG> Bpalcd19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd19::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd19::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd19 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd19> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD19` reader - no description available"]
pub type Bpblcd19R = crate::BitReader<Bpblcd19>;
impl Bpblcd19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd19 {
        match self.bits {
            false => Bpblcd19::B0,
            true => Bpblcd19::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd19::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd19::B1
    }
}
#[doc = "Field `BPBLCD19` writer - no description available"]
pub type Bpblcd19W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd19>;
impl<'a, REG> Bpblcd19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd19::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd19::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd19 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd19> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD19` reader - no description available"]
pub type Bpclcd19R = crate::BitReader<Bpclcd19>;
impl Bpclcd19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd19 {
        match self.bits {
            false => Bpclcd19::B0,
            true => Bpclcd19::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd19::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd19::B1
    }
}
#[doc = "Field `BPCLCD19` writer - no description available"]
pub type Bpclcd19W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd19>;
impl<'a, REG> Bpclcd19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd19::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd19::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd19 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd19> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD19` reader - no description available"]
pub type Bpdlcd19R = crate::BitReader<Bpdlcd19>;
impl Bpdlcd19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd19 {
        match self.bits {
            false => Bpdlcd19::B0,
            true => Bpdlcd19::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd19::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd19::B1
    }
}
#[doc = "Field `BPDLCD19` writer - no description available"]
pub type Bpdlcd19W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd19>;
impl<'a, REG> Bpdlcd19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd19::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd19::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd19 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd19> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD19` reader - no description available"]
pub type Bpelcd19R = crate::BitReader<Bpelcd19>;
impl Bpelcd19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd19 {
        match self.bits {
            false => Bpelcd19::B0,
            true => Bpelcd19::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd19::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd19::B1
    }
}
#[doc = "Field `BPELCD19` writer - no description available"]
pub type Bpelcd19W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd19>;
impl<'a, REG> Bpelcd19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd19::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd19::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd19 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd19> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD19` reader - no description available"]
pub type Bpflcd19R = crate::BitReader<Bpflcd19>;
impl Bpflcd19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd19 {
        match self.bits {
            false => Bpflcd19::B0,
            true => Bpflcd19::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd19::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd19::B1
    }
}
#[doc = "Field `BPFLCD19` writer - no description available"]
pub type Bpflcd19W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd19>;
impl<'a, REG> Bpflcd19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd19::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd19::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd19 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd19> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD19` reader - no description available"]
pub type Bpglcd19R = crate::BitReader<Bpglcd19>;
impl Bpglcd19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd19 {
        match self.bits {
            false => Bpglcd19::B0,
            true => Bpglcd19::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd19::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd19::B1
    }
}
#[doc = "Field `BPGLCD19` writer - no description available"]
pub type Bpglcd19W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd19>;
impl<'a, REG> Bpglcd19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd19::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd19::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd19 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd19> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD19` reader - no description available"]
pub type Bphlcd19R = crate::BitReader<Bphlcd19>;
impl Bphlcd19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd19 {
        match self.bits {
            false => Bphlcd19::B0,
            true => Bphlcd19::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd19::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd19::B1
    }
}
#[doc = "Field `BPHLCD19` writer - no description available"]
pub type Bphlcd19W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd19>;
impl<'a, REG> Bphlcd19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd19::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd19::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd19(&self) -> Bpalcd19R {
        Bpalcd19R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd19(&self) -> Bpblcd19R {
        Bpblcd19R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd19(&self) -> Bpclcd19R {
        Bpclcd19R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd19(&self) -> Bpdlcd19R {
        Bpdlcd19R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd19(&self) -> Bpelcd19R {
        Bpelcd19R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd19(&self) -> Bpflcd19R {
        Bpflcd19R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd19(&self) -> Bpglcd19R {
        Bpglcd19R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd19(&self) -> Bphlcd19R {
        Bphlcd19R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd19(&mut self) -> Bpalcd19W<Wf19Spec> {
        Bpalcd19W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd19(&mut self) -> Bpblcd19W<Wf19Spec> {
        Bpblcd19W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd19(&mut self) -> Bpclcd19W<Wf19Spec> {
        Bpclcd19W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd19(&mut self) -> Bpdlcd19W<Wf19Spec> {
        Bpdlcd19W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd19(&mut self) -> Bpelcd19W<Wf19Spec> {
        Bpelcd19W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd19(&mut self) -> Bpflcd19W<Wf19Spec> {
        Bpflcd19W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd19(&mut self) -> Bpglcd19W<Wf19Spec> {
        Bpglcd19W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd19(&mut self) -> Bphlcd19W<Wf19Spec> {
        Bphlcd19W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 19.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf19Spec;
impl crate::RegisterSpec for Wf19Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf19::R`](R) reader structure"]
impl crate::Readable for Wf19Spec {}
#[doc = "`write(|w| ..)` method takes [`wf19::W`](W) writer structure"]
impl crate::Writable for Wf19Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF19 to value 0"]
impl crate::Resettable for Wf19Spec {
    const RESET_VALUE: u8 = 0;
}
