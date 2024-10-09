#[doc = "Register `BR` reader"]
pub type R = crate::R<BrSpec>;
#[doc = "Register `BR` writer"]
pub type W = crate::W<BrSpec>;
#[doc = "SPI Baud Rate Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Spr {
    #[doc = "0: Baud rate divisor is 2."]
    B0000 = 0,
    #[doc = "1: Baud rate divisor is 4."]
    B0001 = 1,
    #[doc = "2: Baud rate divisor is 8."]
    B0010 = 2,
    #[doc = "3: Baud rate divisor is 16."]
    B0011 = 3,
    #[doc = "4: Baud rate divisor is 32."]
    B0100 = 4,
    #[doc = "5: Baud rate divisor is 64."]
    B0101 = 5,
    #[doc = "6: Baud rate divisor is 128."]
    B0110 = 6,
    #[doc = "7: Baud rate divisor is 256."]
    B0111 = 7,
    #[doc = "8: Baud rate divisor is 512."]
    B1000 = 8,
}
impl From<Spr> for u8 {
    #[inline(always)]
    fn from(variant: Spr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Spr {
    type Ux = u8;
}
impl crate::IsEnum for Spr {}
#[doc = "Field `SPR` reader - SPI Baud Rate Divisor"]
pub type SprR = crate::FieldReader<Spr>;
impl SprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Spr> {
        match self.bits {
            0 => Some(Spr::B0000),
            1 => Some(Spr::B0001),
            2 => Some(Spr::B0010),
            3 => Some(Spr::B0011),
            4 => Some(Spr::B0100),
            5 => Some(Spr::B0101),
            6 => Some(Spr::B0110),
            7 => Some(Spr::B0111),
            8 => Some(Spr::B1000),
            _ => None,
        }
    }
    #[doc = "Baud rate divisor is 2."]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Spr::B0000
    }
    #[doc = "Baud rate divisor is 4."]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Spr::B0001
    }
    #[doc = "Baud rate divisor is 8."]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Spr::B0010
    }
    #[doc = "Baud rate divisor is 16."]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Spr::B0011
    }
    #[doc = "Baud rate divisor is 32."]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Spr::B0100
    }
    #[doc = "Baud rate divisor is 64."]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Spr::B0101
    }
    #[doc = "Baud rate divisor is 128."]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Spr::B0110
    }
    #[doc = "Baud rate divisor is 256."]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == Spr::B0111
    }
    #[doc = "Baud rate divisor is 512."]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Spr::B1000
    }
}
#[doc = "Field `SPR` writer - SPI Baud Rate Divisor"]
pub type SprW<'a, REG> = crate::FieldWriter<'a, REG, 4, Spr>;
impl<'a, REG> SprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Baud rate divisor is 2."]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Spr::B0000)
    }
    #[doc = "Baud rate divisor is 4."]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Spr::B0001)
    }
    #[doc = "Baud rate divisor is 8."]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(Spr::B0010)
    }
    #[doc = "Baud rate divisor is 16."]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(Spr::B0011)
    }
    #[doc = "Baud rate divisor is 32."]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Spr::B0100)
    }
    #[doc = "Baud rate divisor is 64."]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Spr::B0101)
    }
    #[doc = "Baud rate divisor is 128."]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(Spr::B0110)
    }
    #[doc = "Baud rate divisor is 256."]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(Spr::B0111)
    }
    #[doc = "Baud rate divisor is 512."]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Spr::B1000)
    }
}
#[doc = "SPI Baud Rate Prescale Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sppr {
    #[doc = "0: Baud rate prescaler divisor is 1."]
    B000 = 0,
    #[doc = "1: Baud rate prescaler divisor is 2."]
    B001 = 1,
    #[doc = "2: Baud rate prescaler divisor is 3."]
    B010 = 2,
    #[doc = "3: Baud rate prescaler divisor is 4."]
    B011 = 3,
    #[doc = "4: Baud rate prescaler divisor is 5."]
    B100 = 4,
    #[doc = "5: Baud rate prescaler divisor is 6."]
    B101 = 5,
    #[doc = "6: Baud rate prescaler divisor is 7."]
    B110 = 6,
    #[doc = "7: Baud rate prescaler divisor is 8."]
    B111 = 7,
}
impl From<Sppr> for u8 {
    #[inline(always)]
    fn from(variant: Sppr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sppr {
    type Ux = u8;
}
impl crate::IsEnum for Sppr {}
#[doc = "Field `SPPR` reader - SPI Baud Rate Prescale Divisor"]
pub type SpprR = crate::FieldReader<Sppr>;
impl SpprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sppr {
        match self.bits {
            0 => Sppr::B000,
            1 => Sppr::B001,
            2 => Sppr::B010,
            3 => Sppr::B011,
            4 => Sppr::B100,
            5 => Sppr::B101,
            6 => Sppr::B110,
            7 => Sppr::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Baud rate prescaler divisor is 1."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Sppr::B000
    }
    #[doc = "Baud rate prescaler divisor is 2."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Sppr::B001
    }
    #[doc = "Baud rate prescaler divisor is 3."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Sppr::B010
    }
    #[doc = "Baud rate prescaler divisor is 4."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Sppr::B011
    }
    #[doc = "Baud rate prescaler divisor is 5."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Sppr::B100
    }
    #[doc = "Baud rate prescaler divisor is 6."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Sppr::B101
    }
    #[doc = "Baud rate prescaler divisor is 7."]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Sppr::B110
    }
    #[doc = "Baud rate prescaler divisor is 8."]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Sppr::B111
    }
}
#[doc = "Field `SPPR` writer - SPI Baud Rate Prescale Divisor"]
pub type SpprW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sppr, crate::Safe>;
impl<'a, REG> SpprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Baud rate prescaler divisor is 1."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Sppr::B000)
    }
    #[doc = "Baud rate prescaler divisor is 2."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Sppr::B001)
    }
    #[doc = "Baud rate prescaler divisor is 3."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Sppr::B010)
    }
    #[doc = "Baud rate prescaler divisor is 4."]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Sppr::B011)
    }
    #[doc = "Baud rate prescaler divisor is 5."]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Sppr::B100)
    }
    #[doc = "Baud rate prescaler divisor is 6."]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Sppr::B101)
    }
    #[doc = "Baud rate prescaler divisor is 7."]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Sppr::B110)
    }
    #[doc = "Baud rate prescaler divisor is 8."]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Sppr::B111)
    }
}
impl R {
    #[doc = "Bits 0:3 - SPI Baud Rate Divisor"]
    #[inline(always)]
    pub fn spr(&self) -> SprR {
        SprR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - SPI Baud Rate Prescale Divisor"]
    #[inline(always)]
    pub fn sppr(&self) -> SpprR {
        SpprR::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:3 - SPI Baud Rate Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn spr(&mut self) -> SprW<BrSpec> {
        SprW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SPI Baud Rate Prescale Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn sppr(&mut self) -> SpprW<BrSpec> {
        SpprW::new(self, 4)
    }
}
#[doc = "SPI Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`br::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrSpec;
impl crate::RegisterSpec for BrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`br::R`](R) reader structure"]
impl crate::Readable for BrSpec {}
#[doc = "`write(|w| ..)` method takes [`br::W`](W) writer structure"]
impl crate::Writable for BrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BR to value 0"]
impl crate::Resettable for BrSpec {
    const RESET_VALUE: u8 = 0;
}
