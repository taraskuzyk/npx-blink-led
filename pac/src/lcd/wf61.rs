#[doc = "Register `WF61` reader"]
pub type R = crate::R<Wf61Spec>;
#[doc = "Register `WF61` writer"]
pub type W = crate::W<Wf61Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd61 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd61> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD61` reader - no description available"]
pub type Bpalcd61R = crate::BitReader<Bpalcd61>;
impl Bpalcd61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd61 {
        match self.bits {
            false => Bpalcd61::B0,
            true => Bpalcd61::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd61::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd61::B1
    }
}
#[doc = "Field `BPALCD61` writer - no description available"]
pub type Bpalcd61W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd61>;
impl<'a, REG> Bpalcd61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd61::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd61::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd61 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd61> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD61` reader - no description available"]
pub type Bpblcd61R = crate::BitReader<Bpblcd61>;
impl Bpblcd61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd61 {
        match self.bits {
            false => Bpblcd61::B0,
            true => Bpblcd61::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd61::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd61::B1
    }
}
#[doc = "Field `BPBLCD61` writer - no description available"]
pub type Bpblcd61W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd61>;
impl<'a, REG> Bpblcd61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd61::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd61::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd61 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd61> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD61` reader - no description available"]
pub type Bpclcd61R = crate::BitReader<Bpclcd61>;
impl Bpclcd61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd61 {
        match self.bits {
            false => Bpclcd61::B0,
            true => Bpclcd61::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd61::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd61::B1
    }
}
#[doc = "Field `BPCLCD61` writer - no description available"]
pub type Bpclcd61W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd61>;
impl<'a, REG> Bpclcd61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd61::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd61::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd61 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd61> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD61` reader - no description available"]
pub type Bpdlcd61R = crate::BitReader<Bpdlcd61>;
impl Bpdlcd61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd61 {
        match self.bits {
            false => Bpdlcd61::B0,
            true => Bpdlcd61::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd61::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd61::B1
    }
}
#[doc = "Field `BPDLCD61` writer - no description available"]
pub type Bpdlcd61W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd61>;
impl<'a, REG> Bpdlcd61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd61::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd61::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd61 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd61> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD61` reader - no description available"]
pub type Bpelcd61R = crate::BitReader<Bpelcd61>;
impl Bpelcd61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd61 {
        match self.bits {
            false => Bpelcd61::B0,
            true => Bpelcd61::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd61::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd61::B1
    }
}
#[doc = "Field `BPELCD61` writer - no description available"]
pub type Bpelcd61W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd61>;
impl<'a, REG> Bpelcd61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd61::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd61::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd61 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd61> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD61` reader - no description available"]
pub type Bpflcd61R = crate::BitReader<Bpflcd61>;
impl Bpflcd61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd61 {
        match self.bits {
            false => Bpflcd61::B0,
            true => Bpflcd61::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd61::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd61::B1
    }
}
#[doc = "Field `BPFLCD61` writer - no description available"]
pub type Bpflcd61W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd61>;
impl<'a, REG> Bpflcd61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd61::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd61::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd61 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd61> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD61` reader - no description available"]
pub type Bpglcd61R = crate::BitReader<Bpglcd61>;
impl Bpglcd61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd61 {
        match self.bits {
            false => Bpglcd61::B0,
            true => Bpglcd61::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd61::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd61::B1
    }
}
#[doc = "Field `BPGLCD61` writer - no description available"]
pub type Bpglcd61W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd61>;
impl<'a, REG> Bpglcd61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd61::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd61::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd61 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd61> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd61) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD61` reader - no description available"]
pub type Bphlcd61R = crate::BitReader<Bphlcd61>;
impl Bphlcd61R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd61 {
        match self.bits {
            false => Bphlcd61::B0,
            true => Bphlcd61::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd61::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd61::B1
    }
}
#[doc = "Field `BPHLCD61` writer - no description available"]
pub type Bphlcd61W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd61>;
impl<'a, REG> Bphlcd61W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd61::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd61::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd61(&self) -> Bpalcd61R {
        Bpalcd61R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd61(&self) -> Bpblcd61R {
        Bpblcd61R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd61(&self) -> Bpclcd61R {
        Bpclcd61R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd61(&self) -> Bpdlcd61R {
        Bpdlcd61R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd61(&self) -> Bpelcd61R {
        Bpelcd61R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd61(&self) -> Bpflcd61R {
        Bpflcd61R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd61(&self) -> Bpglcd61R {
        Bpglcd61R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd61(&self) -> Bphlcd61R {
        Bphlcd61R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd61(&mut self) -> Bpalcd61W<Wf61Spec> {
        Bpalcd61W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd61(&mut self) -> Bpblcd61W<Wf61Spec> {
        Bpblcd61W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd61(&mut self) -> Bpclcd61W<Wf61Spec> {
        Bpclcd61W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd61(&mut self) -> Bpdlcd61W<Wf61Spec> {
        Bpdlcd61W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd61(&mut self) -> Bpelcd61W<Wf61Spec> {
        Bpelcd61W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd61(&mut self) -> Bpflcd61W<Wf61Spec> {
        Bpflcd61W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd61(&mut self) -> Bpglcd61W<Wf61Spec> {
        Bpglcd61W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd61(&mut self) -> Bphlcd61W<Wf61Spec> {
        Bphlcd61W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 61.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf61::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf61::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf61Spec;
impl crate::RegisterSpec for Wf61Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf61::R`](R) reader structure"]
impl crate::Readable for Wf61Spec {}
#[doc = "`write(|w| ..)` method takes [`wf61::W`](W) writer structure"]
impl crate::Writable for Wf61Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF61 to value 0"]
impl crate::Resettable for Wf61Spec {
    const RESET_VALUE: u8 = 0;
}
