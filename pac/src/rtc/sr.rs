#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Time Invalid Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tif {
    #[doc = "0: Time is valid."]
    B0 = 0,
    #[doc = "1: Time is invalid and time counter is read as zero."]
    B1 = 1,
}
impl From<Tif> for bool {
    #[inline(always)]
    fn from(variant: Tif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIF` reader - Time Invalid Flag"]
pub type TifR = crate::BitReader<Tif>;
impl TifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tif {
        match self.bits {
            false => Tif::B0,
            true => Tif::B1,
        }
    }
    #[doc = "Time is valid."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tif::B0
    }
    #[doc = "Time is invalid and time counter is read as zero."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tif::B1
    }
}
#[doc = "Time Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tof {
    #[doc = "0: Time overflow has not occurred."]
    B0 = 0,
    #[doc = "1: Time overflow has occurred and time counter is read as zero."]
    B1 = 1,
}
impl From<Tof> for bool {
    #[inline(always)]
    fn from(variant: Tof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` reader - Time Overflow Flag"]
pub type TofR = crate::BitReader<Tof>;
impl TofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tof {
        match self.bits {
            false => Tof::B0,
            true => Tof::B1,
        }
    }
    #[doc = "Time overflow has not occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tof::B0
    }
    #[doc = "Time overflow has occurred and time counter is read as zero."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tof::B1
    }
}
#[doc = "Time Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Taf {
    #[doc = "0: Time alarm has not occurred."]
    B0 = 0,
    #[doc = "1: Time alarm has occurred."]
    B1 = 1,
}
impl From<Taf> for bool {
    #[inline(always)]
    fn from(variant: Taf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAF` reader - Time Alarm Flag"]
pub type TafR = crate::BitReader<Taf>;
impl TafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Taf {
        match self.bits {
            false => Taf::B0,
            true => Taf::B1,
        }
    }
    #[doc = "Time alarm has not occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Taf::B0
    }
    #[doc = "Time alarm has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Taf::B1
    }
}
#[doc = "Time Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tce {
    #[doc = "0: Time counter is disabled."]
    B0 = 0,
    #[doc = "1: Time counter is enabled."]
    B1 = 1,
}
impl From<Tce> for bool {
    #[inline(always)]
    fn from(variant: Tce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCE` reader - Time Counter Enable"]
pub type TceR = crate::BitReader<Tce>;
impl TceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tce {
        match self.bits {
            false => Tce::B0,
            true => Tce::B1,
        }
    }
    #[doc = "Time counter is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tce::B0
    }
    #[doc = "Time counter is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tce::B1
    }
}
#[doc = "Field `TCE` writer - Time Counter Enable"]
pub type TceW<'a, REG> = crate::BitWriter<'a, REG, Tce>;
impl<'a, REG> TceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time counter is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tce::B0)
    }
    #[doc = "Time counter is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tce::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Time Invalid Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TifR {
        TifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TofR {
        TofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Flag"]
    #[inline(always)]
    pub fn taf(&self) -> TafR {
        TafR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Counter Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TceR {
        TceR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Time Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TceW<SrSpec> {
        TceW::new(self, 4)
    }
}
#[doc = "RTC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x01;
}
