#[doc = "Register `PSOR` reader"]
pub type R = crate::R<PsorSpec>;
#[doc = "Register `PSOR` writer"]
pub type W = crate::W<PsorSpec>;
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Ptso {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    B00000000000000000000000000000001 = 1,
}
impl From<Ptso> for u32 {
    #[inline(always)]
    fn from(variant: Ptso) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ptso {
    type Ux = u32;
}
impl crate::IsEnum for Ptso {}
#[doc = "Field `PTSO` reader - Port Set Output"]
pub type PtsoR = crate::FieldReader<Ptso>;
impl PtsoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ptso> {
        match self.bits {
            0 => Some(Ptso::B00000000000000000000000000000000),
            1 => Some(Ptso::B00000000000000000000000000000001),
            _ => None,
        }
    }
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Ptso::B00000000000000000000000000000000
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Ptso::B00000000000000000000000000000001
    }
}
#[doc = "Field `PTSO` writer - Port Set Output"]
pub type PtsoW<'a, REG> = crate::FieldWriter<'a, REG, 32, Ptso>;
impl<'a, REG> PtsoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Ptso::B00000000000000000000000000000000)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Ptso::B00000000000000000000000000000001)
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Set Output"]
    #[inline(always)]
    pub fn ptso(&self) -> PtsoR {
        PtsoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso(&mut self) -> PtsoW<PsorSpec> {
        PtsoW::new(self, 0)
    }
}
#[doc = "Port Set Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsorSpec;
impl crate::RegisterSpec for PsorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psor::R`](R) reader structure"]
impl crate::Readable for PsorSpec {}
#[doc = "`write(|w| ..)` method takes [`psor::W`](W) writer structure"]
impl crate::Writable for PsorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSOR to value 0"]
impl crate::Resettable for PsorSpec {
    const RESET_VALUE: u32 = 0;
}
