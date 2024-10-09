#[doc = "Register `LTRIMRNG` reader"]
pub type R = crate::R<LtrimrngSpec>;
#[doc = "LIRC Slow TRIM (2 MHz) Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Strimrng {
    #[doc = "0: Frequency shift by 10%."]
    B00 = 0,
    #[doc = "1: No frequency shift."]
    B01 = 1,
    #[doc = "2: No frequency shift."]
    B10 = 2,
    #[doc = "3: Frequency shift by -10%."]
    B11 = 3,
}
impl From<Strimrng> for u8 {
    #[inline(always)]
    fn from(variant: Strimrng) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Strimrng {
    type Ux = u8;
}
impl crate::IsEnum for Strimrng {}
#[doc = "Field `STRIMRNG` reader - LIRC Slow TRIM (2 MHz) Range"]
pub type StrimrngR = crate::FieldReader<Strimrng>;
impl StrimrngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Strimrng {
        match self.bits {
            0 => Strimrng::B00,
            1 => Strimrng::B01,
            2 => Strimrng::B10,
            3 => Strimrng::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Frequency shift by 10%."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Strimrng::B00
    }
    #[doc = "No frequency shift."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Strimrng::B01
    }
    #[doc = "No frequency shift."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Strimrng::B10
    }
    #[doc = "Frequency shift by -10%."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Strimrng::B11
    }
}
#[doc = "LIRC Fast TRIM (8 MHz) Range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ftrimrng {
    #[doc = "0: Frequency shift by 10%."]
    B00 = 0,
    #[doc = "1: No frequency shift."]
    B01 = 1,
    #[doc = "2: No frequency shift."]
    B10 = 2,
    #[doc = "3: Frequency shift by -10%."]
    B11 = 3,
}
impl From<Ftrimrng> for u8 {
    #[inline(always)]
    fn from(variant: Ftrimrng) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ftrimrng {
    type Ux = u8;
}
impl crate::IsEnum for Ftrimrng {}
#[doc = "Field `FTRIMRNG` reader - LIRC Fast TRIM (8 MHz) Range"]
pub type FtrimrngR = crate::FieldReader<Ftrimrng>;
impl FtrimrngR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ftrimrng {
        match self.bits {
            0 => Ftrimrng::B00,
            1 => Ftrimrng::B01,
            2 => Ftrimrng::B10,
            3 => Ftrimrng::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Frequency shift by 10%."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Ftrimrng::B00
    }
    #[doc = "No frequency shift."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Ftrimrng::B01
    }
    #[doc = "No frequency shift."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Ftrimrng::B10
    }
    #[doc = "Frequency shift by -10%."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Ftrimrng::B11
    }
}
impl R {
    #[doc = "Bits 0:1 - LIRC Slow TRIM (2 MHz) Range"]
    #[inline(always)]
    pub fn strimrng(&self) -> StrimrngR {
        StrimrngR::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - LIRC Fast TRIM (8 MHz) Range"]
    #[inline(always)]
    pub fn ftrimrng(&self) -> FtrimrngR {
        FtrimrngR::new((self.bits >> 2) & 3)
    }
}
#[doc = "MCG Low-frequency IRC Trim Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ltrimrng::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LtrimrngSpec;
impl crate::RegisterSpec for LtrimrngSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ltrimrng::R`](R) reader structure"]
impl crate::Readable for LtrimrngSpec {}
#[doc = "`reset()` method sets LTRIMRNG to value 0"]
impl crate::Resettable for LtrimrngSpec {
    const RESET_VALUE: u8 = 0;
}
