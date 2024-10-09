#[doc = "Register `FM` reader"]
pub type R = crate::R<FmSpec>;
#[doc = "Register `FM` writer"]
pub type W = crate::W<FmSpec>;
#[doc = "Force ROM Boot\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Forcerom {
    #[doc = "0: No effect"]
    B00 = 0,
    #[doc = "1: Force boot from ROM with RCM_MR\\[1\\]
set."]
    B01 = 1,
    #[doc = "2: Force boot from ROM with RCM_MR\\[2\\]
set."]
    B10 = 2,
    #[doc = "3: Force boot from ROM with RCM_MR\\[2:1\\]
set."]
    B11 = 3,
}
impl From<Forcerom> for u8 {
    #[inline(always)]
    fn from(variant: Forcerom) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Forcerom {
    type Ux = u8;
}
impl crate::IsEnum for Forcerom {}
#[doc = "Field `FORCEROM` reader - Force ROM Boot"]
pub type ForceromR = crate::FieldReader<Forcerom>;
impl ForceromR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Forcerom {
        match self.bits {
            0 => Forcerom::B00,
            1 => Forcerom::B01,
            2 => Forcerom::B10,
            3 => Forcerom::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Forcerom::B00
    }
    #[doc = "Force boot from ROM with RCM_MR\\[1\\]
set."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Forcerom::B01
    }
    #[doc = "Force boot from ROM with RCM_MR\\[2\\]
set."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Forcerom::B10
    }
    #[doc = "Force boot from ROM with RCM_MR\\[2:1\\]
set."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Forcerom::B11
    }
}
#[doc = "Field `FORCEROM` writer - Force ROM Boot"]
pub type ForceromW<'a, REG> = crate::FieldWriter<'a, REG, 2, Forcerom, crate::Safe>;
impl<'a, REG> ForceromW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Forcerom::B00)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[1\\]
set."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Forcerom::B01)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[2\\]
set."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Forcerom::B10)
    }
    #[doc = "Force boot from ROM with RCM_MR\\[2:1\\]
set."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Forcerom::B11)
    }
}
impl R {
    #[doc = "Bits 1:2 - Force ROM Boot"]
    #[inline(always)]
    pub fn forcerom(&self) -> ForceromR {
        ForceromR::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bits 1:2 - Force ROM Boot"]
    #[inline(always)]
    #[must_use]
    pub fn forcerom(&mut self) -> ForceromW<FmSpec> {
        ForceromW::new(self, 1)
    }
}
#[doc = "Force Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmSpec;
impl crate::RegisterSpec for FmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fm::R`](R) reader structure"]
impl crate::Readable for FmSpec {}
#[doc = "`write(|w| ..)` method takes [`fm::W`](W) writer structure"]
impl crate::Writable for FmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets FM to value 0"]
impl crate::Resettable for FmSpec {
    const RESET_VALUE: u8 = 0;
}
