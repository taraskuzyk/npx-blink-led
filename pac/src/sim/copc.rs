#[doc = "Register `COPC` reader"]
pub type R = crate::R<CopcSpec>;
#[doc = "Register `COPC` writer"]
pub type W = crate::W<CopcSpec>;
#[doc = "COP Windowed Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Copw {
    #[doc = "0: Normal mode"]
    B0 = 0,
    #[doc = "1: Windowed mode"]
    B1 = 1,
}
impl From<Copw> for bool {
    #[inline(always)]
    fn from(variant: Copw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COPW` reader - COP Windowed Mode"]
pub type CopwR = crate::BitReader<Copw>;
impl CopwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Copw {
        match self.bits {
            false => Copw::B0,
            true => Copw::B1,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Copw::B0
    }
    #[doc = "Windowed mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Copw::B1
    }
}
#[doc = "Field `COPW` writer - COP Windowed Mode"]
pub type CopwW<'a, REG> = crate::BitWriter<'a, REG, Copw>;
impl<'a, REG> CopwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Copw::B0)
    }
    #[doc = "Windowed mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Copw::B1)
    }
}
#[doc = "COP Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Copclks {
    #[doc = "0: COP configured for short timeout"]
    B0 = 0,
    #[doc = "1: COP configured for long timeout"]
    B1 = 1,
}
impl From<Copclks> for bool {
    #[inline(always)]
    fn from(variant: Copclks) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COPCLKS` reader - COP Clock Select"]
pub type CopclksR = crate::BitReader<Copclks>;
impl CopclksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Copclks {
        match self.bits {
            false => Copclks::B0,
            true => Copclks::B1,
        }
    }
    #[doc = "COP configured for short timeout"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Copclks::B0
    }
    #[doc = "COP configured for long timeout"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Copclks::B1
    }
}
#[doc = "Field `COPCLKS` writer - COP Clock Select"]
pub type CopclksW<'a, REG> = crate::BitWriter<'a, REG, Copclks>;
impl<'a, REG> CopclksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COP configured for short timeout"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Copclks::B0)
    }
    #[doc = "COP configured for long timeout"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Copclks::B1)
    }
}
#[doc = "COP Watchdog Timeout\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Copt {
    #[doc = "0: COP disabled"]
    B00 = 0,
    #[doc = "1: COP timeout after 25 cycles for short timeout or 213 cycles for long timeout"]
    B01 = 1,
    #[doc = "2: COP timeout after 28 cycles for short timeout or 216 cycles for long timeout"]
    B10 = 2,
    #[doc = "3: COP timeout after 210 cycles for short timeout or 218 cycles for long timeout"]
    B11 = 3,
}
impl From<Copt> for u8 {
    #[inline(always)]
    fn from(variant: Copt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Copt {
    type Ux = u8;
}
impl crate::IsEnum for Copt {}
#[doc = "Field `COPT` reader - COP Watchdog Timeout"]
pub type CoptR = crate::FieldReader<Copt>;
impl CoptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Copt {
        match self.bits {
            0 => Copt::B00,
            1 => Copt::B01,
            2 => Copt::B10,
            3 => Copt::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "COP disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Copt::B00
    }
    #[doc = "COP timeout after 25 cycles for short timeout or 213 cycles for long timeout"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Copt::B01
    }
    #[doc = "COP timeout after 28 cycles for short timeout or 216 cycles for long timeout"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Copt::B10
    }
    #[doc = "COP timeout after 210 cycles for short timeout or 218 cycles for long timeout"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Copt::B11
    }
}
#[doc = "Field `COPT` writer - COP Watchdog Timeout"]
pub type CoptW<'a, REG> = crate::FieldWriter<'a, REG, 2, Copt, crate::Safe>;
impl<'a, REG> CoptW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "COP disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Copt::B00)
    }
    #[doc = "COP timeout after 25 cycles for short timeout or 213 cycles for long timeout"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Copt::B01)
    }
    #[doc = "COP timeout after 28 cycles for short timeout or 216 cycles for long timeout"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Copt::B10)
    }
    #[doc = "COP timeout after 210 cycles for short timeout or 218 cycles for long timeout"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Copt::B11)
    }
}
#[doc = "COP Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Copstpen {
    #[doc = "0: COP is disabled and the counter is reset in Stop modes"]
    B0 = 0,
    #[doc = "1: COP is enabled in Stop modes"]
    B1 = 1,
}
impl From<Copstpen> for bool {
    #[inline(always)]
    fn from(variant: Copstpen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COPSTPEN` reader - COP Stop Enable"]
pub type CopstpenR = crate::BitReader<Copstpen>;
impl CopstpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Copstpen {
        match self.bits {
            false => Copstpen::B0,
            true => Copstpen::B1,
        }
    }
    #[doc = "COP is disabled and the counter is reset in Stop modes"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Copstpen::B0
    }
    #[doc = "COP is enabled in Stop modes"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Copstpen::B1
    }
}
#[doc = "Field `COPSTPEN` writer - COP Stop Enable"]
pub type CopstpenW<'a, REG> = crate::BitWriter<'a, REG, Copstpen>;
impl<'a, REG> CopstpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COP is disabled and the counter is reset in Stop modes"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Copstpen::B0)
    }
    #[doc = "COP is enabled in Stop modes"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Copstpen::B1)
    }
}
#[doc = "COP Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Copdbgen {
    #[doc = "0: COP is disabled and the counter is reset in Debug mode"]
    B0 = 0,
    #[doc = "1: COP is enabled in Debug mode"]
    B1 = 1,
}
impl From<Copdbgen> for bool {
    #[inline(always)]
    fn from(variant: Copdbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COPDBGEN` reader - COP Debug Enable"]
pub type CopdbgenR = crate::BitReader<Copdbgen>;
impl CopdbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Copdbgen {
        match self.bits {
            false => Copdbgen::B0,
            true => Copdbgen::B1,
        }
    }
    #[doc = "COP is disabled and the counter is reset in Debug mode"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Copdbgen::B0
    }
    #[doc = "COP is enabled in Debug mode"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Copdbgen::B1
    }
}
#[doc = "Field `COPDBGEN` writer - COP Debug Enable"]
pub type CopdbgenW<'a, REG> = crate::BitWriter<'a, REG, Copdbgen>;
impl<'a, REG> CopdbgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COP is disabled and the counter is reset in Debug mode"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Copdbgen::B0)
    }
    #[doc = "COP is enabled in Debug mode"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Copdbgen::B1)
    }
}
#[doc = "COP Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Copclksel {
    #[doc = "0: LPO clock (1 kHz)"]
    B00 = 0,
    #[doc = "1: MCGIRCLK"]
    B01 = 1,
    #[doc = "2: OSCERCLK"]
    B10 = 2,
    #[doc = "3: Bus clock"]
    B11 = 3,
}
impl From<Copclksel> for u8 {
    #[inline(always)]
    fn from(variant: Copclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Copclksel {
    type Ux = u8;
}
impl crate::IsEnum for Copclksel {}
#[doc = "Field `COPCLKSEL` reader - COP Clock Select"]
pub type CopclkselR = crate::FieldReader<Copclksel>;
impl CopclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Copclksel {
        match self.bits {
            0 => Copclksel::B00,
            1 => Copclksel::B01,
            2 => Copclksel::B10,
            3 => Copclksel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Copclksel::B00
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Copclksel::B01
    }
    #[doc = "OSCERCLK"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Copclksel::B10
    }
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Copclksel::B11
    }
}
#[doc = "Field `COPCLKSEL` writer - COP Clock Select"]
pub type CopclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Copclksel, crate::Safe>;
impl<'a, REG> CopclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Copclksel::B00)
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Copclksel::B01)
    }
    #[doc = "OSCERCLK"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Copclksel::B10)
    }
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Copclksel::B11)
    }
}
impl R {
    #[doc = "Bit 0 - COP Windowed Mode"]
    #[inline(always)]
    pub fn copw(&self) -> CopwR {
        CopwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COP Clock Select"]
    #[inline(always)]
    pub fn copclks(&self) -> CopclksR {
        CopclksR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - COP Watchdog Timeout"]
    #[inline(always)]
    pub fn copt(&self) -> CoptR {
        CoptR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - COP Stop Enable"]
    #[inline(always)]
    pub fn copstpen(&self) -> CopstpenR {
        CopstpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COP Debug Enable"]
    #[inline(always)]
    pub fn copdbgen(&self) -> CopdbgenR {
        CopdbgenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - COP Clock Select"]
    #[inline(always)]
    pub fn copclksel(&self) -> CopclkselR {
        CopclkselR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - COP Windowed Mode"]
    #[inline(always)]
    #[must_use]
    pub fn copw(&mut self) -> CopwW<CopcSpec> {
        CopwW::new(self, 0)
    }
    #[doc = "Bit 1 - COP Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn copclks(&mut self) -> CopclksW<CopcSpec> {
        CopclksW::new(self, 1)
    }
    #[doc = "Bits 2:3 - COP Watchdog Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn copt(&mut self) -> CoptW<CopcSpec> {
        CoptW::new(self, 2)
    }
    #[doc = "Bit 4 - COP Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn copstpen(&mut self) -> CopstpenW<CopcSpec> {
        CopstpenW::new(self, 4)
    }
    #[doc = "Bit 5 - COP Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn copdbgen(&mut self) -> CopdbgenW<CopcSpec> {
        CopdbgenW::new(self, 5)
    }
    #[doc = "Bits 6:7 - COP Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn copclksel(&mut self) -> CopclkselW<CopcSpec> {
        CopclkselW::new(self, 6)
    }
}
#[doc = "COP Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`copc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`copc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CopcSpec;
impl crate::RegisterSpec for CopcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`copc::R`](R) reader structure"]
impl crate::Readable for CopcSpec {}
#[doc = "`write(|w| ..)` method takes [`copc::W`](W) writer structure"]
impl crate::Writable for CopcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COPC to value 0x0c"]
impl crate::Resettable for CopcSpec {
    const RESET_VALUE: u32 = 0x0c;
}
