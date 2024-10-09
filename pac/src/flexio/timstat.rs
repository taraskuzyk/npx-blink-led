#[doc = "Register `TIMSTAT` reader"]
pub type R = crate::R<TimstatSpec>;
#[doc = "Register `TIMSTAT` writer"]
pub type W = crate::W<TimstatSpec>;
#[doc = "Timer Status Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tsf {
    #[doc = "0: Timer Status Flag is clear"]
    B0000 = 0,
    #[doc = "1: Timer Status Flag is set"]
    B0001 = 1,
}
impl From<Tsf> for u8 {
    #[inline(always)]
    fn from(variant: Tsf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tsf {
    type Ux = u8;
}
impl crate::IsEnum for Tsf {}
#[doc = "Field `TSF` reader - Timer Status Flags"]
pub type TsfR = crate::FieldReader<Tsf>;
impl TsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tsf> {
        match self.bits {
            0 => Some(Tsf::B0000),
            1 => Some(Tsf::B0001),
            _ => None,
        }
    }
    #[doc = "Timer Status Flag is clear"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Tsf::B0000
    }
    #[doc = "Timer Status Flag is set"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Tsf::B0001
    }
}
#[doc = "Field `TSF` writer - Timer Status Flags"]
pub type TsfW<'a, REG> = crate::FieldWriter<'a, REG, 4, Tsf>;
impl<'a, REG> TsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer Status Flag is clear"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Tsf::B0000)
    }
    #[doc = "Timer Status Flag is set"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Tsf::B0001)
    }
}
impl R {
    #[doc = "Bits 0:3 - Timer Status Flags"]
    #[inline(always)]
    pub fn tsf(&self) -> TsfR {
        TsfR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timer Status Flags"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TsfW<TimstatSpec> {
        TsfW::new(self, 0)
    }
}
#[doc = "Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimstatSpec;
impl crate::RegisterSpec for TimstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timstat::R`](R) reader structure"]
impl crate::Readable for TimstatSpec {}
#[doc = "`write(|w| ..)` method takes [`timstat::W`](W) writer structure"]
impl crate::Writable for TimstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMSTAT to value 0"]
impl crate::Resettable for TimstatSpec {
    const RESET_VALUE: u32 = 0;
}
