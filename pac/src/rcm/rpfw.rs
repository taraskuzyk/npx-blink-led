#[doc = "Register `RPFW` reader"]
pub type R = crate::R<RpfwSpec>;
#[doc = "Register `RPFW` writer"]
pub type W = crate::W<RpfwSpec>;
#[doc = "Reset Pin Filter Bus Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rstfltsel {
    #[doc = "0: Bus clock filter count is 1"]
    B00000 = 0,
    #[doc = "1: Bus clock filter count is 2"]
    B00001 = 1,
    #[doc = "2: Bus clock filter count is 3"]
    B00010 = 2,
    #[doc = "3: Bus clock filter count is 4"]
    B00011 = 3,
    #[doc = "4: Bus clock filter count is 5"]
    B00100 = 4,
    #[doc = "5: Bus clock filter count is 6"]
    B00101 = 5,
    #[doc = "6: Bus clock filter count is 7"]
    B00110 = 6,
    #[doc = "7: Bus clock filter count is 8"]
    B00111 = 7,
    #[doc = "8: Bus clock filter count is 9"]
    B01000 = 8,
    #[doc = "9: Bus clock filter count is 10"]
    B01001 = 9,
    #[doc = "10: Bus clock filter count is 11"]
    B01010 = 10,
    #[doc = "11: Bus clock filter count is 12"]
    B01011 = 11,
    #[doc = "12: Bus clock filter count is 13"]
    B01100 = 12,
    #[doc = "13: Bus clock filter count is 14"]
    B01101 = 13,
    #[doc = "14: Bus clock filter count is 15"]
    B01110 = 14,
    #[doc = "15: Bus clock filter count is 16"]
    B01111 = 15,
    #[doc = "16: Bus clock filter count is 17"]
    B10000 = 16,
    #[doc = "17: Bus clock filter count is 18"]
    B10001 = 17,
    #[doc = "18: Bus clock filter count is 19"]
    B10010 = 18,
    #[doc = "19: Bus clock filter count is 20"]
    B10011 = 19,
    #[doc = "20: Bus clock filter count is 21"]
    B10100 = 20,
    #[doc = "21: Bus clock filter count is 22"]
    B10101 = 21,
    #[doc = "22: Bus clock filter count is 23"]
    B10110 = 22,
    #[doc = "23: Bus clock filter count is 24"]
    B10111 = 23,
    #[doc = "24: Bus clock filter count is 25"]
    B11000 = 24,
    #[doc = "25: Bus clock filter count is 26"]
    B11001 = 25,
    #[doc = "26: Bus clock filter count is 27"]
    B11010 = 26,
    #[doc = "27: Bus clock filter count is 28"]
    B11011 = 27,
    #[doc = "28: Bus clock filter count is 29"]
    B11100 = 28,
    #[doc = "29: Bus clock filter count is 30"]
    B11101 = 29,
    #[doc = "30: Bus clock filter count is 31"]
    B11110 = 30,
    #[doc = "31: Bus clock filter count is 32"]
    B11111 = 31,
}
impl From<Rstfltsel> for u8 {
    #[inline(always)]
    fn from(variant: Rstfltsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rstfltsel {
    type Ux = u8;
}
impl crate::IsEnum for Rstfltsel {}
#[doc = "Field `RSTFLTSEL` reader - Reset Pin Filter Bus Clock Select"]
pub type RstfltselR = crate::FieldReader<Rstfltsel>;
impl RstfltselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rstfltsel {
        match self.bits {
            0 => Rstfltsel::B00000,
            1 => Rstfltsel::B00001,
            2 => Rstfltsel::B00010,
            3 => Rstfltsel::B00011,
            4 => Rstfltsel::B00100,
            5 => Rstfltsel::B00101,
            6 => Rstfltsel::B00110,
            7 => Rstfltsel::B00111,
            8 => Rstfltsel::B01000,
            9 => Rstfltsel::B01001,
            10 => Rstfltsel::B01010,
            11 => Rstfltsel::B01011,
            12 => Rstfltsel::B01100,
            13 => Rstfltsel::B01101,
            14 => Rstfltsel::B01110,
            15 => Rstfltsel::B01111,
            16 => Rstfltsel::B10000,
            17 => Rstfltsel::B10001,
            18 => Rstfltsel::B10010,
            19 => Rstfltsel::B10011,
            20 => Rstfltsel::B10100,
            21 => Rstfltsel::B10101,
            22 => Rstfltsel::B10110,
            23 => Rstfltsel::B10111,
            24 => Rstfltsel::B11000,
            25 => Rstfltsel::B11001,
            26 => Rstfltsel::B11010,
            27 => Rstfltsel::B11011,
            28 => Rstfltsel::B11100,
            29 => Rstfltsel::B11101,
            30 => Rstfltsel::B11110,
            31 => Rstfltsel::B11111,
            _ => unreachable!(),
        }
    }
    #[doc = "Bus clock filter count is 1"]
    #[inline(always)]
    pub fn is_b00000(&self) -> bool {
        *self == Rstfltsel::B00000
    }
    #[doc = "Bus clock filter count is 2"]
    #[inline(always)]
    pub fn is_b00001(&self) -> bool {
        *self == Rstfltsel::B00001
    }
    #[doc = "Bus clock filter count is 3"]
    #[inline(always)]
    pub fn is_b00010(&self) -> bool {
        *self == Rstfltsel::B00010
    }
    #[doc = "Bus clock filter count is 4"]
    #[inline(always)]
    pub fn is_b00011(&self) -> bool {
        *self == Rstfltsel::B00011
    }
    #[doc = "Bus clock filter count is 5"]
    #[inline(always)]
    pub fn is_b00100(&self) -> bool {
        *self == Rstfltsel::B00100
    }
    #[doc = "Bus clock filter count is 6"]
    #[inline(always)]
    pub fn is_b00101(&self) -> bool {
        *self == Rstfltsel::B00101
    }
    #[doc = "Bus clock filter count is 7"]
    #[inline(always)]
    pub fn is_b00110(&self) -> bool {
        *self == Rstfltsel::B00110
    }
    #[doc = "Bus clock filter count is 8"]
    #[inline(always)]
    pub fn is_b00111(&self) -> bool {
        *self == Rstfltsel::B00111
    }
    #[doc = "Bus clock filter count is 9"]
    #[inline(always)]
    pub fn is_b01000(&self) -> bool {
        *self == Rstfltsel::B01000
    }
    #[doc = "Bus clock filter count is 10"]
    #[inline(always)]
    pub fn is_b01001(&self) -> bool {
        *self == Rstfltsel::B01001
    }
    #[doc = "Bus clock filter count is 11"]
    #[inline(always)]
    pub fn is_b01010(&self) -> bool {
        *self == Rstfltsel::B01010
    }
    #[doc = "Bus clock filter count is 12"]
    #[inline(always)]
    pub fn is_b01011(&self) -> bool {
        *self == Rstfltsel::B01011
    }
    #[doc = "Bus clock filter count is 13"]
    #[inline(always)]
    pub fn is_b01100(&self) -> bool {
        *self == Rstfltsel::B01100
    }
    #[doc = "Bus clock filter count is 14"]
    #[inline(always)]
    pub fn is_b01101(&self) -> bool {
        *self == Rstfltsel::B01101
    }
    #[doc = "Bus clock filter count is 15"]
    #[inline(always)]
    pub fn is_b01110(&self) -> bool {
        *self == Rstfltsel::B01110
    }
    #[doc = "Bus clock filter count is 16"]
    #[inline(always)]
    pub fn is_b01111(&self) -> bool {
        *self == Rstfltsel::B01111
    }
    #[doc = "Bus clock filter count is 17"]
    #[inline(always)]
    pub fn is_b10000(&self) -> bool {
        *self == Rstfltsel::B10000
    }
    #[doc = "Bus clock filter count is 18"]
    #[inline(always)]
    pub fn is_b10001(&self) -> bool {
        *self == Rstfltsel::B10001
    }
    #[doc = "Bus clock filter count is 19"]
    #[inline(always)]
    pub fn is_b10010(&self) -> bool {
        *self == Rstfltsel::B10010
    }
    #[doc = "Bus clock filter count is 20"]
    #[inline(always)]
    pub fn is_b10011(&self) -> bool {
        *self == Rstfltsel::B10011
    }
    #[doc = "Bus clock filter count is 21"]
    #[inline(always)]
    pub fn is_b10100(&self) -> bool {
        *self == Rstfltsel::B10100
    }
    #[doc = "Bus clock filter count is 22"]
    #[inline(always)]
    pub fn is_b10101(&self) -> bool {
        *self == Rstfltsel::B10101
    }
    #[doc = "Bus clock filter count is 23"]
    #[inline(always)]
    pub fn is_b10110(&self) -> bool {
        *self == Rstfltsel::B10110
    }
    #[doc = "Bus clock filter count is 24"]
    #[inline(always)]
    pub fn is_b10111(&self) -> bool {
        *self == Rstfltsel::B10111
    }
    #[doc = "Bus clock filter count is 25"]
    #[inline(always)]
    pub fn is_b11000(&self) -> bool {
        *self == Rstfltsel::B11000
    }
    #[doc = "Bus clock filter count is 26"]
    #[inline(always)]
    pub fn is_b11001(&self) -> bool {
        *self == Rstfltsel::B11001
    }
    #[doc = "Bus clock filter count is 27"]
    #[inline(always)]
    pub fn is_b11010(&self) -> bool {
        *self == Rstfltsel::B11010
    }
    #[doc = "Bus clock filter count is 28"]
    #[inline(always)]
    pub fn is_b11011(&self) -> bool {
        *self == Rstfltsel::B11011
    }
    #[doc = "Bus clock filter count is 29"]
    #[inline(always)]
    pub fn is_b11100(&self) -> bool {
        *self == Rstfltsel::B11100
    }
    #[doc = "Bus clock filter count is 30"]
    #[inline(always)]
    pub fn is_b11101(&self) -> bool {
        *self == Rstfltsel::B11101
    }
    #[doc = "Bus clock filter count is 31"]
    #[inline(always)]
    pub fn is_b11110(&self) -> bool {
        *self == Rstfltsel::B11110
    }
    #[doc = "Bus clock filter count is 32"]
    #[inline(always)]
    pub fn is_b11111(&self) -> bool {
        *self == Rstfltsel::B11111
    }
}
#[doc = "Field `RSTFLTSEL` writer - Reset Pin Filter Bus Clock Select"]
pub type RstfltselW<'a, REG> = crate::FieldWriter<'a, REG, 5, Rstfltsel, crate::Safe>;
impl<'a, REG> RstfltselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bus clock filter count is 1"]
    #[inline(always)]
    pub fn b00000(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B00000)
    }
    #[doc = "Bus clock filter count is 2"]
    #[inline(always)]
    pub fn b00001(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B00001)
    }
    #[doc = "Bus clock filter count is 3"]
    #[inline(always)]
    pub fn b00010(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B00010)
    }
    #[doc = "Bus clock filter count is 4"]
    #[inline(always)]
    pub fn b00011(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B00011)
    }
    #[doc = "Bus clock filter count is 5"]
    #[inline(always)]
    pub fn b00100(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B00100)
    }
    #[doc = "Bus clock filter count is 6"]
    #[inline(always)]
    pub fn b00101(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B00101)
    }
    #[doc = "Bus clock filter count is 7"]
    #[inline(always)]
    pub fn b00110(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B00110)
    }
    #[doc = "Bus clock filter count is 8"]
    #[inline(always)]
    pub fn b00111(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B00111)
    }
    #[doc = "Bus clock filter count is 9"]
    #[inline(always)]
    pub fn b01000(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B01000)
    }
    #[doc = "Bus clock filter count is 10"]
    #[inline(always)]
    pub fn b01001(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B01001)
    }
    #[doc = "Bus clock filter count is 11"]
    #[inline(always)]
    pub fn b01010(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B01010)
    }
    #[doc = "Bus clock filter count is 12"]
    #[inline(always)]
    pub fn b01011(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B01011)
    }
    #[doc = "Bus clock filter count is 13"]
    #[inline(always)]
    pub fn b01100(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B01100)
    }
    #[doc = "Bus clock filter count is 14"]
    #[inline(always)]
    pub fn b01101(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B01101)
    }
    #[doc = "Bus clock filter count is 15"]
    #[inline(always)]
    pub fn b01110(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B01110)
    }
    #[doc = "Bus clock filter count is 16"]
    #[inline(always)]
    pub fn b01111(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B01111)
    }
    #[doc = "Bus clock filter count is 17"]
    #[inline(always)]
    pub fn b10000(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B10000)
    }
    #[doc = "Bus clock filter count is 18"]
    #[inline(always)]
    pub fn b10001(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B10001)
    }
    #[doc = "Bus clock filter count is 19"]
    #[inline(always)]
    pub fn b10010(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B10010)
    }
    #[doc = "Bus clock filter count is 20"]
    #[inline(always)]
    pub fn b10011(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B10011)
    }
    #[doc = "Bus clock filter count is 21"]
    #[inline(always)]
    pub fn b10100(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B10100)
    }
    #[doc = "Bus clock filter count is 22"]
    #[inline(always)]
    pub fn b10101(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B10101)
    }
    #[doc = "Bus clock filter count is 23"]
    #[inline(always)]
    pub fn b10110(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B10110)
    }
    #[doc = "Bus clock filter count is 24"]
    #[inline(always)]
    pub fn b10111(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B10111)
    }
    #[doc = "Bus clock filter count is 25"]
    #[inline(always)]
    pub fn b11000(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B11000)
    }
    #[doc = "Bus clock filter count is 26"]
    #[inline(always)]
    pub fn b11001(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B11001)
    }
    #[doc = "Bus clock filter count is 27"]
    #[inline(always)]
    pub fn b11010(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B11010)
    }
    #[doc = "Bus clock filter count is 28"]
    #[inline(always)]
    pub fn b11011(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B11011)
    }
    #[doc = "Bus clock filter count is 29"]
    #[inline(always)]
    pub fn b11100(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B11100)
    }
    #[doc = "Bus clock filter count is 30"]
    #[inline(always)]
    pub fn b11101(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B11101)
    }
    #[doc = "Bus clock filter count is 31"]
    #[inline(always)]
    pub fn b11110(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B11110)
    }
    #[doc = "Bus clock filter count is 32"]
    #[inline(always)]
    pub fn b11111(self) -> &'a mut crate::W<REG> {
        self.variant(Rstfltsel::B11111)
    }
}
impl R {
    #[doc = "Bits 0:4 - Reset Pin Filter Bus Clock Select"]
    #[inline(always)]
    pub fn rstfltsel(&self) -> RstfltselR {
        RstfltselR::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reset Pin Filter Bus Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn rstfltsel(&mut self) -> RstfltselW<RpfwSpec> {
        RstfltselW::new(self, 0)
    }
}
#[doc = "Reset Pin Filter Width register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpfw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpfw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RpfwSpec;
impl crate::RegisterSpec for RpfwSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rpfw::R`](R) reader structure"]
impl crate::Readable for RpfwSpec {}
#[doc = "`write(|w| ..)` method takes [`rpfw::W`](W) writer structure"]
impl crate::Writable for RpfwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets RPFW to value 0"]
impl crate::Resettable for RpfwSpec {
    const RESET_VALUE: u8 = 0;
}
