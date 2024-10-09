#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Time Compensation Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tcr {
    #[doc = "128: Time Prescaler Register overflows every 32896 clock cycles."]
    B10000000 = 128,
    #[doc = "255: Time Prescaler Register overflows every 32769 clock cycles."]
    B11111111 = 255,
    #[doc = "0: Time Prescaler Register overflows every 32768 clock cycles."]
    B00000000 = 0,
    #[doc = "1: Time Prescaler Register overflows every 32767 clock cycles."]
    B00000001 = 1,
    #[doc = "127: Time Prescaler Register overflows every 32641 clock cycles."]
    B01111111 = 127,
}
impl From<Tcr> for u8 {
    #[inline(always)]
    fn from(variant: Tcr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tcr {
    type Ux = u8;
}
impl crate::IsEnum for Tcr {}
#[doc = "Field `TCR` reader - Time Compensation Register"]
pub type TcrR = crate::FieldReader<Tcr>;
impl TcrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tcr> {
        match self.bits {
            128 => Some(Tcr::B10000000),
            255 => Some(Tcr::B11111111),
            0 => Some(Tcr::B00000000),
            1 => Some(Tcr::B00000001),
            127 => Some(Tcr::B01111111),
            _ => None,
        }
    }
    #[doc = "Time Prescaler Register overflows every 32896 clock cycles."]
    #[inline(always)]
    pub fn is_b10000000(&self) -> bool {
        *self == Tcr::B10000000
    }
    #[doc = "Time Prescaler Register overflows every 32769 clock cycles."]
    #[inline(always)]
    pub fn is_b11111111(&self) -> bool {
        *self == Tcr::B11111111
    }
    #[doc = "Time Prescaler Register overflows every 32768 clock cycles."]
    #[inline(always)]
    pub fn is_b00000000(&self) -> bool {
        *self == Tcr::B00000000
    }
    #[doc = "Time Prescaler Register overflows every 32767 clock cycles."]
    #[inline(always)]
    pub fn is_b00000001(&self) -> bool {
        *self == Tcr::B00000001
    }
    #[doc = "Time Prescaler Register overflows every 32641 clock cycles."]
    #[inline(always)]
    pub fn is_b01111111(&self) -> bool {
        *self == Tcr::B01111111
    }
}
#[doc = "Field `TCR` writer - Time Compensation Register"]
pub type TcrW<'a, REG> = crate::FieldWriter<'a, REG, 8, Tcr>;
impl<'a, REG> TcrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Time Prescaler Register overflows every 32896 clock cycles."]
    #[inline(always)]
    pub fn b10000000(self) -> &'a mut crate::W<REG> {
        self.variant(Tcr::B10000000)
    }
    #[doc = "Time Prescaler Register overflows every 32769 clock cycles."]
    #[inline(always)]
    pub fn b11111111(self) -> &'a mut crate::W<REG> {
        self.variant(Tcr::B11111111)
    }
    #[doc = "Time Prescaler Register overflows every 32768 clock cycles."]
    #[inline(always)]
    pub fn b00000000(self) -> &'a mut crate::W<REG> {
        self.variant(Tcr::B00000000)
    }
    #[doc = "Time Prescaler Register overflows every 32767 clock cycles."]
    #[inline(always)]
    pub fn b00000001(self) -> &'a mut crate::W<REG> {
        self.variant(Tcr::B00000001)
    }
    #[doc = "Time Prescaler Register overflows every 32641 clock cycles."]
    #[inline(always)]
    pub fn b01111111(self) -> &'a mut crate::W<REG> {
        self.variant(Tcr::B01111111)
    }
}
#[doc = "Field `CIR` reader - Compensation Interval Register"]
pub type CirR = crate::FieldReader;
#[doc = "Field `CIR` writer - Compensation Interval Register"]
pub type CirW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCV` reader - Time Compensation Value"]
pub type TcvR = crate::FieldReader;
#[doc = "Field `CIC` reader - Compensation Interval Counter"]
pub type CicR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Time Compensation Register"]
    #[inline(always)]
    pub fn tcr(&self) -> TcrR {
        TcrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compensation Interval Register"]
    #[inline(always)]
    pub fn cir(&self) -> CirR {
        CirR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Time Compensation Value"]
    #[inline(always)]
    pub fn tcv(&self) -> TcvR {
        TcvR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compensation Interval Counter"]
    #[inline(always)]
    pub fn cic(&self) -> CicR {
        CicR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time Compensation Register"]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TcrW<TcrSpec> {
        TcrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Compensation Interval Register"]
    #[inline(always)]
    #[must_use]
    pub fn cir(&mut self) -> CirW<TcrSpec> {
        CirW::new(self, 8)
    }
}
#[doc = "RTC Time Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {
    const RESET_VALUE: u32 = 0;
}
