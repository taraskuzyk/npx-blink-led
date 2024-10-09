#[doc = "Register `CLK_RECOVER_CTRL` reader"]
pub type R = crate::R<ClkRecoverCtrlSpec>;
#[doc = "Register `CLK_RECOVER_CTRL` writer"]
pub type W = crate::W<ClkRecoverCtrlSpec>;
#[doc = "Restart from IFR trim value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RestartIfrtrimEn {
    #[doc = "0: Trim fine adjustment always works based on the previous updated trim fine value (default)"]
    B0 = 0,
    #[doc = "1: Trim fine restarts from the IFR trim value whenever bus_reset/bus_resume is detected or module enable is desasserted"]
    B1 = 1,
}
impl From<RestartIfrtrimEn> for bool {
    #[inline(always)]
    fn from(variant: RestartIfrtrimEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESTART_IFRTRIM_EN` reader - Restart from IFR trim value"]
pub type RestartIfrtrimEnR = crate::BitReader<RestartIfrtrimEn>;
impl RestartIfrtrimEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RestartIfrtrimEn {
        match self.bits {
            false => RestartIfrtrimEn::B0,
            true => RestartIfrtrimEn::B1,
        }
    }
    #[doc = "Trim fine adjustment always works based on the previous updated trim fine value (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == RestartIfrtrimEn::B0
    }
    #[doc = "Trim fine restarts from the IFR trim value whenever bus_reset/bus_resume is detected or module enable is desasserted"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == RestartIfrtrimEn::B1
    }
}
#[doc = "Field `RESTART_IFRTRIM_EN` writer - Restart from IFR trim value"]
pub type RestartIfrtrimEnW<'a, REG> = crate::BitWriter<'a, REG, RestartIfrtrimEn>;
impl<'a, REG> RestartIfrtrimEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trim fine adjustment always works based on the previous updated trim fine value (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(RestartIfrtrimEn::B0)
    }
    #[doc = "Trim fine restarts from the IFR trim value whenever bus_reset/bus_resume is detected or module enable is desasserted"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(RestartIfrtrimEn::B1)
    }
}
#[doc = "Reset/resume to rough phase enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetResumeRoughEn {
    #[doc = "0: Always works in tracking phase after the first time rough to track transition (default)"]
    B0 = 0,
    #[doc = "1: Go back to rough stage whenever bus reset or bus resume occurs"]
    B1 = 1,
}
impl From<ResetResumeRoughEn> for bool {
    #[inline(always)]
    fn from(variant: ResetResumeRoughEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET_RESUME_ROUGH_EN` reader - Reset/resume to rough phase enable"]
pub type ResetResumeRoughEnR = crate::BitReader<ResetResumeRoughEn>;
impl ResetResumeRoughEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResetResumeRoughEn {
        match self.bits {
            false => ResetResumeRoughEn::B0,
            true => ResetResumeRoughEn::B1,
        }
    }
    #[doc = "Always works in tracking phase after the first time rough to track transition (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ResetResumeRoughEn::B0
    }
    #[doc = "Go back to rough stage whenever bus reset or bus resume occurs"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ResetResumeRoughEn::B1
    }
}
#[doc = "Field `RESET_RESUME_ROUGH_EN` writer - Reset/resume to rough phase enable"]
pub type ResetResumeRoughEnW<'a, REG> = crate::BitWriter<'a, REG, ResetResumeRoughEn>;
impl<'a, REG> ResetResumeRoughEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Always works in tracking phase after the first time rough to track transition (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ResetResumeRoughEn::B0)
    }
    #[doc = "Go back to rough stage whenever bus reset or bus resume occurs"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ResetResumeRoughEn::B1)
    }
}
#[doc = "Crystal-less USB enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockRecoverEn {
    #[doc = "0: Disable clock recovery block (default)"]
    B0 = 0,
    #[doc = "1: Enable clock recovery block"]
    B1 = 1,
}
impl From<ClockRecoverEn> for bool {
    #[inline(always)]
    fn from(variant: ClockRecoverEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLOCK_RECOVER_EN` reader - Crystal-less USB enable"]
pub type ClockRecoverEnR = crate::BitReader<ClockRecoverEn>;
impl ClockRecoverEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClockRecoverEn {
        match self.bits {
            false => ClockRecoverEn::B0,
            true => ClockRecoverEn::B1,
        }
    }
    #[doc = "Disable clock recovery block (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == ClockRecoverEn::B0
    }
    #[doc = "Enable clock recovery block"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == ClockRecoverEn::B1
    }
}
#[doc = "Field `CLOCK_RECOVER_EN` writer - Crystal-less USB enable"]
pub type ClockRecoverEnW<'a, REG> = crate::BitWriter<'a, REG, ClockRecoverEn>;
impl<'a, REG> ClockRecoverEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable clock recovery block (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(ClockRecoverEn::B0)
    }
    #[doc = "Enable clock recovery block"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(ClockRecoverEn::B1)
    }
}
impl R {
    #[doc = "Bit 5 - Restart from IFR trim value"]
    #[inline(always)]
    pub fn restart_ifrtrim_en(&self) -> RestartIfrtrimEnR {
        RestartIfrtrimEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset/resume to rough phase enable"]
    #[inline(always)]
    pub fn reset_resume_rough_en(&self) -> ResetResumeRoughEnR {
        ResetResumeRoughEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Crystal-less USB enable"]
    #[inline(always)]
    pub fn clock_recover_en(&self) -> ClockRecoverEnR {
        ClockRecoverEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Restart from IFR trim value"]
    #[inline(always)]
    #[must_use]
    pub fn restart_ifrtrim_en(&mut self) -> RestartIfrtrimEnW<ClkRecoverCtrlSpec> {
        RestartIfrtrimEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset/resume to rough phase enable"]
    #[inline(always)]
    #[must_use]
    pub fn reset_resume_rough_en(&mut self) -> ResetResumeRoughEnW<ClkRecoverCtrlSpec> {
        ResetResumeRoughEnW::new(self, 6)
    }
    #[doc = "Bit 7 - Crystal-less USB enable"]
    #[inline(always)]
    #[must_use]
    pub fn clock_recover_en(&mut self) -> ClockRecoverEnW<ClkRecoverCtrlSpec> {
        ClockRecoverEnW::new(self, 7)
    }
}
#[doc = "USB Clock recovery control\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_recover_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_recover_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkRecoverCtrlSpec;
impl crate::RegisterSpec for ClkRecoverCtrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clk_recover_ctrl::R`](R) reader structure"]
impl crate::Readable for ClkRecoverCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_recover_ctrl::W`](W) writer structure"]
impl crate::Writable for ClkRecoverCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CLK_RECOVER_CTRL to value 0"]
impl crate::Resettable for ClkRecoverCtrlSpec {
    const RESET_VALUE: u8 = 0;
}
