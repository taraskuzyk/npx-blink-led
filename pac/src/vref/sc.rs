#[doc = "Register `SC` reader"]
pub type R = crate::R<ScSpec>;
#[doc = "Register `SC` writer"]
pub type W = crate::W<ScSpec>;
#[doc = "Buffer Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ModeLv {
    #[doc = "0: Bandgap on only, for stabilization and startup"]
    B00 = 0,
    #[doc = "1: High power buffer mode enabled"]
    B01 = 1,
    #[doc = "2: Low-power buffer mode enabled"]
    B10 = 2,
}
impl From<ModeLv> for u8 {
    #[inline(always)]
    fn from(variant: ModeLv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ModeLv {
    type Ux = u8;
}
impl crate::IsEnum for ModeLv {}
#[doc = "Field `MODE_LV` reader - Buffer Mode selection"]
pub type ModeLvR = crate::FieldReader<ModeLv>;
impl ModeLvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ModeLv> {
        match self.bits {
            0 => Some(ModeLv::B00),
            1 => Some(ModeLv::B01),
            2 => Some(ModeLv::B10),
            _ => None,
        }
    }
    #[doc = "Bandgap on only, for stabilization and startup"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ModeLv::B00
    }
    #[doc = "High power buffer mode enabled"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ModeLv::B01
    }
    #[doc = "Low-power buffer mode enabled"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ModeLv::B10
    }
}
#[doc = "Field `MODE_LV` writer - Buffer Mode selection"]
pub type ModeLvW<'a, REG> = crate::FieldWriter<'a, REG, 2, ModeLv>;
impl<'a, REG> ModeLvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bandgap on only, for stabilization and startup"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ModeLv::B00)
    }
    #[doc = "High power buffer mode enabled"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ModeLv::B01)
    }
    #[doc = "Low-power buffer mode enabled"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ModeLv::B10)
    }
}
#[doc = "Internal Voltage Reference stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrefst {
    #[doc = "0: The module is disabled or not stable."]
    B0 = 0,
    #[doc = "1: The module is stable."]
    B1 = 1,
}
impl From<Vrefst> for bool {
    #[inline(always)]
    fn from(variant: Vrefst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFST` reader - Internal Voltage Reference stable"]
pub type VrefstR = crate::BitReader<Vrefst>;
impl VrefstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrefst {
        match self.bits {
            false => Vrefst::B0,
            true => Vrefst::B1,
        }
    }
    #[doc = "The module is disabled or not stable."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Vrefst::B0
    }
    #[doc = "The module is stable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Vrefst::B1
    }
}
#[doc = "Second order curvature compensation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icompen {
    #[doc = "0: Disabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Icompen> for bool {
    #[inline(always)]
    fn from(variant: Icompen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICOMPEN` reader - Second order curvature compensation enable"]
pub type IcompenR = crate::BitReader<Icompen>;
impl IcompenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icompen {
        match self.bits {
            false => Icompen::B0,
            true => Icompen::B1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Icompen::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Icompen::B1
    }
}
#[doc = "Field `ICOMPEN` writer - Second order curvature compensation enable"]
pub type IcompenW<'a, REG> = crate::BitWriter<'a, REG, Icompen>;
impl<'a, REG> IcompenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Icompen::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Icompen::B1)
    }
}
#[doc = "Regulator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Regen {
    #[doc = "0: Internal 1.75 V regulator is disabled."]
    B0 = 0,
    #[doc = "1: Internal 1.75 V regulator is enabled."]
    B1 = 1,
}
impl From<Regen> for bool {
    #[inline(always)]
    fn from(variant: Regen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGEN` reader - Regulator enable"]
pub type RegenR = crate::BitReader<Regen>;
impl RegenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Regen {
        match self.bits {
            false => Regen::B0,
            true => Regen::B1,
        }
    }
    #[doc = "Internal 1.75 V regulator is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Regen::B0
    }
    #[doc = "Internal 1.75 V regulator is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Regen::B1
    }
}
#[doc = "Field `REGEN` writer - Regulator enable"]
pub type RegenW<'a, REG> = crate::BitWriter<'a, REG, Regen>;
impl<'a, REG> RegenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal 1.75 V regulator is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Regen::B0)
    }
    #[doc = "Internal 1.75 V regulator is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Regen::B1)
    }
}
#[doc = "Internal Voltage Reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vrefen {
    #[doc = "0: The module is disabled."]
    B0 = 0,
    #[doc = "1: The module is enabled."]
    B1 = 1,
}
impl From<Vrefen> for bool {
    #[inline(always)]
    fn from(variant: Vrefen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREFEN` reader - Internal Voltage Reference enable"]
pub type VrefenR = crate::BitReader<Vrefen>;
impl VrefenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vrefen {
        match self.bits {
            false => Vrefen::B0,
            true => Vrefen::B1,
        }
    }
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Vrefen::B0
    }
    #[doc = "The module is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Vrefen::B1
    }
}
#[doc = "Field `VREFEN` writer - Internal Voltage Reference enable"]
pub type VrefenW<'a, REG> = crate::BitWriter<'a, REG, Vrefen>;
impl<'a, REG> VrefenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefen::B0)
    }
    #[doc = "The module is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Vrefen::B1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Buffer Mode selection"]
    #[inline(always)]
    pub fn mode_lv(&self) -> ModeLvR {
        ModeLvR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Internal Voltage Reference stable"]
    #[inline(always)]
    pub fn vrefst(&self) -> VrefstR {
        VrefstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Second order curvature compensation enable"]
    #[inline(always)]
    pub fn icompen(&self) -> IcompenR {
        IcompenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Regulator enable"]
    #[inline(always)]
    pub fn regen(&self) -> RegenR {
        RegenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Internal Voltage Reference enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VrefenR {
        VrefenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Buffer Mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode_lv(&mut self) -> ModeLvW<ScSpec> {
        ModeLvW::new(self, 0)
    }
    #[doc = "Bit 5 - Second order curvature compensation enable"]
    #[inline(always)]
    #[must_use]
    pub fn icompen(&mut self) -> IcompenW<ScSpec> {
        IcompenW::new(self, 5)
    }
    #[doc = "Bit 6 - Regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn regen(&mut self) -> RegenW<ScSpec> {
        RegenW::new(self, 6)
    }
    #[doc = "Bit 7 - Internal Voltage Reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VrefenW<ScSpec> {
        VrefenW::new(self, 7)
    }
}
#[doc = "VREF Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScSpec;
impl crate::RegisterSpec for ScSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sc::R`](R) reader structure"]
impl crate::Readable for ScSpec {}
#[doc = "`write(|w| ..)` method takes [`sc::W`](W) writer structure"]
impl crate::Writable for ScSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for ScSpec {
    const RESET_VALUE: u8 = 0;
}
