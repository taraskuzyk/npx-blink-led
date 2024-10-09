#[doc = "Register `PG` reader"]
pub type R = crate::R<PgSpec>;
#[doc = "Register `PG` writer"]
pub type W = crate::W<PgSpec>;
#[doc = "Field `PG` reader - Plus-Side Gain"]
pub type PgR = crate::FieldReader<u16>;
#[doc = "Field `PG` writer - Plus-Side Gain"]
pub type PgW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Plus-Side Gain"]
    #[inline(always)]
    pub fn pg(&self) -> PgR {
        PgR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Plus-Side Gain"]
    #[inline(always)]
    #[must_use]
    pub fn pg(&mut self) -> PgW<PgSpec> {
        PgW::new(self, 0)
    }
}
#[doc = "ADC Plus-Side Gain Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PgSpec;
impl crate::RegisterSpec for PgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pg::R`](R) reader structure"]
impl crate::Readable for PgSpec {}
#[doc = "`write(|w| ..)` method takes [`pg::W`](W) writer structure"]
impl crate::Writable for PgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PG to value 0x8200"]
impl crate::Resettable for PgSpec {
    const RESET_VALUE: u32 = 0x8200;
}
