#[doc = "Register `ENDPT%s` reader"]
pub type R = crate::R<EndptSpec>;
#[doc = "Register `ENDPT%s` writer"]
pub type W = crate::W<EndptSpec>;
#[doc = "Field `EPHSHK` reader - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
pub type EphshkR = crate::BitReader;
#[doc = "Field `EPHSHK` writer - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
pub type EphshkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPSTALL` reader - When set this bit indicates that the endpoint is stalled"]
pub type EpstallR = crate::BitReader;
#[doc = "Field `EPSTALL` writer - When set this bit indicates that the endpoint is stalled"]
pub type EpstallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPTXEN` reader - This bit, when set, enables the endpoint for TX transfers. See"]
pub type EptxenR = crate::BitReader;
#[doc = "Field `EPTXEN` writer - This bit, when set, enables the endpoint for TX transfers. See"]
pub type EptxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPRXEN` reader - This bit, when set, enables the endpoint for RX transfers. See"]
pub type EprxenR = crate::BitReader;
#[doc = "Field `EPRXEN` writer - This bit, when set, enables the endpoint for RX transfers. See"]
pub type EprxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPCTLDIS` reader - This bit, when set, disables control (SETUP) transfers"]
pub type EpctldisR = crate::BitReader;
#[doc = "Field `EPCTLDIS` writer - This bit, when set, disables control (SETUP) transfers"]
pub type EpctldisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
    #[inline(always)]
    pub fn ephshk(&self) -> EphshkR {
        EphshkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set this bit indicates that the endpoint is stalled"]
    #[inline(always)]
    pub fn epstall(&self) -> EpstallR {
        EpstallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit, when set, enables the endpoint for TX transfers. See"]
    #[inline(always)]
    pub fn eptxen(&self) -> EptxenR {
        EptxenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit, when set, enables the endpoint for RX transfers. See"]
    #[inline(always)]
    pub fn eprxen(&self) -> EprxenR {
        EprxenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit, when set, disables control (SETUP) transfers"]
    #[inline(always)]
    pub fn epctldis(&self) -> EpctldisR {
        EpctldisR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn ephshk(&mut self) -> EphshkW<EndptSpec> {
        EphshkW::new(self, 0)
    }
    #[doc = "Bit 1 - When set this bit indicates that the endpoint is stalled"]
    #[inline(always)]
    #[must_use]
    pub fn epstall(&mut self) -> EpstallW<EndptSpec> {
        EpstallW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit, when set, enables the endpoint for TX transfers. See"]
    #[inline(always)]
    #[must_use]
    pub fn eptxen(&mut self) -> EptxenW<EndptSpec> {
        EptxenW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit, when set, enables the endpoint for RX transfers. See"]
    #[inline(always)]
    #[must_use]
    pub fn eprxen(&mut self) -> EprxenW<EndptSpec> {
        EprxenW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit, when set, disables control (SETUP) transfers"]
    #[inline(always)]
    #[must_use]
    pub fn epctldis(&mut self) -> EpctldisW<EndptSpec> {
        EpctldisW::new(self, 4)
    }
}
#[doc = "Endpoint Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`endpt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`endpt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EndptSpec;
impl crate::RegisterSpec for EndptSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`endpt::R`](R) reader structure"]
impl crate::Readable for EndptSpec {}
#[doc = "`write(|w| ..)` method takes [`endpt::W`](W) writer structure"]
impl crate::Writable for EndptSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ENDPT%s to value 0"]
impl crate::Resettable for EndptSpec {
    const RESET_VALUE: u8 = 0;
}
