#[doc = "Register `WF18` reader"]
pub type R = crate::R<Wf18Spec>;
#[doc = "Register `WF18` writer"]
pub type W = crate::W<Wf18Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd18 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd18> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD18` reader - no description available"]
pub type Bpalcd18R = crate::BitReader<Bpalcd18>;
impl Bpalcd18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd18 {
        match self.bits {
            false => Bpalcd18::B0,
            true => Bpalcd18::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd18::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd18::B1
    }
}
#[doc = "Field `BPALCD18` writer - no description available"]
pub type Bpalcd18W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd18>;
impl<'a, REG> Bpalcd18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd18::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd18::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd18 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd18> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD18` reader - no description available"]
pub type Bpblcd18R = crate::BitReader<Bpblcd18>;
impl Bpblcd18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd18 {
        match self.bits {
            false => Bpblcd18::B0,
            true => Bpblcd18::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd18::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd18::B1
    }
}
#[doc = "Field `BPBLCD18` writer - no description available"]
pub type Bpblcd18W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd18>;
impl<'a, REG> Bpblcd18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd18::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd18::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd18 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd18> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD18` reader - no description available"]
pub type Bpclcd18R = crate::BitReader<Bpclcd18>;
impl Bpclcd18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd18 {
        match self.bits {
            false => Bpclcd18::B0,
            true => Bpclcd18::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd18::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd18::B1
    }
}
#[doc = "Field `BPCLCD18` writer - no description available"]
pub type Bpclcd18W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd18>;
impl<'a, REG> Bpclcd18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd18::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd18::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd18 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd18> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD18` reader - no description available"]
pub type Bpdlcd18R = crate::BitReader<Bpdlcd18>;
impl Bpdlcd18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd18 {
        match self.bits {
            false => Bpdlcd18::B0,
            true => Bpdlcd18::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd18::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd18::B1
    }
}
#[doc = "Field `BPDLCD18` writer - no description available"]
pub type Bpdlcd18W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd18>;
impl<'a, REG> Bpdlcd18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd18::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd18::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd18 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd18> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD18` reader - no description available"]
pub type Bpelcd18R = crate::BitReader<Bpelcd18>;
impl Bpelcd18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd18 {
        match self.bits {
            false => Bpelcd18::B0,
            true => Bpelcd18::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd18::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd18::B1
    }
}
#[doc = "Field `BPELCD18` writer - no description available"]
pub type Bpelcd18W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd18>;
impl<'a, REG> Bpelcd18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd18::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd18::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd18 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd18> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD18` reader - no description available"]
pub type Bpflcd18R = crate::BitReader<Bpflcd18>;
impl Bpflcd18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd18 {
        match self.bits {
            false => Bpflcd18::B0,
            true => Bpflcd18::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd18::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd18::B1
    }
}
#[doc = "Field `BPFLCD18` writer - no description available"]
pub type Bpflcd18W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd18>;
impl<'a, REG> Bpflcd18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd18::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd18::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd18 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd18> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD18` reader - no description available"]
pub type Bpglcd18R = crate::BitReader<Bpglcd18>;
impl Bpglcd18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd18 {
        match self.bits {
            false => Bpglcd18::B0,
            true => Bpglcd18::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd18::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd18::B1
    }
}
#[doc = "Field `BPGLCD18` writer - no description available"]
pub type Bpglcd18W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd18>;
impl<'a, REG> Bpglcd18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd18::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd18::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd18 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd18> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD18` reader - no description available"]
pub type Bphlcd18R = crate::BitReader<Bphlcd18>;
impl Bphlcd18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd18 {
        match self.bits {
            false => Bphlcd18::B0,
            true => Bphlcd18::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd18::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd18::B1
    }
}
#[doc = "Field `BPHLCD18` writer - no description available"]
pub type Bphlcd18W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd18>;
impl<'a, REG> Bphlcd18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd18::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd18::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd18(&self) -> Bpalcd18R {
        Bpalcd18R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd18(&self) -> Bpblcd18R {
        Bpblcd18R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd18(&self) -> Bpclcd18R {
        Bpclcd18R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd18(&self) -> Bpdlcd18R {
        Bpdlcd18R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd18(&self) -> Bpelcd18R {
        Bpelcd18R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd18(&self) -> Bpflcd18R {
        Bpflcd18R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd18(&self) -> Bpglcd18R {
        Bpglcd18R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd18(&self) -> Bphlcd18R {
        Bphlcd18R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd18(&mut self) -> Bpalcd18W<Wf18Spec> {
        Bpalcd18W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd18(&mut self) -> Bpblcd18W<Wf18Spec> {
        Bpblcd18W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd18(&mut self) -> Bpclcd18W<Wf18Spec> {
        Bpclcd18W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd18(&mut self) -> Bpdlcd18W<Wf18Spec> {
        Bpdlcd18W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd18(&mut self) -> Bpelcd18W<Wf18Spec> {
        Bpelcd18W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd18(&mut self) -> Bpflcd18W<Wf18Spec> {
        Bpflcd18W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd18(&mut self) -> Bpglcd18W<Wf18Spec> {
        Bpglcd18W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd18(&mut self) -> Bphlcd18W<Wf18Spec> {
        Bphlcd18W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 18.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf18Spec;
impl crate::RegisterSpec for Wf18Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf18::R`](R) reader structure"]
impl crate::Readable for Wf18Spec {}
#[doc = "`write(|w| ..)` method takes [`wf18::W`](W) writer structure"]
impl crate::Writable for Wf18Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF18 to value 0"]
impl crate::Resettable for Wf18Spec {
    const RESET_VALUE: u8 = 0;
}
