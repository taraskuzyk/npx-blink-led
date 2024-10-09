#[doc = "Register `PSR` reader"]
pub type R = crate::R<PsrSpec>;
#[doc = "Register `PSR` writer"]
pub type W = crate::W<PsrSpec>;
#[doc = "Prescaler Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pcs {
    #[doc = "0: Prescaler/glitch filter clock 0 selected."]
    B00 = 0,
    #[doc = "1: Prescaler/glitch filter clock 1 selected."]
    B01 = 1,
    #[doc = "2: Prescaler/glitch filter clock 2 selected."]
    B10 = 2,
    #[doc = "3: Prescaler/glitch filter clock 3 selected."]
    B11 = 3,
}
impl From<Pcs> for u8 {
    #[inline(always)]
    fn from(variant: Pcs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pcs {
    type Ux = u8;
}
impl crate::IsEnum for Pcs {}
#[doc = "Field `PCS` reader - Prescaler Clock Select"]
pub type PcsR = crate::FieldReader<Pcs>;
impl PcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcs {
        match self.bits {
            0 => Pcs::B00,
            1 => Pcs::B01,
            2 => Pcs::B10,
            3 => Pcs::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler/glitch filter clock 0 selected."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Pcs::B00
    }
    #[doc = "Prescaler/glitch filter clock 1 selected."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Pcs::B01
    }
    #[doc = "Prescaler/glitch filter clock 2 selected."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Pcs::B10
    }
    #[doc = "Prescaler/glitch filter clock 3 selected."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Pcs::B11
    }
}
#[doc = "Field `PCS` writer - Prescaler Clock Select"]
pub type PcsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Pcs, crate::Safe>;
impl<'a, REG> PcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler/glitch filter clock 0 selected."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Pcs::B00)
    }
    #[doc = "Prescaler/glitch filter clock 1 selected."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Pcs::B01)
    }
    #[doc = "Prescaler/glitch filter clock 2 selected."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Pcs::B10)
    }
    #[doc = "Prescaler/glitch filter clock 3 selected."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Pcs::B11)
    }
}
#[doc = "Prescaler Bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pbyp {
    #[doc = "0: Prescaler/glitch filter is enabled."]
    B0 = 0,
    #[doc = "1: Prescaler/glitch filter is bypassed."]
    B1 = 1,
}
impl From<Pbyp> for bool {
    #[inline(always)]
    fn from(variant: Pbyp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PBYP` reader - Prescaler Bypass"]
pub type PbypR = crate::BitReader<Pbyp>;
impl PbypR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pbyp {
        match self.bits {
            false => Pbyp::B0,
            true => Pbyp::B1,
        }
    }
    #[doc = "Prescaler/glitch filter is enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pbyp::B0
    }
    #[doc = "Prescaler/glitch filter is bypassed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pbyp::B1
    }
}
#[doc = "Field `PBYP` writer - Prescaler Bypass"]
pub type PbypW<'a, REG> = crate::BitWriter<'a, REG, Pbyp>;
impl<'a, REG> PbypW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prescaler/glitch filter is enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pbyp::B0)
    }
    #[doc = "Prescaler/glitch filter is bypassed."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pbyp::B1)
    }
}
#[doc = "Prescale Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescale {
    #[doc = "0: Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
    B0000 = 0,
    #[doc = "1: Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
    B0001 = 1,
    #[doc = "2: Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
    B0010 = 2,
    #[doc = "3: Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
    B0011 = 3,
    #[doc = "4: Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
    B0100 = 4,
    #[doc = "5: Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
    B0101 = 5,
    #[doc = "6: Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
    B0110 = 6,
    #[doc = "7: Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
    B0111 = 7,
    #[doc = "8: Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
    B1000 = 8,
    #[doc = "9: Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
    B1001 = 9,
    #[doc = "10: Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
    B1010 = 10,
    #[doc = "11: Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
    B1011 = 11,
    #[doc = "12: Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
    B1100 = 12,
    #[doc = "13: Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
    B1101 = 13,
    #[doc = "14: Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
    B1110 = 14,
    #[doc = "15: Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
    B1111 = 15,
}
impl From<Prescale> for u8 {
    #[inline(always)]
    fn from(variant: Prescale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescale {
    type Ux = u8;
}
impl crate::IsEnum for Prescale {}
#[doc = "Field `PRESCALE` reader - Prescale Value"]
pub type PrescaleR = crate::FieldReader<Prescale>;
impl PrescaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prescale {
        match self.bits {
            0 => Prescale::B0000,
            1 => Prescale::B0001,
            2 => Prescale::B0010,
            3 => Prescale::B0011,
            4 => Prescale::B0100,
            5 => Prescale::B0101,
            6 => Prescale::B0110,
            7 => Prescale::B0111,
            8 => Prescale::B1000,
            9 => Prescale::B1001,
            10 => Prescale::B1010,
            11 => Prescale::B1011,
            12 => Prescale::B1100,
            13 => Prescale::B1101,
            14 => Prescale::B1110,
            15 => Prescale::B1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Prescale::B0000
    }
    #[doc = "Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Prescale::B0001
    }
    #[doc = "Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Prescale::B0010
    }
    #[doc = "Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Prescale::B0011
    }
    #[doc = "Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Prescale::B0100
    }
    #[doc = "Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Prescale::B0101
    }
    #[doc = "Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Prescale::B0110
    }
    #[doc = "Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == Prescale::B0111
    }
    #[doc = "Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Prescale::B1000
    }
    #[doc = "Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Prescale::B1001
    }
    #[doc = "Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == Prescale::B1010
    }
    #[doc = "Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == Prescale::B1011
    }
    #[doc = "Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == Prescale::B1100
    }
    #[doc = "Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == Prescale::B1101
    }
    #[doc = "Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == Prescale::B1110
    }
    #[doc = "Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == Prescale::B1111
    }
}
#[doc = "Field `PRESCALE` writer - Prescale Value"]
pub type PrescaleW<'a, REG> = crate::FieldWriter<'a, REG, 4, Prescale, crate::Safe>;
impl<'a, REG> PrescaleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Prescaler divides the prescaler clock by 2; glitch filter does not support this configuration."]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B0000)
    }
    #[doc = "Prescaler divides the prescaler clock by 4; glitch filter recognizes change on input pin after 2 rising clock edges."]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B0001)
    }
    #[doc = "Prescaler divides the prescaler clock by 8; glitch filter recognizes change on input pin after 4 rising clock edges."]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B0010)
    }
    #[doc = "Prescaler divides the prescaler clock by 16; glitch filter recognizes change on input pin after 8 rising clock edges."]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B0011)
    }
    #[doc = "Prescaler divides the prescaler clock by 32; glitch filter recognizes change on input pin after 16 rising clock edges."]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B0100)
    }
    #[doc = "Prescaler divides the prescaler clock by 64; glitch filter recognizes change on input pin after 32 rising clock edges."]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B0101)
    }
    #[doc = "Prescaler divides the prescaler clock by 128; glitch filter recognizes change on input pin after 64 rising clock edges."]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B0110)
    }
    #[doc = "Prescaler divides the prescaler clock by 256; glitch filter recognizes change on input pin after 128 rising clock edges."]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B0111)
    }
    #[doc = "Prescaler divides the prescaler clock by 512; glitch filter recognizes change on input pin after 256 rising clock edges."]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B1000)
    }
    #[doc = "Prescaler divides the prescaler clock by 1024; glitch filter recognizes change on input pin after 512 rising clock edges."]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B1001)
    }
    #[doc = "Prescaler divides the prescaler clock by 2048; glitch filter recognizes change on input pin after 1024 rising clock edges."]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B1010)
    }
    #[doc = "Prescaler divides the prescaler clock by 4096; glitch filter recognizes change on input pin after 2048 rising clock edges."]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B1011)
    }
    #[doc = "Prescaler divides the prescaler clock by 8192; glitch filter recognizes change on input pin after 4096 rising clock edges."]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B1100)
    }
    #[doc = "Prescaler divides the prescaler clock by 16,384; glitch filter recognizes change on input pin after 8192 rising clock edges."]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B1101)
    }
    #[doc = "Prescaler divides the prescaler clock by 32,768; glitch filter recognizes change on input pin after 16,384 rising clock edges."]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B1110)
    }
    #[doc = "Prescaler divides the prescaler clock by 65,536; glitch filter recognizes change on input pin after 32,768 rising clock edges."]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(Prescale::B1111)
    }
}
impl R {
    #[doc = "Bits 0:1 - Prescaler Clock Select"]
    #[inline(always)]
    pub fn pcs(&self) -> PcsR {
        PcsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Prescaler Bypass"]
    #[inline(always)]
    pub fn pbyp(&self) -> PbypR {
        PbypR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - Prescale Value"]
    #[inline(always)]
    pub fn prescale(&self) -> PrescaleR {
        PrescaleR::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Prescaler Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PcsW<PsrSpec> {
        PcsW::new(self, 0)
    }
    #[doc = "Bit 2 - Prescaler Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn pbyp(&mut self) -> PbypW<PsrSpec> {
        PbypW::new(self, 2)
    }
    #[doc = "Bits 3:6 - Prescale Value"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PrescaleW<PsrSpec> {
        PrescaleW::new(self, 3)
    }
}
#[doc = "Low Power Timer Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsrSpec;
impl crate::RegisterSpec for PsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psr::R`](R) reader structure"]
impl crate::Readable for PsrSpec {}
#[doc = "`write(|w| ..)` method takes [`psr::W`](W) writer structure"]
impl crate::Writable for PsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PsrSpec {
    const RESET_VALUE: u32 = 0;
}
