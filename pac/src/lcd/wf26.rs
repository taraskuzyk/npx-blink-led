#[doc = "Register `WF26` reader"]
pub type R = crate::R<Wf26Spec>;
#[doc = "Register `WF26` writer"]
pub type W = crate::W<Wf26Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd26 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd26> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD26` reader - no description available"]
pub type Bpalcd26R = crate::BitReader<Bpalcd26>;
impl Bpalcd26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd26 {
        match self.bits {
            false => Bpalcd26::B0,
            true => Bpalcd26::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd26::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd26::B1
    }
}
#[doc = "Field `BPALCD26` writer - no description available"]
pub type Bpalcd26W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd26>;
impl<'a, REG> Bpalcd26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd26::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd26::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd26 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd26> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD26` reader - no description available"]
pub type Bpblcd26R = crate::BitReader<Bpblcd26>;
impl Bpblcd26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd26 {
        match self.bits {
            false => Bpblcd26::B0,
            true => Bpblcd26::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd26::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd26::B1
    }
}
#[doc = "Field `BPBLCD26` writer - no description available"]
pub type Bpblcd26W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd26>;
impl<'a, REG> Bpblcd26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd26::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd26::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd26 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd26> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD26` reader - no description available"]
pub type Bpclcd26R = crate::BitReader<Bpclcd26>;
impl Bpclcd26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd26 {
        match self.bits {
            false => Bpclcd26::B0,
            true => Bpclcd26::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd26::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd26::B1
    }
}
#[doc = "Field `BPCLCD26` writer - no description available"]
pub type Bpclcd26W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd26>;
impl<'a, REG> Bpclcd26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd26::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd26::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd26 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd26> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD26` reader - no description available"]
pub type Bpdlcd26R = crate::BitReader<Bpdlcd26>;
impl Bpdlcd26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd26 {
        match self.bits {
            false => Bpdlcd26::B0,
            true => Bpdlcd26::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd26::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd26::B1
    }
}
#[doc = "Field `BPDLCD26` writer - no description available"]
pub type Bpdlcd26W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd26>;
impl<'a, REG> Bpdlcd26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd26::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd26::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd26 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd26> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD26` reader - no description available"]
pub type Bpelcd26R = crate::BitReader<Bpelcd26>;
impl Bpelcd26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd26 {
        match self.bits {
            false => Bpelcd26::B0,
            true => Bpelcd26::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd26::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd26::B1
    }
}
#[doc = "Field `BPELCD26` writer - no description available"]
pub type Bpelcd26W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd26>;
impl<'a, REG> Bpelcd26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd26::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd26::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd26 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd26> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD26` reader - no description available"]
pub type Bpflcd26R = crate::BitReader<Bpflcd26>;
impl Bpflcd26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd26 {
        match self.bits {
            false => Bpflcd26::B0,
            true => Bpflcd26::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd26::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd26::B1
    }
}
#[doc = "Field `BPFLCD26` writer - no description available"]
pub type Bpflcd26W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd26>;
impl<'a, REG> Bpflcd26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd26::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd26::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd26 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd26> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD26` reader - no description available"]
pub type Bpglcd26R = crate::BitReader<Bpglcd26>;
impl Bpglcd26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd26 {
        match self.bits {
            false => Bpglcd26::B0,
            true => Bpglcd26::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd26::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd26::B1
    }
}
#[doc = "Field `BPGLCD26` writer - no description available"]
pub type Bpglcd26W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd26>;
impl<'a, REG> Bpglcd26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd26::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd26::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd26 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd26> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD26` reader - no description available"]
pub type Bphlcd26R = crate::BitReader<Bphlcd26>;
impl Bphlcd26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd26 {
        match self.bits {
            false => Bphlcd26::B0,
            true => Bphlcd26::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd26::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd26::B1
    }
}
#[doc = "Field `BPHLCD26` writer - no description available"]
pub type Bphlcd26W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd26>;
impl<'a, REG> Bphlcd26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd26::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd26::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd26(&self) -> Bpalcd26R {
        Bpalcd26R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd26(&self) -> Bpblcd26R {
        Bpblcd26R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd26(&self) -> Bpclcd26R {
        Bpclcd26R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd26(&self) -> Bpdlcd26R {
        Bpdlcd26R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd26(&self) -> Bpelcd26R {
        Bpelcd26R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd26(&self) -> Bpflcd26R {
        Bpflcd26R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd26(&self) -> Bpglcd26R {
        Bpglcd26R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd26(&self) -> Bphlcd26R {
        Bphlcd26R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd26(&mut self) -> Bpalcd26W<Wf26Spec> {
        Bpalcd26W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd26(&mut self) -> Bpblcd26W<Wf26Spec> {
        Bpblcd26W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd26(&mut self) -> Bpclcd26W<Wf26Spec> {
        Bpclcd26W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd26(&mut self) -> Bpdlcd26W<Wf26Spec> {
        Bpdlcd26W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd26(&mut self) -> Bpelcd26W<Wf26Spec> {
        Bpelcd26W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd26(&mut self) -> Bpflcd26W<Wf26Spec> {
        Bpflcd26W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd26(&mut self) -> Bpglcd26W<Wf26Spec> {
        Bpglcd26W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd26(&mut self) -> Bphlcd26W<Wf26Spec> {
        Bphlcd26W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 26.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf26Spec;
impl crate::RegisterSpec for Wf26Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf26::R`](R) reader structure"]
impl crate::Readable for Wf26Spec {}
#[doc = "`write(|w| ..)` method takes [`wf26::W`](W) writer structure"]
impl crate::Writable for Wf26Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF26 to value 0"]
impl crate::Resettable for Wf26Spec {
    const RESET_VALUE: u8 = 0;
}
