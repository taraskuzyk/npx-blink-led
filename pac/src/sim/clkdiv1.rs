#[doc = "Register `CLKDIV1` reader"]
pub type R = crate::R<Clkdiv1Spec>;
#[doc = "Register `CLKDIV1` writer"]
pub type W = crate::W<Clkdiv1Spec>;
#[doc = "Clock 4 Output Divider value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outdiv4 {
    #[doc = "0: Divide-by-1."]
    B000 = 0,
    #[doc = "1: Divide-by-2."]
    B001 = 1,
    #[doc = "2: Divide-by-3."]
    B010 = 2,
    #[doc = "3: Divide-by-4."]
    B011 = 3,
    #[doc = "4: Divide-by-5."]
    B100 = 4,
    #[doc = "5: Divide-by-6."]
    B101 = 5,
    #[doc = "6: Divide-by-7."]
    B110 = 6,
    #[doc = "7: Divide-by-8."]
    B111 = 7,
}
impl From<Outdiv4> for u8 {
    #[inline(always)]
    fn from(variant: Outdiv4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outdiv4 {
    type Ux = u8;
}
impl crate::IsEnum for Outdiv4 {}
#[doc = "Field `OUTDIV4` reader - Clock 4 Output Divider value"]
pub type Outdiv4R = crate::FieldReader<Outdiv4>;
impl Outdiv4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outdiv4 {
        match self.bits {
            0 => Outdiv4::B000,
            1 => Outdiv4::B001,
            2 => Outdiv4::B010,
            3 => Outdiv4::B011,
            4 => Outdiv4::B100,
            5 => Outdiv4::B101,
            6 => Outdiv4::B110,
            7 => Outdiv4::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide-by-1."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Outdiv4::B000
    }
    #[doc = "Divide-by-2."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Outdiv4::B001
    }
    #[doc = "Divide-by-3."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Outdiv4::B010
    }
    #[doc = "Divide-by-4."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Outdiv4::B011
    }
    #[doc = "Divide-by-5."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Outdiv4::B100
    }
    #[doc = "Divide-by-6."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Outdiv4::B101
    }
    #[doc = "Divide-by-7."]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Outdiv4::B110
    }
    #[doc = "Divide-by-8."]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Outdiv4::B111
    }
}
#[doc = "Field `OUTDIV4` writer - Clock 4 Output Divider value"]
pub type Outdiv4W<'a, REG> = crate::FieldWriter<'a, REG, 3, Outdiv4, crate::Safe>;
impl<'a, REG> Outdiv4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide-by-1."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::B000)
    }
    #[doc = "Divide-by-2."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::B001)
    }
    #[doc = "Divide-by-3."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::B010)
    }
    #[doc = "Divide-by-4."]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::B011)
    }
    #[doc = "Divide-by-5."]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::B100)
    }
    #[doc = "Divide-by-6."]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::B101)
    }
    #[doc = "Divide-by-7."]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::B110)
    }
    #[doc = "Divide-by-8."]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv4::B111)
    }
}
#[doc = "Clock 1 Output Divider value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Outdiv1 {
    #[doc = "0: Divide-by-1."]
    B0000 = 0,
    #[doc = "1: Divide-by-2."]
    B0001 = 1,
    #[doc = "2: Divide-by-3."]
    B0010 = 2,
    #[doc = "3: Divide-by-4."]
    B0011 = 3,
    #[doc = "4: Divide-by-5."]
    B0100 = 4,
    #[doc = "5: Divide-by-6."]
    B0101 = 5,
    #[doc = "6: Divide-by-7."]
    B0110 = 6,
    #[doc = "7: Divide-by-8."]
    B0111 = 7,
    #[doc = "8: Divide-by-9."]
    B1000 = 8,
    #[doc = "9: Divide-by-10."]
    B1001 = 9,
    #[doc = "10: Divide-by-11."]
    B1010 = 10,
    #[doc = "11: Divide-by-12."]
    B1011 = 11,
    #[doc = "12: Divide-by-13."]
    B1100 = 12,
    #[doc = "13: Divide-by-14."]
    B1101 = 13,
    #[doc = "14: Divide-by-15."]
    B1110 = 14,
    #[doc = "15: Divide-by-16."]
    B1111 = 15,
}
impl From<Outdiv1> for u8 {
    #[inline(always)]
    fn from(variant: Outdiv1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Outdiv1 {
    type Ux = u8;
}
impl crate::IsEnum for Outdiv1 {}
#[doc = "Field `OUTDIV1` reader - Clock 1 Output Divider value"]
pub type Outdiv1R = crate::FieldReader<Outdiv1>;
impl Outdiv1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Outdiv1 {
        match self.bits {
            0 => Outdiv1::B0000,
            1 => Outdiv1::B0001,
            2 => Outdiv1::B0010,
            3 => Outdiv1::B0011,
            4 => Outdiv1::B0100,
            5 => Outdiv1::B0101,
            6 => Outdiv1::B0110,
            7 => Outdiv1::B0111,
            8 => Outdiv1::B1000,
            9 => Outdiv1::B1001,
            10 => Outdiv1::B1010,
            11 => Outdiv1::B1011,
            12 => Outdiv1::B1100,
            13 => Outdiv1::B1101,
            14 => Outdiv1::B1110,
            15 => Outdiv1::B1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide-by-1."]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Outdiv1::B0000
    }
    #[doc = "Divide-by-2."]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Outdiv1::B0001
    }
    #[doc = "Divide-by-3."]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Outdiv1::B0010
    }
    #[doc = "Divide-by-4."]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Outdiv1::B0011
    }
    #[doc = "Divide-by-5."]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Outdiv1::B0100
    }
    #[doc = "Divide-by-6."]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Outdiv1::B0101
    }
    #[doc = "Divide-by-7."]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Outdiv1::B0110
    }
    #[doc = "Divide-by-8."]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == Outdiv1::B0111
    }
    #[doc = "Divide-by-9."]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Outdiv1::B1000
    }
    #[doc = "Divide-by-10."]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Outdiv1::B1001
    }
    #[doc = "Divide-by-11."]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == Outdiv1::B1010
    }
    #[doc = "Divide-by-12."]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == Outdiv1::B1011
    }
    #[doc = "Divide-by-13."]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == Outdiv1::B1100
    }
    #[doc = "Divide-by-14."]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == Outdiv1::B1101
    }
    #[doc = "Divide-by-15."]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == Outdiv1::B1110
    }
    #[doc = "Divide-by-16."]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == Outdiv1::B1111
    }
}
#[doc = "Field `OUTDIV1` writer - Clock 1 Output Divider value"]
pub type Outdiv1W<'a, REG> = crate::FieldWriter<'a, REG, 4, Outdiv1, crate::Safe>;
impl<'a, REG> Outdiv1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide-by-1."]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B0000)
    }
    #[doc = "Divide-by-2."]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B0001)
    }
    #[doc = "Divide-by-3."]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B0010)
    }
    #[doc = "Divide-by-4."]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B0011)
    }
    #[doc = "Divide-by-5."]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B0100)
    }
    #[doc = "Divide-by-6."]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B0101)
    }
    #[doc = "Divide-by-7."]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B0110)
    }
    #[doc = "Divide-by-8."]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B0111)
    }
    #[doc = "Divide-by-9."]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B1000)
    }
    #[doc = "Divide-by-10."]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B1001)
    }
    #[doc = "Divide-by-11."]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B1010)
    }
    #[doc = "Divide-by-12."]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B1011)
    }
    #[doc = "Divide-by-13."]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B1100)
    }
    #[doc = "Divide-by-14."]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B1101)
    }
    #[doc = "Divide-by-15."]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B1110)
    }
    #[doc = "Divide-by-16."]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(Outdiv1::B1111)
    }
}
impl R {
    #[doc = "Bits 16:18 - Clock 4 Output Divider value"]
    #[inline(always)]
    pub fn outdiv4(&self) -> Outdiv4R {
        Outdiv4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Clock 1 Output Divider value"]
    #[inline(always)]
    pub fn outdiv1(&self) -> Outdiv1R {
        Outdiv1R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - Clock 4 Output Divider value"]
    #[inline(always)]
    #[must_use]
    pub fn outdiv4(&mut self) -> Outdiv4W<Clkdiv1Spec> {
        Outdiv4W::new(self, 16)
    }
    #[doc = "Bits 28:31 - Clock 1 Output Divider value"]
    #[inline(always)]
    #[must_use]
    pub fn outdiv1(&mut self) -> Outdiv1W<Clkdiv1Spec> {
        Outdiv1W::new(self, 28)
    }
}
#[doc = "System Clock Divider Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Clkdiv1Spec;
impl crate::RegisterSpec for Clkdiv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkdiv1::R`](R) reader structure"]
impl crate::Readable for Clkdiv1Spec {}
#[doc = "`write(|w| ..)` method takes [`clkdiv1::W`](W) writer structure"]
impl crate::Writable for Clkdiv1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKDIV1 to value 0x0001_0000"]
impl crate::Resettable for Clkdiv1Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
