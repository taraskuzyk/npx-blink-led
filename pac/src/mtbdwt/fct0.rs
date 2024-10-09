#[doc = "Register `FCT0` reader"]
pub type R = crate::R<Fct0Spec>;
#[doc = "Register `FCT0` writer"]
pub type W = crate::W<Fct0Spec>;
#[doc = "Function\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Function {
    #[doc = "0: Disabled."]
    B0000 = 0,
    #[doc = "4: Instruction fetch."]
    B0100 = 4,
    #[doc = "5: Data operand read."]
    B0101 = 5,
    #[doc = "6: Data operand write."]
    B0110 = 6,
    #[doc = "7: Data operand (read + write)."]
    B0111 = 7,
}
impl From<Function> for u8 {
    #[inline(always)]
    fn from(variant: Function) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Function {
    type Ux = u8;
}
impl crate::IsEnum for Function {}
#[doc = "Field `FUNCTION` reader - Function"]
pub type FunctionR = crate::FieldReader<Function>;
impl FunctionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Function> {
        match self.bits {
            0 => Some(Function::B0000),
            4 => Some(Function::B0100),
            5 => Some(Function::B0101),
            6 => Some(Function::B0110),
            7 => Some(Function::B0111),
            _ => None,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Function::B0000
    }
    #[doc = "Instruction fetch."]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Function::B0100
    }
    #[doc = "Data operand read."]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Function::B0101
    }
    #[doc = "Data operand write."]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Function::B0110
    }
    #[doc = "Data operand (read + write)."]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == Function::B0111
    }
}
#[doc = "Field `FUNCTION` writer - Function"]
pub type FunctionW<'a, REG> = crate::FieldWriter<'a, REG, 4, Function>;
impl<'a, REG> FunctionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Function::B0000)
    }
    #[doc = "Instruction fetch."]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Function::B0100)
    }
    #[doc = "Data operand read."]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Function::B0101)
    }
    #[doc = "Data operand write."]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(Function::B0110)
    }
    #[doc = "Data operand (read + write)."]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(Function::B0111)
    }
}
#[doc = "Data Value Match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datavmatch {
    #[doc = "0: Perform address comparison."]
    B0 = 0,
    #[doc = "1: Perform data value comparison."]
    B1 = 1,
}
impl From<Datavmatch> for bool {
    #[inline(always)]
    fn from(variant: Datavmatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DATAVMATCH` reader - Data Value Match"]
pub type DatavmatchR = crate::BitReader<Datavmatch>;
impl DatavmatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datavmatch {
        match self.bits {
            false => Datavmatch::B0,
            true => Datavmatch::B1,
        }
    }
    #[doc = "Perform address comparison."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Datavmatch::B0
    }
    #[doc = "Perform data value comparison."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Datavmatch::B1
    }
}
#[doc = "Field `DATAVMATCH` writer - Data Value Match"]
pub type DatavmatchW<'a, REG> = crate::BitWriter<'a, REG, Datavmatch>;
impl<'a, REG> DatavmatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Perform address comparison."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Datavmatch::B0)
    }
    #[doc = "Perform data value comparison."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Datavmatch::B1)
    }
}
#[doc = "Data Value Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datavsize {
    #[doc = "0: Byte."]
    B00 = 0,
    #[doc = "1: Halfword."]
    B01 = 1,
    #[doc = "2: Word."]
    B10 = 2,
    #[doc = "3: Reserved. Any attempts to use this value results in UNPREDICTABLE behavior."]
    B11 = 3,
}
impl From<Datavsize> for u8 {
    #[inline(always)]
    fn from(variant: Datavsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datavsize {
    type Ux = u8;
}
impl crate::IsEnum for Datavsize {}
#[doc = "Field `DATAVSIZE` reader - Data Value Size"]
pub type DatavsizeR = crate::FieldReader<Datavsize>;
impl DatavsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Datavsize {
        match self.bits {
            0 => Datavsize::B00,
            1 => Datavsize::B01,
            2 => Datavsize::B10,
            3 => Datavsize::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Byte."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Datavsize::B00
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Datavsize::B01
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Datavsize::B10
    }
    #[doc = "Reserved. Any attempts to use this value results in UNPREDICTABLE behavior."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Datavsize::B11
    }
}
#[doc = "Field `DATAVSIZE` writer - Data Value Size"]
pub type DatavsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Datavsize, crate::Safe>;
impl<'a, REG> DatavsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Datavsize::B00)
    }
    #[doc = "Halfword."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Datavsize::B01)
    }
    #[doc = "Word."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Datavsize::B10)
    }
    #[doc = "Reserved. Any attempts to use this value results in UNPREDICTABLE behavior."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Datavsize::B11)
    }
}
#[doc = "Field `DATAVADDR0` reader - Data Value Address 0"]
pub type Datavaddr0R = crate::FieldReader;
#[doc = "Field `DATAVADDR0` writer - Data Value Address 0"]
pub type Datavaddr0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Comparator match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Matched {
    #[doc = "0: No match."]
    B0 = 0,
    #[doc = "1: Match occurred."]
    B1 = 1,
}
impl From<Matched> for bool {
    #[inline(always)]
    fn from(variant: Matched) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MATCHED` reader - Comparator match"]
pub type MatchedR = crate::BitReader<Matched>;
impl MatchedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Matched {
        match self.bits {
            false => Matched::B0,
            true => Matched::B1,
        }
    }
    #[doc = "No match."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Matched::B0
    }
    #[doc = "Match occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Matched::B1
    }
}
impl R {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    pub fn function(&self) -> FunctionR {
        FunctionR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Data Value Match"]
    #[inline(always)]
    pub fn datavmatch(&self) -> DatavmatchR {
        DatavmatchR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Data Value Size"]
    #[inline(always)]
    pub fn datavsize(&self) -> DatavsizeR {
        DatavsizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Data Value Address 0"]
    #[inline(always)]
    pub fn datavaddr0(&self) -> Datavaddr0R {
        Datavaddr0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Comparator match"]
    #[inline(always)]
    pub fn matched(&self) -> MatchedR {
        MatchedR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Function"]
    #[inline(always)]
    #[must_use]
    pub fn function(&mut self) -> FunctionW<Fct0Spec> {
        FunctionW::new(self, 0)
    }
    #[doc = "Bit 8 - Data Value Match"]
    #[inline(always)]
    #[must_use]
    pub fn datavmatch(&mut self) -> DatavmatchW<Fct0Spec> {
        DatavmatchW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Data Value Size"]
    #[inline(always)]
    #[must_use]
    pub fn datavsize(&mut self) -> DatavsizeW<Fct0Spec> {
        DatavsizeW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Data Value Address 0"]
    #[inline(always)]
    #[must_use]
    pub fn datavaddr0(&mut self) -> Datavaddr0W<Fct0Spec> {
        Datavaddr0W::new(self, 12)
    }
}
#[doc = "MTB_DWT Comparator Function Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fct0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fct0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fct0Spec;
impl crate::RegisterSpec for Fct0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fct0::R`](R) reader structure"]
impl crate::Readable for Fct0Spec {}
#[doc = "`write(|w| ..)` method takes [`fct0::W`](W) writer structure"]
impl crate::Writable for Fct0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCT0 to value 0"]
impl crate::Resettable for Fct0Spec {
    const RESET_VALUE: u32 = 0;
}
