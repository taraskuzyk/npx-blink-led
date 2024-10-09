#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `R0T0` reader - Read receive data buffer 0 or write transmit data buffer 0."]
pub type R0t0R = crate::BitReader;
#[doc = "Field `R0T0` writer - Read receive data buffer 0 or write transmit data buffer 0."]
pub type R0t0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R1T1` reader - Read receive data buffer 1 or write transmit data buffer 1."]
pub type R1t1R = crate::BitReader;
#[doc = "Field `R1T1` writer - Read receive data buffer 1 or write transmit data buffer 1."]
pub type R1t1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R2T2` reader - Read receive data buffer 2 or write transmit data buffer 2."]
pub type R2t2R = crate::BitReader;
#[doc = "Field `R2T2` writer - Read receive data buffer 2 or write transmit data buffer 2."]
pub type R2t2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R3T3` reader - Read receive data buffer 3 or write transmit data buffer 3."]
pub type R3t3R = crate::BitReader;
#[doc = "Field `R3T3` writer - Read receive data buffer 3 or write transmit data buffer 3."]
pub type R3t3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R4T4` reader - Read receive data buffer 4 or write transmit data buffer 4."]
pub type R4t4R = crate::BitReader;
#[doc = "Field `R4T4` writer - Read receive data buffer 4 or write transmit data buffer 4."]
pub type R4t4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R5T5` reader - Read receive data buffer 5 or write transmit data buffer 5."]
pub type R5t5R = crate::BitReader;
#[doc = "Field `R5T5` writer - Read receive data buffer 5 or write transmit data buffer 5."]
pub type R5t5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R6T6` reader - Read receive data buffer 6 or write transmit data buffer 6."]
pub type R6t6R = crate::BitReader;
#[doc = "Field `R6T6` writer - Read receive data buffer 6 or write transmit data buffer 6."]
pub type R6t6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R7T7` reader - Read receive data buffer 7 or write transmit data buffer 7."]
pub type R7t7R = crate::BitReader;
#[doc = "Field `R7T7` writer - Read receive data buffer 7 or write transmit data buffer 7."]
pub type R7t7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R8T8` reader - Read receive data buffer 8 or write transmit data buffer 8."]
pub type R8t8R = crate::BitReader;
#[doc = "Field `R8T8` writer - Read receive data buffer 8 or write transmit data buffer 8."]
pub type R8t8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R9T9` reader - Read receive data buffer 9 or write transmit data buffer 9."]
pub type R9t9R = crate::BitReader;
#[doc = "Field `R9T9` writer - Read receive data buffer 9 or write transmit data buffer 9."]
pub type R9t9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Idle Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Idline {
    #[doc = "0: Receiver was not idle before receiving this character."]
    B0 = 0,
    #[doc = "1: Receiver was idle before receiving this character."]
    B1 = 1,
}
impl From<Idline> for bool {
    #[inline(always)]
    fn from(variant: Idline) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDLINE` reader - Idle Line"]
