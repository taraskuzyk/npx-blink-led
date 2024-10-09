#[doc = "Register `MC` reader"]
pub type R = crate::R<McSpec>;
#[doc = "Register `MC` writer"]
pub type W = crate::W<McSpec>;
#[doc = "Second Low-frequency Internal Reference Clock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LircDiv2 {
    #[doc = "0: Division factor is 1."]
    B000 = 0,
    #[doc = "1: Division factor is 2."]
    B001 = 1,
    #[doc = "2: Division factor is 4."]
    B010 = 2,
    #[doc = "3: Division factor is 8."]
    B011 = 3,
    #[doc = "4: Division factor is 16."]
    B100 = 4,
    #[doc = "5: Division factor is 32."]
    B101 = 5,
    #[doc = "6: Division factor is 64."]
    B110 = 6,
    #[doc = "7: Division factor is 128."]
    B111 = 7,
}
impl From<LircDiv2> for u8 {
    #[inline(always)]
    fn from(variant: LircDiv2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LircDiv2 {
    type Ux = u8;
}
impl crate::IsEnum for LircDiv2 {}
#[doc = "Field `LIRC_DIV2` reader - Second Low-frequency Internal Reference Clock Divider"]
pub type LircDiv2R = crate::FieldReader<LircDiv2>;
impl LircDiv2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LircDiv2 {
        match self.bits {
            0 => LircDiv2::B000,
            1 => LircDiv2::B001,
            2 => LircDiv2::B010,
            3 => LircDiv2::B011,
            4 => LircDiv2::B100,
            5 => LircDiv2::B101,
            6 => LircDiv2::B110,
            7 => LircDiv2::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Division factor is 1."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == LircDiv2::B000
    }
    #[doc = "Division factor is 2."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == LircDiv2::B001
    }
    #[doc = "Division factor is 4."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == LircDiv2::B010
    }
    #[doc = "Division factor is 8."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == LircDiv2::B011
    }
    #[doc = "Division factor is 16."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == LircDiv2::B100
    }
    #[doc = "Division factor is 32."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == LircDiv2::B101
    }
    #[doc = "Division factor is 64."]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == LircDiv2::B110
    }
    #[doc = "Division factor is 128."]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == LircDiv2::B111
    }
}
#[doc = "Field `LIRC_DIV2` writer - Second Low-frequency Internal Reference Clock Divider"]
pub type LircDiv2W<'a, REG> = crate::FieldWriter<'a, REG, 3, LircDiv2, crate::Safe>;
impl<'a, REG> LircDiv2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Division factor is 1."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(LircDiv2::B000)
    }
    #[doc = "Division factor is 2."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(LircDiv2::B001)
    }
    #[doc = "Division factor is 4."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(LircDiv2::B010)
    }
    #[doc = "Division factor is 8."]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(LircDiv2::B011)
    }
    #[doc = "Division factor is 16."]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(LircDiv2::B100)
    }
    #[doc = "Division factor is 32."]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(LircDiv2::B101)
    }
    #[doc = "Division factor is 64."]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(LircDiv2::B110)
    }
    #[doc = "Division factor is 128."]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(LircDiv2::B111)
    }
}
#[doc = "High-frequency IRC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hircen {
    #[doc = "0: HIRC source is not enabled."]
    B0 = 0,
    #[doc = "1: HIRC source is enabled."]
    B1 = 1,
}
impl From<Hircen> for bool {
    #[inline(always)]
    fn from(variant: Hircen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIRCEN` reader - High-frequency IRC Enable"]
pub type HircenR = crate::BitReader<Hircen>;
impl HircenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hircen {
        match self.bits {
            false => Hircen::B0,
            true => Hircen::B1,
        }
    }
    #[doc = "HIRC source is not enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Hircen::B0
    }
    #[doc = "HIRC source is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Hircen::B1
    }
}
#[doc = "Field `HIRCEN` writer - High-frequency IRC Enable"]
pub type HircenW<'a, REG> = crate::BitWriter<'a, REG, Hircen>;
impl<'a, REG> HircenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HIRC source is not enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Hircen::B0)
    }
    #[doc = "HIRC source is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Hircen::B1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Second Low-frequency Internal Reference Clock Divider"]
    #[inline(always)]
    pub fn lirc_div2(&self) -> LircDiv2R {
        LircDiv2R::new(self.bits & 7)
    }
    #[doc = "Bit 7 - High-frequency IRC Enable"]
    #[inline(always)]
    pub fn hircen(&self) -> HircenR {
        HircenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Second Low-frequency Internal Reference Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn lirc_div2(&mut self) -> LircDiv2W<McSpec> {
        LircDiv2W::new(self, 0)
    }
    #[doc = "Bit 7 - High-frequency IRC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hircen(&mut self) -> HircenW<McSpec> {
        HircenW::new(self, 7)
    }
}
#[doc = "MCG Miscellaneous Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McSpec;
impl crate::RegisterSpec for McSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mc::R`](R) reader structure"]
impl crate::Readable for McSpec {}
#[doc = "`write(|w| ..)` method takes [`mc::W`](W) writer structure"]
impl crate::Writable for McSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MC to value 0"]
impl crate::Resettable for McSpec {
    const RESET_VALUE: u8 = 0;
}
