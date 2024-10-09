#[doc = "Register `FCFG1` reader"]
pub type R = crate::R<Fcfg1Spec>;
#[doc = "Register `FCFG1` writer"]
pub type W = crate::W<Fcfg1Spec>;
#[doc = "Flash Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashdis {
    #[doc = "0: Flash is enabled."]
    B0 = 0,
    #[doc = "1: Flash is disabled."]
    B1 = 1,
}
impl From<Flashdis> for bool {
    #[inline(always)]
    fn from(variant: Flashdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHDIS` reader - Flash Disable"]
pub type FlashdisR = crate::BitReader<Flashdis>;
impl FlashdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashdis {
        match self.bits {
            false => Flashdis::B0,
            true => Flashdis::B1,
        }
    }
    #[doc = "Flash is enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Flashdis::B0
    }
    #[doc = "Flash is disabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Flashdis::B1
    }
}
#[doc = "Field `FLASHDIS` writer - Flash Disable"]
pub type FlashdisW<'a, REG> = crate::BitWriter<'a, REG, Flashdis>;
impl<'a, REG> FlashdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash is enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashdis::B0)
    }
    #[doc = "Flash is disabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashdis::B1)
    }
}
#[doc = "Flash Doze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flashdoze {
    #[doc = "0: Flash remains enabled during Doze mode."]
    B0 = 0,
    #[doc = "1: Flash is disabled for the duration of Doze mode."]
    B1 = 1,
}
impl From<Flashdoze> for bool {
    #[inline(always)]
    fn from(variant: Flashdoze) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FLASHDOZE` reader - Flash Doze"]
pub type FlashdozeR = crate::BitReader<Flashdoze>;
impl FlashdozeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flashdoze {
        match self.bits {
            false => Flashdoze::B0,
            true => Flashdoze::B1,
        }
    }
    #[doc = "Flash remains enabled during Doze mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Flashdoze::B0
    }
    #[doc = "Flash is disabled for the duration of Doze mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Flashdoze::B1
    }
}
#[doc = "Field `FLASHDOZE` writer - Flash Doze"]
pub type FlashdozeW<'a, REG> = crate::BitWriter<'a, REG, Flashdoze>;
impl<'a, REG> FlashdozeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash remains enabled during Doze mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Flashdoze::B0)
    }
    #[doc = "Flash is disabled for the duration of Doze mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashdoze::B1)
    }
}
#[doc = "Program Flash Size\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pfsize {
    #[doc = "0: 8 KB of program flash memory, 1 KB protection region"]
    B0000 = 0,
    #[doc = "1: 16 KB of program flash memory, 1 KB protection region"]
    B0001 = 1,
    #[doc = "3: 32 KB of program flash memory, 1 KB protection region"]
    B0011 = 3,
    #[doc = "5: 64 KB of program flash memory, 2 KB protection region"]
    B0101 = 5,
    #[doc = "7: 128 KB of program flash memory, 4 KB protection region"]
    B0111 = 7,
    #[doc = "9: 256 KB of program flash memory, 8 KB protection region"]
    B1001 = 9,
    #[doc = "15: 256 KB of program flash memory, 8 KB protection region"]
    B1111 = 15,
}
impl From<Pfsize> for u8 {
    #[inline(always)]
    fn from(variant: Pfsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pfsize {
    type Ux = u8;
}
impl crate::IsEnum for Pfsize {}
#[doc = "Field `PFSIZE` reader - Program Flash Size"]
pub type PfsizeR = crate::FieldReader<Pfsize>;
impl PfsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pfsize> {
        match self.bits {
            0 => Some(Pfsize::B0000),
            1 => Some(Pfsize::B0001),
            3 => Some(Pfsize::B0011),
            5 => Some(Pfsize::B0101),
            7 => Some(Pfsize::B0111),
            9 => Some(Pfsize::B1001),
            15 => Some(Pfsize::B1111),
            _ => None,
        }
    }
    #[doc = "8 KB of program flash memory, 1 KB protection region"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Pfsize::B0000
    }
    #[doc = "16 KB of program flash memory, 1 KB protection region"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Pfsize::B0001
    }
    #[doc = "32 KB of program flash memory, 1 KB protection region"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Pfsize::B0011
    }
    #[doc = "64 KB of program flash memory, 2 KB protection region"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Pfsize::B0101
    }
    #[doc = "128 KB of program flash memory, 4 KB protection region"]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == Pfsize::B0111
    }
    #[doc = "256 KB of program flash memory, 8 KB protection region"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Pfsize::B1001
    }
    #[doc = "256 KB of program flash memory, 8 KB protection region"]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == Pfsize::B1111
    }
}
impl R {
    #[doc = "Bit 0 - Flash Disable"]
    #[inline(always)]
    pub fn flashdis(&self) -> FlashdisR {
        FlashdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Doze"]
    #[inline(always)]
    pub fn flashdoze(&self) -> FlashdozeR {
        FlashdozeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Program Flash Size"]
    #[inline(always)]
    pub fn pfsize(&self) -> PfsizeR {
        PfsizeR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Disable"]
    #[inline(always)]
    #[must_use]
    pub fn flashdis(&mut self) -> FlashdisW<Fcfg1Spec> {
        FlashdisW::new(self, 0)
    }
    #[doc = "Bit 1 - Flash Doze"]
    #[inline(always)]
    #[must_use]
    pub fn flashdoze(&mut self) -> FlashdozeW<Fcfg1Spec> {
        FlashdozeW::new(self, 1)
    }
}
#[doc = "Flash Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fcfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcfg1Spec;
impl crate::RegisterSpec for Fcfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcfg1::R`](R) reader structure"]
impl crate::Readable for Fcfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`fcfg1::W`](W) writer structure"]
impl crate::Writable for Fcfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCFG1 to value 0x0f00_0000"]
impl crate::Resettable for Fcfg1Spec {
    const RESET_VALUE: u32 = 0x0f00_0000;
}
