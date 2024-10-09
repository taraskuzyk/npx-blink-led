#[doc = "Register `SHIFTBUFBBS%s` reader"]
pub type R = crate::R<ShiftbufbbsSpec>;
#[doc = "Register `SHIFTBUFBBS%s` writer"]
pub type W = crate::W<ShiftbufbbsSpec>;
#[doc = "Field `SHIFTBUFBBS` reader - Shift Buffer"]
pub type ShiftbufbbsR = crate::FieldReader<u32>;
#[doc = "Field `SHIFTBUFBBS` writer - Shift Buffer"]
pub type ShiftbufbbsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbbs(&self) -> ShiftbufbbsR {
        ShiftbufbbsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn shiftbufbbs(&mut self) -> ShiftbufbbsW<ShiftbufbbsSpec> {
        ShiftbufbbsW::new(self, 0)
    }
}
#[doc = "Shifter Buffer N Bit Byte Swapped Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftbufbbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftbufbbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftbufbbsSpec;
impl crate::RegisterSpec for ShiftbufbbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shiftbufbbs::R`](R) reader structure"]
impl crate::Readable for ShiftbufbbsSpec {}
#[doc = "`write(|w| ..)` method takes [`shiftbufbbs::W`](W) writer structure"]
impl crate::Writable for ShiftbufbbsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTBUFBBS%s to value 0"]
impl crate::Resettable for ShiftbufbbsSpec {
    const RESET_VALUE: u32 = 0;
}
