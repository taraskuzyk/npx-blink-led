#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "FlexIO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flexen {
    #[doc = "0: FlexIO module is disabled."]
    B0 = 0,
    #[doc = "1: FlexIO module is enabled."]
    B1 = 1,
}
impl From<Flexen> for bool {
    #[inline(always)]
    fn from(variant: Flexen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLEXEN` reader - FlexIO Enable"]
pub type FlexenR = crate::BitReader<Flexen>;
impl FlexenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexen {
        match self.bits {
            false => Flexen::B0,
            true => Flexen::B1,
        }
    }
    #[doc = "FlexIO module is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Flexen::B0
    }
    #[doc = "FlexIO module is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Flexen::B1
    }
}
#[doc = "Field `FLEXEN` writer - FlexIO Enable"]
pub type FlexenW<'a, REG> = crate::BitWriter<'a, REG, Flexen>;
impl<'a, REG> FlexenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FlexIO module is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Flexen::B0)
    }
    #[doc = "FlexIO module is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Flexen::B1)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrst {
    #[doc = "0: Software reset is disabled"]
    B0 = 0,
    #[doc = "1: Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    B1 = 1,
}
impl From<Swrst> for bool {
    #[inline(always)]
    fn from(variant: Swrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader<Swrst>;
impl SwrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrst {
        match self.bits {
            false => Swrst::B0,
            true => Swrst::B1,
        }
    }
    #[doc = "Software reset is disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Swrst::B0
    }
    #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Swrst::B1
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG, Swrst>;
impl<'a, REG> SwrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software reset is disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Swrst::B0)
    }
    #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Swrst::B1)
    }
}
#[doc = "Fast Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fastacc {
    #[doc = "0: Configures for normal register accesses to FlexIO"]
    B0 = 0,
    #[doc = "1: Configures for fast register accesses to FlexIO"]
    B1 = 1,
}
impl From<Fastacc> for bool {
    #[inline(always)]
    fn from(variant: Fastacc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FASTACC` reader - Fast Access"]
pub type FastaccR = crate::BitReader<Fastacc>;
impl FastaccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fastacc {
        match self.bits {
            false => Fastacc::B0,
            true => Fastacc::B1,
        }
    }
    #[doc = "Configures for normal register accesses to FlexIO"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fastacc::B0
    }
    #[doc = "Configures for fast register accesses to FlexIO"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fastacc::B1
    }
}
#[doc = "Field `FASTACC` writer - Fast Access"]
pub type FastaccW<'a, REG> = crate::BitWriter<'a, REG, Fastacc>;
impl<'a, REG> FastaccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configures for normal register accesses to FlexIO"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fastacc::B0)
    }
    #[doc = "Configures for fast register accesses to FlexIO"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fastacc::B1)
    }
}
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbge {
    #[doc = "0: FlexIO is disabled in debug modes."]
    B0 = 0,
    #[doc = "1: FlexIO is enabled in debug modes"]
    B1 = 1,
}
impl From<Dbge> for bool {
    #[inline(always)]
    fn from(variant: Dbge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGE` reader - Debug Enable"]
pub type DbgeR = crate::BitReader<Dbge>;
impl DbgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbge {
        match self.bits {
            false => Dbge::B0,
            true => Dbge::B1,
        }
    }
    #[doc = "FlexIO is disabled in debug modes."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dbge::B0
    }
    #[doc = "FlexIO is enabled in debug modes"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dbge::B1
    }
}
#[doc = "Field `DBGE` writer - Debug Enable"]
pub type DbgeW<'a, REG> = crate::BitWriter<'a, REG, Dbge>;
impl<'a, REG> DbgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FlexIO is disabled in debug modes."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dbge::B0)
    }
    #[doc = "FlexIO is enabled in debug modes"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dbge::B1)
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dozen {
    #[doc = "0: FlexIO enabled in Doze modes."]
    B0 = 0,
    #[doc = "1: FlexIO disabled in Doze modes."]
    B1 = 1,
}
impl From<Dozen> for bool {
    #[inline(always)]
    fn from(variant: Dozen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZEN` reader - Doze Enable"]
pub type DozenR = crate::BitReader<Dozen>;
impl DozenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dozen {
        match self.bits {
            false => Dozen::B0,
            true => Dozen::B1,
        }
    }
    #[doc = "FlexIO enabled in Doze modes."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dozen::B0
    }
    #[doc = "FlexIO disabled in Doze modes."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dozen::B1
    }
}
#[doc = "Field `DOZEN` writer - Doze Enable"]
pub type DozenW<'a, REG> = crate::BitWriter<'a, REG, Dozen>;
impl<'a, REG> DozenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FlexIO enabled in Doze modes."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dozen::B0)
    }
    #[doc = "FlexIO disabled in Doze modes."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dozen::B1)
    }
}
impl R {
    #[doc = "Bit 0 - FlexIO Enable"]
    #[inline(always)]
    pub fn flexen(&self) -> FlexenR {
        FlexenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast Access"]
    #[inline(always)]
    pub fn fastacc(&self) -> FastaccR {
        FastaccR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 30 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&self) -> DbgeR {
        DbgeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Doze Enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DozenR {
        DozenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FlexIO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexen(&mut self) -> FlexenW<CtrlSpec> {
        FlexenW::new(self, 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlSpec> {
        SwrstW::new(self, 1)
    }
    #[doc = "Bit 2 - Fast Access"]
    #[inline(always)]
    #[must_use]
    pub fn fastacc(&mut self) -> FastaccW<CtrlSpec> {
        FastaccW::new(self, 2)
    }
    #[doc = "Bit 30 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbge(&mut self) -> DbgeW<CtrlSpec> {
        DbgeW::new(self, 30)
    }
    #[doc = "Bit 31 - Doze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dozen(&mut self) -> DozenW<CtrlSpec> {
        DozenW::new(self, 31)
    }
}
#[doc = "FlexIO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
