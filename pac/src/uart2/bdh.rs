#[doc = "Register `BDH` reader"]
pub type R = crate::R<BdhSpec>;
#[doc = "Register `BDH` writer"]
pub type W = crate::W<BdhSpec>;
#[doc = "Field `SBR` reader - UART Baud Rate Bits"]
pub type SbrR = crate::FieldReader;
#[doc = "Field `SBR` writer - UART Baud Rate Bits"]
pub type SbrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "RxD Input Active Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxedgie {
    #[doc = "0: Hardware interrupts from RXEDGIF disabled using polling."]
    B0 = 0,
    #[doc = "1: RXEDGIF interrupt request enabled."]
    B1 = 1,
}
impl From<Rxedgie> for bool {
    #[inline(always)]
    fn from(variant: Rxedgie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEDGIE` reader - RxD Input Active Edge Interrupt Enable"]
pub type RxedgieR = crate::BitReader<Rxedgie>;
impl RxedgieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxedgie {
        match self.bits {
            false => Rxedgie::B0,
            true => Rxedgie::B1,
        }
    }
    #[doc = "Hardware interrupts from RXEDGIF disabled using polling."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxedgie::B0
    }
    #[doc = "RXEDGIF interrupt request enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxedgie::B1
    }
}
#[doc = "Field `RXEDGIE` writer - RxD Input Active Edge Interrupt Enable"]
pub type RxedgieW<'a, REG> = crate::BitWriter<'a, REG, Rxedgie>;
impl<'a, REG> RxedgieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Hardware interrupts from RXEDGIF disabled using polling."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedgie::B0)
    }
    #[doc = "RXEDGIF interrupt request enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedgie::B1)
    }
}
impl R {
    #[doc = "Bits 0:4 - UART Baud Rate Bits"]
    #[inline(always)]
    pub fn sbr(&self) -> SbrR {
        SbrR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - RxD Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub fn rxedgie(&self) -> RxedgieR {
        RxedgieR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - UART Baud Rate Bits"]
    #[inline(always)]
    #[must_use]
    pub fn sbr(&mut self) -> SbrW<BdhSpec> {
        SbrW::new(self, 0)
    }
    #[doc = "Bit 6 - RxD Input Active Edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxedgie(&mut self) -> RxedgieW<BdhSpec> {
        RxedgieW::new(self, 6)
    }
}
#[doc = "UART Baud Rate Registers: High\n\nYou can [`read`](crate::Reg::read) this register and get [`bdh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdhSpec;
impl crate::RegisterSpec for BdhSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bdh::R`](R) reader structure"]
impl crate::Readable for BdhSpec {}
#[doc = "`write(|w| ..)` method takes [`bdh::W`](W) writer structure"]
impl crate::Writable for BdhSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BDH to value 0"]
impl crate::Resettable for BdhSpec {
    const RESET_VALUE: u8 = 0;
}
