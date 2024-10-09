#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "DAC Buffer Read Pointer Bottom Position Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacbfrpbf {
    #[doc = "0: The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    B0 = 0,
    #[doc = "1: The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    B1 = 1,
}
impl From<Dacbfrpbf> for bool {
    #[inline(always)]
    fn from(variant: Dacbfrpbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACBFRPBF` reader - DAC Buffer Read Pointer Bottom Position Flag"]
pub type DacbfrpbfR = crate::BitReader<Dacbfrpbf>;
impl DacbfrpbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacbfrpbf {
        match self.bits {
            false => Dacbfrpbf::B0,
            true => Dacbfrpbf::B1,
        }
    }
    #[doc = "The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dacbfrpbf::B0
    }
    #[doc = "The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dacbfrpbf::B1
    }
}
#[doc = "Field `DACBFRPBF` writer - DAC Buffer Read Pointer Bottom Position Flag"]
pub type DacbfrpbfW<'a, REG> = crate::BitWriter<'a, REG, Dacbfrpbf>;
impl<'a, REG> DacbfrpbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DAC buffer read pointer is not equal to C2\\[DACBFUP\\]."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacbfrpbf::B0)
    }
    #[doc = "The DAC buffer read pointer is equal to C2\\[DACBFUP\\]."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacbfrpbf::B1)
    }
}
#[doc = "DAC Buffer Read Pointer Top Position Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dacbfrptf {
    #[doc = "0: The DAC buffer read pointer is not zero."]
    B0 = 0,
    #[doc = "1: The DAC buffer read pointer is zero."]
    B1 = 1,
}
impl From<Dacbfrptf> for bool {
    #[inline(always)]
    fn from(variant: Dacbfrptf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACBFRPTF` reader - DAC Buffer Read Pointer Top Position Flag"]
pub type DacbfrptfR = crate::BitReader<Dacbfrptf>;
impl DacbfrptfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dacbfrptf {
        match self.bits {
            false => Dacbfrptf::B0,
            true => Dacbfrptf::B1,
        }
    }
    #[doc = "The DAC buffer read pointer is not zero."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dacbfrptf::B0
    }
    #[doc = "The DAC buffer read pointer is zero."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dacbfrptf::B1
    }
}
#[doc = "Field `DACBFRPTF` writer - DAC Buffer Read Pointer Top Position Flag"]
pub type DacbfrptfW<'a, REG> = crate::BitWriter<'a, REG, Dacbfrptf>;
impl<'a, REG> DacbfrptfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DAC buffer read pointer is not zero."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dacbfrptf::B0)
    }
    #[doc = "The DAC buffer read pointer is zero."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dacbfrptf::B1)
    }
}
impl R {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline(always)]
    pub fn dacbfrpbf(&self) -> DacbfrpbfR {
        DacbfrpbfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Position Flag"]
    #[inline(always)]
    pub fn dacbfrptf(&self) -> DacbfrptfR {
        DacbfrptfR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Position Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfrpbf(&mut self) -> DacbfrpbfW<SrSpec> {
        DacbfrpbfW::new(self, 0)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Position Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfrptf(&mut self) -> DacbfrptfW<SrSpec> {
        DacbfrptfW::new(self, 1)
    }
}
#[doc = "DAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SR to value 0x02"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u8 = 0x02;
}
