#[doc = "Register `FOPT` reader"]
pub type R = crate::R<FoptSpec>;
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpboot0 {
    #[doc = "0: Core and system clock divider (OUTDIV1) is 0x7 (divide by 8) when LPBOOT1=0 or 0x1 (divide by 2) when LPBOOT1=1."]
    B0 = 0,
    #[doc = "1: Core and system clock divider (OUTDIV1) is 0x3 (divide by 4) when LPBOOT1=0 or 0x0 (divide by 1) when LPBOOT1=1."]
    B1 = 1,
}
impl From<Lpboot0> for bool {
    #[inline(always)]
    fn from(variant: Lpboot0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPBOOT0` reader - no description available"]
pub type Lpboot0R = crate::BitReader<Lpboot0>;
impl Lpboot0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpboot0 {
        match self.bits {
            false => Lpboot0::B0,
            true => Lpboot0::B1,
        }
    }
    #[doc = "Core and system clock divider (OUTDIV1) is 0x7 (divide by 8) when LPBOOT1=0 or 0x1 (divide by 2) when LPBOOT1=1."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpboot0::B0
    }
    #[doc = "Core and system clock divider (OUTDIV1) is 0x3 (divide by 4) when LPBOOT1=0 or 0x0 (divide by 1) when LPBOOT1=1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpboot0::B1
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootpinOpt {
    #[doc = "0: Force Boot from ROM if BOOTCFG0 asserted, where BOOTCFG0 is the boot config function which is muxed with NMI pin"]
    B0 = 0,
    #[doc = "1: Boot source configured by FOPT (BOOTSRC_SEL) bits"]
    B1 = 1,
}
impl From<BootpinOpt> for bool {
    #[inline(always)]
    fn from(variant: BootpinOpt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BOOTPIN_OPT` reader - no description available"]
pub type BootpinOptR = crate::BitReader<BootpinOpt>;
impl BootpinOptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BootpinOpt {
        match self.bits {
            false => BootpinOpt::B0,
            true => BootpinOpt::B1,
        }
    }
    #[doc = "Force Boot from ROM if BOOTCFG0 asserted, where BOOTCFG0 is the boot config function which is muxed with NMI pin"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == BootpinOpt::B0
    }
    #[doc = "Boot source configured by FOPT (BOOTSRC_SEL) bits"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BootpinOpt::B1
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiDis {
    #[doc = "0: NMI interrupts are always blocked"]
    B0 = 0,
    #[doc = "1: NMI_b pin/interrupts reset default to enabled"]
    B1 = 1,
}
impl From<NmiDis> for bool {
    #[inline(always)]
    fn from(variant: NmiDis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMI_DIS` reader - no description available"]
pub type NmiDisR = crate::BitReader<NmiDis>;
impl NmiDisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NmiDis {
        match self.bits {
            false => NmiDis::B0,
            true => NmiDis::B1,
        }
    }
    #[doc = "NMI interrupts are always blocked"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == NmiDis::B0
    }
    #[doc = "NMI_b pin/interrupts reset default to enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == NmiDis::B1
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetPinCfg {
    #[doc = "0: RESET pin is disabled following a POR and cannot be enabled as reset function"]
    B0 = 0,
    #[doc = "1: RESET_b pin is dedicated"]
    B1 = 1,
}
impl From<ResetPinCfg> for bool {
    #[inline(always)]
    fn from(variant: ResetPinCfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET_PIN_CFG` reader - no description available"]
pub type ResetPinCfgR = crate::BitReader<ResetPinCfg>;
impl ResetPinCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResetPinCfg {
        match self.bits {
            false => ResetPinCfg::B0,
            true => ResetPinCfg::B1,
        }
    }
    #[doc = "RESET pin is disabled following a POR and cannot be enabled as reset function"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ResetPinCfg::B0
    }
    #[doc = "RESET_b pin is dedicated"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ResetPinCfg::B1
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lpboot1 {
    #[doc = "0: Core and system clock divider (OUTDIV1) is 0x7 (divide by 8) when LPBOOT0=0 or 0x3 (divide by 4) when LPBOOT0=1."]
    B0 = 0,
    #[doc = "1: Core and system clock divider (OUTDIV1) is 0x1 (divide by 2) when LPBOOT0=0 or 0x0 (divide by 1) when LPBOOT0=1."]
    B1 = 1,
}
impl From<Lpboot1> for bool {
    #[inline(always)]
    fn from(variant: Lpboot1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPBOOT1` reader - no description available"]
pub type Lpboot1R = crate::BitReader<Lpboot1>;
impl Lpboot1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpboot1 {
        match self.bits {
            false => Lpboot1::B0,
            true => Lpboot1::B1,
        }
    }
    #[doc = "Core and system clock divider (OUTDIV1) is 0x7 (divide by 8) when LPBOOT0=0 or 0x3 (divide by 4) when LPBOOT0=1."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lpboot1::B0
    }
    #[doc = "Core and system clock divider (OUTDIV1) is 0x1 (divide by 2) when LPBOOT0=0 or 0x0 (divide by 1) when LPBOOT0=1."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lpboot1::B1
    }
}
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FastInit {
    #[doc = "0: Slower initialization"]
    B0 = 0,
    #[doc = "1: Fast Initialization"]
    B1 = 1,
}
impl From<FastInit> for bool {
    #[inline(always)]
    fn from(variant: FastInit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FAST_INIT` reader - no description available"]
pub type FastInitR = crate::BitReader<FastInit>;
impl FastInitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FastInit {
        match self.bits {
            false => FastInit::B0,
            true => FastInit::B1,
        }
    }
    #[doc = "Slower initialization"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FastInit::B0
    }
    #[doc = "Fast Initialization"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FastInit::B1
    }
}
#[doc = "Boot source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BootsrcSel {
    #[doc = "0: Boot from Flash"]
    B00 = 0,
    #[doc = "2: Boot from ROM"]
    B10 = 2,
    #[doc = "3: Boot from ROM"]
    B11 = 3,
}
impl From<BootsrcSel> for u8 {
    #[inline(always)]
    fn from(variant: BootsrcSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BootsrcSel {
    type Ux = u8;
}
impl crate::IsEnum for BootsrcSel {}
#[doc = "Field `BOOTSRC_SEL` reader - Boot source selection"]
pub type BootsrcSelR = crate::FieldReader<BootsrcSel>;
impl BootsrcSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BootsrcSel> {
        match self.bits {
            0 => Some(BootsrcSel::B00),
            2 => Some(BootsrcSel::B10),
            3 => Some(BootsrcSel::B11),
            _ => None,
        }
    }
    #[doc = "Boot from Flash"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == BootsrcSel::B00
    }
    #[doc = "Boot from ROM"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == BootsrcSel::B10
    }
    #[doc = "Boot from ROM"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == BootsrcSel::B11
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn lpboot0(&self) -> Lpboot0R {
        Lpboot0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn bootpin_opt(&self) -> BootpinOptR {
        BootpinOptR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    pub fn nmi_dis(&self) -> NmiDisR {
        NmiDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - no description available"]
    #[inline(always)]
    pub fn reset_pin_cfg(&self) -> ResetPinCfgR {
        ResetPinCfgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    pub fn lpboot1(&self) -> Lpboot1R {
        Lpboot1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - no description available"]
    #[inline(always)]
    pub fn fast_init(&self) -> FastInitR {
        FastInitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Boot source selection"]
    #[inline(always)]
    pub fn bootsrc_sel(&self) -> BootsrcSelR {
        BootsrcSelR::new((self.bits >> 6) & 3)
    }
}
#[doc = "Non-volatile Flash Option Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fopt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FoptSpec;
impl crate::RegisterSpec for FoptSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fopt::R`](R) reader structure"]
impl crate::Readable for FoptSpec {}
#[doc = "`reset()` method sets FOPT to value 0x3f"]
impl crate::Resettable for FoptSpec {
    const RESET_VALUE: u8 = 0x3f;
}
