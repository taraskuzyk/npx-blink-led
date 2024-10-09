#[doc = "Register `PLASC` reader"]
pub type R = crate::R<PlascSpec>;
#[doc = "Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Asc {
    #[doc = "0: A bus slave connection to AXBS input port n is absent."]
    B00000000 = 0,
    #[doc = "1: A bus slave connection to AXBS input port n is present."]
    B00000001 = 1,
}
impl From<Asc> for u8 {
    #[inline(always)]
    fn from(variant: Asc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Asc {
    type Ux = u8;
}
impl crate::IsEnum for Asc {}
#[doc = "Field `ASC` reader - Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port."]
pub type AscR = crate::FieldReader<Asc>;
impl AscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Asc> {
        match self.bits {
            0 => Some(Asc::B00000000),
            1 => Some(Asc::B00000001),
            _ => None,
        }
    }
    #[doc = "A bus slave connection to AXBS input port n is absent."]
    #[inline(always)]
    pub fn is_b00000000(&self) -> bool {
        *self == Asc::B00000000
    }
    #[doc = "A bus slave connection to AXBS input port n is present."]
    #[inline(always)]
    pub fn is_b00000001(&self) -> bool {
        *self == Asc::B00000001
    }
}
impl R {
    #[doc = "Bits 0:7 - Each bit in the ASC field indicates whether there is a corresponding connection to the crossbar switch's slave input port."]
    #[inline(always)]
    pub fn asc(&self) -> AscR {
        AscR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Crossbar Switch (AXBS) Slave Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`plasc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlascSpec;
impl crate::RegisterSpec for PlascSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`plasc::R`](R) reader structure"]
impl crate::Readable for PlascSpec {}
#[doc = "`reset()` method sets PLASC to value 0x07"]
impl crate::Resettable for PlascSpec {
    const RESET_VALUE: u16 = 0x07;
}
