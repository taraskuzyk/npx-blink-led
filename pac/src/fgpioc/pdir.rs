#[doc = "Register `PDIR` reader"]
pub type R = crate::R<PdirSpec>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pdi {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    B00000000000000000000000000000001 = 1,
}
impl From<Pdi> for u32 {
    #[inline(always)]
    fn from(variant: Pdi) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pdi {
    type Ux = u32;
}
impl crate::IsEnum for Pdi {}
#[doc = "Field `PDI` reader - Port Data Input"]
pub type PdiR = crate::FieldReader<Pdi>;
impl PdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pdi> {
        match self.bits {
            0 => Some(Pdi::B00000000000000000000000000000000),
            1 => Some(Pdi::B00000000000000000000000000000001),
            _ => None,
        }
    }
    #[doc = "Pin logic level is logic 0, or is not configured for use by digital function."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Pdi::B00000000000000000000000000000000
    }
    #[doc = "Pin logic level is logic 1."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Pdi::B00000000000000000000000000000001
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Input"]
    #[inline(always)]
    pub fn pdi(&self) -> PdiR {
        PdiR::new(self.bits)
    }
}
#[doc = "Port Data Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdirSpec;
impl crate::RegisterSpec for PdirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdir::R`](R) reader structure"]
impl crate::Readable for PdirSpec {}
#[doc = "`reset()` method sets PDIR to value 0"]
impl crate::Resettable for PdirSpec {
    const RESET_VALUE: u32 = 0;
}
