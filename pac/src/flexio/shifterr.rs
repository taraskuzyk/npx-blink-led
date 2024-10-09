#[doc = "Register `SHIFTERR` reader"]
pub type R = crate::R<ShifterrSpec>;
#[doc = "Register `SHIFTERR` writer"]
pub type W = crate::W<ShifterrSpec>;
#[doc = "Shifter Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sef {
    #[doc = "0: Shifter Error Flag is clear"]
    B0000 = 0,
    #[doc = "1: Shifter Error Flag is set"]
    B0001 = 1,
}
impl From<Sef> for u8 {
    #[inline(always)]
    fn from(variant: Sef) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sef {
    type Ux = u8;
}
impl crate::IsEnum for Sef {}
#[doc = "Field `SEF` reader - Shifter Error Flags"]
pub type SefR = crate::FieldReader<Sef>;
impl SefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sef> {
        match self.bits {
            0 => Some(Sef::B0000),
            1 => Some(Sef::B0001),
            _ => None,
        }
    }
    #[doc = "Shifter Error Flag is clear"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Sef::B0000
    }
    #[doc = "Shifter Error Flag is set"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Sef::B0001
    }
}
#[doc = "Field `SEF` writer - Shifter Error Flags"]
pub type SefW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sef>;
impl<'a, REG> SefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Shifter Error Flag is clear"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Sef::B0000)
    }
    #[doc = "Shifter Error Flag is set"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Sef::B0001)
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Error Flags"]
    #[inline(always)]
    pub fn sef(&self) -> SefR {
        SefR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Error Flags"]
    #[inline(always)]
    #[must_use]
    pub fn sef(&mut self) -> SefW<ShifterrSpec> {
        SefW::new(self, 0)
    }
}
#[doc = "Shifter Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shifterr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shifterr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShifterrSpec;
impl crate::RegisterSpec for ShifterrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shifterr::R`](R) reader structure"]
impl crate::Readable for ShifterrSpec {}
#[doc = "`write(|w| ..)` method takes [`shifterr::W`](W) writer structure"]
impl crate::Writable for ShifterrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTERR to value 0"]
impl crate::Resettable for ShifterrSpec {
    const RESET_VALUE: u32 = 0;
}
