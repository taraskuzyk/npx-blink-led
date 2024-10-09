#[doc = "Register `WF41` reader"]
pub type R = crate::R<Wf41Spec>;
#[doc = "Register `WF41` writer"]
pub type W = crate::W<Wf41Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd41 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd41> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD41` reader - no description available"]
pub type Bpalcd41R = crate::BitReader<Bpalcd41>;
impl Bpalcd41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd41 {
        match self.bits {
            false => Bpalcd41::B0,
            true => Bpalcd41::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd41::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd41::B1
    }
}
#[doc = "Field `BPALCD41` writer - no description available"]
pub type Bpalcd41W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd41>;
impl<'a, REG> Bpalcd41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd41::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd41::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd41 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd41> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD41` reader - no description available"]
pub type Bpblcd41R = crate::BitReader<Bpblcd41>;
impl Bpblcd41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd41 {
        match self.bits {
            false => Bpblcd41::B0,
            true => Bpblcd41::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd41::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd41::B1
    }
}
#[doc = "Field `BPBLCD41` writer - no description available"]
pub type Bpblcd41W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd41>;
impl<'a, REG> Bpblcd41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd41::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd41::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd41 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd41> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD41` reader - no description available"]
pub type Bpclcd41R = crate::BitReader<Bpclcd41>;
impl Bpclcd41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd41 {
        match self.bits {
            false => Bpclcd41::B0,
            true => Bpclcd41::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd41::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd41::B1
    }
}
#[doc = "Field `BPCLCD41` writer - no description available"]
pub type Bpclcd41W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd41>;
impl<'a, REG> Bpclcd41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd41::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd41::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd41 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd41> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD41` reader - no description available"]
pub type Bpdlcd41R = crate::BitReader<Bpdlcd41>;
impl Bpdlcd41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd41 {
        match self.bits {
            false => Bpdlcd41::B0,
            true => Bpdlcd41::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd41::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd41::B1
    }
}
#[doc = "Field `BPDLCD41` writer - no description available"]
pub type Bpdlcd41W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd41>;
impl<'a, REG> Bpdlcd41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd41::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd41::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd41 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd41> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD41` reader - no description available"]
pub type Bpelcd41R = crate::BitReader<Bpelcd41>;
impl Bpelcd41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd41 {
        match self.bits {
            false => Bpelcd41::B0,
            true => Bpelcd41::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd41::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd41::B1
    }
}
#[doc = "Field `BPELCD41` writer - no description available"]
pub type Bpelcd41W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd41>;
impl<'a, REG> Bpelcd41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd41::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd41::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd41 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd41> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD41` reader - no description available"]
pub type Bpflcd41R = crate::BitReader<Bpflcd41>;
impl Bpflcd41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd41 {
        match self.bits {
            false => Bpflcd41::B0,
            true => Bpflcd41::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd41::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd41::B1
    }
}
#[doc = "Field `BPFLCD41` writer - no description available"]
pub type Bpflcd41W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd41>;
impl<'a, REG> Bpflcd41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd41::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd41::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd41 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd41> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD41` reader - no description available"]
pub type Bpglcd41R = crate::BitReader<Bpglcd41>;
impl Bpglcd41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd41 {
        match self.bits {
            false => Bpglcd41::B0,
            true => Bpglcd41::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd41::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd41::B1
    }
}
#[doc = "Field `BPGLCD41` writer - no description available"]
pub type Bpglcd41W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd41>;
impl<'a, REG> Bpglcd41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd41::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd41::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd41 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd41> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd41) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD41` reader - no description available"]
pub type Bphlcd41R = crate::BitReader<Bphlcd41>;
impl Bphlcd41R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd41 {
        match self.bits {
            false => Bphlcd41::B0,
            true => Bphlcd41::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd41::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd41::B1
    }
}
#[doc = "Field `BPHLCD41` writer - no description available"]
pub type Bphlcd41W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd41>;
impl<'a, REG> Bphlcd41W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd41::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd41::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd41(&self) -> Bpalcd41R {
        Bpalcd41R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd41(&self) -> Bpblcd41R {
        Bpblcd41R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd41(&self) -> Bpclcd41R {
        Bpclcd41R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd41(&self) -> Bpdlcd41R {
        Bpdlcd41R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd41(&self) -> Bpelcd41R {
        Bpelcd41R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd41(&self) -> Bpflcd41R {
        Bpflcd41R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd41(&self) -> Bpglcd41R {
        Bpglcd41R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd41(&self) -> Bphlcd41R {
        Bphlcd41R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd41(&mut self) -> Bpalcd41W<Wf41Spec> {
        Bpalcd41W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd41(&mut self) -> Bpblcd41W<Wf41Spec> {
        Bpblcd41W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd41(&mut self) -> Bpclcd41W<Wf41Spec> {
        Bpclcd41W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd41(&mut self) -> Bpdlcd41W<Wf41Spec> {
        Bpdlcd41W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd41(&mut self) -> Bpelcd41W<Wf41Spec> {
        Bpelcd41W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd41(&mut self) -> Bpflcd41W<Wf41Spec> {
        Bpflcd41W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd41(&mut self) -> Bpglcd41W<Wf41Spec> {
        Bpglcd41W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd41(&mut self) -> Bphlcd41W<Wf41Spec> {
        Bphlcd41W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 41.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf41::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf41::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf41Spec;
impl crate::RegisterSpec for Wf41Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf41::R`](R) reader structure"]
impl crate::Readable for Wf41Spec {}
#[doc = "`write(|w| ..)` method takes [`wf41::W`](W) writer structure"]
impl crate::Writable for Wf41Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF41 to value 0"]
impl crate::Resettable for Wf41Spec {
    const RESET_VALUE: u8 = 0;
}
