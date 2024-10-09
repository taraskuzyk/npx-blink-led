#[doc = "Register `PLAMC` reader"]
pub type R = crate::R<PlamcSpec>;
#[doc = "Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port.\n\nValue on reset: 13"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Amc {
    #[doc = "0: A bus master connection to AXBS input port n is absent"]
    B00000000 = 0,
    #[doc = "1: A bus master connection to AXBS input port n is present"]
    B00000001 = 1,
}
impl From<Amc> for u8 {
    #[inline(always)]
    fn from(variant: Amc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Amc {
    type Ux = u8;
}
impl crate::IsEnum for Amc {}
#[doc = "Field `AMC` reader - Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port."]
pub type AmcR = crate::FieldReader<Amc>;
impl AmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Amc> {
        match self.bits {
            0 => Some(Amc::B00000000),
            1 => Some(Amc::B00000001),
            _ => None,
        }
    }
    #[doc = "A bus master connection to AXBS input port n is absent"]
    #[inline(always)]
    pub fn is_b00000000(&self) -> bool {
        *self == Amc::B00000000
    }
    #[doc = "A bus master connection to AXBS input port n is present"]
    #[inline(always)]
    pub fn is_b00000001(&self) -> bool {
        *self == Amc::B00000001
    }
}
impl R {
    #[doc = "Bits 0:7 - Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port."]
    #[inline(always)]
    pub fn amc(&self) -> AmcR {
        AmcR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Crossbar Switch (AXBS) Master Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`plamc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlamcSpec;
impl crate::RegisterSpec for PlamcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`plamc::R`](R) reader structure"]
impl crate::Readable for PlamcSpec {}
#[doc = "`reset()` method sets PLAMC to value 0x0d"]
impl crate::Resettable for PlamcSpec {
    const RESET_VALUE: u16 = 0x0d;
}
