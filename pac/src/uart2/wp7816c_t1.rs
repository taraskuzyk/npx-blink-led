#[doc = "Register `WP7816C_T1` reader"]
pub type R = crate::R<Wp7816cT1Spec>;
#[doc = "Register `WP7816C_T1` writer"]
pub type W = crate::W<Wp7816cT1Spec>;
#[doc = "Field `CWI2` reader - Character Wait Time Integer 2 (C7816\\[TTYPE\\]
= 1)"]
pub type Cwi2R = crate::FieldReader;
#[doc = "Field `CWI2` writer - Character Wait Time Integer 2 (C7816\\[TTYPE\\]
= 1)"]
pub type Cwi2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Character Wait Time Integer 2 (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    pub fn cwi2(&self) -> Cwi2R {
        Cwi2R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Character Wait Time Integer 2 (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    #[must_use]
    pub fn cwi2(&mut self) -> Cwi2W<Wp7816cT1Spec> {
        Cwi2W::new(self, 0)
    }
}
#[doc = "UART 7816 Wait Parameter Register C\n\nYou can [`read`](crate::Reg::read) this register and get [`wp7816c_t1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp7816c_t1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wp7816cT1Spec;
impl crate::RegisterSpec for Wp7816cT1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wp7816c_t1::R`](R) reader structure"]
impl crate::Readable for Wp7816cT1Spec {}
#[doc = "`write(|w| ..)` method takes [`wp7816c_t1::W`](W) writer structure"]
impl crate::Writable for Wp7816cT1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WP7816C_T1 to value 0x0b"]
impl crate::Resettable for Wp7816cT1Spec {
    const RESET_VALUE: u8 = 0x0b;
}
