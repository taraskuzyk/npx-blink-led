#[doc = "Register `DSR_BCR2` reader"]
pub type R = crate::R<DsrBcr2Spec>;
#[doc = "Register `DSR_BCR2` writer"]
pub type W = crate::W<DsrBcr2Spec>;
#[doc = "Field `BCR` reader - BCR"]
pub type BcrR = crate::FieldReader<u32>;
#[doc = "Field `BCR` writer - BCR"]
pub type BcrW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Transactions Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Done {
    #[doc = "0: DMA transfer is not yet complete. Writing a 0 has no effect."]
    B0 = 0,
    #[doc = "1: DMA transfer completed. Writing a 1 to this bit clears all DMA status bits and should be used in an interrupt service routine to clear the DMA interrupt and error bits."]
    B1 = 1,
}
impl From<Done> for bool {
    #[inline(always)]
    fn from(variant: Done) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DONE` reader - Transactions Done"]
pub type DoneR = crate::BitReader<Done>;
impl DoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Done {
        match self.bits {
            false => Done::B0,
            true => Done::B1,
        }
    }
    #[doc = "DMA transfer is not yet complete. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Done::B0
    }
    #[doc = "DMA transfer completed. Writing a 1 to this bit clears all DMA status bits and should be used in an interrupt service routine to clear the DMA interrupt and error bits."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Done::B1
    }
}
#[doc = "Field `DONE` writer - Transactions Done"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG, Done>;
impl<'a, REG> DoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA transfer is not yet complete. Writing a 0 has no effect."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Done::B0)
    }
    #[doc = "DMA transfer completed. Writing a 1 to this bit clears all DMA status bits and should be used in an interrupt service routine to clear the DMA interrupt and error bits."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Done::B1)
    }
}
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bsy {
    #[doc = "0: DMA channel is inactive. Cleared when the DMA has finished the last transaction."]
    B0 = 0,
    #[doc = "1: BSY is set the first time the channel is enabled after a transfer is initiated."]
    B1 = 1,
}
impl From<Bsy> for bool {
    #[inline(always)]
    fn from(variant: Bsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BSY` reader - Busy"]
pub type BsyR = crate::BitReader<Bsy>;
impl BsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bsy {
        match self.bits {
            false => Bsy::B0,
            true => Bsy::B1,
        }
    }
    #[doc = "DMA channel is inactive. Cleared when the DMA has finished the last transaction."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bsy::B0
    }
    #[doc = "BSY is set the first time the channel is enabled after a transfer is initiated."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bsy::B1
    }
}
#[doc = "Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Req {
    #[doc = "0: No request is pending or the channel is currently active. Cleared when the channel is selected."]
    B0 = 0,
    #[doc = "1: The DMA channel has a transfer remaining and the channel is not selected."]
    B1 = 1,
}
impl From<Req> for bool {
    #[inline(always)]
    fn from(variant: Req) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQ` reader - Request"]
pub type ReqR = crate::BitReader<Req>;
impl ReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Req {
        match self.bits {
            false => Req::B0,
            true => Req::B1,
        }
    }
    #[doc = "No request is pending or the channel is currently active. Cleared when the channel is selected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Req::B0
    }
    #[doc = "The DMA channel has a transfer remaining and the channel is not selected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Req::B1
    }
}
#[doc = "Bus Error on Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bed {
    #[doc = "0: No bus error occurred."]
    B0 = 0,
    #[doc = "1: The DMA channel terminated with a bus error during the write portion of a transfer."]
    B1 = 1,
}
impl From<Bed> for bool {
    #[inline(always)]
    fn from(variant: Bed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BED` reader - Bus Error on Destination"]
pub type BedR = crate::BitReader<Bed>;
impl BedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bed {
        match self.bits {
            false => Bed::B0,
            true => Bed::B1,
        }
    }
    #[doc = "No bus error occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bed::B0
    }
    #[doc = "The DMA channel terminated with a bus error during the write portion of a transfer."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bed::B1
    }
}
#[doc = "Bus Error on Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bes {
    #[doc = "0: No bus error occurred."]
    B0 = 0,
    #[doc = "1: The DMA channel terminated with a bus error during the read portion of a transfer."]
    B1 = 1,
}
impl From<Bes> for bool {
    #[inline(always)]
    fn from(variant: Bes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BES` reader - Bus Error on Source"]
pub type BesR = crate::BitReader<Bes>;
impl BesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bes {
        match self.bits {
            false => Bes::B0,
            true => Bes::B1,
        }
    }
    #[doc = "No bus error occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bes::B0
    }
    #[doc = "The DMA channel terminated with a bus error during the read portion of a transfer."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bes::B1
    }
}
#[doc = "Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ce {
    #[doc = "0: No configuration error exists."]
    B0 = 0,
    #[doc = "1: A configuration error has occurred."]
    B1 = 1,
}
impl From<Ce> for bool {
    #[inline(always)]
    fn from(variant: Ce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CE` reader - Configuration Error"]
pub type CeR = crate::BitReader<Ce>;
impl CeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ce {
        match self.bits {
            false => Ce::B0,
            true => Ce::B1,
        }
    }
    #[doc = "No configuration error exists."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ce::B0
    }
    #[doc = "A configuration error has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ce::B1
    }
}
impl R {
    #[doc = "Bits 0:23 - BCR"]
    #[inline(always)]
    pub fn bcr(&self) -> BcrR {
        BcrR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Transactions Done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Busy"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Request"]
    #[inline(always)]
    pub fn req(&self) -> ReqR {
        ReqR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Bus Error on Destination"]
    #[inline(always)]
    pub fn bed(&self) -> BedR {
        BedR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Bus Error on Source"]
    #[inline(always)]
    pub fn bes(&self) -> BesR {
        BesR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Configuration Error"]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - BCR"]
    #[inline(always)]
    #[must_use]
    pub fn bcr(&mut self) -> BcrW<DsrBcr2Spec> {
        BcrW::new(self, 0)
    }
    #[doc = "Bit 24 - Transactions Done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<DsrBcr2Spec> {
        DoneW::new(self, 24)
    }
}
#[doc = "DMA Status Register / Byte Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr_bcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr_bcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsrBcr2Spec;
impl crate::RegisterSpec for DsrBcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr_bcr2::R`](R) reader structure"]
impl crate::Readable for DsrBcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dsr_bcr2::W`](W) writer structure"]
impl crate::Writable for DsrBcr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DSR_BCR2 to value 0"]
impl crate::Resettable for DsrBcr2Spec {
    const RESET_VALUE: u32 = 0;
}
