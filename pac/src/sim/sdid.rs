#[doc = "Register `SDID` reader"]
pub type R = crate::R<SdidSpec>;
#[doc = "Pincount Identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pinid {
    #[doc = "2: 32-pin"]
    B0010 = 2,
    #[doc = "4: 48-pin"]
    B0100 = 4,
    #[doc = "5: 64-pin"]
    B0101 = 5,
    #[doc = "11: Custom pinout (WLCSP)"]
    B1011 = 11,
}
impl From<Pinid> for u8 {
    #[inline(always)]
    fn from(variant: Pinid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pinid {
    type Ux = u8;
}
impl crate::IsEnum for Pinid {}
#[doc = "Field `PINID` reader - Pincount Identification"]
pub type PinidR = crate::FieldReader<Pinid>;
impl PinidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pinid> {
        match self.bits {
            2 => Some(Pinid::B0010),
            4 => Some(Pinid::B0100),
            5 => Some(Pinid::B0101),
            11 => Some(Pinid::B1011),
            _ => None,
        }
    }
    #[doc = "32-pin"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Pinid::B0010
    }
    #[doc = "48-pin"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Pinid::B0100
    }
    #[doc = "64-pin"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Pinid::B0101
    }
    #[doc = "Custom pinout (WLCSP)"]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == Pinid::B1011
    }
}
#[doc = "Field `REVID` reader - Device Revision Number"]
pub type RevidR = crate::FieldReader;
#[doc = "System SRAM Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sramsize {
    #[doc = "5: 16 KB"]
    B0101 = 5,
    #[doc = "6: 32 KB"]
    B0110 = 6,
}
impl From<Sramsize> for u8 {
    #[inline(always)]
    fn from(variant: Sramsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sramsize {
    type Ux = u8;
}
impl crate::IsEnum for Sramsize {}
#[doc = "Field `SRAMSIZE` reader - System SRAM Size"]
pub type SramsizeR = crate::FieldReader<Sramsize>;
impl SramsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sramsize> {
        match self.bits {
            5 => Some(Sramsize::B0101),
            6 => Some(Sramsize::B0110),
            _ => None,
        }
    }
    #[doc = "16 KB"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Sramsize::B0101
    }
    #[doc = "32 KB"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Sramsize::B0110
    }
}
#[doc = "Kinetis Series ID\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Seriesid {
    #[doc = "1: KL family"]
    B0001 = 1,
}
impl From<Seriesid> for u8 {
    #[inline(always)]
    fn from(variant: Seriesid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Seriesid {
    type Ux = u8;
}
impl crate::IsEnum for Seriesid {}
#[doc = "Field `SERIESID` reader - Kinetis Series ID"]
pub type SeriesidR = crate::FieldReader<Seriesid>;
impl SeriesidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Seriesid> {
        match self.bits {
            1 => Some(Seriesid::B0001),
            _ => None,
        }
    }
    #[doc = "KL family"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Seriesid::B0001
    }
}
#[doc = "TBD Sub-Family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Subfamid {
    #[doc = "3: TBD Subfamily"]
    B0011 = 3,
}
impl From<Subfamid> for u8 {
    #[inline(always)]
    fn from(variant: Subfamid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Subfamid {
    type Ux = u8;
}
impl crate::IsEnum for Subfamid {}
#[doc = "Field `SUBFAMID` reader - TBD Sub-Family ID"]
pub type SubfamidR = crate::FieldReader<Subfamid>;
impl SubfamidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Subfamid> {
        match self.bits {
            3 => Some(Subfamid::B0011),
            _ => None,
        }
    }
    #[doc = "TBD Subfamily"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Subfamid::B0011
    }
}
#[doc = "Family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Famid {
    #[doc = "1: TBD"]
    B0001 = 1,
    #[doc = "2: TBD"]
    B0010 = 2,
    #[doc = "3: TBD"]
    B0011 = 3,
    #[doc = "4: TBD"]
    B0100 = 4,
}
impl From<Famid> for u8 {
    #[inline(always)]
    fn from(variant: Famid) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Famid {
    type Ux = u8;
}
impl crate::IsEnum for Famid {}
#[doc = "Field `FAMID` reader - Family ID"]
pub type FamidR = crate::FieldReader<Famid>;
impl FamidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Famid> {
        match self.bits {
            1 => Some(Famid::B0001),
            2 => Some(Famid::B0010),
            3 => Some(Famid::B0011),
            4 => Some(Famid::B0100),
            _ => None,
        }
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Famid::B0001
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Famid::B0010
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Famid::B0011
    }
    #[doc = "TBD"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Famid::B0100
    }
}
impl R {
    #[doc = "Bits 0:3 - Pincount Identification"]
    #[inline(always)]
    pub fn pinid(&self) -> PinidR {
        PinidR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Device Revision Number"]
    #[inline(always)]
    pub fn revid(&self) -> RevidR {
        RevidR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - System SRAM Size"]
    #[inline(always)]
    pub fn sramsize(&self) -> SramsizeR {
        SramsizeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Kinetis Series ID"]
    #[inline(always)]
    pub fn seriesid(&self) -> SeriesidR {
        SeriesidR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - TBD Sub-Family ID"]
    #[inline(always)]
    pub fn subfamid(&self) -> SubfamidR {
        SubfamidR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Family ID"]
    #[inline(always)]
    pub fn famid(&self) -> FamidR {
        FamidR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "System Device Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdidSpec;
impl crate::RegisterSpec for SdidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdid::R`](R) reader structure"]
impl crate::Readable for SdidSpec {}
#[doc = "`reset()` method sets SDID to value 0x0010_0d80"]
impl crate::Resettable for SdidSpec {
    const RESET_VALUE: u32 = 0x0010_0d80;
}
