#[doc = "Register `FCFG2` reader"]
pub type R = crate::R<Fcfg2Spec>;
#[doc = "Field `MAXADDR1` reader - This field concatenated with leading zeros plus the value of the MAXADDR0 field indicates the first invalid address of the second program flash block (flash block 1)"]
pub type Maxaddr1R = crate::FieldReader;
#[doc = "Field `MAXADDR0` reader - Max Address lock"]
pub type Maxaddr0R = crate::FieldReader;
impl R {
    #[doc = "Bits 16:22 - This field concatenated with leading zeros plus the value of the MAXADDR0 field indicates the first invalid address of the second program flash block (flash block 1)"]
    #[inline(always)]
    pub fn maxaddr1(&self) -> Maxaddr1R {
        Maxaddr1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Max Address lock"]
    #[inline(always)]
    pub fn maxaddr0(&self) -> Maxaddr0R {
        Maxaddr0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "Flash Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fcfg2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcfg2Spec;
impl crate::RegisterSpec for Fcfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg2::R`](R) reader structure"]
impl crate::Readable for Fcfg2Spec {}
#[doc = "`reset()` method sets FCFG2 to value 0x7fff_0000"]
impl crate::Resettable for Fcfg2Spec {
    const RESET_VALUE: u32 = 0x7fff_0000;
}
