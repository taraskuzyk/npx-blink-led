#[doc = "Register `S2` reader"]
pub type R = crate::R<S2Spec>;
#[doc = "Register `S2` writer"]
pub type W = crate::W<S2Spec>;
#[doc = "Receiver Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Raf {
    #[doc = "0: UART receiver idle/inactive waiting for a start bit."]
    B0 = 0,
    #[doc = "1: UART receiver active, RxD input not idle."]
    B1 = 1,
}
impl From<Raf> for bool {
    #[inline(always)]
    fn from(variant: Raf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RAF` reader - Receiver Active Flag"]
pub type RafR = crate::BitReader<Raf>;
impl RafR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Raf {
        match self.bits {
            false => Raf::B0,
            true => Raf::B1,
        }
    }
    #[doc = "UART receiver idle/inactive waiting for a start bit."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Raf::B0
    }
    #[doc = "UART receiver active, RxD input not idle."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Raf::B1
    }
}
#[doc = "Break Transmit Character Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brk13 {
    #[doc = "0: Break character is 10, 11, or 12 bits long."]
    B0 = 0,
    #[doc = "1: Break character is 13 or 14 bits long."]
    B1 = 1,
}
impl From<Brk13> for bool {
    #[inline(always)]
    fn from(variant: Brk13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BRK13` reader - Break Transmit Character Length"]
pub type Brk13R = crate::BitReader<Brk13>;
impl Brk13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Brk13 {
        match self.bits {
            false => Brk13::B0,
            true => Brk13::B1,
        }
    }
    #[doc = "Break character is 10, 11, or 12 bits long."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Brk13::B0
    }
    #[doc = "Break character is 13 or 14 bits long."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Brk13::B1
    }
}
#[doc = "Field `BRK13` writer - Break Transmit Character Length"]
pub type Brk13W<'a, REG> = crate::BitWriter<'a, REG, Brk13>;
impl<'a, REG> Brk13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break character is 10, 11, or 12 bits long."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Brk13::B0)
    }
    #[doc = "Break character is 13 or 14 bits long."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Brk13::B1)
    }
}
#[doc = "Receive Wakeup Idle Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rwuid {
    #[doc = "0: S1\\[IDLE\\]
is not set upon detection of an idle character."]
    B0 = 0,
    #[doc = "1: S1\\[IDLE\\]
