#[doc = "Register `WGP7816_T1` reader"]
pub type R = crate::R<Wgp7816T1Spec>;
#[doc = "Register `WGP7816_T1` writer"]
pub type W = crate::W<Wgp7816T1Spec>;
#[doc = "Field `BGI` reader - Block Guard Time Integer (C7816\\[TTYPE\\]
= 1)"]
pub type BgiR = crate::FieldReader;
#[doc = "Field `BGI` writer - Block Guard Time Integer (C7816\\[TTYPE\\]
= 1)"]
pub type BgiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CWI1` reader - Character Wait Time Integer 1 (C7816\\[TTYPE\\]
= 1)"]
pub type Cwi1R = crate::FieldReader;
#[doc = "Field `CWI1` writer - Character Wait Time Integer 1 (C7816\\[TTYPE\\]
= 1)"]
pub type Cwi1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Block Guard Time Integer (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    pub fn bgi(&self) -> BgiR {
        BgiR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Character Wait Time Integer 1 (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    pub fn cwi1(&self) -> Cwi1R {
        Cwi1R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Block Guard Time Integer (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    #[must_use]
    pub fn bgi(&mut self) -> BgiW<Wgp7816T1Spec> {
        BgiW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Character Wait Time Integer 1 (C7816\\[TTYPE\\]
= 1)"]
    #[inline(always)]
    #[must_use]
    pub fn cwi1(&mut self) -> Cwi1W<Wgp7816T1Spec> {
        Cwi1W::new(self, 4)
    }
}
#[doc = "UART 7816 Wait and Guard Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wgp7816_t1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wgp7816_t1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wgp7816T1Spec;
impl crate::RegisterSpec for Wgp7816T1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wgp7816_t1::R`](R) reader structure"]
impl crate::Readable for Wgp7816T1Spec {}
#[doc = "`write(|w| ..)` method takes [`wgp7816_t1::W`](W) writer structure"]
impl crate::Writable for Wgp7816T1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WGP7816_T1 to value 0x06"]
impl crate::Resettable for Wgp7816T1Spec {
    const RESET_VALUE: u8 = 0x06;
}
