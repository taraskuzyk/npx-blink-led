#[doc = "Register `FSEC` reader"]
pub type R = crate::R<FsecSpec>;
#[doc = "Flash Security\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sec {
    #[doc = "0: MCU security status is secure."]
    B00 = 0,
    #[doc = "1: MCU security status is secure."]
    B01 = 1,
    #[doc = "2: MCU security status is unsecure. (The standard shipping condition of the flash memory module is unsecure.)"]
    B10 = 2,
    #[doc = "3: MCU security status is secure."]
    B11 = 3,
}
impl From<Sec> for u8 {
    #[inline(always)]
    fn from(variant: Sec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sec {
    type Ux = u8;
}
impl crate::IsEnum for Sec {}
#[doc = "Field `SEC` reader - Flash Security"]
pub type SecR = crate::FieldReader<Sec>;
impl SecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sec {
        match self.bits {
            0 => Sec::B00,
            1 => Sec::B01,
            2 => Sec::B10,
            3 => Sec::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "MCU security status is secure."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Sec::B00
    }
    #[doc = "MCU security status is secure."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Sec::B01
    }
    #[doc = "MCU security status is unsecure. (The standard shipping condition of the flash memory module is unsecure.)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Sec::B10
    }
    #[doc = "MCU security status is secure."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Sec::B11
    }
}
#[doc = "Freescale Failure Analysis Access Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fslacc {
    #[doc = "0: Freescale factory access granted"]
    B00 = 0,
    #[doc = "1: Freescale factory access denied"]
    B01 = 1,
    #[doc = "2: Freescale factory access denied"]
    B10 = 2,
    #[doc = "3: Freescale factory access granted"]
    B11 = 3,
}
impl From<Fslacc> for u8 {
    #[inline(always)]
    fn from(variant: Fslacc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fslacc {
    type Ux = u8;
}
impl crate::IsEnum for Fslacc {}
#[doc = "Field `FSLACC` reader - Freescale Failure Analysis Access Code"]
pub type FslaccR = crate::FieldReader<Fslacc>;
impl FslaccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fslacc {
        match self.bits {
            0 => Fslacc::B00,
            1 => Fslacc::B01,
            2 => Fslacc::B10,
            3 => Fslacc::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Freescale factory access granted"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Fslacc::B00
    }
    #[doc = "Freescale factory access denied"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Fslacc::B01
    }
    #[doc = "Freescale factory access denied"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Fslacc::B10
    }
    #[doc = "Freescale factory access granted"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Fslacc::B11
    }
}
#[doc = "Mass Erase Enable Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Meen {
    #[doc = "0: Mass erase is enabled"]
    B00 = 0,
    #[doc = "1: Mass erase is enabled"]
    B01 = 1,
    #[doc = "2: Mass erase is disabled"]
    B10 = 2,
    #[doc = "3: Mass erase is enabled"]
    B11 = 3,
}
impl From<Meen> for u8 {
    #[inline(always)]
    fn from(variant: Meen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Meen {
    type Ux = u8;
}
impl crate::IsEnum for Meen {}
#[doc = "Field `MEEN` reader - Mass Erase Enable Bits"]
pub type MeenR = crate::FieldReader<Meen>;
impl MeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Meen {
        match self.bits {
            0 => Meen::B00,
            1 => Meen::B01,
            2 => Meen::B10,
            3 => Meen::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Mass erase is enabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Meen::B00
    }
    #[doc = "Mass erase is enabled"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Meen::B01
    }
    #[doc = "Mass erase is disabled"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Meen::B10
    }
    #[doc = "Mass erase is enabled"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Meen::B11
    }
}
#[doc = "Backdoor Key Security Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Keyen {
    #[doc = "0: Backdoor key access disabled"]
    B00 = 0,
    #[doc = "1: Backdoor key access disabled (preferred KEYEN state to disable backdoor key access)"]
    B01 = 1,
    #[doc = "2: Backdoor key access enabled"]
    B10 = 2,
    #[doc = "3: Backdoor key access disabled"]
    B11 = 3,
}
impl From<Keyen> for u8 {
    #[inline(always)]
    fn from(variant: Keyen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Keyen {
    type Ux = u8;
}
impl crate::IsEnum for Keyen {}
#[doc = "Field `KEYEN` reader - Backdoor Key Security Enable"]
pub type KeyenR = crate::FieldReader<Keyen>;
impl KeyenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Keyen {
        match self.bits {
            0 => Keyen::B00,
            1 => Keyen::B01,
            2 => Keyen::B10,
            3 => Keyen::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Backdoor key access disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Keyen::B00
    }
    #[doc = "Backdoor key access disabled (preferred KEYEN state to disable backdoor key access)"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Keyen::B01
    }
    #[doc = "Backdoor key access enabled"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Keyen::B10
    }
    #[doc = "Backdoor key access disabled"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Keyen::B11
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash Security"]
    #[inline(always)]
    pub fn sec(&self) -> SecR {
        SecR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Freescale Failure Analysis Access Code"]
    #[inline(always)]
    pub fn fslacc(&self) -> FslaccR {
        FslaccR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Mass Erase Enable Bits"]
    #[inline(always)]
    pub fn meen(&self) -> MeenR {
        MeenR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable"]
    #[inline(always)]
    pub fn keyen(&self) -> KeyenR {
        KeyenR::new((self.bits >> 6) & 3)
    }
}
#[doc = "Flash Security Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsec::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsecSpec;
impl crate::RegisterSpec for FsecSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`fsec::R`](R) reader structure"]
impl crate::Readable for FsecSpec {}
#[doc = "`reset()` method sets FSEC to value 0"]
impl crate::Resettable for FsecSpec {
    const RESET_VALUE: u8 = 0;
}
