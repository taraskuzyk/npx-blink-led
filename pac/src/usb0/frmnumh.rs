#[doc = "Register `FRMNUMH` reader"]
pub type R = crate::R<FrmnumhSpec>;
#[doc = "Register `FRMNUMH` writer"]
pub type W = crate::W<FrmnumhSpec>;
#[doc = "Field `FRM` reader - This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
pub type FrmR = crate::FieldReader;
#[doc = "Field `FRM` writer - This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
pub type FrmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
    #[inline(always)]
    pub fn frm(&self) -> FrmR {
        FrmR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - This 3-bit field and the 8-bit field in the Frame Number Register Low are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
    #[inline(always)]
    #[must_use]
    pub fn frm(&mut self) -> FrmW<FrmnumhSpec> {
        FrmW::new(self, 0)
    }
}
#[doc = "Frame Number register High\n\nYou can [`read`](crate::Reg::read) this register and get [`frmnumh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmnumh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrmnumhSpec;
impl crate::RegisterSpec for FrmnumhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`frmnumh::R`](R) reader structure"]
impl crate::Readable for FrmnumhSpec {}
#[doc = "`write(|w| ..)` method takes [`frmnumh::W`](W) writer structure"]
impl crate::Writable for FrmnumhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FRMNUMH to value 0"]
impl crate::Resettable for FrmnumhSpec {
    const RESET_VALUE: u8 = 0;
}
