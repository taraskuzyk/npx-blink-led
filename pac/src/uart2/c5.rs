#[doc = "Register `C5` reader"]
pub type R = crate::R<C5Spec>;
#[doc = "Register `C5` writer"]
pub type W = crate::W<C5Spec>;
#[doc = "Receiver Full DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdmas {
    #[doc = "0: If C2\\[RIE\\]
and S1\\[RDRF\\]
are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    B0 = 0,
    #[doc = "1: If C2\\[RIE\\]
and S1\\[RDRF\\]
are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    B1 = 1,
}
impl From<Rdmas> for bool {
    #[inline(always)]
    fn from(variant: Rdmas) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDMAS` reader - Receiver Full DMA Select"]
pub type RdmasR = crate::BitReader<Rdmas>;
impl RdmasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdmas {
        match self.bits {
            false => Rdmas::B0,
            true => Rdmas::B1,
        }
    }
    #[doc = "If C2\\[RIE\\]
and S1\\[RDRF\\]
are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rdmas::B0
    }
    #[doc = "If C2\\[RIE\\]
and S1\\[RDRF\\]
are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rdmas::B1
    }
}
#[doc = "Field `RDMAS` writer - Receiver Full DMA Select"]
pub type RdmasW<'a, REG> = crate::BitWriter<'a, REG, Rdmas>;
impl<'a, REG> RdmasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If C2\\[RIE\\]
and S1\\[RDRF\\]
are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rdmas::B0)
    }
    #[doc = "If C2\\[RIE\\]
and S1\\[RDRF\\]
are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rdmas::B1)
    }
}
#[doc = "Transmitter DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdmas {
    #[doc = "0: If C2\\[TIE\\]
is set and the S1\\[TDRE\\]
flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    B0 = 0,
    #[doc = "1: If C2\\[TIE\\]
is set and the S1\\[TDRE\\]
flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    B1 = 1,
}
impl From<Tdmas> for bool {
    #[inline(always)]
    fn from(variant: Tdmas) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDMAS` reader - Transmitter DMA Select"]
pub type TdmasR = crate::BitReader<Tdmas>;
impl TdmasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdmas {
        match self.bits {
            false => Tdmas::B0,
            true => Tdmas::B1,
        }
    }
    #[doc = "If C2\\[TIE\\]
is set and the S1\\[TDRE\\]
flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tdmas::B0
    }
    #[doc = "If C2\\[TIE\\]
is set and the S1\\[TDRE\\]
flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tdmas::B1
    }
}
#[doc = "Field `TDMAS` writer - Transmitter DMA Select"]
pub type TdmasW<'a, REG> = crate::BitWriter<'a, REG, Tdmas>;
impl<'a, REG> TdmasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "If C2\\[TIE\\]
is set and the S1\\[TDRE\\]
flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmas::B0)
    }
    #[doc = "If C2\\[TIE\\]
is set and the S1\\[TDRE\\]
flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tdmas::B1)
    }
}
impl R {
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline(always)]
    pub fn rdmas(&self) -> RdmasR {
        RdmasR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline(always)]
    pub fn tdmas(&self) -> TdmasR {
        TdmasR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn rdmas(&mut self) -> RdmasW<C5Spec> {
        RdmasW::new(self, 5)
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn tdmas(&mut self) -> TdmasW<C5Spec> {
        TdmasW::new(self, 7)
    }
}
#[doc = "UART Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`c5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C5Spec;
impl crate::RegisterSpec for C5Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c5::R`](R) reader structure"]
impl crate::Readable for C5Spec {}
#[doc = "`write(|w| ..)` method takes [`c5::W`](W) writer structure"]
impl crate::Writable for C5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C5 to value 0"]
impl crate::Resettable for C5Spec {
    const RESET_VALUE: u8 = 0;
}
