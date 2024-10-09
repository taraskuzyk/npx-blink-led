#[doc = "Register `SCGC4` reader"]
pub type R = crate::R<Scgc4Spec>;
#[doc = "Register `SCGC4` writer"]
pub type W = crate::W<Scgc4Spec>;
#[doc = "I2C0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c0 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<I2c0> for bool {
    #[inline(always)]
    fn from(variant: I2c0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C0` reader - I2C0 Clock Gate Control"]
pub type I2c0R = crate::BitReader<I2c0>;
impl I2c0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c0 {
        match self.bits {
            false => I2c0::B0,
            true => I2c0::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == I2c0::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == I2c0::B1
    }
}
#[doc = "Field `I2C0` writer - I2C0 Clock Gate Control"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG, I2c0>;
impl<'a, REG> I2c0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c0::B1)
    }
}
#[doc = "I2C1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c1 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<I2c1> for bool {
    #[inline(always)]
    fn from(variant: I2c1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C1` reader - I2C1 Clock Gate Control"]
pub type I2c1R = crate::BitReader<I2c1>;
impl I2c1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2c1 {
        match self.bits {
            false => I2c1::B0,
            true => I2c1::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == I2c1::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == I2c1::B1
    }
}
#[doc = "Field `I2C1` writer - I2C1 Clock Gate Control"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG, I2c1>;
impl<'a, REG> I2c1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(I2c1::B1)
    }
}
#[doc = "UART2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Uart2 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Uart2> for bool {
    #[inline(always)]
    fn from(variant: Uart2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART2` reader - UART2 Clock Gate Control"]
pub type Uart2R = crate::BitReader<Uart2>;
impl Uart2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Uart2 {
        match self.bits {
            false => Uart2::B0,
            true => Uart2::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Uart2::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Uart2::B1
    }
}
#[doc = "Field `UART2` writer - UART2 Clock Gate Control"]
pub type Uart2W<'a, REG> = crate::BitWriter<'a, REG, Uart2>;
impl<'a, REG> Uart2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Uart2::B1)
    }
}
#[doc = "USB Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbfs {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Usbfs> for bool {
    #[inline(always)]
    fn from(variant: Usbfs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBFS` reader - USB Clock Gate Control"]
pub type UsbfsR = crate::BitReader<Usbfs>;
impl UsbfsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbfs {
        match self.bits {
            false => Usbfs::B0,
            true => Usbfs::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbfs::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbfs::B1
    }
}
#[doc = "Field `USBFS` writer - USB Clock Gate Control"]
pub type UsbfsW<'a, REG> = crate::BitWriter<'a, REG, Usbfs>;
impl<'a, REG> UsbfsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfs::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbfs::B1)
    }
}
#[doc = "Comparator Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmp0 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Cmp0> for bool {
    #[inline(always)]
    fn from(variant: Cmp0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP0` reader - Comparator Clock Gate Control"]
pub type Cmp0R = crate::BitReader<Cmp0>;
impl Cmp0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmp0 {
        match self.bits {
            false => Cmp0::B0,
            true => Cmp0::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cmp0::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cmp0::B1
    }
}
#[doc = "Field `CMP0` writer - Comparator Clock Gate Control"]
pub type Cmp0W<'a, REG> = crate::BitWriter<'a, REG, Cmp0>;
impl<'a, REG> Cmp0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0::B1)
    }
}
#[doc = "VREF Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vref {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Vref> for bool {
    #[inline(always)]
    fn from(variant: Vref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VREF` reader - VREF Clock Gate Control"]
pub type VrefR = crate::BitReader<Vref>;
impl VrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vref {
        match self.bits {
            false => Vref::B0,
            true => Vref::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Vref::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Vref::B1
    }
}
#[doc = "Field `VREF` writer - VREF Clock Gate Control"]
pub type VrefW<'a, REG> = crate::BitWriter<'a, REG, Vref>;
impl<'a, REG> VrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Vref::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Vref::B1)
    }
}
#[doc = "SPI0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi0 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Spi0> for bool {
    #[inline(always)]
    fn from(variant: Spi0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI0` reader - SPI0 Clock Gate Control"]
pub type Spi0R = crate::BitReader<Spi0>;
impl Spi0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi0 {
        match self.bits {
            false => Spi0::B0,
            true => Spi0::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spi0::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spi0::B1
    }
}
#[doc = "Field `SPI0` writer - SPI0 Clock Gate Control"]
pub type Spi0W<'a, REG> = crate::BitWriter<'a, REG, Spi0>;
impl<'a, REG> Spi0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi0::B1)
    }
}
#[doc = "SPI1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spi1 {
    #[doc = "0: Clock disabled"]
    B0 = 0,
    #[doc = "1: Clock enabled"]
    B1 = 1,
}
impl From<Spi1> for bool {
    #[inline(always)]
    fn from(variant: Spi1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPI1` reader - SPI1 Clock Gate Control"]
pub type Spi1R = crate::BitReader<Spi1>;
impl Spi1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spi1 {
        match self.bits {
            false => Spi1::B0,
            true => Spi1::B1,
        }
    }
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spi1::B0
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spi1::B1
    }
}
#[doc = "Field `SPI1` writer - SPI1 Clock Gate Control"]
pub type Spi1W<'a, REG> = crate::BitWriter<'a, REG, Spi1>;
impl<'a, REG> Spi1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1::B0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spi1::B1)
    }
}
impl R {
    #[doc = "Bit 6 - I2C0 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C1 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - UART2 Clock Gate Control"]
    #[inline(always)]
    pub fn uart2(&self) -> Uart2R {
        Uart2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - USB Clock Gate Control"]
    #[inline(always)]
    pub fn usbfs(&self) -> UsbfsR {
        UsbfsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Comparator Clock Gate Control"]
    #[inline(always)]
    pub fn cmp0(&self) -> Cmp0R {
        Cmp0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF Clock Gate Control"]
    #[inline(always)]
    pub fn vref(&self) -> VrefR {
        VrefR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&self) -> Spi0R {
        Spi0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SPI1 Clock Gate Control"]
    #[inline(always)]
    pub fn spi1(&self) -> Spi1R {
        Spi1R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - I2C0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2c0W<Scgc4Spec> {
        I2c0W::new(self, 6)
    }
    #[doc = "Bit 7 - I2C1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2c1W<Scgc4Spec> {
        I2c1W::new(self, 7)
    }
    #[doc = "Bit 12 - UART2 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> Uart2W<Scgc4Spec> {
        Uart2W::new(self, 12)
    }
    #[doc = "Bit 18 - USB Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn usbfs(&mut self) -> UsbfsW<Scgc4Spec> {
        UsbfsW::new(self, 18)
    }
    #[doc = "Bit 19 - Comparator Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn cmp0(&mut self) -> Cmp0W<Scgc4Spec> {
        Cmp0W::new(self, 19)
    }
    #[doc = "Bit 20 - VREF Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn vref(&mut self) -> VrefW<Scgc4Spec> {
        VrefW::new(self, 20)
    }
    #[doc = "Bit 22 - SPI0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> Spi0W<Scgc4Spec> {
        Spi0W::new(self, 22)
    }
    #[doc = "Bit 23 - SPI1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> Spi1W<Scgc4Spec> {
        Spi1W::new(self, 23)
    }
}
#[doc = "System Clock Gating Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`scgc4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scgc4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scgc4Spec;
impl crate::RegisterSpec for Scgc4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scgc4::R`](R) reader structure"]
impl crate::Readable for Scgc4Spec {}
#[doc = "`write(|w| ..)` method takes [`scgc4::W`](W) writer structure"]
impl crate::Writable for Scgc4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCGC4 to value 0xf000_0030"]
impl crate::Resettable for Scgc4Spec {
    const RESET_VALUE: u32 = 0xf000_0030;
}
