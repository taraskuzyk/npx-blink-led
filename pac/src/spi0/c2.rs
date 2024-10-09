#[doc = "Register `C2` reader"]
pub type R = crate::R<C2Spec>;
#[doc = "Register `C2` writer"]
pub type W = crate::W<C2Spec>;
#[doc = "SPI Pin Control 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spc0 {
    #[doc = "0: SPI uses separate pins for data input and data output (pin mode is normal). In master mode of operation: MISO is master in and MOSI is master out. In slave mode of operation: MISO is slave out and MOSI is slave in."]
    B0 = 0,
    #[doc = "1: SPI configured for single-wire bidirectional operation (pin mode is bidirectional). In master mode of operation: MISO is not used by SPI; MOSI is master in when BIDIROE is 0 or master I/O when BIDIROE is 1. In slave mode of operation: MISO is slave in when BIDIROE is 0 or slave I/O when BIDIROE is 1; MOSI is not used by SPI."]
    B1 = 1,
}
impl From<Spc0> for bool {
    #[inline(always)]
    fn from(variant: Spc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPC0` reader - SPI Pin Control 0"]
pub type Spc0R = crate::BitReader<Spc0>;
impl Spc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spc0 {
        match self.bits {
            false => Spc0::B0,
            true => Spc0::B1,
        }
    }
    #[doc = "SPI uses separate pins for data input and data output (pin mode is normal). In master mode of operation: MISO is master in and MOSI is master out. In slave mode of operation: MISO is slave out and MOSI is slave in."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spc0::B0
    }
    #[doc = "SPI configured for single-wire bidirectional operation (pin mode is bidirectional). In master mode of operation: MISO is not used by SPI; MOSI is master in when BIDIROE is 0 or master I/O when BIDIROE is 1. In slave mode of operation: MISO is slave in when BIDIROE is 0 or slave I/O when BIDIROE is 1; MOSI is not used by SPI."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spc0::B1
    }
}
#[doc = "Field `SPC0` writer - SPI Pin Control 0"]
pub type Spc0W<'a, REG> = crate::BitWriter<'a, REG, Spc0>;
impl<'a, REG> Spc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI uses separate pins for data input and data output (pin mode is normal). In master mode of operation: MISO is master in and MOSI is master out. In slave mode of operation: MISO is slave out and MOSI is slave in."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spc0::B0)
    }
    #[doc = "SPI configured for single-wire bidirectional operation (pin mode is bidirectional). In master mode of operation: MISO is not used by SPI; MOSI is master in when BIDIROE is 0 or master I/O when BIDIROE is 1. In slave mode of operation: MISO is slave in when BIDIROE is 0 or slave I/O when BIDIROE is 1; MOSI is not used by SPI."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spc0::B1)
    }
}
#[doc = "SPI Stop in Wait Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spiswai {
    #[doc = "0: SPI clocks continue to operate in Wait mode."]
    B0 = 0,
    #[doc = "1: SPI clocks stop when the MCU enters Wait mode."]
    B1 = 1,
}
impl From<Spiswai> for bool {
    #[inline(always)]
    fn from(variant: Spiswai) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPISWAI` reader - SPI Stop in Wait Mode"]
pub type SpiswaiR = crate::BitReader<Spiswai>;
impl SpiswaiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spiswai {
        match self.bits {
            false => Spiswai::B0,
            true => Spiswai::B1,
        }
    }
    #[doc = "SPI clocks continue to operate in Wait mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spiswai::B0
    }
    #[doc = "SPI clocks stop when the MCU enters Wait mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spiswai::B1
    }
}
#[doc = "Field `SPISWAI` writer - SPI Stop in Wait Mode"]
pub type SpiswaiW<'a, REG> = crate::BitWriter<'a, REG, Spiswai>;
impl<'a, REG> SpiswaiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI clocks continue to operate in Wait mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spiswai::B0)
    }
    #[doc = "SPI clocks stop when the MCU enters Wait mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spiswai::B1)
    }
}
#[doc = "Receive DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdmae {
    #[doc = "0: DMA request for receive is disabled and interrupt from SPRF is allowed"]
    B0 = 0,
    #[doc = "1: DMA request for receive is enabled and interrupt from SPRF is disabled"]
    B1 = 1,
}
impl From<Rxdmae> for bool {
    #[inline(always)]
    fn from(variant: Rxdmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAE` reader - Receive DMA enable"]
pub type RxdmaeR = crate::BitReader<Rxdmae>;
impl RxdmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdmae {
        match self.bits {
            false => Rxdmae::B0,
            true => Rxdmae::B1,
        }
    }
    #[doc = "DMA request for receive is disabled and interrupt from SPRF is allowed"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Rxdmae::B0
    }
    #[doc = "DMA request for receive is enabled and interrupt from SPRF is disabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Rxdmae::B1
    }
}
#[doc = "Field `RXDMAE` writer - Receive DMA enable"]
pub type RxdmaeW<'a, REG> = crate::BitWriter<'a, REG, Rxdmae>;
impl<'a, REG> RxdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request for receive is disabled and interrupt from SPRF is allowed"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmae::B0)
    }
    #[doc = "DMA request for receive is enabled and interrupt from SPRF is disabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdmae::B1)
    }
}
#[doc = "Bidirectional Mode Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bidiroe {
    #[doc = "0: Output driver disabled so SPI data I/O pin acts as an input"]
    B0 = 0,
    #[doc = "1: SPI I/O pin enabled as an output"]
    B1 = 1,
}
impl From<Bidiroe> for bool {
    #[inline(always)]
    fn from(variant: Bidiroe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIDIROE` reader - Bidirectional Mode Output Enable"]
pub type BidiroeR = crate::BitReader<Bidiroe>;
impl BidiroeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bidiroe {
        match self.bits {
            false => Bidiroe::B0,
            true => Bidiroe::B1,
        }
    }
    #[doc = "Output driver disabled so SPI data I/O pin acts as an input"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Bidiroe::B0
    }
    #[doc = "SPI I/O pin enabled as an output"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Bidiroe::B1
    }
}
#[doc = "Field `BIDIROE` writer - Bidirectional Mode Output Enable"]
pub type BidiroeW<'a, REG> = crate::BitWriter<'a, REG, Bidiroe>;
impl<'a, REG> BidiroeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output driver disabled so SPI data I/O pin acts as an input"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Bidiroe::B0)
    }
    #[doc = "SPI I/O pin enabled as an output"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Bidiroe::B1)
    }
}
#[doc = "Master Mode-Fault Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Modfen {
    #[doc = "0: Mode fault function disabled, master SS pin reverts to general-purpose I/O not controlled by SPI"]
    B0 = 0,
    #[doc = "1: Mode fault function enabled, master SS pin acts as the mode fault input or the slave select output"]
    B1 = 1,
}
impl From<Modfen> for bool {
    #[inline(always)]
    fn from(variant: Modfen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODFEN` reader - Master Mode-Fault Function Enable"]
pub type ModfenR = crate::BitReader<Modfen>;
impl ModfenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Modfen {
        match self.bits {
            false => Modfen::B0,
            true => Modfen::B1,
        }
    }
    #[doc = "Mode fault function disabled, master SS pin reverts to general-purpose I/O not controlled by SPI"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Modfen::B0
    }
    #[doc = "Mode fault function enabled, master SS pin acts as the mode fault input or the slave select output"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Modfen::B1
    }
}
#[doc = "Field `MODFEN` writer - Master Mode-Fault Function Enable"]
pub type ModfenW<'a, REG> = crate::BitWriter<'a, REG, Modfen>;
impl<'a, REG> ModfenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode fault function disabled, master SS pin reverts to general-purpose I/O not controlled by SPI"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Modfen::B0)
    }
    #[doc = "Mode fault function enabled, master SS pin acts as the mode fault input or the slave select output"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Modfen::B1)
    }
}
#[doc = "Transmit DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdmae {
    #[doc = "0: DMA request for transmit is disabled and interrupt from SPTEF is allowed"]
    B0 = 0,
    #[doc = "1: DMA request for transmit is enabled and interrupt from SPTEF is disabled"]
    B1 = 1,
}
impl From<Txdmae> for bool {
    #[inline(always)]
    fn from(variant: Txdmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAE` reader - Transmit DMA enable"]
pub type TxdmaeR = crate::BitReader<Txdmae>;
impl TxdmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdmae {
        match self.bits {
            false => Txdmae::B0,
            true => Txdmae::B1,
        }
    }
    #[doc = "DMA request for transmit is disabled and interrupt from SPTEF is allowed"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Txdmae::B0
    }
    #[doc = "DMA request for transmit is enabled and interrupt from SPTEF is disabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Txdmae::B1
    }
}
#[doc = "Field `TXDMAE` writer - Transmit DMA enable"]
pub type TxdmaeW<'a, REG> = crate::BitWriter<'a, REG, Txdmae>;
impl<'a, REG> TxdmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA request for transmit is disabled and interrupt from SPTEF is allowed"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmae::B0)
    }
    #[doc = "DMA request for transmit is enabled and interrupt from SPTEF is disabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Txdmae::B1)
    }
}
#[doc = "SPI 8-bit or 16-bit mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spimode {
    #[doc = "0: 8-bit SPI shift register, match register, and buffers"]
    B0 = 0,
    #[doc = "1: 16-bit SPI shift register, match register, and buffers"]
    B1 = 1,
}
impl From<Spimode> for bool {
    #[inline(always)]
    fn from(variant: Spimode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPIMODE` reader - SPI 8-bit or 16-bit mode"]
pub type SpimodeR = crate::BitReader<Spimode>;
impl SpimodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spimode {
        match self.bits {
            false => Spimode::B0,
            true => Spimode::B1,
        }
    }
    #[doc = "8-bit SPI shift register, match register, and buffers"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spimode::B0
    }
    #[doc = "16-bit SPI shift register, match register, and buffers"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spimode::B1
    }
}
#[doc = "Field `SPIMODE` writer - SPI 8-bit or 16-bit mode"]
pub type SpimodeW<'a, REG> = crate::BitWriter<'a, REG, Spimode>;
impl<'a, REG> SpimodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "8-bit SPI shift register, match register, and buffers"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spimode::B0)
    }
    #[doc = "16-bit SPI shift register, match register, and buffers"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spimode::B1)
    }
}
#[doc = "SPI Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Spmie {
    #[doc = "0: Interrupts from SPMF inhibited (use polling)"]
    B0 = 0,
    #[doc = "1: When SPMF is 1, requests a hardware interrupt"]
    B1 = 1,
}
impl From<Spmie> for bool {
    #[inline(always)]
    fn from(variant: Spmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPMIE` reader - SPI Match Interrupt Enable"]
pub type SpmieR = crate::BitReader<Spmie>;
impl SpmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Spmie {
        match self.bits {
            false => Spmie::B0,
            true => Spmie::B1,
        }
    }
    #[doc = "Interrupts from SPMF inhibited (use polling)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Spmie::B0
    }
    #[doc = "When SPMF is 1, requests a hardware interrupt"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Spmie::B1
    }
}
#[doc = "Field `SPMIE` writer - SPI Match Interrupt Enable"]
pub type SpmieW<'a, REG> = crate::BitWriter<'a, REG, Spmie>;
impl<'a, REG> SpmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupts from SPMF inhibited (use polling)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Spmie::B0)
    }
    #[doc = "When SPMF is 1, requests a hardware interrupt"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Spmie::B1)
    }
}
impl R {
    #[doc = "Bit 0 - SPI Pin Control 0"]
    #[inline(always)]
    pub fn spc0(&self) -> Spc0R {
        Spc0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI Stop in Wait Mode"]
    #[inline(always)]
    pub fn spiswai(&self) -> SpiswaiR {
        SpiswaiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive DMA enable"]
    #[inline(always)]
    pub fn rxdmae(&self) -> RxdmaeR {
        RxdmaeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bidirectional Mode Output Enable"]
    #[inline(always)]
    pub fn bidiroe(&self) -> BidiroeR {
        BidiroeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Mode-Fault Function Enable"]
    #[inline(always)]
    pub fn modfen(&self) -> ModfenR {
        ModfenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit DMA enable"]
    #[inline(always)]
    pub fn txdmae(&self) -> TxdmaeR {
        TxdmaeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI 8-bit or 16-bit mode"]
    #[inline(always)]
    pub fn spimode(&self) -> SpimodeR {
        SpimodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI Match Interrupt Enable"]
    #[inline(always)]
    pub fn spmie(&self) -> SpmieR {
        SpmieR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Pin Control 0"]
    #[inline(always)]
    #[must_use]
    pub fn spc0(&mut self) -> Spc0W<C2Spec> {
        Spc0W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI Stop in Wait Mode"]
    #[inline(always)]
    #[must_use]
    pub fn spiswai(&mut self) -> SpiswaiW<C2Spec> {
        SpiswaiW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmae(&mut self) -> RxdmaeW<C2Spec> {
        RxdmaeW::new(self, 2)
    }
    #[doc = "Bit 3 - Bidirectional Mode Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bidiroe(&mut self) -> BidiroeW<C2Spec> {
        BidiroeW::new(self, 3)
    }
    #[doc = "Bit 4 - Master Mode-Fault Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn modfen(&mut self) -> ModfenW<C2Spec> {
        ModfenW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmae(&mut self) -> TxdmaeW<C2Spec> {
        TxdmaeW::new(self, 5)
    }
    #[doc = "Bit 6 - SPI 8-bit or 16-bit mode"]
    #[inline(always)]
    #[must_use]
    pub fn spimode(&mut self) -> SpimodeW<C2Spec> {
        SpimodeW::new(self, 6)
    }
    #[doc = "Bit 7 - SPI Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spmie(&mut self) -> SpmieW<C2Spec> {
        SpmieW::new(self, 7)
    }
}
#[doc = "SPI Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2Spec;
impl crate::RegisterSpec for C2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c2::R`](R) reader structure"]
impl crate::Readable for C2Spec {}
#[doc = "`write(|w| ..)` method takes [`c2::W`](W) writer structure"]
impl crate::Writable for C2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2Spec {
    const RESET_VALUE: u8 = 0;
}
