#[doc = "Register `WF62` reader"]
pub type R = crate::R<Wf62Spec>;
#[doc = "Register `WF62` writer"]
pub type W = crate::W<Wf62Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd62 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd62> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD62` reader - no description available"]
pub type Bpalcd62R = crate::BitReader<Bpalcd62>;
impl Bpalcd62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd62 {
        match self.bits {
            false => Bpalcd62::B0,
            true => Bpalcd62::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd62::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd62::B1
    }
}
#[doc = "Field `BPALCD62` writer - no description available"]
pub type Bpalcd62W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd62>;
impl<'a, REG> Bpalcd62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd62::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd62::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd62 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd62> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD62` reader - no description available"]
pub type Bpblcd62R = crate::BitReader<Bpblcd62>;
impl Bpblcd62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd62 {
        match self.bits {
            false => Bpblcd62::B0,
            true => Bpblcd62::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd62::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd62::B1
    }
}
#[doc = "Field `BPBLCD62` writer - no description available"]
pub type Bpblcd62W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd62>;
impl<'a, REG> Bpblcd62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd62::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd62::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd62 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd62> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD62` reader - no description available"]
pub type Bpclcd62R = crate::BitReader<Bpclcd62>;
impl Bpclcd62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd62 {
        match self.bits {
            false => Bpclcd62::B0,
            true => Bpclcd62::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd62::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd62::B1
    }
}
#[doc = "Field `BPCLCD62` writer - no description available"]
pub type Bpclcd62W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd62>;
impl<'a, REG> Bpclcd62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd62::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd62::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd62 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd62> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD62` reader - no description available"]
pub type Bpdlcd62R = crate::BitReader<Bpdlcd62>;
impl Bpdlcd62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd62 {
        match self.bits {
            false => Bpdlcd62::B0,
            true => Bpdlcd62::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd62::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd62::B1
    }
}
#[doc = "Field `BPDLCD62` writer - no description available"]
pub type Bpdlcd62W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd62>;
impl<'a, REG> Bpdlcd62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd62::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd62::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd62 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd62> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD62` reader - no description available"]
pub type Bpelcd62R = crate::BitReader<Bpelcd62>;
impl Bpelcd62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd62 {
        match self.bits {
            false => Bpelcd62::B0,
            true => Bpelcd62::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd62::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd62::B1
    }
}
#[doc = "Field `BPELCD62` writer - no description available"]
pub type Bpelcd62W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd62>;
impl<'a, REG> Bpelcd62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd62::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd62::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd62 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd62> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD62` reader - no description available"]
pub type Bpflcd62R = crate::BitReader<Bpflcd62>;
impl Bpflcd62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd62 {
        match self.bits {
            false => Bpflcd62::B0,
            true => Bpflcd62::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd62::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd62::B1
    }
}
#[doc = "Field `BPFLCD62` writer - no description available"]
pub type Bpflcd62W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd62>;
impl<'a, REG> Bpflcd62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd62::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd62::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd62 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd62> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD62` reader - no description available"]
pub type Bpglcd62R = crate::BitReader<Bpglcd62>;
impl Bpglcd62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd62 {
        match self.bits {
            false => Bpglcd62::B0,
            true => Bpglcd62::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd62::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd62::B1
    }
}
#[doc = "Field `BPGLCD62` writer - no description available"]
pub type Bpglcd62W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd62>;
impl<'a, REG> Bpglcd62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd62::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd62::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd62 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd62> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd62) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD62` reader - no description available"]
pub type Bphlcd62R = crate::BitReader<Bphlcd62>;
impl Bphlcd62R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd62 {
        match self.bits {
            false => Bphlcd62::B0,
            true => Bphlcd62::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd62::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd62::B1
    }
}
#[doc = "Field `BPHLCD62` writer - no description available"]
pub type Bphlcd62W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd62>;
impl<'a, REG> Bphlcd62W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd62::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd62::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd62(&self) -> Bpalcd62R {
        Bpalcd62R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd62(&self) -> Bpblcd62R {
        Bpblcd62R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd62(&self) -> Bpclcd62R {
        Bpclcd62R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd62(&self) -> Bpdlcd62R {
        Bpdlcd62R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd62(&self) -> Bpelcd62R {
        Bpelcd62R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd62(&self) -> Bpflcd62R {
        Bpflcd62R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd62(&self) -> Bpglcd62R {
        Bpglcd62R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd62(&self) -> Bphlcd62R {
        Bphlcd62R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd62(&mut self) -> Bpalcd62W<Wf62Spec> {
        Bpalcd62W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd62(&mut self) -> Bpblcd62W<Wf62Spec> {
        Bpblcd62W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd62(&mut self) -> Bpclcd62W<Wf62Spec> {
        Bpclcd62W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd62(&mut self) -> Bpdlcd62W<Wf62Spec> {
        Bpdlcd62W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd62(&mut self) -> Bpelcd62W<Wf62Spec> {
        Bpelcd62W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd62(&mut self) -> Bpflcd62W<Wf62Spec> {
        Bpflcd62W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd62(&mut self) -> Bpglcd62W<Wf62Spec> {
        Bpglcd62W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd62(&mut self) -> Bphlcd62W<Wf62Spec> {
        Bphlcd62W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 62.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf62::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf62::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf62Spec;
impl crate::RegisterSpec for Wf62Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf62::R`](R) reader structure"]
impl crate::Readable for Wf62Spec {}
#[doc = "`write(|w| ..)` method takes [`wf62::W`](W) writer structure"]
impl crate::Writable for Wf62Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF62 to value 0"]
impl crate::Resettable for Wf62Spec {
    const RESET_VALUE: u8 = 0;
}
