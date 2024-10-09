#[doc = "Register `TABLEMARK` reader"]
pub type R = crate::R<TablemarkSpec>;
#[doc = "Field `MARK` reader - MARK"]
pub type MarkR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - MARK"]
    #[inline(always)]
    pub fn mark(&self) -> MarkR {
        MarkR::new(self.bits)
    }
}
#[doc = "End of Table Marker Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tablemark::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TablemarkSpec;
impl crate::RegisterSpec for TablemarkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tablemark::R`](R) reader structure"]
impl crate::Readable for TablemarkSpec {}
#[doc = "`reset()` method sets TABLEMARK to value 0"]
impl crate::Resettable for TablemarkSpec {
    const RESET_VALUE: u32 = 0;
}
