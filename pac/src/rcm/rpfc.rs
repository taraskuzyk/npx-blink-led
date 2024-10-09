#[doc = "Register `RPFC` reader"]
pub type R = crate::R<RpfcSpec>;
#[doc = "Register `RPFC` writer"]
pub type W = crate::W<RpfcSpec>;
#[doc = "Reset Pin Filter Select in Run and Wait Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rstfltsrw {
    #[doc = "0: All filtering disabled"]
    B00 = 0,
    #[doc = "1: Bus clock filter enabled for normal operation"]
    B01 = 1,
    #[doc = "2: LPO clock filter enabled for normal operation"]
    B10 = 2,
}
impl From<Rstfltsrw> for u8 {
    #[inline(always)]
    fn from(variant: Rstfltsrw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rstfltsrw {
    type Ux = u8;
}
impl crate::IsEnum for Rstfltsrw {}
#[doc = "Field `RSTFLTSRW` reader - Reset Pin Filter Select in Run and Wait Modes"]
pub type RstfltsrwR = crate::FieldReader<Rstfltsrw>;
impl RstfltsrwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Rstfltsrw> {
        match self.bits {
            0 => Some(Rstfltsrw::B00),
            1 => Some(Rstfltsrw::B01),
            2 => Some(Rstfltsrw::B10),
            _ => None,
        }
    }
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Rstfltsrw::B00
    }
    #[doc = "Bus clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Rstfltsrw::B01
    }
    #[doc = "LPO clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Rstfltsrw::B10
    }
}
#[doc = "Field `RSTFLTSRW` writer - Reset Pin Filter Select in Run and Wait Modes"]
pub type RstfltsrwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rstfltsrw>;
impl<'a, REG> RstfltsrwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsrw::B00)
    }
    #[doc = "Bus clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsrw::B01)
    }
    #[doc = "LPO clock filter enabled for normal operation"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsrw::B10)
    }
}
#[doc = "Reset Pin Filter Select in Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rstfltss {
    #[doc = "0: All filtering disabled"]
    B0 = 0,
    #[doc = "1: LPO clock filter enabled"]
    B1 = 1,
}
impl From<Rstfltss> for bool {
    #[inline(always)]
    fn from(variant: Rstfltss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTFLTSS` reader - Reset Pin Filter Select in Stop Mode"]
pub type RstfltssR = crate::BitReader<Rstfltss>;
impl RstfltssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstfltss {
        match self.bits {
            false => Rstfltss::B0,
            true => Rstfltss::B1,
        }
    }
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rstfltss::B0
    }
    #[doc = "LPO clock filter enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rstfltss::B1
    }
}
#[doc = "Field `RSTFLTSS` writer - Reset Pin Filter Select in Stop Mode"]
pub type RstfltssW<'a, REG> = crate::BitWriter<'a, REG, Rstfltss>;
impl<'a, REG> RstfltssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All filtering disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltss::B0)
    }
    #[doc = "LPO clock filter enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltss::B1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Reset Pin Filter Select in Run and Wait Modes"]
    #[inline(always)]
    pub fn rstfltsrw(&self) -> RstfltsrwR {
        RstfltsrwR::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Reset Pin Filter Select in Stop Mode"]
    #[inline(always)]
    pub fn rstfltss(&self) -> RstfltssR {
        RstfltssR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reset Pin Filter Select in Run and Wait Modes"]
    #[inline(always)]
    #[must_use]
    pub fn rstfltsrw(&mut self) -> RstfltsrwW<RpfcSpec> {
        RstfltsrwW::new(self, 0)
    }
    #[doc = "Bit 2 - Reset Pin Filter Select in Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn rstfltss(&mut self) -> RstfltssW<RpfcSpec> {
        RstfltssW::new(self, 2)
    }
}
#[doc = "Reset Pin Filter Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RpfcSpec;
impl crate::RegisterSpec for RpfcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rpfc::R`](R) reader structure"]
impl crate::Readable for RpfcSpec {}
#[doc = "`write(|w| ..)` method takes [`rpfc::W`](W) writer structure"]
impl crate::Writable for RpfcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RPFC to value 0"]
impl crate::Resettable for RpfcSpec {
    const RESET_VALUE: u8 = 0;
}
