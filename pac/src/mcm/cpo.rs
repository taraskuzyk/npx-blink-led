#[doc = "Register `CPO` reader"]
pub type R = crate::R<CpoSpec>;
#[doc = "Register `CPO` writer"]
pub type W = crate::W<CpoSpec>;
#[doc = "Compute Operation Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cporeq {
    #[doc = "0: Request is cleared."]
    B0 = 0,
    #[doc = "1: Request Compute Operation."]
    B1 = 1,
}
impl From<Cporeq> for bool {
    #[inline(always)]
    fn from(variant: Cporeq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOREQ` reader - Compute Operation Request"]
pub type CporeqR = crate::BitReader<Cporeq>;
impl CporeqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cporeq {
        match self.bits {
            false => Cporeq::B0,
            true => Cporeq::B1,
        }
    }
    #[doc = "Request is cleared."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cporeq::B0
    }
    #[doc = "Request Compute Operation."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cporeq::B1
    }
}
#[doc = "Field `CPOREQ` writer - Compute Operation Request"]
pub type CporeqW<'a, REG> = crate::BitWriter<'a, REG, Cporeq>;
impl<'a, REG> CporeqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Request is cleared."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cporeq::B0)
    }
    #[doc = "Request Compute Operation."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cporeq::B1)
    }
}
#[doc = "Compute Operation Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpoack {
    #[doc = "0: Compute operation entry has not completed or compute operation exit has completed."]
    B0 = 0,
    #[doc = "1: Compute operation entry has completed or compute operation exit has not completed."]
    B1 = 1,
}
impl From<Cpoack> for bool {
    #[inline(always)]
    fn from(variant: Cpoack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOACK` reader - Compute Operation Acknowledge"]
pub type CpoackR = crate::BitReader<Cpoack>;
impl CpoackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpoack {
        match self.bits {
            false => Cpoack::B0,
            true => Cpoack::B1,
        }
    }
    #[doc = "Compute operation entry has not completed or compute operation exit has completed."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cpoack::B0
    }
    #[doc = "Compute operation entry has completed or compute operation exit has not completed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cpoack::B1
    }
}
#[doc = "Compute Operation Wake-up on Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpowoi {
    #[doc = "0: No effect."]
    B0 = 0,
    #[doc = "1: When set, the CPOREQ is cleared on any interrupt or exception vector fetch."]
    B1 = 1,
}
impl From<Cpowoi> for bool {
    #[inline(always)]
    fn from(variant: Cpowoi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOWOI` reader - Compute Operation Wake-up on Interrupt"]
pub type CpowoiR = crate::BitReader<Cpowoi>;
impl CpowoiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpowoi {
        match self.bits {
            false => Cpowoi::B0,
            true => Cpowoi::B1,
        }
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cpowoi::B0
    }
    #[doc = "When set, the CPOREQ is cleared on any interrupt or exception vector fetch."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cpowoi::B1
    }
}
#[doc = "Field `CPOWOI` writer - Compute Operation Wake-up on Interrupt"]
pub type CpowoiW<'a, REG> = crate::BitWriter<'a, REG, Cpowoi>;
impl<'a, REG> CpowoiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cpowoi::B0)
    }
    #[doc = "When set, the CPOREQ is cleared on any interrupt or exception vector fetch."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpowoi::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Compute Operation Request"]
    #[inline(always)]
    pub fn cporeq(&self) -> CporeqR {
        CporeqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compute Operation Acknowledge"]
    #[inline(always)]
    pub fn cpoack(&self) -> CpoackR {
        CpoackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compute Operation Wake-up on Interrupt"]
    #[inline(always)]
    pub fn cpowoi(&self) -> CpowoiR {
        CpowoiR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compute Operation Request"]
    #[inline(always)]
    #[must_use]
    pub fn cporeq(&mut self) -> CporeqW<CpoSpec> {
        CporeqW::new(self, 0)
    }
    #[doc = "Bit 2 - Compute Operation Wake-up on Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cpowoi(&mut self) -> CpowoiW<CpoSpec> {
        CpowoiW::new(self, 2)
    }
}
#[doc = "Compute Operation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpoSpec;
impl crate::RegisterSpec for CpoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpo::R`](R) reader structure"]
impl crate::Readable for CpoSpec {}
#[doc = "`write(|w| ..)` method takes [`cpo::W`](W) writer structure"]
impl crate::Writable for CpoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPO to value 0"]
impl crate::Resettable for CpoSpec {
    const RESET_VALUE: u32 = 0;
}
