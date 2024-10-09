#[doc = "Register `WF10` reader"]
pub type R = crate::R<Wf10Spec>;
#[doc = "Register `WF10` writer"]
pub type W = crate::W<Wf10Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd10 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd10> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD10` reader - no description available"]
pub type Bpalcd10R = crate::BitReader<Bpalcd10>;
impl Bpalcd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd10 {
        match self.bits {
            false => Bpalcd10::B0,
            true => Bpalcd10::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd10::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd10::B1
    }
}
#[doc = "Field `BPALCD10` writer - no description available"]
pub type Bpalcd10W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd10>;
impl<'a, REG> Bpalcd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd10::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd10::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd10 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd10> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD10` reader - no description available"]
pub type Bpblcd10R = crate::BitReader<Bpblcd10>;
impl Bpblcd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd10 {
        match self.bits {
            false => Bpblcd10::B0,
            true => Bpblcd10::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd10::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd10::B1
    }
}
#[doc = "Field `BPBLCD10` writer - no description available"]
pub type Bpblcd10W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd10>;
impl<'a, REG> Bpblcd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd10::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd10::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd10 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd10> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD10` reader - no description available"]
pub type Bpclcd10R = crate::BitReader<Bpclcd10>;
impl Bpclcd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd10 {
        match self.bits {
            false => Bpclcd10::B0,
            true => Bpclcd10::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd10::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd10::B1
    }
}
#[doc = "Field `BPCLCD10` writer - no description available"]
pub type Bpclcd10W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd10>;
impl<'a, REG> Bpclcd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd10::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd10::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd10 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd10> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD10` reader - no description available"]
pub type Bpdlcd10R = crate::BitReader<Bpdlcd10>;
impl Bpdlcd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd10 {
        match self.bits {
            false => Bpdlcd10::B0,
            true => Bpdlcd10::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd10::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd10::B1
    }
}
#[doc = "Field `BPDLCD10` writer - no description available"]
pub type Bpdlcd10W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd10>;
impl<'a, REG> Bpdlcd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd10::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd10::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd10 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd10> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD10` reader - no description available"]
pub type Bpelcd10R = crate::BitReader<Bpelcd10>;
impl Bpelcd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd10 {
        match self.bits {
            false => Bpelcd10::B0,
            true => Bpelcd10::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd10::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd10::B1
    }
}
#[doc = "Field `BPELCD10` writer - no description available"]
pub type Bpelcd10W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd10>;
impl<'a, REG> Bpelcd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd10::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd10::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd10 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd10> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD10` reader - no description available"]
pub type Bpflcd10R = crate::BitReader<Bpflcd10>;
impl Bpflcd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd10 {
        match self.bits {
            false => Bpflcd10::B0,
            true => Bpflcd10::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd10::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd10::B1
    }
}
#[doc = "Field `BPFLCD10` writer - no description available"]
pub type Bpflcd10W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd10>;
impl<'a, REG> Bpflcd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd10::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd10::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd10 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd10> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD10` reader - no description available"]
pub type Bpglcd10R = crate::BitReader<Bpglcd10>;
impl Bpglcd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd10 {
        match self.bits {
            false => Bpglcd10::B0,
            true => Bpglcd10::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd10::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd10::B1
    }
}
#[doc = "Field `BPGLCD10` writer - no description available"]
pub type Bpglcd10W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd10>;
impl<'a, REG> Bpglcd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd10::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd10::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd10 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd10> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD10` reader - no description available"]
pub type Bphlcd10R = crate::BitReader<Bphlcd10>;
impl Bphlcd10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd10 {
        match self.bits {
            false => Bphlcd10::B0,
            true => Bphlcd10::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd10::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd10::B1
    }
}
#[doc = "Field `BPHLCD10` writer - no description available"]
pub type Bphlcd10W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd10>;
impl<'a, REG> Bphlcd10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd10::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd10::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd10(&self) -> Bpalcd10R {
        Bpalcd10R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd10(&self) -> Bpblcd10R {
        Bpblcd10R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd10(&self) -> Bpclcd10R {
        Bpclcd10R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd10(&self) -> Bpdlcd10R {
        Bpdlcd10R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd10(&self) -> Bpelcd10R {
        Bpelcd10R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd10(&self) -> Bpflcd10R {
        Bpflcd10R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd10(&self) -> Bpglcd10R {
        Bpglcd10R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd10(&self) -> Bphlcd10R {
        Bphlcd10R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd10(&mut self) -> Bpalcd10W<Wf10Spec> {
        Bpalcd10W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd10(&mut self) -> Bpblcd10W<Wf10Spec> {
        Bpblcd10W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd10(&mut self) -> Bpclcd10W<Wf10Spec> {
        Bpclcd10W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd10(&mut self) -> Bpdlcd10W<Wf10Spec> {
        Bpdlcd10W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd10(&mut self) -> Bpelcd10W<Wf10Spec> {
        Bpelcd10W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd10(&mut self) -> Bpflcd10W<Wf10Spec> {
        Bpflcd10W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd10(&mut self) -> Bpglcd10W<Wf10Spec> {
        Bpglcd10W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd10(&mut self) -> Bphlcd10W<Wf10Spec> {
        Bphlcd10W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 10.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf10Spec;
impl crate::RegisterSpec for Wf10Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf10::R`](R) reader structure"]
impl crate::Readable for Wf10Spec {}
#[doc = "`write(|w| ..)` method takes [`wf10::W`](W) writer structure"]
impl crate::Writable for Wf10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF10 to value 0"]
impl crate::Resettable for Wf10Spec {
    const RESET_VALUE: u8 = 0;
}
