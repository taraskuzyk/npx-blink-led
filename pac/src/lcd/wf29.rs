#[doc = "Register `WF29` reader"]
pub type R = crate::R<Wf29Spec>;
#[doc = "Register `WF29` writer"]
pub type W = crate::W<Wf29Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd29 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd29> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD29` reader - no description available"]
pub type Bpalcd29R = crate::BitReader<Bpalcd29>;
impl Bpalcd29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd29 {
        match self.bits {
            false => Bpalcd29::B0,
            true => Bpalcd29::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd29::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd29::B1
    }
}
#[doc = "Field `BPALCD29` writer - no description available"]
pub type Bpalcd29W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd29>;
impl<'a, REG> Bpalcd29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd29::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd29::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd29 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd29> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD29` reader - no description available"]
pub type Bpblcd29R = crate::BitReader<Bpblcd29>;
impl Bpblcd29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd29 {
        match self.bits {
            false => Bpblcd29::B0,
            true => Bpblcd29::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd29::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd29::B1
    }
}
#[doc = "Field `BPBLCD29` writer - no description available"]
pub type Bpblcd29W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd29>;
impl<'a, REG> Bpblcd29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd29::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd29::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd29 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd29> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD29` reader - no description available"]
pub type Bpclcd29R = crate::BitReader<Bpclcd29>;
impl Bpclcd29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd29 {
        match self.bits {
            false => Bpclcd29::B0,
            true => Bpclcd29::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd29::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd29::B1
    }
}
#[doc = "Field `BPCLCD29` writer - no description available"]
pub type Bpclcd29W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd29>;
impl<'a, REG> Bpclcd29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd29::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd29::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd29 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd29> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD29` reader - no description available"]
pub type Bpdlcd29R = crate::BitReader<Bpdlcd29>;
impl Bpdlcd29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd29 {
        match self.bits {
            false => Bpdlcd29::B0,
            true => Bpdlcd29::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd29::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd29::B1
    }
}
#[doc = "Field `BPDLCD29` writer - no description available"]
pub type Bpdlcd29W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd29>;
impl<'a, REG> Bpdlcd29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd29::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd29::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd29 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd29> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD29` reader - no description available"]
pub type Bpelcd29R = crate::BitReader<Bpelcd29>;
impl Bpelcd29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd29 {
        match self.bits {
            false => Bpelcd29::B0,
            true => Bpelcd29::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd29::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd29::B1
    }
}
#[doc = "Field `BPELCD29` writer - no description available"]
pub type Bpelcd29W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd29>;
impl<'a, REG> Bpelcd29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd29::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd29::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd29 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd29> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD29` reader - no description available"]
pub type Bpflcd29R = crate::BitReader<Bpflcd29>;
impl Bpflcd29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd29 {
        match self.bits {
            false => Bpflcd29::B0,
            true => Bpflcd29::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd29::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd29::B1
    }
}
#[doc = "Field `BPFLCD29` writer - no description available"]
pub type Bpflcd29W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd29>;
impl<'a, REG> Bpflcd29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd29::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd29::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd29 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd29> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD29` reader - no description available"]
pub type Bpglcd29R = crate::BitReader<Bpglcd29>;
impl Bpglcd29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd29 {
        match self.bits {
            false => Bpglcd29::B0,
            true => Bpglcd29::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd29::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd29::B1
    }
}
#[doc = "Field `BPGLCD29` writer - no description available"]
pub type Bpglcd29W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd29>;
impl<'a, REG> Bpglcd29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd29::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd29::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd29 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd29> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD29` reader - no description available"]
pub type Bphlcd29R = crate::BitReader<Bphlcd29>;
impl Bphlcd29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd29 {
        match self.bits {
            false => Bphlcd29::B0,
            true => Bphlcd29::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd29::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd29::B1
    }
}
#[doc = "Field `BPHLCD29` writer - no description available"]
pub type Bphlcd29W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd29>;
impl<'a, REG> Bphlcd29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd29::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd29::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd29(&self) -> Bpalcd29R {
        Bpalcd29R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd29(&self) -> Bpblcd29R {
        Bpblcd29R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd29(&self) -> Bpclcd29R {
        Bpclcd29R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd29(&self) -> Bpdlcd29R {
        Bpdlcd29R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd29(&self) -> Bpelcd29R {
        Bpelcd29R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd29(&self) -> Bpflcd29R {
        Bpflcd29R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd29(&self) -> Bpglcd29R {
        Bpglcd29R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd29(&self) -> Bphlcd29R {
        Bphlcd29R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd29(&mut self) -> Bpalcd29W<Wf29Spec> {
        Bpalcd29W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd29(&mut self) -> Bpblcd29W<Wf29Spec> {
        Bpblcd29W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd29(&mut self) -> Bpclcd29W<Wf29Spec> {
        Bpclcd29W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd29(&mut self) -> Bpdlcd29W<Wf29Spec> {
        Bpdlcd29W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd29(&mut self) -> Bpelcd29W<Wf29Spec> {
        Bpelcd29W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd29(&mut self) -> Bpflcd29W<Wf29Spec> {
        Bpflcd29W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd29(&mut self) -> Bpglcd29W<Wf29Spec> {
        Bpglcd29W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd29(&mut self) -> Bphlcd29W<Wf29Spec> {
        Bphlcd29W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 29.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf29Spec;
impl crate::RegisterSpec for Wf29Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf29::R`](R) reader structure"]
impl crate::Readable for Wf29Spec {}
#[doc = "`write(|w| ..)` method takes [`wf29::W`](W) writer structure"]
impl crate::Writable for Wf29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF29 to value 0"]
impl crate::Resettable for Wf29Spec {
    const RESET_VALUE: u8 = 0;
}
