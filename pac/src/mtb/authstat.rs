#[doc = "Register `AUTHSTAT` reader"]
pub type R = crate::R<AuthstatSpec>;
#[doc = "Field `BIT0` reader - Connected to DBGEN."]
pub type Bit0R = crate::BitReader;
#[doc = "Field `BIT1` reader - BIT1"]
pub type Bit1R = crate::BitReader;
#[doc = "Field `BIT2` reader - BIT2"]
pub type Bit2R = crate::BitReader;
#[doc = "Field `BIT3` reader - BIT3"]
pub type Bit3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Connected to DBGEN."]
    #[inline(always)]
    pub fn bit0(&self) -> Bit0R {
        Bit0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BIT1"]
    #[inline(always)]
    pub fn bit1(&self) -> Bit1R {
        Bit1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BIT2"]
    #[inline(always)]
    pub fn bit2(&self) -> Bit2R {
        Bit2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BIT3"]
    #[inline(always)]
    pub fn bit3(&self) -> Bit3R {
        Bit3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Authentication Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`authstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AuthstatSpec;
impl crate::RegisterSpec for AuthstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`authstat::R`](R) reader structure"]
impl crate::Readable for AuthstatSpec {}
#[doc = "`reset()` method sets AUTHSTAT to value 0"]
impl crate::Resettable for AuthstatSpec {
    const RESET_VALUE: u32 = 0;
}
