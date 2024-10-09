#[doc = "Register `WF57` reader"]
pub type R = crate::R<Wf57Spec>;
#[doc = "Register `WF57` writer"]
pub type W = crate::W<Wf57Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd57 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd57> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD57` reader - no description available"]
pub type Bpalcd57R = crate::BitReader<Bpalcd57>;
impl Bpalcd57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd57 {
        match self.bits {
            false => Bpalcd57::B0,
            true => Bpalcd57::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd57::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd57::B1
    }
}
#[doc = "Field `BPALCD57` writer - no description available"]
pub type Bpalcd57W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd57>;
impl<'a, REG> Bpalcd57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd57::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd57::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd57 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd57> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD57` reader - no description available"]
pub type Bpblcd57R = crate::BitReader<Bpblcd57>;
impl Bpblcd57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd57 {
        match self.bits {
            false => Bpblcd57::B0,
            true => Bpblcd57::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd57::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd57::B1
    }
}
#[doc = "Field `BPBLCD57` writer - no description available"]
pub type Bpblcd57W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd57>;
impl<'a, REG> Bpblcd57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd57::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd57::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd57 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd57> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD57` reader - no description available"]
pub type Bpclcd57R = crate::BitReader<Bpclcd57>;
impl Bpclcd57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd57 {
        match self.bits {
            false => Bpclcd57::B0,
            true => Bpclcd57::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd57::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd57::B1
    }
}
#[doc = "Field `BPCLCD57` writer - no description available"]
pub type Bpclcd57W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd57>;
impl<'a, REG> Bpclcd57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd57::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd57::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd57 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd57> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD57` reader - no description available"]
pub type Bpdlcd57R = crate::BitReader<Bpdlcd57>;
impl Bpdlcd57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd57 {
        match self.bits {
            false => Bpdlcd57::B0,
            true => Bpdlcd57::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd57::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd57::B1
    }
}
#[doc = "Field `BPDLCD57` writer - no description available"]
pub type Bpdlcd57W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd57>;
impl<'a, REG> Bpdlcd57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd57::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd57::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd57 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd57> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD57` reader - no description available"]
pub type Bpelcd57R = crate::BitReader<Bpelcd57>;
impl Bpelcd57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd57 {
        match self.bits {
            false => Bpelcd57::B0,
            true => Bpelcd57::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd57::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd57::B1
    }
}
#[doc = "Field `BPELCD57` writer - no description available"]
pub type Bpelcd57W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd57>;
impl<'a, REG> Bpelcd57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd57::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd57::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd57 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd57> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD57` reader - no description available"]
pub type Bpflcd57R = crate::BitReader<Bpflcd57>;
impl Bpflcd57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd57 {
        match self.bits {
            false => Bpflcd57::B0,
            true => Bpflcd57::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd57::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd57::B1
    }
}
#[doc = "Field `BPFLCD57` writer - no description available"]
pub type Bpflcd57W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd57>;
impl<'a, REG> Bpflcd57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd57::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd57::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd57 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd57> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD57` reader - no description available"]
pub type Bpglcd57R = crate::BitReader<Bpglcd57>;
impl Bpglcd57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd57 {
        match self.bits {
            false => Bpglcd57::B0,
            true => Bpglcd57::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd57::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd57::B1
    }
}
#[doc = "Field `BPGLCD57` writer - no description available"]
pub type Bpglcd57W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd57>;
impl<'a, REG> Bpglcd57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd57::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd57::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd57 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd57> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd57) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD57` reader - no description available"]
pub type Bphlcd57R = crate::BitReader<Bphlcd57>;
impl Bphlcd57R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd57 {
        match self.bits {
            false => Bphlcd57::B0,
            true => Bphlcd57::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd57::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd57::B1
    }
}
#[doc = "Field `BPHLCD57` writer - no description available"]
pub type Bphlcd57W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd57>;
impl<'a, REG> Bphlcd57W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd57::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd57::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd57(&self) -> Bpalcd57R {
        Bpalcd57R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd57(&self) -> Bpblcd57R {
        Bpblcd57R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd57(&self) -> Bpclcd57R {
        Bpclcd57R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd57(&self) -> Bpdlcd57R {
        Bpdlcd57R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd57(&self) -> Bpelcd57R {
        Bpelcd57R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd57(&self) -> Bpflcd57R {
        Bpflcd57R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd57(&self) -> Bpglcd57R {
        Bpglcd57R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd57(&self) -> Bphlcd57R {
        Bphlcd57R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd57(&mut self) -> Bpalcd57W<Wf57Spec> {
        Bpalcd57W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd57(&mut self) -> Bpblcd57W<Wf57Spec> {
        Bpblcd57W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd57(&mut self) -> Bpclcd57W<Wf57Spec> {
        Bpclcd57W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd57(&mut self) -> Bpdlcd57W<Wf57Spec> {
        Bpdlcd57W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd57(&mut self) -> Bpelcd57W<Wf57Spec> {
        Bpelcd57W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd57(&mut self) -> Bpflcd57W<Wf57Spec> {
        Bpflcd57W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd57(&mut self) -> Bpglcd57W<Wf57Spec> {
        Bpglcd57W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd57(&mut self) -> Bphlcd57W<Wf57Spec> {
        Bphlcd57W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 57.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf57::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf57::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf57Spec;
impl crate::RegisterSpec for Wf57Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf57::R`](R) reader structure"]
impl crate::Readable for Wf57Spec {}
#[doc = "`write(|w| ..)` method takes [`wf57::W`](W) writer structure"]
impl crate::Writable for Wf57Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF57 to value 0"]
impl crate::Resettable for Wf57Spec {
    const RESET_VALUE: u8 = 0;
}
