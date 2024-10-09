#[doc = "Register `WF47` reader"]
pub type R = crate::R<Wf47Spec>;
#[doc = "Register `WF47` writer"]
pub type W = crate::W<Wf47Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd47 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd47> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD47` reader - no description available"]
pub type Bpalcd47R = crate::BitReader<Bpalcd47>;
impl Bpalcd47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd47 {
        match self.bits {
            false => Bpalcd47::B0,
            true => Bpalcd47::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd47::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd47::B1
    }
}
#[doc = "Field `BPALCD47` writer - no description available"]
pub type Bpalcd47W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd47>;
impl<'a, REG> Bpalcd47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd47::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd47::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd47 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd47> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD47` reader - no description available"]
pub type Bpblcd47R = crate::BitReader<Bpblcd47>;
impl Bpblcd47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd47 {
        match self.bits {
            false => Bpblcd47::B0,
            true => Bpblcd47::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd47::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd47::B1
    }
}
#[doc = "Field `BPBLCD47` writer - no description available"]
pub type Bpblcd47W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd47>;
impl<'a, REG> Bpblcd47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd47::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd47::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd47 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd47> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD47` reader - no description available"]
pub type Bpclcd47R = crate::BitReader<Bpclcd47>;
impl Bpclcd47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd47 {
        match self.bits {
            false => Bpclcd47::B0,
            true => Bpclcd47::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd47::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd47::B1
    }
}
#[doc = "Field `BPCLCD47` writer - no description available"]
pub type Bpclcd47W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd47>;
impl<'a, REG> Bpclcd47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd47::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd47::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd47 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd47> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD47` reader - no description available"]
pub type Bpdlcd47R = crate::BitReader<Bpdlcd47>;
impl Bpdlcd47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd47 {
        match self.bits {
            false => Bpdlcd47::B0,
            true => Bpdlcd47::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd47::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd47::B1
    }
}
#[doc = "Field `BPDLCD47` writer - no description available"]
pub type Bpdlcd47W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd47>;
impl<'a, REG> Bpdlcd47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd47::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd47::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd47 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd47> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD47` reader - no description available"]
pub type Bpelcd47R = crate::BitReader<Bpelcd47>;
impl Bpelcd47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd47 {
        match self.bits {
            false => Bpelcd47::B0,
            true => Bpelcd47::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd47::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd47::B1
    }
}
#[doc = "Field `BPELCD47` writer - no description available"]
pub type Bpelcd47W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd47>;
impl<'a, REG> Bpelcd47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd47::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd47::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd47 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd47> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD47` reader - no description available"]
pub type Bpflcd47R = crate::BitReader<Bpflcd47>;
impl Bpflcd47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd47 {
        match self.bits {
            false => Bpflcd47::B0,
            true => Bpflcd47::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd47::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd47::B1
    }
}
#[doc = "Field `BPFLCD47` writer - no description available"]
pub type Bpflcd47W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd47>;
impl<'a, REG> Bpflcd47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd47::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd47::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd47 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd47> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD47` reader - no description available"]
pub type Bpglcd47R = crate::BitReader<Bpglcd47>;
impl Bpglcd47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd47 {
        match self.bits {
            false => Bpglcd47::B0,
            true => Bpglcd47::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd47::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd47::B1
    }
}
#[doc = "Field `BPGLCD47` writer - no description available"]
pub type Bpglcd47W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd47>;
impl<'a, REG> Bpglcd47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd47::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd47::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd47 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd47> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd47) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD47` reader - no description available"]
pub type Bphlcd47R = crate::BitReader<Bphlcd47>;
impl Bphlcd47R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd47 {
        match self.bits {
            false => Bphlcd47::B0,
            true => Bphlcd47::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd47::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd47::B1
    }
}
#[doc = "Field `BPHLCD47` writer - no description available"]
pub type Bphlcd47W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd47>;
impl<'a, REG> Bphlcd47W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd47::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd47::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd47(&self) -> Bpalcd47R {
        Bpalcd47R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd47(&self) -> Bpblcd47R {
        Bpblcd47R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd47(&self) -> Bpclcd47R {
        Bpclcd47R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd47(&self) -> Bpdlcd47R {
        Bpdlcd47R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd47(&self) -> Bpelcd47R {
        Bpelcd47R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd47(&self) -> Bpflcd47R {
        Bpflcd47R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd47(&self) -> Bpglcd47R {
        Bpglcd47R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd47(&self) -> Bphlcd47R {
        Bphlcd47R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd47(&mut self) -> Bpalcd47W<Wf47Spec> {
        Bpalcd47W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd47(&mut self) -> Bpblcd47W<Wf47Spec> {
        Bpblcd47W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd47(&mut self) -> Bpclcd47W<Wf47Spec> {
        Bpclcd47W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd47(&mut self) -> Bpdlcd47W<Wf47Spec> {
        Bpdlcd47W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd47(&mut self) -> Bpelcd47W<Wf47Spec> {
        Bpelcd47W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd47(&mut self) -> Bpflcd47W<Wf47Spec> {
        Bpflcd47W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd47(&mut self) -> Bpglcd47W<Wf47Spec> {
        Bpglcd47W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd47(&mut self) -> Bphlcd47W<Wf47Spec> {
        Bphlcd47W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 47.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf47Spec;
impl crate::RegisterSpec for Wf47Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf47::R`](R) reader structure"]
impl crate::Readable for Wf47Spec {}
#[doc = "`write(|w| ..)` method takes [`wf47::W`](W) writer structure"]
impl crate::Writable for Wf47Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF47 to value 0"]
impl crate::Resettable for Wf47Spec {
    const RESET_VALUE: u8 = 0;
}
