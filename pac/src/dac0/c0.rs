#[doc = "Register `C0` reader"]
pub type R = crate::R<C0Spec>;
#[doc = "Register `C0` writer"]
pub type W = crate::W<C0Spec>;
#[doc = "DAC Buffer Read Pointer Bottom Flag Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacbbien {
    #[doc = "0: The DAC buffer read pointer bottom flag interrupt is disabled."]
    B0 = 0,
    #[doc = "1: The DAC buffer read pointer bottom flag interrupt is enabled."]
    B1 = 1,
}
impl From<Dacbbien> for bool {
    #[inline(always)]
    fn from(variant: Dacbbien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACBBIEN` reader - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
pub type DacbbienR = crate::BitReader<Dacbbien>;
impl DacbbienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacbbien {
        match self.bits {
            false => Dacbbien::B0,
            true => Dacbbien::B1,
        }
    }
    #[doc = "The DAC buffer read pointer bottom flag interrupt is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dacbbien::B0
    }
    #[doc = "The DAC buffer read pointer bottom flag interrupt is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dacbbien::B1
    }
}
#[doc = "Field `DACBBIEN` writer - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
pub type DacbbienW<'a, REG> = crate::BitWriter<'a, REG, Dacbbien>;
impl<'a, REG> DacbbienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DAC buffer read pointer bottom flag interrupt is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacbbien::B0)
    }
    #[doc = "The DAC buffer read pointer bottom flag interrupt is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacbbien::B1)
    }
}
#[doc = "DAC Buffer Read Pointer Top Flag Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacbtien {
    #[doc = "0: The DAC buffer read pointer top flag interrupt is disabled."]
    B0 = 0,
    #[doc = "1: The DAC buffer read pointer top flag interrupt is enabled."]
    B1 = 1,
}
impl From<Dacbtien> for bool {
    #[inline(always)]
    fn from(variant: Dacbtien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACBTIEN` reader - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
pub type DacbtienR = crate::BitReader<Dacbtien>;
impl DacbtienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacbtien {
        match self.bits {
            false => Dacbtien::B0,
            true => Dacbtien::B1,
        }
    }
    #[doc = "The DAC buffer read pointer top flag interrupt is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dacbtien::B0
    }
    #[doc = "The DAC buffer read pointer top flag interrupt is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dacbtien::B1
    }
}
#[doc = "Field `DACBTIEN` writer - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
pub type DacbtienW<'a, REG> = crate::BitWriter<'a, REG, Dacbtien>;
impl<'a, REG> DacbtienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DAC buffer read pointer top flag interrupt is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacbtien::B0)
    }
    #[doc = "The DAC buffer read pointer top flag interrupt is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacbtien::B1)
    }
}
#[doc = "DAC Low Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpen {
    #[doc = "0: High-Power mode"]
    B0 = 0,
    #[doc = "1: Low-Power mode"]
    B1 = 1,
}
impl From<Lpen> for bool {
    #[inline(always)]
    fn from(variant: Lpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPEN` reader - DAC Low Power Control"]
pub type LpenR = crate::BitReader<Lpen>;
impl LpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpen {
        match self.bits {
            false => Lpen::B0,
            true => Lpen::B1,
        }
    }
    #[doc = "High-Power mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpen::B0
    }
    #[doc = "Low-Power mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpen::B1
    }
}
#[doc = "Field `LPEN` writer - DAC Low Power Control"]
pub type LpenW<'a, REG> = crate::BitWriter<'a, REG, Lpen>;
impl<'a, REG> LpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High-Power mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpen::B0)
    }
    #[doc = "Low-Power mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpen::B1)
    }
}
#[doc = "DAC Software Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacswtrg {
    #[doc = "0: The DAC soft trigger is not valid."]
    B0 = 0,
    #[doc = "1: The DAC soft trigger is valid."]
    B1 = 1,
}
impl From<Dacswtrg> for bool {
    #[inline(always)]
    fn from(variant: Dacswtrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACSWTRG` reader - DAC Software Trigger"]
pub type DacswtrgR = crate::BitReader<Dacswtrg>;
impl DacswtrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacswtrg {
        match self.bits {
            false => Dacswtrg::B0,
            true => Dacswtrg::B1,
        }
    }
    #[doc = "The DAC soft trigger is not valid."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dacswtrg::B0
    }
    #[doc = "The DAC soft trigger is valid."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dacswtrg::B1
    }
}
#[doc = "Field `DACSWTRG` writer - DAC Software Trigger"]
pub type DacswtrgW<'a, REG> = crate::BitWriter<'a, REG, Dacswtrg>;
impl<'a, REG> DacswtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DAC soft trigger is not valid."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacswtrg::B0)
    }
    #[doc = "The DAC soft trigger is valid."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacswtrg::B1)
    }
}
#[doc = "DAC Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dactrgsel {
    #[doc = "0: The DAC hardware trigger is selected."]
    B0 = 0,
    #[doc = "1: The DAC software trigger is selected."]
    B1 = 1,
}
impl From<Dactrgsel> for bool {
    #[inline(always)]
    fn from(variant: Dactrgsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACTRGSEL` reader - DAC Trigger Select"]
pub type DactrgselR = crate::BitReader<Dactrgsel>;
impl DactrgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dactrgsel {
        match self.bits {
            false => Dactrgsel::B0,
            true => Dactrgsel::B1,
        }
    }
    #[doc = "The DAC hardware trigger is selected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dactrgsel::B0
    }
    #[doc = "The DAC software trigger is selected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dactrgsel::B1
    }
}
#[doc = "Field `DACTRGSEL` writer - DAC Trigger Select"]
pub type DactrgselW<'a, REG> = crate::BitWriter<'a, REG, Dactrgsel>;
impl<'a, REG> DactrgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DAC hardware trigger is selected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dactrgsel::B0)
    }
    #[doc = "The DAC software trigger is selected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dactrgsel::B1)
    }
}
#[doc = "DAC Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacrfs {
    #[doc = "0: The DAC selects DACREF_1 as the reference voltage."]
    B0 = 0,
    #[doc = "1: The DAC selects DACREF_2 as the reference voltage."]
    B1 = 1,
}
impl From<Dacrfs> for bool {
    #[inline(always)]
    fn from(variant: Dacrfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACRFS` reader - DAC Reference Select"]
pub type DacrfsR = crate::BitReader<Dacrfs>;
impl DacrfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacrfs {
        match self.bits {
            false => Dacrfs::B0,
            true => Dacrfs::B1,
        }
    }
    #[doc = "The DAC selects DACREF_1 as the reference voltage."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dacrfs::B0
    }
    #[doc = "The DAC selects DACREF_2 as the reference voltage."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dacrfs::B1
    }
}
#[doc = "Field `DACRFS` writer - DAC Reference Select"]
pub type DacrfsW<'a, REG> = crate::BitWriter<'a, REG, Dacrfs>;
impl<'a, REG> DacrfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DAC selects DACREF_1 as the reference voltage."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacrfs::B0)
    }
    #[doc = "The DAC selects DACREF_2 as the reference voltage."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacrfs::B1)
    }
}
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacen {
    #[doc = "0: The DAC system is disabled."]
    B0 = 0,
    #[doc = "1: The DAC system is enabled."]
    B1 = 1,
}
impl From<Dacen> for bool {
    #[inline(always)]
    fn from(variant: Dacen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACEN` reader - DAC Enable"]
pub type DacenR = crate::BitReader<Dacen>;
impl DacenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacen {
        match self.bits {
            false => Dacen::B0,
            true => Dacen::B1,
        }
    }
    #[doc = "The DAC system is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dacen::B0
    }
    #[doc = "The DAC system is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dacen::B1
    }
}
#[doc = "Field `DACEN` writer - DAC Enable"]
pub type DacenW<'a, REG> = crate::BitWriter<'a, REG, Dacen>;
impl<'a, REG> DacenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DAC system is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacen::B0)
    }
    #[doc = "The DAC system is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacen::B1)
    }
}
impl R {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbbien(&self) -> DacbbienR {
        DacbbienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbtien(&self) -> DacbtienR {
        DacbtienR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC Low Power Control"]
    #[inline(always)]
    pub fn lpen(&self) -> LpenR {
        LpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DAC Software Trigger"]
    #[inline(always)]
    pub fn dacswtrg(&self) -> DacswtrgR {
        DacswtrgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC Trigger Select"]
    #[inline(always)]
    pub fn dactrgsel(&self) -> DactrgselR {
        DactrgselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline(always)]
    pub fn dacrfs(&self) -> DacrfsR {
        DacrfsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DacenR {
        DacenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacbbien(&mut self) -> DacbbienW<C0Spec> {
        DacbbienW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacbtien(&mut self) -> DacbtienW<C0Spec> {
        DacbtienW::new(self, 1)
    }
    #[doc = "Bit 3 - DAC Low Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn lpen(&mut self) -> LpenW<C0Spec> {
        LpenW::new(self, 3)
    }
    #[doc = "Bit 4 - DAC Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dacswtrg(&mut self) -> DacswtrgW<C0Spec> {
        DacswtrgW::new(self, 4)
    }
    #[doc = "Bit 5 - DAC Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn dactrgsel(&mut self) -> DactrgselW<C0Spec> {
        DactrgselW::new(self, 5)
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline(always)]
    #[must_use]
    pub fn dacrfs(&mut self) -> DacrfsW<C0Spec> {
        DacrfsW::new(self, 6)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DacenW<C0Spec> {
        DacenW::new(self, 7)
    }
}
#[doc = "DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C0Spec;
impl crate::RegisterSpec for C0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c0::R`](R) reader structure"]
impl crate::Readable for C0Spec {}
#[doc = "`write(|w| ..)` method takes [`c0::W`](W) writer structure"]
impl crate::Writable for C0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C0 to value 0"]
impl crate::Resettable for C0Spec {
    const RESET_VALUE: u8 = 0;
}
