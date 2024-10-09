#[doc = "Register `PMPROT` reader"]
pub type R = crate::R<PmprotSpec>;
#[doc = "Register `PMPROT` writer"]
pub type W = crate::W<PmprotSpec>;
#[doc = "Allow Very-Low-Leakage Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avlls {
    #[doc = "0: Any VLLSx mode is not allowed"]
    B0 = 0,
    #[doc = "1: Any VLLSx mode is allowed"]
    B1 = 1,
}
impl From<Avlls> for bool {
    #[inline(always)]
    fn from(variant: Avlls) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVLLS` reader - Allow Very-Low-Leakage Stop Mode"]
pub type AvllsR = crate::BitReader<Avlls>;
impl AvllsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avlls {
        match self.bits {
            false => Avlls::B0,
            true => Avlls::B1,
        }
    }
    #[doc = "Any VLLSx mode is not allowed"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Avlls::B0
    }
    #[doc = "Any VLLSx mode is allowed"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Avlls::B1
    }
}
#[doc = "Field `AVLLS` writer - Allow Very-Low-Leakage Stop Mode"]
pub type AvllsW<'a, REG> = crate::BitWriter<'a, REG, Avlls>;
impl<'a, REG> AvllsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Any VLLSx mode is not allowed"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Avlls::B0)
    }
    #[doc = "Any VLLSx mode is allowed"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Avlls::B1)
    }
}
#[doc = "Allow Low-Leakage Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alls {
    #[doc = "0: LLS is not allowed"]
    B0 = 0,
    #[doc = "1: LLS is allowed"]
    B1 = 1,
}
impl From<Alls> for bool {
    #[inline(always)]
    fn from(variant: Alls) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALLS` reader - Allow Low-Leakage Stop Mode"]
pub type AllsR = crate::BitReader<Alls>;
impl AllsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alls {
        match self.bits {
            false => Alls::B0,
            true => Alls::B1,
        }
    }
    #[doc = "LLS is not allowed"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Alls::B0
    }
    #[doc = "LLS is allowed"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Alls::B1
    }
}
#[doc = "Field `ALLS` writer - Allow Low-Leakage Stop Mode"]
pub type AllsW<'a, REG> = crate::BitWriter<'a, REG, Alls>;
impl<'a, REG> AllsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LLS is not allowed"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Alls::B0)
    }
    #[doc = "LLS is allowed"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Alls::B1)
    }
}
#[doc = "Allow Very-Low-Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Avlp {
    #[doc = "0: VLPR, VLPW, and VLPS are not allowed."]
    B0 = 0,
    #[doc = "1: VLPR, VLPW, and VLPS are allowed."]
    B1 = 1,
}
impl From<Avlp> for bool {
    #[inline(always)]
    fn from(variant: Avlp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AVLP` reader - Allow Very-Low-Power Modes"]
pub type AvlpR = crate::BitReader<Avlp>;
impl AvlpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Avlp {
        match self.bits {
            false => Avlp::B0,
            true => Avlp::B1,
        }
    }
    #[doc = "VLPR, VLPW, and VLPS are not allowed."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Avlp::B0
    }
    #[doc = "VLPR, VLPW, and VLPS are allowed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Avlp::B1
    }
}
#[doc = "Field `AVLP` writer - Allow Very-Low-Power Modes"]
pub type AvlpW<'a, REG> = crate::BitWriter<'a, REG, Avlp>;
impl<'a, REG> AvlpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VLPR, VLPW, and VLPS are not allowed."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Avlp::B0)
    }
    #[doc = "VLPR, VLPW, and VLPS are allowed."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Avlp::B1)
    }
}
impl R {
    #[doc = "Bit 1 - Allow Very-Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn avlls(&self) -> AvllsR {
        AvllsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Allow Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn alls(&self) -> AllsR {
        AllsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    pub fn avlp(&self) -> AvlpR {
        AvlpR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Allow Very-Low-Leakage Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn avlls(&mut self) -> AvllsW<PmprotSpec> {
        AvllsW::new(self, 1)
    }
    #[doc = "Bit 3 - Allow Low-Leakage Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn alls(&mut self) -> AllsW<PmprotSpec> {
        AllsW::new(self, 3)
    }
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    #[must_use]
    pub fn avlp(&mut self) -> AvlpW<PmprotSpec> {
        AvlpW::new(self, 5)
    }
}
#[doc = "Power Mode Protection register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmprot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmprot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmprotSpec;
impl crate::RegisterSpec for PmprotSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmprot::R`](R) reader structure"]
impl crate::Readable for PmprotSpec {}
#[doc = "`write(|w| ..)` method takes [`pmprot::W`](W) writer structure"]
impl crate::Writable for PmprotSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PMPROT to value 0"]
impl crate::Resettable for PmprotSpec {
    const RESET_VALUE: u8 = 0;
}
