#[doc = "Register `CPUID` reader"]
pub type R = crate::R<CpuidSpec>;
#[doc = "Field `REVISION` reader - Minor revision number m in the rnpm revision status"]
pub type RevisionR = crate::FieldReader;
#[doc = "Field `PARTNO` reader - Indicates part number"]
pub type PartnoR = crate::FieldReader<u16>;
#[doc = "Field `ARCHITECTURE` reader - Indicates the architecture"]
pub type ArchitectureR = crate::FieldReader;
#[doc = "Field `VARIANT` reader - Major revision number n in the npm revision status"]
pub type VariantR = crate::FieldReader;
#[doc = "Field `IMPLEMENTER` reader - Implementer code"]
pub type ImplementerR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Minor revision number m in the rnpm revision status"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Indicates part number"]
    #[inline(always)]
    pub fn partno(&self) -> PartnoR {
        PartnoR::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Indicates the architecture"]
    #[inline(always)]
    pub fn architecture(&self) -> ArchitectureR {
        ArchitectureR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Major revision number n in the npm revision status"]
    #[inline(always)]
    pub fn variant(&self) -> VariantR {
        VariantR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:31 - Implementer code"]
    #[inline(always)]
    pub fn implementer(&self) -> ImplementerR {
        ImplementerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "CPUID Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuidSpec;
impl crate::RegisterSpec for CpuidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuid::R`](R) reader structure"]
impl crate::Readable for CpuidSpec {}
#[doc = "`reset()` method sets CPUID to value 0x410c_c601"]
impl crate::Resettable for CpuidSpec {
    const RESET_VALUE: u32 = 0x410c_c601;
}
