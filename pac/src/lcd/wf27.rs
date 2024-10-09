#[doc = "Register `WF27` reader"]
pub type R = crate::R<Wf27Spec>;
#[doc = "Register `WF27` writer"]
pub type W = crate::W<Wf27Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd27 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd27> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD27` reader - no description available"]
pub type Bpalcd27R = crate::BitReader<Bpalcd27>;
impl Bpalcd27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd27 {
        match self.bits {
            false => Bpalcd27::B0,
            true => Bpalcd27::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd27::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd27::B1
    }
}
#[doc = "Field `BPALCD27` writer - no description available"]
pub type Bpalcd27W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd27>;
impl<'a, REG> Bpalcd27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd27::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd27::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd27 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd27> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD27` reader - no description available"]
pub type Bpblcd27R = crate::BitReader<Bpblcd27>;
impl Bpblcd27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd27 {
        match self.bits {
            false => Bpblcd27::B0,
            true => Bpblcd27::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd27::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd27::B1
    }
}
#[doc = "Field `BPBLCD27` writer - no description available"]
pub type Bpblcd27W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd27>;
impl<'a, REG> Bpblcd27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd27::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd27::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd27 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd27> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD27` reader - no description available"]
pub type Bpclcd27R = crate::BitReader<Bpclcd27>;
impl Bpclcd27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd27 {
        match self.bits {
            false => Bpclcd27::B0,
            true => Bpclcd27::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd27::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd27::B1
    }
}
#[doc = "Field `BPCLCD27` writer - no description available"]
pub type Bpclcd27W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd27>;
impl<'a, REG> Bpclcd27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd27::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd27::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd27 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd27> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD27` reader - no description available"]
pub type Bpdlcd27R = crate::BitReader<Bpdlcd27>;
impl Bpdlcd27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd27 {
        match self.bits {
            false => Bpdlcd27::B0,
            true => Bpdlcd27::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd27::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd27::B1
    }
}
#[doc = "Field `BPDLCD27` writer - no description available"]
pub type Bpdlcd27W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd27>;
impl<'a, REG> Bpdlcd27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd27::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd27::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd27 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd27> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD27` reader - no description available"]
pub type Bpelcd27R = crate::BitReader<Bpelcd27>;
impl Bpelcd27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd27 {
        match self.bits {
            false => Bpelcd27::B0,
            true => Bpelcd27::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd27::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd27::B1
    }
}
#[doc = "Field `BPELCD27` writer - no description available"]
pub type Bpelcd27W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd27>;
impl<'a, REG> Bpelcd27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd27::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd27::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd27 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd27> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD27` reader - no description available"]
pub type Bpflcd27R = crate::BitReader<Bpflcd27>;
impl Bpflcd27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd27 {
        match self.bits {
            false => Bpflcd27::B0,
            true => Bpflcd27::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd27::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd27::B1
    }
}
#[doc = "Field `BPFLCD27` writer - no description available"]
pub type Bpflcd27W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd27>;
impl<'a, REG> Bpflcd27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd27::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd27::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd27 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd27> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD27` reader - no description available"]
pub type Bpglcd27R = crate::BitReader<Bpglcd27>;
impl Bpglcd27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd27 {
        match self.bits {
            false => Bpglcd27::B0,
            true => Bpglcd27::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd27::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd27::B1
    }
}
#[doc = "Field `BPGLCD27` writer - no description available"]
pub type Bpglcd27W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd27>;
impl<'a, REG> Bpglcd27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd27::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd27::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd27 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd27> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD27` reader - no description available"]
pub type Bphlcd27R = crate::BitReader<Bphlcd27>;
impl Bphlcd27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd27 {
        match self.bits {
            false => Bphlcd27::B0,
            true => Bphlcd27::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd27::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd27::B1
    }
}
#[doc = "Field `BPHLCD27` writer - no description available"]
pub type Bphlcd27W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd27>;
impl<'a, REG> Bphlcd27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd27::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd27::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd27(&self) -> Bpalcd27R {
        Bpalcd27R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd27(&self) -> Bpblcd27R {
        Bpblcd27R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd27(&self) -> Bpclcd27R {
        Bpclcd27R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd27(&self) -> Bpdlcd27R {
        Bpdlcd27R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd27(&self) -> Bpelcd27R {
        Bpelcd27R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd27(&self) -> Bpflcd27R {
        Bpflcd27R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd27(&self) -> Bpglcd27R {
        Bpglcd27R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd27(&self) -> Bphlcd27R {
        Bphlcd27R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd27(&mut self) -> Bpalcd27W<Wf27Spec> {
        Bpalcd27W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd27(&mut self) -> Bpblcd27W<Wf27Spec> {
        Bpblcd27W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd27(&mut self) -> Bpclcd27W<Wf27Spec> {
        Bpclcd27W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd27(&mut self) -> Bpdlcd27W<Wf27Spec> {
        Bpdlcd27W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd27(&mut self) -> Bpelcd27W<Wf27Spec> {
        Bpelcd27W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd27(&mut self) -> Bpflcd27W<Wf27Spec> {
        Bpflcd27W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd27(&mut self) -> Bpglcd27W<Wf27Spec> {
        Bpglcd27W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd27(&mut self) -> Bphlcd27W<Wf27Spec> {
        Bphlcd27W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 27.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf27Spec;
impl crate::RegisterSpec for Wf27Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf27::R`](R) reader structure"]
impl crate::Readable for Wf27Spec {}
#[doc = "`write(|w| ..)` method takes [`wf27::W`](W) writer structure"]
impl crate::Writable for Wf27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF27 to value 0"]
impl crate::Resettable for Wf27Spec {
    const RESET_VALUE: u8 = 0;
}
