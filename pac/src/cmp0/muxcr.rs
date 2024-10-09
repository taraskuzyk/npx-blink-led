#[doc = "Register `MUXCR` reader"]
pub type R = crate::R<MuxcrSpec>;
#[doc = "Register `MUXCR` writer"]
pub type W = crate::W<MuxcrSpec>;
#[doc = "Minus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Msel {
    #[doc = "0: IN0"]
    B000 = 0,
    #[doc = "1: IN1"]
    B001 = 1,
    #[doc = "2: IN2"]
    B010 = 2,
    #[doc = "3: IN3"]
    B011 = 3,
    #[doc = "4: IN4"]
    B100 = 4,
    #[doc = "5: IN5"]
    B101 = 5,
    #[doc = "6: IN6"]
    B110 = 6,
    #[doc = "7: IN7"]
    B111 = 7,
}
impl From<Msel> for u8 {
    #[inline(always)]
    fn from(variant: Msel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Msel {
    type Ux = u8;
}
impl crate::IsEnum for Msel {}
#[doc = "Field `MSEL` reader - Minus Input Mux Control"]
pub type MselR = crate::FieldReader<Msel>;
impl MselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msel {
        match self.bits {
            0 => Msel::B000,
            1 => Msel::B001,
            2 => Msel::B010,
            3 => Msel::B011,
            4 => Msel::B100,
            5 => Msel::B101,
            6 => Msel::B110,
            7 => Msel::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Msel::B000
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Msel::B001
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Msel::B010
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Msel::B011
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Msel::B100
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Msel::B101
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Msel::B110
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Msel::B111
    }
}
#[doc = "Field `MSEL` writer - Minus Input Mux Control"]
pub type MselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Msel, crate::Safe>;
impl<'a, REG> MselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IN0"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B000)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B001)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B010)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B011)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B100)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B101)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B110)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Msel::B111)
    }
}
#[doc = "Plus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Psel {
    #[doc = "0: IN0"]
    B000 = 0,
    #[doc = "1: IN1"]
    B001 = 1,
    #[doc = "2: IN2"]
    B010 = 2,
    #[doc = "3: IN3"]
    B011 = 3,
    #[doc = "4: IN4"]
    B100 = 4,
    #[doc = "5: IN5"]
    B101 = 5,
    #[doc = "6: IN6"]
    B110 = 6,
    #[doc = "7: IN7"]
    B111 = 7,
}
impl From<Psel> for u8 {
    #[inline(always)]
    fn from(variant: Psel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Psel {
    type Ux = u8;
}
impl crate::IsEnum for Psel {}
#[doc = "Field `PSEL` reader - Plus Input Mux Control"]
pub type PselR = crate::FieldReader<Psel>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Psel {
        match self.bits {
            0 => Psel::B000,
            1 => Psel::B001,
            2 => Psel::B010,
            3 => Psel::B011,
            4 => Psel::B100,
            5 => Psel::B101,
            6 => Psel::B110,
            7 => Psel::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "IN0"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Psel::B000
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Psel::B001
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Psel::B010
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Psel::B011
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Psel::B100
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Psel::B101
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Psel::B110
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Psel::B111
    }
}
#[doc = "Field `PSEL` writer - Plus Input Mux Control"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Psel, crate::Safe>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "IN0"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::B000)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::B001)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::B010)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::B011)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::B100)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::B101)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::B110)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Psel::B111)
    }
}
#[doc = "Pass Through Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pstm {
    #[doc = "0: Pass Through Mode is disabled."]
    B0 = 0,
    #[doc = "1: Pass Through Mode is enabled."]
    B1 = 1,
}
impl From<Pstm> for bool {
    #[inline(always)]
    fn from(variant: Pstm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSTM` reader - Pass Through Mode Enable"]
pub type PstmR = crate::BitReader<Pstm>;
impl PstmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pstm {
        match self.bits {
            false => Pstm::B0,
            true => Pstm::B1,
        }
    }
    #[doc = "Pass Through Mode is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pstm::B0
    }
    #[doc = "Pass Through Mode is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pstm::B1
    }
}
#[doc = "Field `PSTM` writer - Pass Through Mode Enable"]
pub type PstmW<'a, REG> = crate::BitWriter<'a, REG, Pstm>;
impl<'a, REG> PstmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pass Through Mode is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pstm::B0)
    }
    #[doc = "Pass Through Mode is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pstm::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    pub fn msel(&self) -> MselR {
        MselR::new(self.bits & 7)
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new((self.bits >> 3) & 7)
    }
    #[doc = "Bit 7 - Pass Through Mode Enable"]
    #[inline(always)]
    pub fn pstm(&self) -> PstmR {
        PstmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    #[must_use]
    pub fn msel(&mut self) -> MselW<MuxcrSpec> {
        MselW::new(self, 0)
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<MuxcrSpec> {
        PselW::new(self, 3)
    }
    #[doc = "Bit 7 - Pass Through Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pstm(&mut self) -> PstmW<MuxcrSpec> {
        PstmW::new(self, 7)
    }
}
#[doc = "MUX Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MuxcrSpec;
impl crate::RegisterSpec for MuxcrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`muxcr::R`](R) reader structure"]
impl crate::Readable for MuxcrSpec {}
#[doc = "`write(|w| ..)` method takes [`muxcr::W`](W) writer structure"]
impl crate::Writable for MuxcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MUXCR to value 0"]
impl crate::Resettable for MuxcrSpec {
    const RESET_VALUE: u8 = 0;
}
