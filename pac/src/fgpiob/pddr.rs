#[doc = "Register `PDDR` reader"]
pub type R = crate::R<PddrSpec>;
#[doc = "Register `PDDR` writer"]
pub type W = crate::W<PddrSpec>;
#[doc = "Port Data Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pdd {
    #[doc = "0: Pin is configured as general-purpose input, for the GPIO function."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Pin is configured as general-purpose output, for the GPIO function."]
    B00000000000000000000000000000001 = 1,
}
impl From<Pdd> for u32 {
    #[inline(always)]
    fn from(variant: Pdd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pdd {
    type Ux = u32;
}
impl crate::IsEnum for Pdd {}
#[doc = "Field `PDD` reader - Port Data Direction"]
pub type PddR = crate::FieldReader<Pdd>;
impl PddR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pdd> {
        match self.bits {
            0 => Some(Pdd::B00000000000000000000000000000000),
            1 => Some(Pdd::B00000000000000000000000000000001),
            _ => None,
        }
    }
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Pdd::B00000000000000000000000000000000
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Pdd::B00000000000000000000000000000001
    }
}
#[doc = "Field `PDD` writer - Port Data Direction"]
pub type PddW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pdd>;
impl<'a, REG> PddW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Pin is configured as general-purpose input, for the GPIO function."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Pdd::B00000000000000000000000000000000)
    }
    #[doc = "Pin is configured as general-purpose output, for the GPIO function."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Pdd::B00000000000000000000000000000001)
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    pub fn pdd(&self) -> PddR {
        PddR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Direction"]
    #[inline(always)]
    #[must_use]
    pub fn pdd(&mut self) -> PddW<PddrSpec> {
        PddW::new(self, 0)
    }
}
#[doc = "Port Data Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PddrSpec;
impl crate::RegisterSpec for PddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pddr::R`](R) reader structure"]
impl crate::Readable for PddrSpec {}
#[doc = "`write(|w| ..)` method takes [`pddr::W`](W) writer structure"]
impl crate::Writable for PddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDDR to value 0"]
impl crate::Resettable for PddrSpec {
    const RESET_VALUE: u32 = 0;
}