pub type IdlineR = crate::BitReader<Idline>;
impl IdlineR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Idline {
        match self.bits {
            false => Idline::B0,
            true => Idline::B1,
        }
    }
    #[doc = "Receiver was not idle before receiving this character."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Idline::B0
    }
    #[doc = "Receiver was idle before receiving this character."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Idline::B1
    }
}
#[doc = "Receive Buffer Empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxempt {
    #[doc = "0: Receive buffer contains valid data."]
    B0 = 0,
    #[doc = "1: Receive buffer is empty, data returned on read is not valid."]
    B1 = 1,
}
impl From<Rxempt> for bool {
    #[inline(always)]
    fn from(variant: Rxempt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEMPT` reader - Receive Buffer Empty"]
pub type RxemptR = crate::BitReader<Rxempt>;
impl RxemptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxempt {
        match self.bits {
            false => Rxempt::B0,
            true => Rxempt::B1,
        }
    }
    #[doc = "Receive buffer contains valid data."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxempt::B0
    }
    #[doc = "Receive buffer is empty, data returned on read is not valid."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxempt::B1
    }
}
#[doc = "Frame Error / Transmit Special Character\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fretsc {
    #[doc = "0: The dataword was received without a frame error on read, transmit a normal character on write."]
    B0 = 0,
    #[doc = "1: The dataword was received with a frame error, transmit an idle or break character on transmit."]
    B1 = 1,
}
impl From<Fretsc> for bool {
    #[inline(always)]
    fn from(variant: Fretsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRETSC` reader - Frame Error / Transmit Special Character"]
pub type FretscR = crate::BitReader<Fretsc>;
impl FretscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fretsc {
        match self.bits {
            false => Fretsc::B0,
            true => Fretsc::B1,
        }
    }
    #[doc = "The dataword was received without a frame error on read, transmit a normal character on write."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fretsc::B0
    }
    #[doc = "The dataword was received with a frame error, transmit an idle or break character on transmit."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fretsc::B1
    }
}
#[doc = "Field `FRETSC` writer - Frame Error / Transmit Special Character"]
pub type FretscW<'a, REG> = crate::BitWriter<'a, REG, Fretsc>;
impl<'a, REG> FretscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The dataword was received without a frame error on read, transmit a normal character on write."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fretsc::B0)
    }
    #[doc = "The dataword was received with a frame error, transmit an idle or break character on transmit."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fretsc::B1)
    }
}
#[doc = "The current received dataword contained in DATA\\[R9:R0\\]
was received with a parity error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Paritye {
    #[doc = "0: The dataword was received without a parity error."]
    B0 = 0,
    #[doc = "1: The dataword was received with a parity error."]
    B1 = 1,
}
impl From<Paritye> for bool {
    #[inline(always)]
    fn from(variant: Paritye) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITYE` reader - The current received dataword contained in DATA\\[R9:R0\\]
was received with a parity error."]
pub type ParityeR = crate::BitReader<Paritye>;
impl ParityeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Paritye {
        match self.bits {
            false => Paritye::B0,
            true => Paritye::B1,
        }
    }
    #[doc = "The dataword was received without a parity error."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Paritye::B0
    }
    #[doc = "The dataword was received with a parity error."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Paritye::B1
    }
}
#[doc = "The current received dataword contained in DATA\\[R9:R0\\]
was received with noise.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noisy {
    #[doc = "0: The dataword was received without noise."]
    B0 = 0,
    #[doc = "1: The data was received with noise."]
    B1 = 1,
}
impl From<Noisy> for bool {
    #[inline(always)]
    fn from(variant: Noisy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOISY` reader - The current received dataword contained in DATA\\[R9:R0\\]
was received with noise."]
pub type NoisyR = crate::BitReader<Noisy>;
impl NoisyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Noisy {
        match self.bits {
            false => Noisy::B0,
            true => Noisy::B1,
        }
    }
    #[doc = "The dataword was received without noise."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Noisy::B0
    }
    #[doc = "The data was received with noise."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Noisy::B1
    }
}
impl R {
    #[doc = "Bit 0 - Read receive data buffer 0 or write transmit data buffer 0."]
    #[inline(always)]
    pub fn r0t0(&self) -> R0t0R {
        R0t0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Read receive data buffer 1 or write transmit data buffer 1."]
    #[inline(always)]
    pub fn r1t1(&self) -> R1t1R {
        R1t1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Read receive data buffer 2 or write transmit data buffer 2."]
    #[inline(always)]
    pub fn r2t2(&self) -> R2t2R {
        R2t2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read receive data buffer 3 or write transmit data buffer 3."]
    #[inline(always)]
    pub fn r3t3(&self) -> R3t3R {
        R3t3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Read receive data buffer 4 or write transmit data buffer 4."]
    #[inline(always)]
    pub fn r4t4(&self) -> R4t4R {
        R4t4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Read receive data buffer 5 or write transmit data buffer 5."]
    #[inline(always)]
    pub fn r5t5(&self) -> R5t5R {
        R5t5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Read receive data buffer 6 or write transmit data buffer 6."]
    #[inline(always)]
    pub fn r6t6(&self) -> R6t6R {
        R6t6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Read receive data buffer 7 or write transmit data buffer 7."]
    #[inline(always)]
    pub fn r7t7(&self) -> R7t7R {
        R7t7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Read receive data buffer 8 or write transmit data buffer 8."]
    #[inline(always)]
    pub fn r8t8(&self) -> R8t8R {
        R8t8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read receive data buffer 9 or write transmit data buffer 9."]
    #[inline(always)]
    pub fn r9t9(&self) -> R9t9R {
        R9t9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Idle Line"]
    #[inline(always)]
    pub fn idline(&self) -> IdlineR {
        IdlineR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive Buffer Empty"]
    #[inline(always)]
    pub fn rxempt(&self) -> RxemptR {
        RxemptR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Frame Error / Transmit Special Character"]
    #[inline(always)]
    pub fn fretsc(&self) -> FretscR {
        FretscR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The current received dataword contained in DATA\\[R9:R0\\]
was received with a parity error."]
    #[inline(always)]
    pub fn paritye(&self) -> ParityeR {
        ParityeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The current received dataword contained in DATA\\[R9:R0\\]
was received with noise."]
    #[inline(always)]
    pub fn noisy(&self) -> NoisyR {
        NoisyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read receive data buffer 0 or write transmit data buffer 0."]
    #[inline(always)]
    #[must_use]
    pub fn r0t0(&mut self) -> R0t0W<DataSpec> {
        R0t0W::new(self, 0)
    }
    #[doc = "Bit 1 - Read receive data buffer 1 or write transmit data buffer 1."]
    #[inline(always)]
    #[must_use]
    pub fn r1t1(&mut self) -> R1t1W<DataSpec> {
        R1t1W::new(self, 1)
    }
    #[doc = "Bit 2 - Read receive data buffer 2 or write transmit data buffer 2."]
    #[inline(always)]
    #[must_use]
    pub fn r2t2(&mut self) -> R2t2W<DataSpec> {
        R2t2W::new(self, 2)
    }
    #[doc = "Bit 3 - Read receive data buffer 3 or write transmit data buffer 3."]
    #[inline(always)]
    #[must_use]
    pub fn r3t3(&mut self) -> R3t3W<DataSpec> {
        R3t3W::new(self, 3)
    }
    #[doc = "Bit 4 - Read receive data buffer 4 or write transmit data buffer 4."]
    #[inline(always)]
    #[must_use]
    pub fn r4t4(&mut self) -> R4t4W<DataSpec> {
        R4t4W::new(self, 4)
    }
    #[doc = "Bit 5 - Read receive data buffer 5 or write transmit data buffer 5."]
    #[inline(always)]
    #[must_use]
    pub fn r5t5(&mut self) -> R5t5W<DataSpec> {
        R5t5W::new(self, 5)
    }
    #[doc = "Bit 6 - Read receive data buffer 6 or write transmit data buffer 6."]
    #[inline(always)]
    #[must_use]
    pub fn r6t6(&mut self) -> R6t6W<DataSpec> {
        R6t6W::new(self, 6)
    }
    #[doc = "Bit 7 - Read receive data buffer 7 or write transmit data buffer 7."]
    #[inline(always)]
    #[must_use]
    pub fn r7t7(&mut self) -> R7t7W<DataSpec> {
        R7t7W::new(self, 7)
    }
    #[doc = "Bit 8 - Read receive data buffer 8 or write transmit data buffer 8."]
    #[inline(always)]
    #[must_use]
    pub fn r8t8(&mut self) -> R8t8W<DataSpec> {
        R8t8W::new(self, 8)
    }
    #[doc = "Bit 9 - Read receive data buffer 9 or write transmit data buffer 9."]
    #[inline(always)]
    #[must_use]
    pub fn r9t9(&mut self) -> R9t9W<DataSpec> {
        R9t9W::new(self, 9)
    }
    #[doc = "Bit 13 - Frame Error / Transmit Special Character"]
    #[inline(always)]
    #[must_use]
    pub fn fretsc(&mut self) -> FretscW<DataSpec> {
        FretscW::new(self, 13)
    }
}
#[doc = "LPUART Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA to value 0x1000"]
impl crate::Resettable for DataSpec {
    const RESET_VALUE: u32 = 0x1000;
}
