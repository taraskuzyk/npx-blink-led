#[doc = "Register `FPROT%s` reader"]
pub type R = crate::R<FprotSpec>;
#[doc = "Register `FPROT%s` writer"]
pub type W = crate::W<FprotSpec>;
#[doc = "Program Flash Region Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prot {
    #[doc = "0: Program flash region is protected."]
    B00000000 = 0,
    #[doc = "1: Program flash region is not protected"]
    B00000001 = 1,
}
impl From<Prot> for u8 {
    #[inline(always)]
    fn from(variant: Prot) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prot {
    type Ux = u8;
}
impl crate::IsEnum for Prot {}
#[doc = "Field `PROT` reader - Program Flash Region Protect"]
pub type ProtR = crate::FieldReader<Prot>;
impl ProtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prot> {
        match self.bits {
            0 => Some(Prot::B00000000),
            1 => Some(Prot::B00000001),
            _ => None,
        }
    }
    #[doc = "Program flash region is protected."]
    #[inline(always)]
    pub fn is_b00000000(&self) -> bool {
        *self == Prot::B00000000
    }
    #[doc = "Program flash region is not protected"]
    #[inline(always)]
    pub fn is_b00000001(&self) -> bool {
        *self == Prot::B00000001
    }
}
#[doc = "Field `PROT` writer - Program Flash Region Protect"]
pub type ProtW<'a, REG> = crate::FieldWriter<'a, REG, 8, Prot>;
impl<'a, REG> ProtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Program flash region is protected."]
    #[inline(always)]
    pub fn b00000000(self) -> &'a mut crate::W<REG> {
        self.variant(Prot::B00000000)
    }
    #[doc = "Program flash region is not protected"]
    #[inline(always)]
    pub fn b00000001(self) -> &'a mut crate::W<REG> {
        self.variant(Prot::B00000001)
    }
}
impl R {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> ProtR {
        ProtR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    #[must_use]
    pub fn prot(&mut self) -> ProtW<FprotSpec> {
        ProtW::new(self, 0)
    }
}
#[doc = "Program Flash Protection Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`fprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FprotSpec;
impl crate::RegisterSpec for FprotSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fprot::R`](R) reader structure"]
impl crate::Readable for FprotSpec {}
#[doc = "`write(|w| ..)` method takes [`fprot::W`](W) writer structure"]
impl crate::Writable for FprotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FPROT%s to value 0"]
impl crate::Resettable for FprotSpec {
    const RESET_VALUE: u8 = 0;
}
