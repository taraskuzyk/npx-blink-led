#[doc = "Register `WF56` reader"]
pub type R = crate::R<LcdWf56Spec>;
#[doc = "Register `WF56` writer"]
pub type W = crate::W<LcdWf56Spec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpalcd56 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase A"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase A"]
    B1 = 1,
}
impl From<Bpalcd56> for bool {
    #[inline(always)]
    fn from(variant: Bpalcd56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPALCD56` reader - no description available"]
pub type Bpalcd56R = crate::BitReader<Bpalcd56>;
impl Bpalcd56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpalcd56 {
        match self.bits {
            false => Bpalcd56::B0,
            true => Bpalcd56::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpalcd56::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpalcd56::B1
    }
}
#[doc = "Field `BPALCD56` writer - no description available"]
pub type Bpalcd56W<'a, REG> = crate::BitWriter<'a, REG, Bpalcd56>;
impl<'a, REG> Bpalcd56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase A"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd56::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase A"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpalcd56::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpblcd56 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase B"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase B"]
    B1 = 1,
}
impl From<Bpblcd56> for bool {
    #[inline(always)]
    fn from(variant: Bpblcd56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPBLCD56` reader - no description available"]
pub type Bpblcd56R = crate::BitReader<Bpblcd56>;
impl Bpblcd56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpblcd56 {
        match self.bits {
            false => Bpblcd56::B0,
            true => Bpblcd56::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpblcd56::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpblcd56::B1
    }
}
#[doc = "Field `BPBLCD56` writer - no description available"]
pub type Bpblcd56W<'a, REG> = crate::BitWriter<'a, REG, Bpblcd56>;
impl<'a, REG> Bpblcd56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase B"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd56::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase B"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpblcd56::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpclcd56 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase C"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase C"]
    B1 = 1,
}
impl From<Bpclcd56> for bool {
    #[inline(always)]
    fn from(variant: Bpclcd56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPCLCD56` reader - no description available"]
pub type Bpclcd56R = crate::BitReader<Bpclcd56>;
impl Bpclcd56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpclcd56 {
        match self.bits {
            false => Bpclcd56::B0,
            true => Bpclcd56::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpclcd56::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpclcd56::B1
    }
}
#[doc = "Field `BPCLCD56` writer - no description available"]
pub type Bpclcd56W<'a, REG> = crate::BitWriter<'a, REG, Bpclcd56>;
impl<'a, REG> Bpclcd56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase C"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd56::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase C"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpclcd56::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpdlcd56 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase D"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase D"]
    B1 = 1,
}
impl From<Bpdlcd56> for bool {
    #[inline(always)]
    fn from(variant: Bpdlcd56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPDLCD56` reader - no description available"]
pub type Bpdlcd56R = crate::BitReader<Bpdlcd56>;
impl Bpdlcd56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpdlcd56 {
        match self.bits {
            false => Bpdlcd56::B0,
            true => Bpdlcd56::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpdlcd56::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpdlcd56::B1
    }
}
#[doc = "Field `BPDLCD56` writer - no description available"]
pub type Bpdlcd56W<'a, REG> = crate::BitWriter<'a, REG, Bpdlcd56>;
impl<'a, REG> Bpdlcd56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase D"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd56::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase D"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpdlcd56::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpelcd56 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase E"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase E"]
    B1 = 1,
}
impl From<Bpelcd56> for bool {
    #[inline(always)]
    fn from(variant: Bpelcd56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPELCD56` reader - no description available"]
pub type Bpelcd56R = crate::BitReader<Bpelcd56>;
impl Bpelcd56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpelcd56 {
        match self.bits {
            false => Bpelcd56::B0,
            true => Bpelcd56::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpelcd56::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpelcd56::B1
    }
}
#[doc = "Field `BPELCD56` writer - no description available"]
pub type Bpelcd56W<'a, REG> = crate::BitWriter<'a, REG, Bpelcd56>;
impl<'a, REG> Bpelcd56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase E"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd56::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase E"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpelcd56::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpflcd56 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase F"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase F"]
    B1 = 1,
}
impl From<Bpflcd56> for bool {
    #[inline(always)]
    fn from(variant: Bpflcd56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPFLCD56` reader - no description available"]
pub type Bpflcd56R = crate::BitReader<Bpflcd56>;
impl Bpflcd56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpflcd56 {
        match self.bits {
            false => Bpflcd56::B0,
            true => Bpflcd56::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpflcd56::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpflcd56::B1
    }
}
#[doc = "Field `BPFLCD56` writer - no description available"]
pub type Bpflcd56W<'a, REG> = crate::BitWriter<'a, REG, Bpflcd56>;
impl<'a, REG> Bpflcd56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase F"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd56::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase F"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpflcd56::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bpglcd56 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase G"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase G"]
    B1 = 1,
}
impl From<Bpglcd56> for bool {
    #[inline(always)]
    fn from(variant: Bpglcd56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPGLCD56` reader - no description available"]
pub type Bpglcd56R = crate::BitReader<Bpglcd56>;
impl Bpglcd56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bpglcd56 {
        match self.bits {
            false => Bpglcd56::B0,
            true => Bpglcd56::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bpglcd56::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bpglcd56::B1
    }
}
#[doc = "Field `BPGLCD56` writer - no description available"]
pub type Bpglcd56W<'a, REG> = crate::BitWriter<'a, REG, Bpglcd56>;
impl<'a, REG> Bpglcd56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase G"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd56::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase G"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bpglcd56::B1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bphlcd56 {
    #[doc = "0: LCD segment off or LCD backplane inactive for phase H"]
    B0 = 0,
    #[doc = "1: LCD segment on or LCD backplane active for phase H"]
    B1 = 1,
}
impl From<Bphlcd56> for bool {
    #[inline(always)]
    fn from(variant: Bphlcd56) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BPHLCD56` reader - no description available"]
pub type Bphlcd56R = crate::BitReader<Bphlcd56>;
impl Bphlcd56R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bphlcd56 {
        match self.bits {
            false => Bphlcd56::B0,
            true => Bphlcd56::B1,
        }
    }
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bphlcd56::B0
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bphlcd56::B1
    }
}
#[doc = "Field `BPHLCD56` writer - no description available"]
pub type Bphlcd56W<'a, REG> = crate::BitWriter<'a, REG, Bphlcd56>;
impl<'a, REG> Bphlcd56W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD segment off or LCD backplane inactive for phase H"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd56::B0)
    }
    #[doc = "LCD segment on or LCD backplane active for phase H"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bphlcd56::B1)
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn bpalcd56(&self) -> Bpalcd56R {
        Bpalcd56R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bpblcd56(&self) -> Bpblcd56R {
        Bpblcd56R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn bpclcd56(&self) -> Bpclcd56R {
        Bpclcd56R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn bpdlcd56(&self) -> Bpdlcd56R {
        Bpdlcd56R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn bpelcd56(&self) -> Bpelcd56R {
        Bpelcd56R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn bpflcd56(&self) -> Bpflcd56R {
        Bpflcd56R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    pub fn bpglcd56(&self) -> Bpglcd56R {
        Bpglcd56R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    pub fn bphlcd56(&self) -> Bphlcd56R {
        Bphlcd56R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpalcd56(&mut self) -> Bpalcd56W<LcdWf56Spec> {
        Bpalcd56W::new(self, 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpblcd56(&mut self) -> Bpblcd56W<LcdWf56Spec> {
        Bpblcd56W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpclcd56(&mut self) -> Bpclcd56W<LcdWf56Spec> {
        Bpclcd56W::new(self, 2)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpdlcd56(&mut self) -> Bpdlcd56W<LcdWf56Spec> {
        Bpdlcd56W::new(self, 3)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpelcd56(&mut self) -> Bpelcd56W<LcdWf56Spec> {
        Bpelcd56W::new(self, 4)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpflcd56(&mut self) -> Bpflcd56W<LcdWf56Spec> {
        Bpflcd56W::new(self, 5)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bpglcd56(&mut self) -> Bpglcd56W<LcdWf56Spec> {
        Bpglcd56W::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn bphlcd56(&mut self) -> Bphlcd56W<LcdWf56Spec> {
        Bphlcd56W::new(self, 7)
    }
}
#[doc = "LCD Waveform Register 56.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf56::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf56::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf56Spec;
impl crate::RegisterSpec for LcdWf56Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lcd_wf56::R`](R) reader structure"]
impl crate::Readable for LcdWf56Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf56::W`](W) writer structure"]
impl crate::Writable for LcdWf56Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WF56 to value 0"]
impl crate::Resettable for LcdWf56Spec {
    const RESET_VALUE: u8 = 0;
}
