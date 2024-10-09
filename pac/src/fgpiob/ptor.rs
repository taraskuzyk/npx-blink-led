#[doc = "Register `PTOR` reader"]
pub type R = crate::R<PtorSpec>;
#[doc = "Register `PTOR` writer"]
pub type W = crate::W<PtorSpec>;
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ptto {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    B00000000000000000000000000000001 = 1,
}
impl From<Ptto> for u32 {
    #[inline(always)]
    fn from(variant: Ptto) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ptto {
    type Ux = u32;
}
impl crate::IsEnum for Ptto {}
#[doc = "Field `PTTO` reader - Port Toggle Output"]
pub type PttoR = crate::FieldReader<Ptto>;
impl PttoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ptto> {
        match self.bits {
            0 => Some(Ptto::B00000000000000000000000000000000),
            1 => Some(Ptto::B00000000000000000000000000000001),
            _ => None,
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Ptto::B00000000000000000000000000000000
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Ptto::B00000000000000000000000000000001
    }
}
#[doc = "Field `PTTO` writer - Port Toggle Output"]
pub type PttoW<'a, REG> = crate::FieldWriter<'a, REG, 32, Ptto>;
impl<'a, REG> PttoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Ptto::B00000000000000000000000000000000)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Ptto::B00000000000000000000000000000001)
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Toggle Output"]
    #[inline(always)]
    pub fn ptto(&self) -> PttoR {
        PttoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto(&mut self) -> PttoW<PtorSpec> {
        PttoW::new(self, 0)
    }
}
#[doc = "Port Toggle Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PtorSpec;
impl crate::RegisterSpec for PtorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ptor::R`](R) reader structure"]
impl crate::Readable for PtorSpec {}
#[doc = "`write(|w| ..)` method takes [`ptor::W`](W) writer structure"]
impl crate::Writable for PtorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PTOR to value 0"]
impl crate::Resettable for PtorSpec {
    const RESET_VALUE: u32 = 0;
}
