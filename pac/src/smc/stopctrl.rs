#[doc = "Register `STOPCTRL` reader"]
pub type R = crate::R<StopctrlSpec>;
#[doc = "Register `STOPCTRL` writer"]
pub type W = crate::W<StopctrlSpec>;
#[doc = "VLLS Mode Control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vllsm {
    #[doc = "0: VLLS0"]
    B000 = 0,
    #[doc = "1: VLLS1"]
    B001 = 1,
    #[doc = "3: VLLS3"]
    B011 = 3,
}
impl From<Vllsm> for u8 {
    #[inline(always)]
    fn from(variant: Vllsm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vllsm {
    type Ux = u8;
}
impl crate::IsEnum for Vllsm {}
#[doc = "Field `VLLSM` reader - VLLS Mode Control"]
pub type VllsmR = crate::FieldReader<Vllsm>;
impl VllsmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vllsm> {
        match self.bits {
            0 => Some(Vllsm::B000),
            1 => Some(Vllsm::B001),
            3 => Some(Vllsm::B011),
            _ => None,
        }
    }
    #[doc = "VLLS0"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Vllsm::B000
    }
    #[doc = "VLLS1"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Vllsm::B001
    }
    #[doc = "VLLS3"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Vllsm::B011
    }
}
#[doc = "Field `VLLSM` writer - VLLS Mode Control"]
pub type VllsmW<'a, REG> = crate::FieldWriter<'a, REG, 3, Vllsm>;
impl<'a, REG> VllsmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VLLS0"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Vllsm::B000)
    }
    #[doc = "VLLS1"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Vllsm::B001)
    }
    #[doc = "VLLS3"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Vllsm::B011)
    }
}
#[doc = "POR Power Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Porpo {
    #[doc = "0: POR detect circuit is enabled in VLLS0"]
    B0 = 0,
    #[doc = "1: POR detect circuit is disabled in VLLS0"]
    B1 = 1,
}
impl From<Porpo> for bool {
    #[inline(always)]
    fn from(variant: Porpo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORPO` reader - POR Power Option"]
pub type PorpoR = crate::BitReader<Porpo>;
impl PorpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Porpo {
        match self.bits {
            false => Porpo::B0,
            true => Porpo::B1,
        }
    }
    #[doc = "POR detect circuit is enabled in VLLS0"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Porpo::B0
    }
    #[doc = "POR detect circuit is disabled in VLLS0"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Porpo::B1
    }
}
#[doc = "Field `PORPO` writer - POR Power Option"]
pub type PorpoW<'a, REG> = crate::BitWriter<'a, REG, Porpo>;
impl<'a, REG> PorpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "POR detect circuit is enabled in VLLS0"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Porpo::B0)
    }
    #[doc = "POR detect circuit is disabled in VLLS0"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Porpo::B1)
    }
}
#[doc = "Partial Stop Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pstopo {
    #[doc = "0: STOP - Normal Stop mode"]
    B00 = 0,
    #[doc = "1: PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    B01 = 1,
    #[doc = "2: PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    B10 = 2,
}
impl From<Pstopo> for u8 {
    #[inline(always)]
    fn from(variant: Pstopo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pstopo {
    type Ux = u8;
}
impl crate::IsEnum for Pstopo {}
#[doc = "Field `PSTOPO` reader - Partial Stop Option"]
pub type PstopoR = crate::FieldReader<Pstopo>;
impl PstopoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pstopo> {
        match self.bits {
            0 => Some(Pstopo::B00),
            1 => Some(Pstopo::B01),
            2 => Some(Pstopo::B10),
            _ => None,
        }
    }
    #[doc = "STOP - Normal Stop mode"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Pstopo::B00
    }
    #[doc = "PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Pstopo::B01
    }
    #[doc = "PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Pstopo::B10
    }
}
#[doc = "Field `PSTOPO` writer - Partial Stop Option"]
pub type PstopoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pstopo>;
impl<'a, REG> PstopoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "STOP - Normal Stop mode"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Pstopo::B00)
    }
    #[doc = "PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Pstopo::B01)
    }
    #[doc = "PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Pstopo::B10)
    }
}
impl R {
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline(always)]
    pub fn vllsm(&self) -> VllsmR {
        VllsmR::new(self.bits & 7)
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    pub fn porpo(&self) -> PorpoR {
        PorpoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Partial Stop Option"]
    #[inline(always)]
    pub fn pstopo(&self) -> PstopoR {
        PstopoR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn vllsm(&mut self) -> VllsmW<StopctrlSpec> {
        VllsmW::new(self, 0)
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    #[must_use]
    pub fn porpo(&mut self) -> PorpoW<StopctrlSpec> {
        PorpoW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Partial Stop Option"]
    #[inline(always)]
    #[must_use]
    pub fn pstopo(&mut self) -> PstopoW<StopctrlSpec> {
        PstopoW::new(self, 6)
    }
}
#[doc = "Stop Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stopctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stopctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StopctrlSpec;
impl crate::RegisterSpec for StopctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`stopctrl::R`](R) reader structure"]
impl crate::Readable for StopctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`stopctrl::W`](W) writer structure"]
impl crate::Writable for StopctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets STOPCTRL to value 0x03"]
impl crate::Resettable for StopctrlSpec {
    const RESET_VALUE: u8 = 0x03;
}
