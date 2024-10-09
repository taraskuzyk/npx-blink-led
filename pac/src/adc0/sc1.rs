#[doc = "Register `SC1%s` reader"]
pub type R = crate::R<Sc1Spec>;
#[doc = "Register `SC1%s` writer"]
pub type W = crate::W<Sc1Spec>;
#[doc = "Input channel select\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adch {
    #[doc = "0: When DIFF=0, DADP0 is selected as input; when DIFF=1, DAD0 is selected as input."]
    B00000 = 0,
    #[doc = "1: When DIFF=0, DADP1 is selected as input; when DIFF=1, DAD1 is selected as input."]
    B00001 = 1,
    #[doc = "2: When DIFF=0, DADP2 is selected as input; when DIFF=1, DAD2 is selected as input."]
    B00010 = 2,
    #[doc = "3: When DIFF=0, DADP3 is selected as input; when DIFF=1, DAD3 is selected as input."]
    B00011 = 3,
    #[doc = "4: When DIFF=0, AD4 is selected as input; when DIFF=1, it is reserved."]
    B00100 = 4,
    #[doc = "5: When DIFF=0, AD5 is selected as input; when DIFF=1, it is reserved."]
    B00101 = 5,
    #[doc = "6: When DIFF=0, AD6 is selected as input; when DIFF=1, it is reserved."]
    B00110 = 6,
    #[doc = "7: When DIFF=0, AD7 is selected as input; when DIFF=1, it is reserved."]
    B00111 = 7,
    #[doc = "8: When DIFF=0, AD8 is selected as input; when DIFF=1, it is reserved."]
    B01000 = 8,
    #[doc = "9: When DIFF=0, AD9 is selected as input; when DIFF=1, it is reserved."]
    B01001 = 9,
    #[doc = "10: When DIFF=0, AD10 is selected as input; when DIFF=1, it is reserved."]
    B01010 = 10,
    #[doc = "11: When DIFF=0, AD11 is selected as input; when DIFF=1, it is reserved."]
    B01011 = 11,
    #[doc = "12: When DIFF=0, AD12 is selected as input; when DIFF=1, it is reserved."]
    B01100 = 12,
    #[doc = "13: When DIFF=0, AD13 is selected as input; when DIFF=1, it is reserved."]
    B01101 = 13,
    #[doc = "14: When DIFF=0, AD14 is selected as input; when DIFF=1, it is reserved."]
    B01110 = 14,
    #[doc = "15: When DIFF=0, AD15 is selected as input; when DIFF=1, it is reserved."]
    B01111 = 15,
    #[doc = "16: When DIFF=0, AD16 is selected as input; when DIFF=1, it is reserved."]
    B10000 = 16,
    #[doc = "17: When DIFF=0, AD17 is selected as input; when DIFF=1, it is reserved."]
    B10001 = 17,
    #[doc = "18: When DIFF=0, AD18 is selected as input; when DIFF=1, it is reserved."]
    B10010 = 18,
    #[doc = "19: When DIFF=0, AD19 is selected as input; when DIFF=1, it is reserved."]
    B10011 = 19,
    #[doc = "20: When DIFF=0, AD20 is selected as input; when DIFF=1, it is reserved."]
    B10100 = 20,
    #[doc = "21: When DIFF=0, AD21 is selected as input; when DIFF=1, it is reserved."]
    B10101 = 21,
    #[doc = "22: When DIFF=0, AD22 is selected as input; when DIFF=1, it is reserved."]
    B10110 = 22,
    #[doc = "23: When DIFF=0, AD23 is selected as input; when DIFF=1, it is reserved."]
    B10111 = 23,
    #[doc = "26: When DIFF=0, Temp Sensor (single-ended) is selected as input; when DIFF=1, Temp Sensor (differential) is selected as input."]
    B11010 = 26,
    #[doc = "27: When DIFF=0, Bandgap (single-ended) is selected as input; when DIFF=1, Bandgap (differential) is selected as input."]
    B11011 = 27,
    #[doc = "29: When DIFF=0,VREFSH is selected as input; when DIFF=1, -VREFSH (differential) is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    B11101 = 29,
    #[doc = "30: When DIFF=0,VREFSL is selected as input; when DIFF=1, it is reserved. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    B11110 = 30,
    #[doc = "31: Module is disabled."]
    B11111 = 31,
}
impl From<Adch> for u8 {
    #[inline(always)]
    fn from(variant: Adch) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adch {
    type Ux = u8;
}
impl crate::IsEnum for Adch {}
#[doc = "Field `ADCH` reader - Input channel select"]
pub type AdchR = crate::FieldReader<Adch>;
impl AdchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adch> {
        match self.bits {
            0 => Some(Adch::B00000),
            1 => Some(Adch::B00001),
            2 => Some(Adch::B00010),
            3 => Some(Adch::B00011),
            4 => Some(Adch::B00100),
            5 => Some(Adch::B00101),
            6 => Some(Adch::B00110),
            7 => Some(Adch::B00111),
            8 => Some(Adch::B01000),
            9 => Some(Adch::B01001),
            10 => Some(Adch::B01010),
            11 => Some(Adch::B01011),
            12 => Some(Adch::B01100),
            13 => Some(Adch::B01101),
            14 => Some(Adch::B01110),
            15 => Some(Adch::B01111),
            16 => Some(Adch::B10000),
            17 => Some(Adch::B10001),
            18 => Some(Adch::B10010),
            19 => Some(Adch::B10011),
            20 => Some(Adch::B10100),
            21 => Some(Adch::B10101),
            22 => Some(Adch::B10110),
            23 => Some(Adch::B10111),
            26 => Some(Adch::B11010),
            27 => Some(Adch::B11011),
            29 => Some(Adch::B11101),
            30 => Some(Adch::B11110),
            31 => Some(Adch::B11111),
            _ => None,
        }
    }
    #[doc = "When DIFF=0, DADP0 is selected as input; when DIFF=1, DAD0 is selected as input."]
    #[inline(always)]
    pub fn is_b00000(&self) -> bool {
        *self == Adch::B00000
    }
    #[doc = "When DIFF=0, DADP1 is selected as input; when DIFF=1, DAD1 is selected as input."]
    #[inline(always)]
    pub fn is_b00001(&self) -> bool {
        *self == Adch::B00001
    }
    #[doc = "When DIFF=0, DADP2 is selected as input; when DIFF=1, DAD2 is selected as input."]
    #[inline(always)]
    pub fn is_b00010(&self) -> bool {
        *self == Adch::B00010
    }
    #[doc = "When DIFF=0, DADP3 is selected as input; when DIFF=1, DAD3 is selected as input."]
    #[inline(always)]
    pub fn is_b00011(&self) -> bool {
        *self == Adch::B00011
    }
    #[doc = "When DIFF=0, AD4 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b00100(&self) -> bool {
        *self == Adch::B00100
    }
    #[doc = "When DIFF=0, AD5 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b00101(&self) -> bool {
        *self == Adch::B00101
    }
    #[doc = "When DIFF=0, AD6 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b00110(&self) -> bool {
        *self == Adch::B00110
    }
    #[doc = "When DIFF=0, AD7 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b00111(&self) -> bool {
        *self == Adch::B00111
    }
    #[doc = "When DIFF=0, AD8 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b01000(&self) -> bool {
        *self == Adch::B01000
    }
    #[doc = "When DIFF=0, AD9 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b01001(&self) -> bool {
        *self == Adch::B01001
    }
    #[doc = "When DIFF=0, AD10 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b01010(&self) -> bool {
        *self == Adch::B01010
    }
    #[doc = "When DIFF=0, AD11 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b01011(&self) -> bool {
        *self == Adch::B01011
    }
    #[doc = "When DIFF=0, AD12 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b01100(&self) -> bool {
        *self == Adch::B01100
    }
    #[doc = "When DIFF=0, AD13 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b01101(&self) -> bool {
        *self == Adch::B01101
    }
    #[doc = "When DIFF=0, AD14 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b01110(&self) -> bool {
        *self == Adch::B01110
    }
    #[doc = "When DIFF=0, AD15 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b01111(&self) -> bool {
        *self == Adch::B01111
    }
    #[doc = "When DIFF=0, AD16 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b10000(&self) -> bool {
        *self == Adch::B10000
    }
    #[doc = "When DIFF=0, AD17 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b10001(&self) -> bool {
        *self == Adch::B10001
    }
    #[doc = "When DIFF=0, AD18 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b10010(&self) -> bool {
        *self == Adch::B10010
    }
    #[doc = "When DIFF=0, AD19 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b10011(&self) -> bool {
        *self == Adch::B10011
    }
    #[doc = "When DIFF=0, AD20 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b10100(&self) -> bool {
        *self == Adch::B10100
    }
    #[doc = "When DIFF=0, AD21 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b10101(&self) -> bool {
        *self == Adch::B10101
    }
    #[doc = "When DIFF=0, AD22 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b10110(&self) -> bool {
        *self == Adch::B10110
    }
    #[doc = "When DIFF=0, AD23 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn is_b10111(&self) -> bool {
        *self == Adch::B10111
    }
    #[doc = "When DIFF=0, Temp Sensor (single-ended) is selected as input; when DIFF=1, Temp Sensor (differential) is selected as input."]
    #[inline(always)]
    pub fn is_b11010(&self) -> bool {
        *self == Adch::B11010
    }
    #[doc = "When DIFF=0, Bandgap (single-ended) is selected as input; when DIFF=1, Bandgap (differential) is selected as input."]
    #[inline(always)]
    pub fn is_b11011(&self) -> bool {
        *self == Adch::B11011
    }
    #[doc = "When DIFF=0,VREFSH is selected as input; when DIFF=1, -VREFSH (differential) is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn is_b11101(&self) -> bool {
        *self == Adch::B11101
    }
    #[doc = "When DIFF=0,VREFSL is selected as input; when DIFF=1, it is reserved. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn is_b11110(&self) -> bool {
        *self == Adch::B11110
    }
    #[doc = "Module is disabled."]
    #[inline(always)]
    pub fn is_b11111(&self) -> bool {
        *self == Adch::B11111
    }
}
#[doc = "Field `ADCH` writer - Input channel select"]
pub type AdchW<'a, REG> = crate::FieldWriter<'a, REG, 5, Adch>;
impl<'a, REG> AdchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When DIFF=0, DADP0 is selected as input; when DIFF=1, DAD0 is selected as input."]
    #[inline(always)]
    pub fn b00000(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B00000)
    }
    #[doc = "When DIFF=0, DADP1 is selected as input; when DIFF=1, DAD1 is selected as input."]
    #[inline(always)]
    pub fn b00001(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B00001)
    }
    #[doc = "When DIFF=0, DADP2 is selected as input; when DIFF=1, DAD2 is selected as input."]
    #[inline(always)]
    pub fn b00010(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B00010)
    }
    #[doc = "When DIFF=0, DADP3 is selected as input; when DIFF=1, DAD3 is selected as input."]
    #[inline(always)]
    pub fn b00011(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B00011)
    }
    #[doc = "When DIFF=0, AD4 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b00100(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B00100)
    }
    #[doc = "When DIFF=0, AD5 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b00101(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B00101)
    }
    #[doc = "When DIFF=0, AD6 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b00110(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B00110)
    }
    #[doc = "When DIFF=0, AD7 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b00111(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B00111)
    }
    #[doc = "When DIFF=0, AD8 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b01000(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B01000)
    }
    #[doc = "When DIFF=0, AD9 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b01001(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B01001)
    }
    #[doc = "When DIFF=0, AD10 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b01010(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B01010)
    }
    #[doc = "When DIFF=0, AD11 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b01011(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B01011)
    }
    #[doc = "When DIFF=0, AD12 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b01100(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B01100)
    }
    #[doc = "When DIFF=0, AD13 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b01101(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B01101)
    }
    #[doc = "When DIFF=0, AD14 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b01110(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B01110)
    }
    #[doc = "When DIFF=0, AD15 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b01111(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B01111)
    }
    #[doc = "When DIFF=0, AD16 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b10000(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B10000)
    }
    #[doc = "When DIFF=0, AD17 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b10001(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B10001)
    }
    #[doc = "When DIFF=0, AD18 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b10010(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B10010)
    }
    #[doc = "When DIFF=0, AD19 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b10011(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B10011)
    }
    #[doc = "When DIFF=0, AD20 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b10100(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B10100)
    }
    #[doc = "When DIFF=0, AD21 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b10101(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B10101)
    }
    #[doc = "When DIFF=0, AD22 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b10110(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B10110)
    }
    #[doc = "When DIFF=0, AD23 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn b10111(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B10111)
    }
    #[doc = "When DIFF=0, Temp Sensor (single-ended) is selected as input; when DIFF=1, Temp Sensor (differential) is selected as input."]
    #[inline(always)]
    pub fn b11010(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B11010)
    }
    #[doc = "When DIFF=0, Bandgap (single-ended) is selected as input; when DIFF=1, Bandgap (differential) is selected as input."]
    #[inline(always)]
    pub fn b11011(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B11011)
    }
    #[doc = "When DIFF=0,VREFSH is selected as input; when DIFF=1, -VREFSH (differential) is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn b11101(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B11101)
    }
    #[doc = "When DIFF=0,VREFSL is selected as input; when DIFF=1, it is reserved. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn b11110(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B11110)
    }
    #[doc = "Module is disabled."]
    #[inline(always)]
    pub fn b11111(self) -> &'a mut crate::W<REG> {
        self.variant(Adch::B11111)
    }
}
#[doc = "Differential Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diff {
    #[doc = "0: Single-ended conversions and input channels are selected."]
    B0 = 0,
    #[doc = "1: Differential conversions and input channels are selected."]
    B1 = 1,
}
impl From<Diff> for bool {
    #[inline(always)]
    fn from(variant: Diff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIFF` reader - Differential Mode Enable"]
pub type DiffR = crate::BitReader<Diff>;
impl DiffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Diff {
        match self.bits {
            false => Diff::B0,
            true => Diff::B1,
        }
    }
    #[doc = "Single-ended conversions and input channels are selected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Diff::B0
    }
    #[doc = "Differential conversions and input channels are selected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Diff::B1
    }
}
#[doc = "Field `DIFF` writer - Differential Mode Enable"]
pub type DiffW<'a, REG> = crate::BitWriter<'a, REG, Diff>;
impl<'a, REG> DiffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single-ended conversions and input channels are selected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Diff::B0)
    }
    #[doc = "Differential conversions and input channels are selected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Diff::B1)
    }
}
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aien {
    #[doc = "0: Conversion complete interrupt is disabled."]
    B0 = 0,
    #[doc = "1: Conversion complete interrupt is enabled."]
    B1 = 1,
}
impl From<Aien> for bool {
    #[inline(always)]
    fn from(variant: Aien) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AIEN` reader - Interrupt Enable"]
pub type AienR = crate::BitReader<Aien>;
impl AienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aien {
        match self.bits {
            false => Aien::B0,
            true => Aien::B1,
        }
    }
    #[doc = "Conversion complete interrupt is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Aien::B0
    }
    #[doc = "Conversion complete interrupt is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Aien::B1
    }
}
#[doc = "Field `AIEN` writer - Interrupt Enable"]
pub type AienW<'a, REG> = crate::BitWriter<'a, REG, Aien>;
impl<'a, REG> AienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Conversion complete interrupt is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Aien::B0)
    }
    #[doc = "Conversion complete interrupt is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Aien::B1)
    }
}
#[doc = "Conversion Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Coco {
    #[doc = "0: Conversion is not completed."]
    B0 = 0,
    #[doc = "1: Conversion is completed."]
    B1 = 1,
}
impl From<Coco> for bool {
    #[inline(always)]
    fn from(variant: Coco) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COCO` reader - Conversion Complete Flag"]
pub type CocoR = crate::BitReader<Coco>;
impl CocoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Coco {
        match self.bits {
            false => Coco::B0,
            true => Coco::B1,
        }
    }
    #[doc = "Conversion is not completed."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Coco::B0
    }
    #[doc = "Conversion is completed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Coco::B1
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&self) -> AdchR {
        AdchR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Differential Mode Enable"]
    #[inline(always)]
    pub fn diff(&self) -> DiffR {
        DiffR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&self) -> AienR {
        AienR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Conversion Complete Flag"]
    #[inline(always)]
    pub fn coco(&self) -> CocoR {
        CocoR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    #[must_use]
    pub fn adch(&mut self) -> AdchW<Sc1Spec> {
        AdchW::new(self, 0)
    }
    #[doc = "Bit 5 - Differential Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DiffW<Sc1Spec> {
        DiffW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aien(&mut self) -> AienW<Sc1Spec> {
        AienW::new(self, 6)
    }
}
#[doc = "ADC Status and Control Registers 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sc1Spec;
impl crate::RegisterSpec for Sc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc1::R`](R) reader structure"]
impl crate::Readable for Sc1Spec {}
#[doc = "`write(|w| ..)` method takes [`sc1::W`](W) writer structure"]
impl crate::Writable for Sc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SC1%s to value 0x1f"]
impl crate::Resettable for Sc1Spec {
    const RESET_VALUE: u32 = 0x1f;
}