is set upon detection of an idle character."]
    B1 = 1,
}
impl From<Rwuid> for bool {
    #[inline(always)]
    fn from(variant: Rwuid) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RWUID` reader - Receive Wakeup Idle Detect"]
pub type RwuidR = crate::BitReader<Rwuid>;
impl RwuidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rwuid {
        match self.bits {
            false => Rwuid::B0,
            true => Rwuid::B1,
        }
    }
    #[doc = "S1\\[IDLE\\]
is not set upon detection of an idle character."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rwuid::B0
    }
    #[doc = "S1\\[IDLE\\]
is set upon detection of an idle character."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rwuid::B1
    }
}
#[doc = "Field `RWUID` writer - Receive Wakeup Idle Detect"]
pub type RwuidW<'a, REG> = crate::BitWriter<'a, REG, Rwuid>;
impl<'a, REG> RwuidW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "S1\\[IDLE\\]
is not set upon detection of an idle character."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rwuid::B0)
    }
    #[doc = "S1\\[IDLE\\]
is set upon detection of an idle character."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rwuid::B1)
    }
}
#[doc = "Receive Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxinv {
    #[doc = "0: Receive data is not inverted."]
    B0 = 0,
    #[doc = "1: Receive data is inverted."]
    B1 = 1,
}
impl From<Rxinv> for bool {
    #[inline(always)]
    fn from(variant: Rxinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - Receive Data Inversion"]
pub type RxinvR = crate::BitReader<Rxinv>;
impl RxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxinv {
        match self.bits {
            false => Rxinv::B0,
            true => Rxinv::B1,
        }
    }
    #[doc = "Receive data is not inverted."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxinv::B0
    }
    #[doc = "Receive data is inverted."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxinv::B1
    }
}
#[doc = "Field `RXINV` writer - Receive Data Inversion"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG, Rxinv>;
impl<'a, REG> RxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Receive data is not inverted."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::B0)
    }
    #[doc = "Receive data is inverted."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinv::B1)
    }
}
#[doc = "Most Significant Bit First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Msbf {
    #[doc = "0: LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    B0 = 0,
    #[doc = "1: MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1\\[M\\]
and C1\\[PE\\]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1\\[M\\]
and C1\\[PE\\]."]
    B1 = 1,
}
impl From<Msbf> for bool {
    #[inline(always)]
    fn from(variant: Msbf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MsbfR = crate::BitReader<Msbf>;
impl MsbfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Msbf {
        match self.bits {
            false => Msbf::B0,
            true => Msbf::B1,
        }
    }
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Msbf::B0
    }
    #[doc = "MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1\\[M\\]
and C1\\[PE\\]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1\\[M\\]
and C1\\[PE\\]."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Msbf::B1
    }
}
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MsbfW<'a, REG> = crate::BitWriter<'a, REG, Msbf>;
impl<'a, REG> MsbfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Msbf::B0)
    }
    #[doc = "MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1\\[M\\]
and C1\\[PE\\]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1\\[M\\]
and C1\\[PE\\]."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Msbf::B1)
    }
}
#[doc = "RxD Pin Active Edge Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxedgif {
    #[doc = "0: No active edge on the receive pin has occurred."]
    B0 = 0,
    #[doc = "1: An active edge on the receive pin has occurred."]
    B1 = 1,
}
impl From<Rxedgif> for bool {
    #[inline(always)]
    fn from(variant: Rxedgif) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEDGIF` reader - RxD Pin Active Edge Interrupt Flag"]
pub type RxedgifR = crate::BitReader<Rxedgif>;
impl RxedgifR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxedgif {
        match self.bits {
            false => Rxedgif::B0,
            true => Rxedgif::B1,
        }
    }
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxedgif::B0
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxedgif::B1
    }
}
#[doc = "Field `RXEDGIF` writer - RxD Pin Active Edge Interrupt Flag"]
pub type RxedgifW<'a, REG> = crate::BitWriter<'a, REG, Rxedgif>;
impl<'a, REG> RxedgifW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedgif::B0)
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedgif::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Active Flag"]
    #[inline(always)]
    pub fn raf(&self) -> RafR {
        RafR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Break Transmit Character Length"]
    #[inline(always)]
    pub fn brk13(&self) -> Brk13R {
        Brk13R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Wakeup Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&self) -> RwuidR {
        RwuidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MsbfR {
        MsbfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&self) -> RxedgifR {
        RxedgifR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Break Transmit Character Length"]
    #[inline(always)]
    #[must_use]
    pub fn brk13(&mut self) -> Brk13W<S2Spec> {
        Brk13W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive Wakeup Idle Detect"]
    #[inline(always)]
    #[must_use]
    pub fn rwuid(&mut self) -> RwuidW<S2Spec> {
        RwuidW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RxinvW<S2Spec> {
        RxinvW::new(self, 4)
    }
    #[doc = "Bit 5 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MsbfW<S2Spec> {
        MsbfW::new(self, 5)
    }
    #[doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxedgif(&mut self) -> RxedgifW<S2Spec> {
        RxedgifW::new(self, 6)
    }
}
#[doc = "UART Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`s2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2Spec;
impl crate::RegisterSpec for S2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s2::R`](R) reader structure"]
impl crate::Readable for S2Spec {}
#[doc = "`write(|w| ..)` method takes [`s2::W`](W) writer structure"]
impl crate::Writable for S2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets S2 to value 0"]
impl crate::Resettable for S2Spec {
    const RESET_VALUE: u8 = 0;
}
