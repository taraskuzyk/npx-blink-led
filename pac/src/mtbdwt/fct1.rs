#[doc = "Register `FCT1` reader"]
pub type R = crate::R<Fct1Spec>;
#[doc = "Register `FCT1` writer"]
pub type W = crate::W<Fct1Spec>;
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
    pub fn function(&mut self) -> FunctionW<Fct1Spec> {
        FunctionW::new(self, 0)
    }
}
#[doc = "MTB_DWT Comparator Function Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fct1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fct1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fct1Spec;
impl crate::RegisterSpec for Fct1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fct1::R`](R) reader structure"]
impl crate::Readable for Fct1Spec {}
#[doc = "`write(|w| ..)` method takes [`fct1::W`](W) writer structure"]
impl crate::Writable for Fct1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCT1 to value 0"]
impl crate::Resettable for Fct1Spec {
    const RESET_VALUE: u32 = 0;
}
