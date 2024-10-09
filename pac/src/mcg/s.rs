#[doc = "Register `S` reader"]
pub type R = crate::R<SSpec>;
#[doc = "OSC Initialization Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscinit0 {
    #[doc = "0: OSC is not ready."]
    B0 = 0,
    #[doc = "1: OSC clock is ready."]
    B1 = 1,
}
impl From<Oscinit0> for bool {
    #[inline(always)]
    fn from(variant: Oscinit0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCINIT0` reader - OSC Initialization Status"]
pub type Oscinit0R = crate::BitReader<Oscinit0>;
impl Oscinit0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscinit0 {
        match self.bits {
            false => Oscinit0::B0,
            true => Oscinit0::B1,
        }
    }
    #[doc = "OSC is not ready."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Oscinit0::B0
    }
    #[doc = "OSC clock is ready."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Oscinit0::B1
    }
}
#[doc = "Clock Mode Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkst {
    #[doc = "0: HIRC clock is selected as the main clock source, and MCG_Lite works at HIRC mode."]
    B00 = 0,
    #[doc = "1: LIRC clock is selected as the main clock source, and MCG_Lite works at LIRC2M or LIRC8M mode."]
    B01 = 1,
    #[doc = "2: External clock is selected as the main clock source, and MCG_Lite works at EXT mode."]
    B10 = 2,
}
impl From<Clkst> for u8 {
    #[inline(always)]
    fn from(variant: Clkst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkst {
    type Ux = u8;
}
impl crate::IsEnum for Clkst {}
#[doc = "Field `CLKST` reader - Clock Mode Status"]
pub type ClkstR = crate::FieldReader<Clkst>;
impl ClkstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkst> {
        match self.bits {
            0 => Some(Clkst::B00),
            1 => Some(Clkst::B01),
            2 => Some(Clkst::B10),
            _ => None,
        }
    }
    #[doc = "HIRC clock is selected as the main clock source, and MCG_Lite works at HIRC mode."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Clkst::B00
    }
    #[doc = "LIRC clock is selected as the main clock source, and MCG_Lite works at LIRC2M or LIRC8M mode."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Clkst::B01
    }
    #[doc = "External clock is selected as the main clock source, and MCG_Lite works at EXT mode."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Clkst::B10
    }
}
impl R {
    #[doc = "Bit 1 - OSC Initialization Status"]
    #[inline(always)]
    pub fn oscinit0(&self) -> Oscinit0R {
        Oscinit0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Clock Mode Status"]
    #[inline(always)]
    pub fn clkst(&self) -> ClkstR {
        ClkstR::new((self.bits >> 2) & 3)
    }
}
#[doc = "MCG Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SSpec;
impl crate::RegisterSpec for SSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s::R`](R) reader structure"]
impl crate::Readable for SSpec {}
#[doc = "`reset()` method sets S to value 0x04"]
impl crate::Resettable for SSpec {
    const RESET_VALUE: u8 = 0x04;
}
