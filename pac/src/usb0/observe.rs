#[doc = "Register `OBSERVE` reader"]
pub type R = crate::R<ObserveSpec>;
#[doc = "Provides observability of the D- Pulldown signal output from USB.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmpd {
    #[doc = "0: D- pulldown disabled."]
    B0 = 0,
    #[doc = "1: D- pulldown enabled."]
    B1 = 1,
}
impl From<Dmpd> for bool {
    #[inline(always)]
    fn from(variant: Dmpd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMPD` reader - Provides observability of the D- Pulldown signal output from USB."]
pub type DmpdR = crate::BitReader<Dmpd>;
impl DmpdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmpd {
        match self.bits {
            false => Dmpd::B0,
            true => Dmpd::B1,
        }
    }
    #[doc = "D- pulldown disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dmpd::B0
    }
    #[doc = "D- pulldown enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dmpd::B1
    }
}
#[doc = "Provides observability of the D+ Pulldown signal output from USB.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dppd {
    #[doc = "0: D+ pulldown disabled."]
    B0 = 0,
    #[doc = "1: D+ pulldown enabled."]
    B1 = 1,
}
impl From<Dppd> for bool {
    #[inline(always)]
    fn from(variant: Dppd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPPD` reader - Provides observability of the D+ Pulldown signal output from USB."]
pub type DppdR = crate::BitReader<Dppd>;
impl DppdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dppd {
        match self.bits {
            false => Dppd::B0,
            true => Dppd::B1,
        }
    }
    #[doc = "D+ pulldown disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dppd::B0
    }
    #[doc = "D+ pulldown enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dppd::B1
    }
}
#[doc = "Provides observability of the D+ Pullup signal output from USB .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dppu {
    #[doc = "0: D+ pullup disabled."]
    B0 = 0,
    #[doc = "1: D+ pullup enabled."]
    B1 = 1,
}
impl From<Dppu> for bool {
    #[inline(always)]
    fn from(variant: Dppu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPPU` reader - Provides observability of the D+ Pullup signal output from USB ."]
pub type DppuR = crate::BitReader<Dppu>;
impl DppuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dppu {
        match self.bits {
            false => Dppu::B0,
            true => Dppu::B1,
        }
    }
    #[doc = "D+ pullup disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dppu::B0
    }
    #[doc = "D+ pullup enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dppu::B1
    }
}
impl R {
    #[doc = "Bit 4 - Provides observability of the D- Pulldown signal output from USB."]
    #[inline(always)]
    pub fn dmpd(&self) -> DmpdR {
        DmpdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Provides observability of the D+ Pulldown signal output from USB."]
    #[inline(always)]
    pub fn dppd(&self) -> DppdR {
        DppdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Provides observability of the D+ Pullup signal output from USB ."]
    #[inline(always)]
    pub fn dppu(&self) -> DppuR {
        DppuR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "USB OTG Observe register\n\nYou can [`read`](crate::Reg::read) this register and get [`observe::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ObserveSpec;
impl crate::RegisterSpec for ObserveSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`observe::R`](R) reader structure"]
impl crate::Readable for ObserveSpec {}
#[doc = "`reset()` method sets OBSERVE to value 0x50"]
impl crate::Resettable for ObserveSpec {
    const RESET_VALUE: u8 = 0x50;
}
