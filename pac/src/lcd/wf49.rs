#[doc = "Register `WF49` reader"]
pub type R = crate::R<Wf49Spec>;
#[doc = "Register `WF49` writer"]
pub type W = crate::W<Wf49Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd49 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd49> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD49` reader - no description available"]
pub type Bpalcd49R = crate::BitReader<Bpalcd49>;
impl Bpalcd49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd49 {
        match self.bits {
            false => Bpalcd49::B0,
            true => Bpalcd49::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd49::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd49::B1
    }
}
#[doc = "Field `BPALCD49` writer - no description available"]
pub type Bpalcd49W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd49>;
impl<'a, REG> Bpalcd49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd49::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd49::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd49 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd49> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD49` reader - no description available"]
pub type Bpblcd49R = crate::BitReader<Bpblcd49>;
impl Bpblcd49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd49 {
        match self.bits {
            false => Bpblcd49::B0,
            true => Bpblcd49::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd49::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd49::B1
    }
}
#[doc = "Field `BPBLCD49` writer - no description available"]
pub type Bpblcd49W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd49>;
impl<'a, REG> Bpblcd49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd49::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd49::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd49 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd49> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD49` reader - no description available"]
pub type Bpclcd49R = crate::BitReader<Bpclcd49>;
impl Bpclcd49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd49 {
        match self.bits {
            false => Bpclcd49::B0,
            true => Bpclcd49::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd49::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd49::B1
    }
}
#[doc = "Field `BPCLCD49` writer - no description available"]
pub type Bpclcd49W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd49>;
impl<'a, REG> Bpclcd49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd49::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd49::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd49 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd49> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD49` reader - no description available"]
pub type Bpdlcd49R = crate::BitReader<Bpdlcd49>;
impl Bpdlcd49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd49 {
        match self.bits {
            false => Bpdlcd49::B0,
            true => Bpdlcd49::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd49::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd49::B1
    }
}
#[doc = "Field `BPDLCD49` writer - no description available"]
pub type Bpdlcd49W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd49>;
impl<'a, REG> Bpdlcd49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd49::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd49::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd49 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd49> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD49` reader - no description available"]
pub type Bpelcd49R = crate::BitReader<Bpelcd49>;
impl Bpelcd49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd49 {
        match self.bits {
            false => Bpelcd49::B0,
            true => Bpelcd49::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd49::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd49::B1
    }
}
#[doc = "Field `BPELCD49` writer - no description available"]
pub type Bpelcd49W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd49>;
impl<'a, REG> Bpelcd49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd49::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd49::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd49 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd49> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD49` reader - no description available"]
pub type Bpflcd49R = crate::BitReader<Bpflcd49>;
impl Bpflcd49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd49 {
        match self.bits {
            false => Bpflcd49::B0,
            true => Bpflcd49::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd49::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd49::B1
    }
}
#[doc = "Field `BPFLCD49` writer - no description available"]
pub type Bpflcd49W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd49>;
impl<'a, REG> Bpflcd49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd49::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd49::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd49 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd49> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD49` reader - no description available"]
pub type Bpglcd49R = crate::BitReader<Bpglcd49>;
impl Bpglcd49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd49 {
        match self.bits {
            false => Bpglcd49::B0,
            true => Bpglcd49::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd49::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd49::B1
    }
}
#[doc = "Field `BPGLCD49` writer - no description available"]
pub type Bpglcd49W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd49>;
impl<'a, REG> Bpglcd49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd49::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd49::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd49 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd49> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd49) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD49` reader - no description available"]
pub type Bphlcd49R = crate::BitReader<Bphlcd49>;
impl Bphlcd49R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd49 {
        match self.bits {
            false => Bphlcd49::B0,
            true => Bphlcd49::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd49::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd49::B1
    }
}
#[doc = "Field `BPHLCD49` writer - no description available"]
pub type Bphlcd49W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd49>;
impl<'a, REG> Bphlcd49W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd49::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd49::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd49(&self) -> Bpalcd49R {
        Bpalcd49R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd49(&self) -> Bpblcd49R {
        Bpblcd49R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd49(&self) -> Bpclcd49R {
        Bpclcd49R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd49(&self) -> Bpdlcd49R {
        Bpdlcd49R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd49(&self) -> Bpelcd49R {
        Bpelcd49R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd49(&self) -> Bpflcd49R {
        Bpflcd49R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd49(&self) -> Bpglcd49R {
        Bpglcd49R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd49(&self) -> Bphlcd49R {
        Bphlcd49R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd49(&mut self) -> Bpalcd49W<Wf49Spec> {
        Bpalcd49W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd49(&mut self) -> Bpblcd49W<Wf49Spec> {
        Bpblcd49W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd49(&mut self) -> Bpclcd49W<Wf49Spec> {
        Bpclcd49W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd49(&mut self) -> Bpdlcd49W<Wf49Spec> {
        Bpdlcd49W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd49(&mut self) -> Bpelcd49W<Wf49Spec> {
        Bpelcd49W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd49(&mut self) -> Bpflcd49W<Wf49Spec> {
        Bpflcd49W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd49(&mut self) -> Bpglcd49W<Wf49Spec> {
        Bpglcd49W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd49(&mut self) -> Bphlcd49W<Wf49Spec> {
        Bphlcd49W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 49.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf49Spec;
impl crate::RegisterSpec for Wf49Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf49::R`](R) reader structure"]
impl crate::Readable for Wf49Spec {}
#[doc = "`write(|w| ..)` method takes [`wf49::W`](W) writer structure"]
impl crate::Writable for Wf49Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF49 to value 0"]
impl crate::Resettable for Wf49Spec {
    const RESET_VALUE: u8 = 0;
}
