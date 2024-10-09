#[doc = "Register `TBCTRL` reader"]
pub type R = crate::R<TbctrlSpec>;
#[doc = "Register `TBCTRL` writer"]
pub type W = crate::W<TbctrlSpec>;
#[doc = "Action based on Comparator 0 match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acomp0 {
    #[doc = "0: Trigger TSTOP based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    B0 = 0,
    #[doc = "1: Trigger TSTART based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    B1 = 1,
}
impl From<Acomp0> for bool {
    #[inline(always)]
    fn from(variant: Acomp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACOMP0` reader - Action based on Comparator 0 match"]
pub type Acomp0R = crate::BitReader<Acomp0>;
impl Acomp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acomp0 {
        match self.bits {
            false => Acomp0::B0,
            true => Acomp0::B1,
        }
    }
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acomp0::B0
    }
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acomp0::B1
    }
}
#[doc = "Field `ACOMP0` writer - Action based on Comparator 0 match"]
pub type Acomp0W<'a, REG> = crate::BitWriter<'a, REG, Acomp0>;
impl<'a, REG> Acomp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Acomp0::B0)
    }
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT0\\[MATCHED\\]."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Acomp0::B1)
    }
}
#[doc = "Action based on Comparator 1 match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acomp1 {
    #[doc = "0: Trigger TSTOP based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    B0 = 0,
    #[doc = "1: Trigger TSTART based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    B1 = 1,
}
impl From<Acomp1> for bool {
    #[inline(always)]
    fn from(variant: Acomp1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACOMP1` reader - Action based on Comparator 1 match"]
pub type Acomp1R = crate::BitReader<Acomp1>;
impl Acomp1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acomp1 {
        match self.bits {
            false => Acomp1::B0,
            true => Acomp1::B1,
        }
    }
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acomp1::B0
    }
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acomp1::B1
    }
}
#[doc = "Field `ACOMP1` writer - Action based on Comparator 1 match"]
pub type Acomp1W<'a, REG> = crate::BitWriter<'a, REG, Acomp1>;
impl<'a, REG> Acomp1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger TSTOP based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Acomp1::B0)
    }
    #[doc = "Trigger TSTART based on the assertion of MTBDWT_FCT1\\[MATCHED\\]."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Acomp1::B1)
    }
}
#[doc = "Field `NUMCOMP` reader - Number of Comparators"]
pub type NumcompR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Action based on Comparator 0 match"]
    #[inline(always)]
    pub fn acomp0(&self) -> Acomp0R {
        Acomp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Action based on Comparator 1 match"]
    #[inline(always)]
    pub fn acomp1(&self) -> Acomp1R {
        Acomp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Number of Comparators"]
    #[inline(always)]
    pub fn numcomp(&self) -> NumcompR {
        NumcompR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Action based on Comparator 0 match"]
    #[inline(always)]
    #[must_use]
    pub fn acomp0(&mut self) -> Acomp0W<TbctrlSpec> {
        Acomp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Action based on Comparator 1 match"]
    #[inline(always)]
    #[must_use]
    pub fn acomp1(&mut self) -> Acomp1W<TbctrlSpec> {
        Acomp1W::new(self, 1)
    }
}
#[doc = "MTB_DWT Trace Buffer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbctrlSpec;
impl crate::RegisterSpec for TbctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbctrl::R`](R) reader structure"]
impl crate::Readable for TbctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tbctrl::W`](W) writer structure"]
impl crate::Writable for TbctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBCTRL to value 0x2000_0000"]
impl crate::Resettable for TbctrlSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
