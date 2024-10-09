#[doc = "Register `ET7816` reader"]
pub type R = crate::R<Et7816Spec>;
#[doc = "Register `ET7816` writer"]
pub type W = crate::W<Et7816Spec>;
#[doc = "Field `RXTHRESHOLD` reader - Receive NACK Threshold"]
pub type RxthresholdR = crate::FieldReader;
#[doc = "Field `RXTHRESHOLD` writer - Receive NACK Threshold"]
pub type RxthresholdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Transmit NACK Threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txthreshold {
    #[doc = "0: TXT asserts on the first NACK that is received."]
    B0000 = 0,
    #[doc = "1: TXT asserts on the second NACK that is received."]
    B0001 = 1,
}
impl From<Txthreshold> for u8 {
    #[inline(always)]
    fn from(variant: Txthreshold) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txthreshold {
    type Ux = u8;
}
impl crate::IsEnum for Txthreshold {}
#[doc = "Field `TXTHRESHOLD` reader - Transmit NACK Threshold"]
pub type TxthresholdR = crate::FieldReader<Txthreshold>;
impl TxthresholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txthreshold> {
        match self.bits {
            0 => Some(Txthreshold::B0000),
            1 => Some(Txthreshold::B0001),
            _ => None,
        }
    }
    #[doc = "TXT asserts on the first NACK that is received."]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Txthreshold::B0000
    }
    #[doc = "TXT asserts on the second NACK that is received."]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Txthreshold::B0001
    }
}
#[doc = "Field `TXTHRESHOLD` writer - Transmit NACK Threshold"]
pub type TxthresholdW<'a, REG> = crate::FieldWriter<'a, REG, 4, Txthreshold>;
impl<'a, REG> TxthresholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TXT asserts on the first NACK that is received."]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Txthreshold::B0000)
    }
    #[doc = "TXT asserts on the second NACK that is received."]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Txthreshold::B0001)
    }
}
impl R {
    #[doc = "Bits 0:3 - Receive NACK Threshold"]
    #[inline(always)]
    pub fn rxthreshold(&self) -> RxthresholdR {
        RxthresholdR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Transmit NACK Threshold"]
    #[inline(always)]
    pub fn txthreshold(&self) -> TxthresholdR {
        TxthresholdR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Receive NACK Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rxthreshold(&mut self) -> RxthresholdW<Et7816Spec> {
        RxthresholdW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Transmit NACK Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn txthreshold(&mut self) -> TxthresholdW<Et7816Spec> {
        TxthresholdW::new(self, 4)
    }
}
#[doc = "UART 7816 Error Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`et7816::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`et7816::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Et7816Spec;
impl crate::RegisterSpec for Et7816Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`et7816::R`](R) reader structure"]
impl crate::Readable for Et7816Spec {}
#[doc = "`write(|w| ..)` method takes [`et7816::W`](W) writer structure"]
impl crate::Writable for Et7816Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ET7816 to value 0"]
impl crate::Resettable for Et7816Spec {
    const RESET_VALUE: u8 = 0;
}
