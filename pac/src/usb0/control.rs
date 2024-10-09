#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Provides control of the DP Pullup in USB, if USB is configured in non-OTG device mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dppullupnonotg {
    #[doc = "0: DP Pullup in non-OTG device mode is not enabled."]
    B0 = 0,
    #[doc = "1: DP Pullup in non-OTG device mode is enabled."]
    B1 = 1,
}
impl From<Dppullupnonotg> for bool {
    #[inline(always)]
    fn from(variant: Dppullupnonotg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPPULLUPNONOTG` reader - Provides control of the DP Pullup in USB, if USB is configured in non-OTG device mode."]
pub type DppullupnonotgR = crate::BitReader<Dppullupnonotg>;
impl DppullupnonotgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dppullupnonotg {
        match self.bits {
            false => Dppullupnonotg::B0,
            true => Dppullupnonotg::B1,
        }
    }
    #[doc = "DP Pullup in non-OTG device mode is not enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dppullupnonotg::B0
    }
    #[doc = "DP Pullup in non-OTG device mode is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dppullupnonotg::B1
    }
}
#[doc = "Field `DPPULLUPNONOTG` writer - Provides control of the DP Pullup in USB, if USB is configured in non-OTG device mode."]
pub type DppullupnonotgW<'a, REG> = crate::BitWriter<'a, REG, Dppullupnonotg>;
impl<'a, REG> DppullupnonotgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DP Pullup in non-OTG device mode is not enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dppullupnonotg::B0)
    }
    #[doc = "DP Pullup in non-OTG device mode is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dppullupnonotg::B1)
    }
}
impl R {
    #[doc = "Bit 4 - Provides control of the DP Pullup in USB, if USB is configured in non-OTG device mode."]
    #[inline(always)]
    pub fn dppullupnonotg(&self) -> DppullupnonotgR {
        DppullupnonotgR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Provides control of the DP Pullup in USB, if USB is configured in non-OTG device mode."]
    #[inline(always)]
    #[must_use]
    pub fn dppullupnonotg(&mut self) -> DppullupnonotgW<ControlSpec> {
        DppullupnonotgW::new(self, 4)
    }
}
#[doc = "USB OTG Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u8 = 0;
}
