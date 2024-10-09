#[doc = "Register `SCGC7` reader"]
pub type R = crate::R<Scgc7Spec>;
#[doc = "Register `SCGC7` writer"]
pub type W = crate::W<Scgc7Spec>;
#[doc = "DMA Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dma {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Dma> for bool {
    #[inline(always)]
    fn from(variant: Dma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA` reader - DMA Clock Gate Control"]
pub type DmaR = crate::BitReader<Dma>;
impl DmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dma {
        match self.bits {
            false => Dma::B0,
            true => Dma::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dma::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dma::B1
    }
}
#[doc = "Field `DMA` writer - DMA Clock Gate Control"]
pub type DmaW<'a, REG> = crate::BitWriter<'a, REG, Dma>;
impl<'a, REG> DmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dma::B1)
    }
}
impl R {
    #[doc = "Bit 8 - DMA Clock Gate Control"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - DMA Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DmaW<Scgc7Spec> {
        DmaW::new(self, 8)
    }
}
#[doc = "System Clock Gating Control Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc7Spec;
impl crate::RegisterSpec for Scgc7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc7::R`](R) reader structure"]
impl crate::Readable for Scgc7Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc7::W`](W) writer structure"]
impl crate::Writable for Scgc7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGC7 to value 0x0100"]
impl crate::Resettable for Scgc7Spec {
    const RESET_VALUE: u32 = 0x0100;
}
