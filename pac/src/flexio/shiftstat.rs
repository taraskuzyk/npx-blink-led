#[doc = "Register `SHIFTSTAT` reader"]
pub type R = crate::R<ShiftstatSpec>;
#[doc = "Register `SHIFTSTAT` writer"]
pub type W = crate::W<ShiftstatSpec>;
#[doc = "Shifter Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssf {
    #[doc = "0: Status flag is clear"]
    B0000 = 0,
    #[doc = "1: Status flag is set"]
    B0001 = 1,
}
impl From<Ssf> for u8 {
    #[inline(always)]
    fn from(variant: Ssf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssf {
    type Ux = u8;
}
impl crate::IsEnum for Ssf {}
#[doc = "Field `SSF` reader - Shifter Status Flag"]
pub type SsfR = crate::FieldReader<Ssf>;
impl SsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssf> {
        match self.bits {
            0 => Some(Ssf::B0000),
            1 => Some(Ssf::B0001),
            _ => None,
        }
    }
    #[doc = "Status flag is clear"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Ssf::B0000
    }
    #[doc = "Status flag is set"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Ssf::B0001
    }
}
#[doc = "Field `SSF` writer - Shifter Status Flag"]
pub type SsfW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ssf>;
impl<'a, REG> SsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Status flag is clear"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Ssf::B0000)
    }
    #[doc = "Status flag is set"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Ssf::B0001)
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Status Flag"]
    #[inline(always)]
    pub fn ssf(&self) -> SsfR {
        SsfR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssf(&mut self) -> SsfW<ShiftstatSpec> {
        SsfW::new(self, 0)
    }
}
#[doc = "Shifter Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftstatSpec;
impl crate::RegisterSpec for ShiftstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shiftstat::R`](R) reader structure"]
impl crate::Readable for ShiftstatSpec {}
#[doc = "`write(|w| ..)` method takes [`shiftstat::W`](W) writer structure"]
impl crate::Writable for ShiftstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTSTAT to value 0"]
impl crate::Resettable for ShiftstatSpec {
    const RESET_VALUE: u32 = 0;
}
