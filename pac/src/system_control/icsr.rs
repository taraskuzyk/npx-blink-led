#[doc = "Register `ICSR` reader"]
pub type R = crate::R<IcsrSpec>;
#[doc = "Register `ICSR` writer"]
pub type W = crate::W<IcsrSpec>;
#[doc = "Field `VECTPENDING` reader - Exception number of the highest priority pending enabled exception"]
pub type VectpendingR = crate::FieldReader;
#[doc = "SysTick exception clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendstclr {
    #[doc = "0: no effect"]
    B0 = 0,
    #[doc = "1: removes the pending state from the SysTick exception"]
    B1 = 1,
}
impl From<Pendstclr> for bool {
    #[inline(always)]
    fn from(variant: Pendstclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTCLR` writer - SysTick exception clear-pending bit"]
pub type PendstclrW<'a, REG> = crate::BitWriter<'a, REG, Pendstclr>;
impl<'a, REG> PendstclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstclr::B0)
    }
    #[doc = "removes the pending state from the SysTick exception"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstclr::B1)
    }
}
#[doc = "SysTick exception set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendstset {
    #[doc = "0: write: no effect; read: SysTick exception is not pending"]
    B0 = 0,
    #[doc = "1: write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    B1 = 1,
}
impl From<Pendstset> for bool {
    #[inline(always)]
    fn from(variant: Pendstset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSTSET` reader - SysTick exception set-pending bit"]
pub type PendstsetR = crate::BitReader<Pendstset>;
impl PendstsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pendstset {
        match self.bits {
            false => Pendstset::B0,
            true => Pendstset::B1,
        }
    }
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pendstset::B0
    }
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pendstset::B1
    }
}
#[doc = "Field `PENDSTSET` writer - SysTick exception set-pending bit"]
pub type PendstsetW<'a, REG> = crate::BitWriter<'a, REG, Pendstset>;
impl<'a, REG> PendstsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write: no effect; read: SysTick exception is not pending"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstset::B0)
    }
    #[doc = "write: changes SysTick exception state to pending; read: SysTick exception is pending"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendstset::B1)
    }
}
#[doc = "PendSV clear-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendsvclr {
    #[doc = "0: no effect"]
    B0 = 0,
    #[doc = "1: removes the pending state from the PendSV exception"]
    B1 = 1,
}
impl From<Pendsvclr> for bool {
    #[inline(always)]
    fn from(variant: Pendsvclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVCLR` writer - PendSV clear-pending bit"]
pub type PendsvclrW<'a, REG> = crate::BitWriter<'a, REG, Pendsvclr>;
impl<'a, REG> PendsvclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "no effect"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvclr::B0)
    }
    #[doc = "removes the pending state from the PendSV exception"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvclr::B1)
    }
}
#[doc = "PendSV set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pendsvset {
    #[doc = "0: write: no effect; read: PendSV exception is not pending"]
    B0 = 0,
    #[doc = "1: write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    B1 = 1,
}
impl From<Pendsvset> for bool {
    #[inline(always)]
    fn from(variant: Pendsvset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PENDSVSET` reader - PendSV set-pending bit"]
pub type PendsvsetR = crate::BitReader<Pendsvset>;
impl PendsvsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pendsvset {
        match self.bits {
            false => Pendsvset::B0,
            true => Pendsvset::B1,
        }
    }
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pendsvset::B0
    }
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pendsvset::B1
    }
}
#[doc = "Field `PENDSVSET` writer - PendSV set-pending bit"]
pub type PendsvsetW<'a, REG> = crate::BitWriter<'a, REG, Pendsvset>;
impl<'a, REG> PendsvsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write: no effect; read: PendSV exception is not pending"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvset::B0)
    }
    #[doc = "write: changes PendSV exception state to pending; read: PendSV exception is pending"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pendsvset::B1)
    }
}
#[doc = "NMI set-pending bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmipendset {
    #[doc = "0: write: no effect; read: NMI exception is not pending"]
    B0 = 0,
    #[doc = "1: write: changes NMI exception state to pending; read: NMI exception is pending"]
    B1 = 1,
}
impl From<Nmipendset> for bool {
    #[inline(always)]
    fn from(variant: Nmipendset) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIPENDSET` reader - NMI set-pending bit"]
pub type NmipendsetR = crate::BitReader<Nmipendset>;
impl NmipendsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmipendset {
        match self.bits {
            false => Nmipendset::B0,
            true => Nmipendset::B1,
        }
    }
    #[doc = "write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Nmipendset::B0
    }
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Nmipendset::B1
    }
}
#[doc = "Field `NMIPENDSET` writer - NMI set-pending bit"]
pub type NmipendsetW<'a, REG> = crate::BitWriter<'a, REG, Nmipendset>;
impl<'a, REG> NmipendsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "write: no effect; read: NMI exception is not pending"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Nmipendset::B0)
    }
    #[doc = "write: changes NMI exception state to pending; read: NMI exception is pending"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Nmipendset::B1)
    }
}
impl R {
    #[doc = "Bits 12:17 - Exception number of the highest priority pending enabled exception"]
    #[inline(always)]
    pub fn vectpending(&self) -> VectpendingR {
        VectpendingR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    pub fn pendstset(&self) -> PendstsetR {
        PendstsetR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    pub fn pendsvset(&self) -> PendsvsetR {
        PendsvsetR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    pub fn nmipendset(&self) -> NmipendsetR {
        NmipendsetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - SysTick exception clear-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendstclr(&mut self) -> PendstclrW<IcsrSpec> {
        PendstclrW::new(self, 25)
    }
    #[doc = "Bit 26 - SysTick exception set-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendstset(&mut self) -> PendstsetW<IcsrSpec> {
        PendstsetW::new(self, 26)
    }
    #[doc = "Bit 27 - PendSV clear-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvclr(&mut self) -> PendsvclrW<IcsrSpec> {
        PendsvclrW::new(self, 27)
    }
    #[doc = "Bit 28 - PendSV set-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn pendsvset(&mut self) -> PendsvsetW<IcsrSpec> {
        PendsvsetW::new(self, 28)
    }
    #[doc = "Bit 31 - NMI set-pending bit"]
    #[inline(always)]
    #[must_use]
    pub fn nmipendset(&mut self) -> NmipendsetW<IcsrSpec> {
        NmipendsetW::new(self, 31)
    }
}
#[doc = "Interrupt Control and State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcsrSpec;
impl crate::RegisterSpec for IcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icsr::R`](R) reader structure"]
impl crate::Readable for IcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`icsr::W`](W) writer structure"]
impl crate::Writable for IcsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICSR to value 0"]
impl crate::Resettable for IcsrSpec {
    const RESET_VALUE: u32 = 0;
}
