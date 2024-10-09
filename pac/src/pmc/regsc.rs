#[doc = "Register `REGSC` reader"]
pub type R = crate::R<RegscSpec>;
#[doc = "Register `REGSC` writer"]
pub type W = crate::W<RegscSpec>;
#[doc = "Bandgap Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bgbe {
    #[doc = "0: Bandgap buffer not enabled"]
    B0 = 0,
    #[doc = "1: Bandgap buffer enabled"]
    B1 = 1,
}
impl From<Bgbe> for bool {
    #[inline(always)]
    fn from(variant: Bgbe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGBE` reader - Bandgap Buffer Enable"]
pub type BgbeR = crate::BitReader<Bgbe>;
impl BgbeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bgbe {
        match self.bits {
            false => Bgbe::B0,
            true => Bgbe::B1,
        }
    }
    #[doc = "Bandgap buffer not enabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bgbe::B0
    }
    #[doc = "Bandgap buffer enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bgbe::B1
    }
}
#[doc = "Field `BGBE` writer - Bandgap Buffer Enable"]
pub type BgbeW<'a, REG> = crate::BitWriter<'a, REG, Bgbe>;
impl<'a, REG> BgbeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bandgap buffer not enabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bgbe::B0)
    }
    #[doc = "Bandgap buffer enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bgbe::B1)
    }
}
#[doc = "Regulator In Run Regulation Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Regons {
    #[doc = "0: Regulator is in stop regulation or in transition to/from it"]
    B0 = 0,
    #[doc = "1: Regulator is in run regulation"]
    B1 = 1,
}
impl From<Regons> for bool {
    #[inline(always)]
    fn from(variant: Regons) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGONS` reader - Regulator In Run Regulation Status"]
pub type RegonsR = crate::BitReader<Regons>;
impl RegonsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Regons {
        match self.bits {
            false => Regons::B0,
            true => Regons::B1,
        }
    }
    #[doc = "Regulator is in stop regulation or in transition to/from it"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Regons::B0
    }
    #[doc = "Regulator is in run regulation"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Regons::B1
    }
}
#[doc = "Acknowledge Isolation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ackiso {
    #[doc = "0: Peripherals and I/O pads are in normal run state."]
    B0 = 0,
    #[doc = "1: Certain peripherals and I/O pads are in an isolated and latched state."]
    B1 = 1,
}
impl From<Ackiso> for bool {
    #[inline(always)]
    fn from(variant: Ackiso) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKISO` reader - Acknowledge Isolation"]
pub type AckisoR = crate::BitReader<Ackiso>;
impl AckisoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ackiso {
        match self.bits {
            false => Ackiso::B0,
            true => Ackiso::B1,
        }
    }
    #[doc = "Peripherals and I/O pads are in normal run state."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ackiso::B0
    }
    #[doc = "Certain peripherals and I/O pads are in an isolated and latched state."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ackiso::B1
    }
}
#[doc = "Field `ACKISO` writer - Acknowledge Isolation"]
pub type AckisoW<'a, REG> = crate::BitWriter<'a, REG, Ackiso>;
impl<'a, REG> AckisoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripherals and I/O pads are in normal run state."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ackiso::B0)
    }
    #[doc = "Certain peripherals and I/O pads are in an isolated and latched state."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ackiso::B1)
    }
}
#[doc = "Bandgap Enable In VLPx Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bgen {
    #[doc = "0: Bandgap voltage reference is disabled in VLPx , LLS , and VLLSx modes."]
    B0 = 0,
    #[doc = "1: Bandgap voltage reference is enabled in VLPx , LLS , and VLLSx modes."]
    B1 = 1,
}
impl From<Bgen> for bool {
    #[inline(always)]
    fn from(variant: Bgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BGEN` reader - Bandgap Enable In VLPx Operation"]
pub type BgenR = crate::BitReader<Bgen>;
impl BgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bgen {
        match self.bits {
            false => Bgen::B0,
            true => Bgen::B1,
        }
    }
    #[doc = "Bandgap voltage reference is disabled in VLPx , LLS , and VLLSx modes."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bgen::B0
    }
    #[doc = "Bandgap voltage reference is enabled in VLPx , LLS , and VLLSx modes."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bgen::B1
    }
}
#[doc = "Field `BGEN` writer - Bandgap Enable In VLPx Operation"]
pub type BgenW<'a, REG> = crate::BitWriter<'a, REG, Bgen>;
impl<'a, REG> BgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Bandgap voltage reference is disabled in VLPx , LLS , and VLLSx modes."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bgen::B0)
    }
    #[doc = "Bandgap voltage reference is enabled in VLPx , LLS , and VLLSx modes."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bgen::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline(always)]
    pub fn bgbe(&self) -> BgbeR {
        BgbeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Regulator In Run Regulation Status"]
    #[inline(always)]
    pub fn regons(&self) -> RegonsR {
        RegonsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Isolation"]
    #[inline(always)]
    pub fn ackiso(&self) -> AckisoR {
        AckisoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bandgap Enable In VLPx Operation"]
    #[inline(always)]
    pub fn bgen(&self) -> BgenR {
        BgenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bandgap Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgbe(&mut self) -> BgbeW<RegscSpec> {
        BgbeW::new(self, 0)
    }
    #[doc = "Bit 3 - Acknowledge Isolation"]
    #[inline(always)]
    #[must_use]
    pub fn ackiso(&mut self) -> AckisoW<RegscSpec> {
        AckisoW::new(self, 3)
    }
    #[doc = "Bit 4 - Bandgap Enable In VLPx Operation"]
    #[inline(always)]
    #[must_use]
    pub fn bgen(&mut self) -> BgenW<RegscSpec> {
        BgenW::new(self, 4)
    }
}
#[doc = "Regulator Status And Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`regsc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regsc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegscSpec;
impl crate::RegisterSpec for RegscSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`regsc::R`](R) reader structure"]
impl crate::Readable for RegscSpec {}
#[doc = "`write(|w| ..)` method takes [`regsc::W`](W) writer structure"]
impl crate::Writable for RegscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets REGSC to value 0x04"]
impl crate::Resettable for RegscSpec {
    const RESET_VALUE: u8 = 0x04;
}
