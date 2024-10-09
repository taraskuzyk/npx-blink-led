#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "MCLK Input Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mics {
    #[doc = "0: MCLK divider input clock 0 selected."]
    B00 = 0,
    #[doc = "1: MCLK divider input clock 1 selected."]
    B01 = 1,
    #[doc = "2: MCLK divider input clock 2 selected."]
    B10 = 2,
    #[doc = "3: MCLK divider input clock 3 selected."]
    B11 = 3,
}
impl From<Mics> for u8 {
    #[inline(always)]
    fn from(variant: Mics) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mics {
    type Ux = u8;
}
impl crate::IsEnum for Mics {}
#[doc = "Field `MICS` reader - MCLK Input Clock Select"]
pub type MicsR = crate::FieldReader<Mics>;
impl MicsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mics {
        match self.bits {
            0 => Mics::B00,
            1 => Mics::B01,
            2 => Mics::B10,
            3 => Mics::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "MCLK divider input clock 0 selected."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Mics::B00
    }
    #[doc = "MCLK divider input clock 1 selected."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Mics::B01
    }
    #[doc = "MCLK divider input clock 2 selected."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Mics::B10
    }
    #[doc = "MCLK divider input clock 3 selected."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Mics::B11
    }
}
#[doc = "Field `MICS` writer - MCLK Input Clock Select"]
pub type MicsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mics, crate::Safe>;
impl<'a, REG> MicsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MCLK divider input clock 0 selected."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Mics::B00)
    }
    #[doc = "MCLK divider input clock 1 selected."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Mics::B01)
    }
    #[doc = "MCLK divider input clock 2 selected."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Mics::B10)
    }
    #[doc = "MCLK divider input clock 3 selected."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Mics::B11)
    }
}
#[doc = "MCLK Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Moe {
    #[doc = "0: MCLK signal pin is configured as an input that bypasses the MCLK divider."]
    B0 = 0,
    #[doc = "1: MCLK signal pin is configured as an output from the MCLK divider and the MCLK divider is enabled."]
    B1 = 1,
}
impl From<Moe> for bool {
    #[inline(always)]
    fn from(variant: Moe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOE` reader - MCLK Output Enable"]
pub type MoeR = crate::BitReader<Moe>;
impl MoeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Moe {
        match self.bits {
            false => Moe::B0,
            true => Moe::B1,
        }
    }
    #[doc = "MCLK signal pin is configured as an input that bypasses the MCLK divider."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Moe::B0
    }
    #[doc = "MCLK signal pin is configured as an output from the MCLK divider and the MCLK divider is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Moe::B1
    }
}
#[doc = "Field `MOE` writer - MCLK Output Enable"]
pub type MoeW<'a, REG> = crate::BitWriter<'a, REG, Moe>;
impl<'a, REG> MoeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCLK signal pin is configured as an input that bypasses the MCLK divider."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Moe::B0)
    }
    #[doc = "MCLK signal pin is configured as an output from the MCLK divider and the MCLK divider is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Moe::B1)
    }
}
#[doc = "Divider Update Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duf {
    #[doc = "0: MCLK divider ratio is not being updated currently."]
    B0 = 0,
    #[doc = "1: MCLK divider ratio is updating on-the-fly. Further updates to the MCLK divider ratio are blocked while this flag remains set."]
    B1 = 1,
}
impl From<Duf> for bool {
    #[inline(always)]
    fn from(variant: Duf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUF` reader - Divider Update Flag"]
pub type DufR = crate::BitReader<Duf>;
impl DufR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Duf {
        match self.bits {
            false => Duf::B0,
            true => Duf::B1,
        }
    }
    #[doc = "MCLK divider ratio is not being updated currently."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Duf::B0
    }
    #[doc = "MCLK divider ratio is updating on-the-fly. Further updates to the MCLK divider ratio are blocked while this flag remains set."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Duf::B1
    }
}
impl R {
    #[doc = "Bits 24:25 - MCLK Input Clock Select"]
    #[inline(always)]
    pub fn mics(&self) -> MicsR {
        MicsR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - MCLK Output Enable"]
    #[inline(always)]
    pub fn moe(&self) -> MoeR {
        MoeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Divider Update Flag"]
    #[inline(always)]
    pub fn duf(&self) -> DufR {
        DufR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - MCLK Input Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn mics(&mut self) -> MicsW<McrSpec> {
        MicsW::new(self, 24)
    }
    #[doc = "Bit 30 - MCLK Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MoeW<McrSpec> {
        MoeW::new(self, 30)
    }
}
#[doc = "SAI MCLK Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {
    const RESET_VALUE: u32 = 0;
}
