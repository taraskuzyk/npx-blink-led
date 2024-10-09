#[doc = "Register `FDSR` reader"]
pub type R = crate::R<FdsrSpec>;
#[doc = "Register `FDSR` writer"]
pub type W = crate::W<FdsrSpec>;
#[doc = "Fault Detect Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fdcnt {
    #[doc = "0: No \"one\" samples."]
    B00000000 = 0,
    #[doc = "1: 1 \"one\" samples."]
    B00000001 = 1,
    #[doc = "2: 2 \"one\" samples."]
    B00000010 = 2,
    #[doc = "254: 254 \"one\" samples."]
    B11111110 = 254,
    #[doc = "255: 255 or more \"one\" samples. The FDCNT can overflow. Therefore, FDSWW and FDPRS must be reconfigured for proper sampling."]
    B11111111 = 255,
}
impl From<Fdcnt> for u8 {
    #[inline(always)]
    fn from(variant: Fdcnt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fdcnt {
    type Ux = u8;
}
impl crate::IsEnum for Fdcnt {}
#[doc = "Field `FDCNT` reader - Fault Detect Counter"]
pub type FdcntR = crate::FieldReader<Fdcnt>;
impl FdcntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fdcnt> {
        match self.bits {
            0 => Some(Fdcnt::B00000000),
            1 => Some(Fdcnt::B00000001),
            2 => Some(Fdcnt::B00000010),
            254 => Some(Fdcnt::B11111110),
            255 => Some(Fdcnt::B11111111),
            _ => None,
        }
    }
    #[doc = "No \"one\" samples."]
    #[inline(always)]
    pub fn is_b00000000(&self) -> bool {
        *self == Fdcnt::B00000000
    }
    #[doc = "1 \"one\" samples."]
    #[inline(always)]
    pub fn is_b00000001(&self) -> bool {
        *self == Fdcnt::B00000001
    }
    #[doc = "2 \"one\" samples."]
    #[inline(always)]
    pub fn is_b00000010(&self) -> bool {
        *self == Fdcnt::B00000010
    }
    #[doc = "254 \"one\" samples."]
    #[inline(always)]
    pub fn is_b11111110(&self) -> bool {
        *self == Fdcnt::B11111110
    }
    #[doc = "255 or more \"one\" samples. The FDCNT can overflow. Therefore, FDSWW and FDPRS must be reconfigured for proper sampling."]
    #[inline(always)]
    pub fn is_b11111111(&self) -> bool {
        *self == Fdcnt::B11111111
    }
}
#[doc = "Fault Detection Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fdcf {
    #[doc = "0: Fault detection is not completed."]
    B0 = 0,
    #[doc = "1: Fault detection is completed."]
    B1 = 1,
}
impl From<Fdcf> for bool {
    #[inline(always)]
    fn from(variant: Fdcf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FDCF` reader - Fault Detection Complete Flag"]
pub type FdcfR = crate::BitReader<Fdcf>;
impl FdcfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fdcf {
        match self.bits {
            false => Fdcf::B0,
            true => Fdcf::B1,
        }
    }
    #[doc = "Fault detection is not completed."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fdcf::B0
    }
    #[doc = "Fault detection is completed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fdcf::B1
    }
}
#[doc = "Field `FDCF` writer - Fault Detection Complete Flag"]
pub type FdcfW<'a, REG> = crate::BitWriter<'a, REG, Fdcf>;
impl<'a, REG> FdcfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fault detection is not completed."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fdcf::B0)
    }
    #[doc = "Fault detection is completed."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fdcf::B1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Fault Detect Counter"]
    #[inline(always)]
    pub fn fdcnt(&self) -> FdcntR {
        FdcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Fault Detection Complete Flag"]
    #[inline(always)]
    pub fn fdcf(&self) -> FdcfR {
        FdcfR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Fault Detection Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fdcf(&mut self) -> FdcfW<FdsrSpec> {
        FdcfW::new(self, 15)
    }
}
#[doc = "LCD Fault Detect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdsrSpec;
impl crate::RegisterSpec for FdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdsr::R`](R) reader structure"]
impl crate::Readable for FdsrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdsr::W`](W) writer structure"]
impl crate::Writable for FdsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDSR to value 0"]
impl crate::Resettable for FdsrSpec {
    const RESET_VALUE: u32 = 0;
}
