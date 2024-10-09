#[doc = "Register `CLK_RECOVER_INT_EN` reader"]
pub type R = crate::R<ClkRecoverIntEnSpec>;
#[doc = "Register `CLK_RECOVER_INT_EN` writer"]
pub type W = crate::W<ClkRecoverIntEnSpec>;
#[doc = "Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OvfErrorEn {
    #[doc = "0: The interrupt will be masked"]
    B0 = 0,
    #[doc = "1: The interrupt will be enabled (default)"]
    B1 = 1,
}
impl From<OvfErrorEn> for bool {
    #[inline(always)]
    fn from(variant: OvfErrorEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVF_ERROR_EN` reader - Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT."]
pub type OvfErrorEnR = crate::BitReader<OvfErrorEn>;
impl OvfErrorEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OvfErrorEn {
        match self.bits {
            false => OvfErrorEn::B0,
            true => OvfErrorEn::B1,
        }
    }
    #[doc = "The interrupt will be masked"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == OvfErrorEn::B0
    }
    #[doc = "The interrupt will be enabled (default)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == OvfErrorEn::B1
    }
}
#[doc = "Field `OVF_ERROR_EN` writer - Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT."]
pub type OvfErrorEnW<'a, REG> = crate::BitWriter<'a, REG, OvfErrorEn>;
impl<'a, REG> OvfErrorEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The interrupt will be masked"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(OvfErrorEn::B0)
    }
    #[doc = "The interrupt will be enabled (default)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(OvfErrorEn::B1)
    }
}
impl R {
    #[doc = "Bit 4 - Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT."]
    #[inline(always)]
    pub fn ovf_error_en(&self) -> OvfErrorEnR {
        OvfErrorEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Determines whether OVF_ERROR condition signal is used in generation of USB_CLK_RECOVERY_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ovf_error_en(&mut self) -> OvfErrorEnW<ClkRecoverIntEnSpec> {
        OvfErrorEnW::new(self, 4)
    }
}
#[doc = "Clock recovery combined interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_recover_int_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_recover_int_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkRecoverIntEnSpec;
impl crate::RegisterSpec for ClkRecoverIntEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clk_recover_int_en::R`](R) reader structure"]
impl crate::Readable for ClkRecoverIntEnSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_recover_int_en::W`](W) writer structure"]
impl crate::Writable for ClkRecoverIntEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CLK_RECOVER_INT_EN to value 0x10"]
impl crate::Resettable for ClkRecoverIntEnSpec {
    const RESET_VALUE: u8 = 0x10;
}
