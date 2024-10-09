#[doc = "Register `SHIFTEIEN` reader"]
pub type R = crate::R<ShifteienSpec>;
#[doc = "Register `SHIFTEIEN` writer"]
pub type W = crate::W<ShifteienSpec>;
#[doc = "Shifter Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seie {
    #[doc = "0: Shifter Error Flag interrupt disabled"]
    B0000 = 0,
    #[doc = "1: Shifter Error Flag interrupt enabled"]
    B0001 = 1,
}
impl From<Seie> for u8 {
    #[inline(always)]
    fn from(variant: Seie) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Seie {
    type Ux = u8;
}
impl crate::IsEnum for Seie {}
#[doc = "Field `SEIE` reader - Shifter Error Interrupt Enable"]
pub type SeieR = crate::FieldReader<Seie>;
impl SeieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Seie> {
        match self.bits {
            0 => Some(Seie::B0000),
            1 => Some(Seie::B0001),
            _ => None,
        }
    }
    #[doc = "Shifter Error Flag interrupt disabled"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Seie::B0000
    }
    #[doc = "Shifter Error Flag interrupt enabled"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Seie::B0001
    }
}
#[doc = "Field `SEIE` writer - Shifter Error Interrupt Enable"]
pub type SeieW<'a, REG> = crate::FieldWriter<'a, REG, 4, Seie>;
impl<'a, REG> SeieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Shifter Error Flag interrupt disabled"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Seie::B0000)
    }
    #[doc = "Shifter Error Flag interrupt enabled"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Seie::B0001)
    }
}
impl R {
    #[doc = "Bits 0:3 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&self) -> SeieR {
        SeieR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SeieW<ShifteienSpec> {
        SeieW::new(self, 0)
    }
}
#[doc = "Shifter Error Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`shifteien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shifteien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShifteienSpec;
impl crate::RegisterSpec for ShifteienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shifteien::R`](R) reader structure"]
impl crate::Readable for ShifteienSpec {}
#[doc = "`write(|w| ..)` method takes [`shifteien::W`](W) writer structure"]
impl crate::Writable for ShifteienSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTEIEN to value 0"]
impl crate::Resettable for ShifteienSpec {
    const RESET_VALUE: u32 = 0;
}
