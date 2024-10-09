#[doc = "Register `SHIFTSDEN` reader"]
pub type R = crate::R<ShiftsdenSpec>;
#[doc = "Register `SHIFTSDEN` writer"]
pub type W = crate::W<ShiftsdenSpec>;
#[doc = "Shifter Status DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssde {
    #[doc = "0: Shifter Status Flag DMA request is disabled"]
    B0000 = 0,
    #[doc = "1: Shifter Status Flag DMA request is enabled"]
    B0001 = 1,
}
impl From<Ssde> for u8 {
    #[inline(always)]
    fn from(variant: Ssde) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssde {
    type Ux = u8;
}
impl crate::IsEnum for Ssde {}
#[doc = "Field `SSDE` reader - Shifter Status DMA Enable"]
pub type SsdeR = crate::FieldReader<Ssde>;
impl SsdeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssde> {
        match self.bits {
            0 => Some(Ssde::B0000),
            1 => Some(Ssde::B0001),
            _ => None,
        }
    }
    #[doc = "Shifter Status Flag DMA request is disabled"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Ssde::B0000
    }
    #[doc = "Shifter Status Flag DMA request is enabled"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Ssde::B0001
    }
}
#[doc = "Field `SSDE` writer - Shifter Status DMA Enable"]
pub type SsdeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ssde>;
impl<'a, REG> SsdeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Shifter Status Flag DMA request is disabled"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Ssde::B0000)
    }
    #[doc = "Shifter Status Flag DMA request is enabled"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Ssde::B0001)
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Status DMA Enable"]
    #[inline(always)]
    pub fn ssde(&self) -> SsdeR {
        SsdeR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Status DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssde(&mut self) -> SsdeW<ShiftsdenSpec> {
        SsdeW::new(self, 0)
    }
}
#[doc = "Shifter Status DMA Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftsden::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftsden::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftsdenSpec;
impl crate::RegisterSpec for ShiftsdenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shiftsden::R`](R) reader structure"]
impl crate::Readable for ShiftsdenSpec {}
#[doc = "`write(|w| ..)` method takes [`shiftsden::W`](W) writer structure"]
impl crate::Writable for ShiftsdenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTSDEN to value 0"]
impl crate::Resettable for ShiftsdenSpec {
    const RESET_VALUE: u32 = 0;
}
