#[doc = "Register `SHIFTCFG%s` reader"]
pub type R = crate::R<ShiftcfgSpec>;
#[doc = "Register `SHIFTCFG%s` writer"]
pub type W = crate::W<ShiftcfgSpec>;
#[doc = "Shifter Start bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sstart {
    #[doc = "0: Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    B00 = 0,
    #[doc = "1: Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    B01 = 1,
    #[doc = "2: Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    B10 = 2,
    #[doc = "3: Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    B11 = 3,
}
impl From<Sstart> for u8 {
    #[inline(always)]
    fn from(variant: Sstart) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sstart {
    type Ux = u8;
}
impl crate::IsEnum for Sstart {}
#[doc = "Field `SSTART` reader - Shifter Start bit"]
pub type SstartR = crate::FieldReader<Sstart>;
impl SstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sstart {
        match self.bits {
            0 => Sstart::B00,
            1 => Sstart::B01,
            2 => Sstart::B10,
            3 => Sstart::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Sstart::B00
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Sstart::B01
    }
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Sstart::B10
    }
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Sstart::B11
    }
}
#[doc = "Field `SSTART` writer - Shifter Start bit"]
pub type SstartW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sstart, crate::Safe>;
impl<'a, REG> SstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on enable"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Sstart::B00)
    }
    #[doc = "Start bit disabled for transmitter/receiver/match store, transmitter loads data on first shift"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Sstart::B01)
    }
    #[doc = "Transmitter outputs start bit value 0 before loading data on first shift, receiver/match store sets error flag if start bit is not 0"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Sstart::B10)
    }
    #[doc = "Transmitter outputs start bit value 1 before loading data on first shift, receiver/match store sets error flag if start bit is not 1"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Sstart::B11)
    }
}
#[doc = "Shifter Stop bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sstop {
    #[doc = "0: Stop bit disabled for transmitter/receiver/match store"]
    B00 = 0,
    #[doc = "1: Reserved for transmitter/receiver/match store"]
    B01 = 1,
    #[doc = "2: Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    B10 = 2,
    #[doc = "3: Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    B11 = 3,
}
impl From<Sstop> for u8 {
    #[inline(always)]
    fn from(variant: Sstop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sstop {
    type Ux = u8;
}
impl crate::IsEnum for Sstop {}
#[doc = "Field `SSTOP` reader - Shifter Stop bit"]
pub type SstopR = crate::FieldReader<Sstop>;
impl SstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sstop {
        match self.bits {
            0 => Sstop::B00,
            1 => Sstop::B01,
            2 => Sstop::B10,
            3 => Sstop::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Sstop::B00
    }
    #[doc = "Reserved for transmitter/receiver/match store"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Sstop::B01
    }
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Sstop::B10
    }
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Sstop::B11
    }
}
#[doc = "Field `SSTOP` writer - Shifter Stop bit"]
pub type SstopW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sstop, crate::Safe>;
impl<'a, REG> SstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop bit disabled for transmitter/receiver/match store"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Sstop::B00)
    }
    #[doc = "Reserved for transmitter/receiver/match store"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Sstop::B01)
    }
    #[doc = "Transmitter outputs stop bit value 0 on store, receiver/match store sets error flag if stop bit is not 0"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Sstop::B10)
    }
    #[doc = "Transmitter outputs stop bit value 1 on store, receiver/match store sets error flag if stop bit is not 1"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Sstop::B11)
    }
}
#[doc = "Input Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Insrc {
    #[doc = "0: Pin"]
    B0 = 0,
    #[doc = "1: Shifter N+1 Output"]
    B1 = 1,
}
impl From<Insrc> for bool {
    #[inline(always)]
    fn from(variant: Insrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INSRC` reader - Input Source"]
pub type InsrcR = crate::BitReader<Insrc>;
impl InsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Insrc {
        match self.bits {
            false => Insrc::B0,
            true => Insrc::B1,
        }
    }
    #[doc = "Pin"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Insrc::B0
    }
    #[doc = "Shifter N+1 Output"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Insrc::B1
    }
}
#[doc = "Field `INSRC` writer - Input Source"]
pub type InsrcW<'a, REG> = crate::BitWriter<'a, REG, Insrc>;
impl<'a, REG> InsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pin"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Insrc::B0)
    }
    #[doc = "Shifter N+1 Output"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Insrc::B1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline(always)]
    pub fn sstart(&self) -> SstartR {
        SstartR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline(always)]
    pub fn sstop(&self) -> SstopR {
        SstopR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline(always)]
    pub fn insrc(&self) -> InsrcR {
        InsrcR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Shifter Start bit"]
    #[inline(always)]
    #[must_use]
    pub fn sstart(&mut self) -> SstartW<ShiftcfgSpec> {
        SstartW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Shifter Stop bit"]
    #[inline(always)]
    #[must_use]
    pub fn sstop(&mut self) -> SstopW<ShiftcfgSpec> {
        SstopW::new(self, 4)
    }
    #[doc = "Bit 8 - Input Source"]
    #[inline(always)]
    #[must_use]
    pub fn insrc(&mut self) -> InsrcW<ShiftcfgSpec> {
        InsrcW::new(self, 8)
    }
}
#[doc = "Shifter Configuration N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftcfgSpec;
impl crate::RegisterSpec for ShiftcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shiftcfg::R`](R) reader structure"]
impl crate::Readable for ShiftcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`shiftcfg::W`](W) writer structure"]
impl crate::Writable for ShiftcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTCFG%s to value 0"]
impl crate::Resettable for ShiftcfgSpec {
    const RESET_VALUE: u32 = 0;
}
