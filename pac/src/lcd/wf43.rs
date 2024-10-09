#[doc = "Register `WF43` reader"]
pub type R = crate::R<Wf43Spec>;
#[doc = "Register `WF43` writer"]
pub type W = crate::W<Wf43Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd43 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd43> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD43` reader - no description available"]
pub type Bpalcd43R = crate::BitReader<Bpalcd43>;
impl Bpalcd43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd43 {
        match self.bits {
            false => Bpalcd43::B0,
            true => Bpalcd43::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd43::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd43::B1
    }
}
#[doc = "Field `BPALCD43` writer - no description available"]
pub type Bpalcd43W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd43>;
impl<'a, REG> Bpalcd43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd43::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd43::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd43 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd43> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD43` reader - no description available"]
pub type Bpblcd43R = crate::BitReader<Bpblcd43>;
impl Bpblcd43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd43 {
        match self.bits {
            false => Bpblcd43::B0,
            true => Bpblcd43::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd43::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd43::B1
    }
}
#[doc = "Field `BPBLCD43` writer - no description available"]
pub type Bpblcd43W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd43>;
impl<'a, REG> Bpblcd43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd43::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd43::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd43 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd43> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD43` reader - no description available"]
pub type Bpclcd43R = crate::BitReader<Bpclcd43>;
impl Bpclcd43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd43 {
        match self.bits {
            false => Bpclcd43::B0,
            true => Bpclcd43::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd43::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd43::B1
    }
}
#[doc = "Field `BPCLCD43` writer - no description available"]
pub type Bpclcd43W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd43>;
impl<'a, REG> Bpclcd43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd43::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd43::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd43 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd43> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD43` reader - no description available"]
pub type Bpdlcd43R = crate::BitReader<Bpdlcd43>;
impl Bpdlcd43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd43 {
        match self.bits {
            false => Bpdlcd43::B0,
            true => Bpdlcd43::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd43::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd43::B1
    }
}
#[doc = "Field `BPDLCD43` writer - no description available"]
pub type Bpdlcd43W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd43>;
impl<'a, REG> Bpdlcd43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd43::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd43::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd43 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd43> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD43` reader - no description available"]
pub type Bpelcd43R = crate::BitReader<Bpelcd43>;
impl Bpelcd43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd43 {
        match self.bits {
            false => Bpelcd43::B0,
            true => Bpelcd43::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd43::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd43::B1
    }
}
#[doc = "Field `BPELCD43` writer - no description available"]
pub type Bpelcd43W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd43>;
impl<'a, REG> Bpelcd43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd43::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd43::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd43 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd43> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD43` reader - no description available"]
pub type Bpflcd43R = crate::BitReader<Bpflcd43>;
impl Bpflcd43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd43 {
        match self.bits {
            false => Bpflcd43::B0,
            true => Bpflcd43::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd43::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd43::B1
    }
}
#[doc = "Field `BPFLCD43` writer - no description available"]
pub type Bpflcd43W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd43>;
impl<'a, REG> Bpflcd43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd43::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd43::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd43 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd43> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD43` reader - no description available"]
pub type Bpglcd43R = crate::BitReader<Bpglcd43>;
impl Bpglcd43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd43 {
        match self.bits {
            false => Bpglcd43::B0,
            true => Bpglcd43::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd43::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd43::B1
    }
}
#[doc = "Field `BPGLCD43` writer - no description available"]
pub type Bpglcd43W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd43>;
impl<'a, REG> Bpglcd43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd43::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd43::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd43 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd43> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd43) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD43` reader - no description available"]
pub type Bphlcd43R = crate::BitReader<Bphlcd43>;
impl Bphlcd43R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd43 {
        match self.bits {
            false => Bphlcd43::B0,
            true => Bphlcd43::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd43::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd43::B1
    }
}
#[doc = "Field `BPHLCD43` writer - no description available"]
pub type Bphlcd43W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd43>;
impl<'a, REG> Bphlcd43W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd43::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd43::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd43(&self) -> Bpalcd43R {
        Bpalcd43R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd43(&self) -> Bpblcd43R {
        Bpblcd43R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd43(&self) -> Bpclcd43R {
        Bpclcd43R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd43(&self) -> Bpdlcd43R {
        Bpdlcd43R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd43(&self) -> Bpelcd43R {
        Bpelcd43R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd43(&self) -> Bpflcd43R {
        Bpflcd43R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd43(&self) -> Bpglcd43R {
        Bpglcd43R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd43(&self) -> Bphlcd43R {
        Bphlcd43R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd43(&mut self) -> Bpalcd43W<Wf43Spec> {
        Bpalcd43W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd43(&mut self) -> Bpblcd43W<Wf43Spec> {
        Bpblcd43W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd43(&mut self) -> Bpclcd43W<Wf43Spec> {
        Bpclcd43W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd43(&mut self) -> Bpdlcd43W<Wf43Spec> {
        Bpdlcd43W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd43(&mut self) -> Bpelcd43W<Wf43Spec> {
        Bpelcd43W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd43(&mut self) -> Bpflcd43W<Wf43Spec> {
        Bpflcd43W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd43(&mut self) -> Bpglcd43W<Wf43Spec> {
        Bpglcd43W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd43(&mut self) -> Bphlcd43W<Wf43Spec> {
        Bphlcd43W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 43.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wf43Spec;
impl crate::RegisterSpec for Wf43Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wf43::R`](R) reader structure"]
impl crate::Readable for Wf43Spec {}
#[doc = "`write(|w| ..)` method takes [`wf43::W`](W) writer structure"]
impl crate::Writable for Wf43Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF43 to value 0"]
impl crate::Resettable for Wf43Spec {
    const RESET_VALUE: u8 = 0;
}
