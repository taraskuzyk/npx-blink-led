#[doc = "Register `LR` reader"]
pub type R = crate::R<LrSpec>;
#[doc = "Register `LR` writer"]
pub type W = crate::W<LrSpec>;
#[doc = "Time Compensation Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcl {
    #[doc = "0: Time Compensation Register is locked and writes are ignored."]
    B0 = 0,
    #[doc = "1: Time Compensation Register is not locked and writes complete as normal."]
    B1 = 1,
}
impl From<Tcl> for bool {
    #[inline(always)]
    fn from(variant: Tcl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCL` reader - Time Compensation Lock"]
pub type TclR = crate::BitReader<Tcl>;
impl TclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcl {
        match self.bits {
            false => Tcl::B0,
            true => Tcl::B1,
        }
    }
    #[doc = "Time Compensation Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tcl::B0
    }
    #[doc = "Time Compensation Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tcl::B1
    }
}
#[doc = "Field `TCL` writer - Time Compensation Lock"]
pub type TclW<'a, REG> = crate::BitWriter<'a, REG, Tcl>;
impl<'a, REG> TclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Time Compensation Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tcl::B0)
    }
    #[doc = "Time Compensation Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tcl::B1)
    }
}
#[doc = "Control Register Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crl {
    #[doc = "0: Control Register is locked and writes are ignored."]
    B0 = 0,
    #[doc = "1: Control Register is not locked and writes complete as normal."]
    B1 = 1,
}
impl From<Crl> for bool {
    #[inline(always)]
    fn from(variant: Crl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRL` reader - Control Register Lock"]
pub type CrlR = crate::BitReader<Crl>;
impl CrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crl {
        match self.bits {
            false => Crl::B0,
            true => Crl::B1,
        }
    }
    #[doc = "Control Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Crl::B0
    }
    #[doc = "Control Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Crl::B1
    }
}
#[doc = "Field `CRL` writer - Control Register Lock"]
pub type CrlW<'a, REG> = crate::BitWriter<'a, REG, Crl>;
impl<'a, REG> CrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Control Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Crl::B0)
    }
    #[doc = "Control Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Crl::B1)
    }
}
#[doc = "Status Register Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srl {
    #[doc = "0: Status Register is locked and writes are ignored."]
    B0 = 0,
    #[doc = "1: Status Register is not locked and writes complete as normal."]
    B1 = 1,
}
impl From<Srl> for bool {
    #[inline(always)]
    fn from(variant: Srl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRL` reader - Status Register Lock"]
pub type SrlR = crate::BitReader<Srl>;
impl SrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srl {
        match self.bits {
            false => Srl::B0,
            true => Srl::B1,
        }
    }
    #[doc = "Status Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Srl::B0
    }
    #[doc = "Status Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Srl::B1
    }
}
#[doc = "Field `SRL` writer - Status Register Lock"]
pub type SrlW<'a, REG> = crate::BitWriter<'a, REG, Srl>;
impl<'a, REG> SrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Status Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Srl::B0)
    }
    #[doc = "Status Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Srl::B1)
    }
}
#[doc = "Lock Register Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lrl {
    #[doc = "0: Lock Register is locked and writes are ignored."]
    B0 = 0,
    #[doc = "1: Lock Register is not locked and writes complete as normal."]
    B1 = 1,
}
impl From<Lrl> for bool {
    #[inline(always)]
    fn from(variant: Lrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRL` reader - Lock Register Lock"]
pub type LrlR = crate::BitReader<Lrl>;
impl LrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lrl {
        match self.bits {
            false => Lrl::B0,
            true => Lrl::B1,
        }
    }
    #[doc = "Lock Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Lrl::B0
    }
    #[doc = "Lock Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Lrl::B1
    }
}
#[doc = "Field `LRL` writer - Lock Register Lock"]
pub type LrlW<'a, REG> = crate::BitWriter<'a, REG, Lrl>;
impl<'a, REG> LrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Lock Register is locked and writes are ignored."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Lrl::B0)
    }
    #[doc = "Lock Register is not locked and writes complete as normal."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Lrl::B1)
    }
}
impl R {
    #[doc = "Bit 3 - Time Compensation Lock"]
    #[inline(always)]
    pub fn tcl(&self) -> TclR {
        TclR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control Register Lock"]
    #[inline(always)]
    pub fn crl(&self) -> CrlR {
        CrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Register Lock"]
    #[inline(always)]
    pub fn srl(&self) -> SrlR {
        SrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock Register Lock"]
    #[inline(always)]
    pub fn lrl(&self) -> LrlR {
        LrlR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Time Compensation Lock"]
    #[inline(always)]
    #[must_use]
    pub fn tcl(&mut self) -> TclW<LrSpec> {
        TclW::new(self, 3)
    }
    #[doc = "Bit 4 - Control Register Lock"]
    #[inline(always)]
    #[must_use]
    pub fn crl(&mut self) -> CrlW<LrSpec> {
        CrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Status Register Lock"]
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SrlW<LrSpec> {
        SrlW::new(self, 5)
    }
    #[doc = "Bit 6 - Lock Register Lock"]
    #[inline(always)]
    #[must_use]
    pub fn lrl(&mut self) -> LrlW<LrSpec> {
        LrlW::new(self, 6)
    }
}
#[doc = "RTC Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LrSpec;
impl crate::RegisterSpec for LrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lr::R`](R) reader structure"]
impl crate::Readable for LrSpec {}
#[doc = "`write(|w| ..)` method takes [`lr::W`](W) writer structure"]
impl crate::Writable for LrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LR to value 0xff"]
impl crate::Resettable for LrSpec {
    const RESET_VALUE: u32 = 0xff;
}
