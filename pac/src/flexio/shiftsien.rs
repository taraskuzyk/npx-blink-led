#[doc = "Register `SHIFTSIEN` reader"]
pub type R = crate::R<ShiftsienSpec>;
#[doc = "Register `SHIFTSIEN` writer"]
pub type W = crate::W<ShiftsienSpec>;
#[doc = "Shifter Status Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssie {
    #[doc = "0: Shifter Status Flag interrupt disabled"]
    B0000 = 0,
    #[doc = "1: Shifter Status Flag interrupt enabled"]
    B0001 = 1,
}
impl From<Ssie> for u8 {
    #[inline(always)]
    fn from(variant: Ssie) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssie {
    type Ux = u8;
}
impl crate::IsEnum for Ssie {}
#[doc = "Field `SSIE` reader - Shifter Status Interrupt Enable"]
pub type SsieR = crate::FieldReader<Ssie>;
impl SsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ssie> {
        match self.bits {
            0 => Some(Ssie::B0000),
            1 => Some(Ssie::B0001),
            _ => None,
        }
    }
    #[doc = "Shifter Status Flag interrupt disabled"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Ssie::B0000
    }
    #[doc = "Shifter Status Flag interrupt enabled"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Ssie::B0001
    }
}
#[doc = "Field `SSIE` writer - Shifter Status Interrupt Enable"]
pub type SsieW<'a, REG> = crate::FieldWriter<'a, REG, 4, Ssie>;
impl<'a, REG> SsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Shifter Status Flag interrupt disabled"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Ssie::B0000)
    }
    #[doc = "Shifter Status Flag interrupt enabled"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Ssie::B0001)
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub fn ssie(&self) -> SsieR {
        SsieR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Status Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssie(&mut self) -> SsieW<ShiftsienSpec> {
        SsieW::new(self, 0)
    }
}
#[doc = "Shifter Status Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftsien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftsien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftsienSpec;
impl crate::RegisterSpec for ShiftsienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shiftsien::R`](R) reader structure"]
impl crate::Readable for ShiftsienSpec {}
#[doc = "`write(|w| ..)` method takes [`shiftsien::W`](W) writer structure"]
impl crate::Writable for ShiftsienSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTSIEN to value 0"]
impl crate::Resettable for ShiftsienSpec {
    const RESET_VALUE: u32 = 0;
}
