#[doc = "Register `WF53` reader"]
pub type R = crate::R<Wf53Spec>;
#[doc = "Register `WF53` writer"]
pub type W = crate::W<Wf53Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd53 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd53> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD53` reader - no description available"]
pub type Bpalcd53R = crate::BitReader<Bpalcd53>;
impl Bpalcd53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd53 {
        match self.bits {
            false => Bpalcd53::B0,
            true => Bpalcd53::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd53::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd53::B1
    }
}
#[doc = "Field `BPALCD53` writer - no description available"]
pub type Bpalcd53W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd53>;
impl<'a, REG> Bpalcd53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd53::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd53::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd53 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd53> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD53` reader - no description available"]
pub type Bpblcd53R = crate::BitReader<Bpblcd53>;
impl Bpblcd53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd53 {
        match self.bits {
            false => Bpblcd53::B0,
            true => Bpblcd53::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd53::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd53::B1
    }
}
#[doc = "Field `BPBLCD53` writer - no description available"]
pub type Bpblcd53W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd53>;
impl<'a, REG> Bpblcd53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd53::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd53::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd53 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd53> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD53` reader - no description available"]
pub type Bpclcd53R = crate::BitReader<Bpclcd53>;
impl Bpclcd53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd53 {
        match self.bits {
            false => Bpclcd53::B0,
            true => Bpclcd53::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd53::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd53::B1
    }
}
#[doc = "Field `BPCLCD53` writer - no description available"]
pub type Bpclcd53W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd53>;
impl<'a, REG> Bpclcd53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd53::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd53::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd53 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd53> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD53` reader - no description available"]
pub type Bpdlcd53R = crate::BitReader<Bpdlcd53>;
impl Bpdlcd53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd53 {
        match self.bits {
            false => Bpdlcd53::B0,
            true => Bpdlcd53::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd53::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd53::B1
    }
}
#[doc = "Field `BPDLCD53` writer - no description available"]
pub type Bpdlcd53W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd53>;
impl<'a, REG> Bpdlcd53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd53::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd53::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd53 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd53> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD53` reader - no description available"]
pub type Bpelcd53R = crate::BitReader<Bpelcd53>;
impl Bpelcd53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd53 {
        match self.bits {
            false => Bpelcd53::B0,
            true => Bpelcd53::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd53::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd53::B1
    }
}
#[doc = "Field `BPELCD53` writer - no description available"]
pub type Bpelcd53W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd53>;
impl<'a, REG> Bpelcd53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd53::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd53::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd53 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd53> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD53` reader - no description available"]
pub type Bpflcd53R = crate::BitReader<Bpflcd53>;
impl Bpflcd53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd53 {
        match self.bits {
            false => Bpflcd53::B0,
            true => Bpflcd53::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd53::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd53::B1
    }
}
#[doc = "Field `BPFLCD53` writer - no description available"]
pub type Bpflcd53W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd53>;
impl<'a, REG> Bpflcd53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd53::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd53::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd53 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd53> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD53` reader - no description available"]
pub type Bpglcd53R = crate::BitReader<Bpglcd53>;
impl Bpglcd53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd53 {
        match self.bits {
            false => Bpglcd53::B0,
            true => Bpglcd53::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd53::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd53::B1
    }
}
#[doc = "Field `BPGLCD53` writer - no description available"]
pub type Bpglcd53W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd53>;
impl<'a, REG> Bpglcd53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd53::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd53::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd53 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd53> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd53) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD53` reader - no description available"]
pub type Bphlcd53R = crate::BitReader<Bphlcd53>;
impl Bphlcd53R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd53 {
        match self.bits {
            false => Bphlcd53::B0,
            true => Bphlcd53::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd53::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd53::B1
    }
}
#[doc = "Field `BPHLCD53` writer - no description available"]
pub type Bphlcd53W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd53>;
impl<'a, REG> Bphlcd53W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd53::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd53::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd53(&self) -> Bpalcd53R {
        Bpalcd53R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd53(&self) -> Bpblcd53R {
        Bpblcd53R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd53(&self) -> Bpclcd53R {
        Bpclcd53R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd53(&self) -> Bpdlcd53R {
        Bpdlcd53R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd53(&self) -> Bpelcd53R {
        Bpelcd53R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd53(&self) -> Bpflcd53R {
        Bpflcd53R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd53(&self) -> Bpglcd53R {
        Bpglcd53R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd53(&self) -> Bphlcd53R {
        Bphlcd53R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd53(&mut self) -> Bpalcd53W<Wf53Spec> {
        Bpalcd53W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd53(&mut self) -> Bpblcd53W<Wf53Spec> {
        Bpblcd53W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd53(&mut self) -> Bpclcd53W<Wf53Spec> {
        Bpclcd53W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd53(&mut self) -> Bpdlcd53W<Wf53Spec> {
        Bpdlcd53W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd53(&mut self) -> Bpelcd53W<Wf53Spec> {
        Bpelcd53W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd53(&mut self) -> Bpflcd53W<Wf53Spec> {
        Bpflcd53W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd53(&mut self) -> Bpglcd53W<Wf53Spec> {
        Bpglcd53W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd53(&mut self) -> Bphlcd53W<Wf53Spec> {
        Bphlcd53W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 53.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf53::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf53::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf53Spec;
impl crate::RegisterSpec for Wf53Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf53::R`](R) reader structure"]
impl crate::Readable for Wf53Spec {}
#[doc = "`write(|w| ..)` method takes [`wf53::W`](W) writer structure"]
impl crate::Writable for Wf53Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF53 to value 0"]
impl crate::Resettable for Wf53Spec {
    const RESET_VALUE: u8 = 0;
}
