#[doc = "Register `CHCFG%s` reader"]
pub type R = crate::R<ChcfgSpec>;
#[doc = "Register `CHCFG%s` writer"]
pub type W = crate::W<ChcfgSpec>;
#[doc = "DMA Channel Source (Slot)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Source {
    #[doc = "0: Disable_Signal"]
    B000000 = 0,
    #[doc = "2: LPUART0_Rx_Signal"]
    B000010 = 2,
    #[doc = "3: LPUART0_Tx_Signal"]
    B000011 = 3,
    #[doc = "4: LPUART1_Rx_Signal"]
    B000100 = 4,
    #[doc = "5: LPUART1_Tx_Signal"]
    B000101 = 5,
    #[doc = "6: UART2_Rx_Signal"]
    B000110 = 6,
    #[doc = "7: UART2_Tx_Signal"]
    B000111 = 7,
    #[doc = "10: FlexIO_Channel0_Signal"]
    B001010 = 10,
    #[doc = "11: FlexIO_Channel1_Signal"]
    B001011 = 11,
    #[doc = "12: FlexIO_Channel2_Signal"]
    B001100 = 12,
    #[doc = "13: FlexIO_Channel3_Signal"]
    B001101 = 13,
    #[doc = "14: I2S0_Rx_Signal"]
    B001110 = 14,
    #[doc = "15: I2S0_Tx_Signal"]
    B001111 = 15,
    #[doc = "16: SPI0_Rx_Signal"]
    B010000 = 16,
    #[doc = "17: SPI0_Tx_Signal"]
    B010001 = 17,
    #[doc = "18: SPI1_Rx_Signal"]
    B010010 = 18,
    #[doc = "19: SPI1_Tx_Signal"]
    B010011 = 19,
    #[doc = "22: I2C0_Signal"]
    B010110 = 22,
    #[doc = "23: I2C1_Signal"]
    B010111 = 23,
    #[doc = "24: TPM0_Channel0_Signal"]
    B011000 = 24,
    #[doc = "25: TPM0_Channel1_Signal"]
    B011001 = 25,
    #[doc = "26: TPM0_Channel2_Signal"]
    B011010 = 26,
    #[doc = "27: TPM0_Channel3_Signal"]
    B011011 = 27,
    #[doc = "28: TPM0_Channel4_Signal"]
    B011100 = 28,
    #[doc = "29: TPM0_Channel5_Signal"]
    B011101 = 29,
    #[doc = "32: TPM1_Channel0_Signal"]
    B100000 = 32,
    #[doc = "33: TPM1_Channel1_Signal"]
    B100001 = 33,
    #[doc = "34: TPM2_Channel0_Signal"]
    B100010 = 34,
    #[doc = "35: TPM2_Channel1_Signal"]
    B100011 = 35,
    #[doc = "40: ADC0_Signal"]
    B101000 = 40,
    #[doc = "42: CMP0_Signal"]
    B101010 = 42,
    #[doc = "45: DAC0_Signal"]
    B101101 = 45,
    #[doc = "49: Port_A_Signal"]
    B110001 = 49,
    #[doc = "51: Port_C_Signal"]
    B110011 = 51,
    #[doc = "52: Port_D_Signal"]
    B110100 = 52,
    #[doc = "54: TPM0_Overflow_Signal"]
    B110110 = 54,
    #[doc = "55: TPM1_Overflow_Signal"]
    B110111 = 55,
    #[doc = "56: TPM2_Overflow_Signal"]
    B111000 = 56,
    #[doc = "60: AlwaysOn60_Signal"]
    B111100 = 60,
    #[doc = "61: AlwaysOn61_Signal"]
    B111101 = 61,
    #[doc = "62: AlwaysOn62_Signal"]
    B111110 = 62,
    #[doc = "63: AlwaysOn63_Signal"]
    B111111 = 63,
}
impl From<Source> for u8 {
    #[inline(always)]
    fn from(variant: Source) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Source {
    type Ux = u8;
}
impl crate::IsEnum for Source {}
#[doc = "Field `SOURCE` reader - DMA Channel Source (Slot)"]
pub type SourceR = crate::FieldReader<Source>;
impl SourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Source> {
        match self.bits {
            0 => Some(Source::B000000),
            2 => Some(Source::B000010),
            3 => Some(Source::B000011),
            4 => Some(Source::B000100),
            5 => Some(Source::B000101),
            6 => Some(Source::B000110),
            7 => Some(Source::B000111),
            10 => Some(Source::B001010),
            11 => Some(Source::B001011),
            12 => Some(Source::B001100),
            13 => Some(Source::B001101),
            14 => Some(Source::B001110),
            15 => Some(Source::B001111),
            16 => Some(Source::B010000),
            17 => Some(Source::B010001),
            18 => Some(Source::B010010),
            19 => Some(Source::B010011),
            22 => Some(Source::B010110),
            23 => Some(Source::B010111),
            24 => Some(Source::B011000),
            25 => Some(Source::B011001),
            26 => Some(Source::B011010),
            27 => Some(Source::B011011),
            28 => Some(Source::B011100),
            29 => Some(Source::B011101),
            32 => Some(Source::B100000),
            33 => Some(Source::B100001),
            34 => Some(Source::B100010),
            35 => Some(Source::B100011),
            40 => Some(Source::B101000),
            42 => Some(Source::B101010),
            45 => Some(Source::B101101),
            49 => Some(Source::B110001),
            51 => Some(Source::B110011),
            52 => Some(Source::B110100),
            54 => Some(Source::B110110),
            55 => Some(Source::B110111),
            56 => Some(Source::B111000),
            60 => Some(Source::B111100),
            61 => Some(Source::B111101),
            62 => Some(Source::B111110),
            63 => Some(Source::B111111),
            _ => None,
        }
    }
    #[doc = "Disable_Signal"]
    #[inline(always)]
    pub fn is_b000000(&self) -> bool {
        *self == Source::B000000
    }
    #[doc = "LPUART0_Rx_Signal"]
    #[inline(always)]
    pub fn is_b000010(&self) -> bool {
        *self == Source::B000010
    }
    #[doc = "LPUART0_Tx_Signal"]
    #[inline(always)]
    pub fn is_b000011(&self) -> bool {
        *self == Source::B000011
    }
    #[doc = "LPUART1_Rx_Signal"]
    #[inline(always)]
    pub fn is_b000100(&self) -> bool {
        *self == Source::B000100
    }
    #[doc = "LPUART1_Tx_Signal"]
    #[inline(always)]
    pub fn is_b000101(&self) -> bool {
        *self == Source::B000101
    }
    #[doc = "UART2_Rx_Signal"]
    #[inline(always)]
    pub fn is_b000110(&self) -> bool {
        *self == Source::B000110
    }
    #[doc = "UART2_Tx_Signal"]
    #[inline(always)]
    pub fn is_b000111(&self) -> bool {
        *self == Source::B000111
    }
    #[doc = "FlexIO_Channel0_Signal"]
    #[inline(always)]
    pub fn is_b001010(&self) -> bool {
        *self == Source::B001010
    }
    #[doc = "FlexIO_Channel1_Signal"]
    #[inline(always)]
    pub fn is_b001011(&self) -> bool {
        *self == Source::B001011
    }
    #[doc = "FlexIO_Channel2_Signal"]
    #[inline(always)]
    pub fn is_b001100(&self) -> bool {
        *self == Source::B001100
    }
    #[doc = "FlexIO_Channel3_Signal"]
    #[inline(always)]
    pub fn is_b001101(&self) -> bool {
        *self == Source::B001101
    }
    #[doc = "I2S0_Rx_Signal"]
    #[inline(always)]
    pub fn is_b001110(&self) -> bool {
        *self == Source::B001110
    }
    #[doc = "I2S0_Tx_Signal"]
    #[inline(always)]
    pub fn is_b001111(&self) -> bool {
        *self == Source::B001111
    }
    #[doc = "SPI0_Rx_Signal"]
    #[inline(always)]
    pub fn is_b010000(&self) -> bool {
        *self == Source::B010000
    }
    #[doc = "SPI0_Tx_Signal"]
    #[inline(always)]
    pub fn is_b010001(&self) -> bool {
        *self == Source::B010001
    }
    #[doc = "SPI1_Rx_Signal"]
    #[inline(always)]
    pub fn is_b010010(&self) -> bool {
        *self == Source::B010010
    }
    #[doc = "SPI1_Tx_Signal"]
    #[inline(always)]
    pub fn is_b010011(&self) -> bool {
        *self == Source::B010011
    }
    #[doc = "I2C0_Signal"]
    #[inline(always)]
    pub fn is_b010110(&self) -> bool {
        *self == Source::B010110
    }
    #[doc = "I2C1_Signal"]
    #[inline(always)]
    pub fn is_b010111(&self) -> bool {
        *self == Source::B010111
    }
    #[doc = "TPM0_Channel0_Signal"]
    #[inline(always)]
    pub fn is_b011000(&self) -> bool {
        *self == Source::B011000
    }
    #[doc = "TPM0_Channel1_Signal"]
    #[inline(always)]
    pub fn is_b011001(&self) -> bool {
        *self == Source::B011001
    }
    #[doc = "TPM0_Channel2_Signal"]
    #[inline(always)]
    pub fn is_b011010(&self) -> bool {
        *self == Source::B011010
    }
    #[doc = "TPM0_Channel3_Signal"]
    #[inline(always)]
    pub fn is_b011011(&self) -> bool {
        *self == Source::B011011
    }
    #[doc = "TPM0_Channel4_Signal"]
    #[inline(always)]
    pub fn is_b011100(&self) -> bool {
        *self == Source::B011100
    }
    #[doc = "TPM0_Channel5_Signal"]
    #[inline(always)]
    pub fn is_b011101(&self) -> bool {
        *self == Source::B011101
    }
    #[doc = "TPM1_Channel0_Signal"]
    #[inline(always)]
    pub fn is_b100000(&self) -> bool {
        *self == Source::B100000
    }
    #[doc = "TPM1_Channel1_Signal"]
    #[inline(always)]
    pub fn is_b100001(&self) -> bool {
        *self == Source::B100001
    }
    #[doc = "TPM2_Channel0_Signal"]
    #[inline(always)]
    pub fn is_b100010(&self) -> bool {
        *self == Source::B100010
    }
    #[doc = "TPM2_Channel1_Signal"]
    #[inline(always)]
    pub fn is_b100011(&self) -> bool {
        *self == Source::B100011
    }
    #[doc = "ADC0_Signal"]
    #[inline(always)]
    pub fn is_b101000(&self) -> bool {
        *self == Source::B101000
    }
    #[doc = "CMP0_Signal"]
    #[inline(always)]
    pub fn is_b101010(&self) -> bool {
        *self == Source::B101010
    }
    #[doc = "DAC0_Signal"]
    #[inline(always)]
    pub fn is_b101101(&self) -> bool {
        *self == Source::B101101
    }
    #[doc = "Port_A_Signal"]
    #[inline(always)]
    pub fn is_b110001(&self) -> bool {
        *self == Source::B110001
    }
    #[doc = "Port_C_Signal"]
    #[inline(always)]
    pub fn is_b110011(&self) -> bool {
        *self == Source::B110011
    }
    #[doc = "Port_D_Signal"]
    #[inline(always)]
    pub fn is_b110100(&self) -> bool {
        *self == Source::B110100
    }
    #[doc = "TPM0_Overflow_Signal"]
    #[inline(always)]
    pub fn is_b110110(&self) -> bool {
        *self == Source::B110110
    }
    #[doc = "TPM1_Overflow_Signal"]
    #[inline(always)]
    pub fn is_b110111(&self) -> bool {
        *self == Source::B110111
    }
    #[doc = "TPM2_Overflow_Signal"]
    #[inline(always)]
    pub fn is_b111000(&self) -> bool {
        *self == Source::B111000
    }
    #[doc = "AlwaysOn60_Signal"]
    #[inline(always)]
    pub fn is_b111100(&self) -> bool {
        *self == Source::B111100
    }
    #[doc = "AlwaysOn61_Signal"]
    #[inline(always)]
    pub fn is_b111101(&self) -> bool {
        *self == Source::B111101
    }
    #[doc = "AlwaysOn62_Signal"]
    #[inline(always)]
    pub fn is_b111110(&self) -> bool {
        *self == Source::B111110
    }
    #[doc = "AlwaysOn63_Signal"]
    #[inline(always)]
    pub fn is_b111111(&self) -> bool {
        *self == Source::B111111
    }
}
#[doc = "Field `SOURCE` writer - DMA Channel Source (Slot)"]
pub type SourceW<'a, REG> = crate::FieldWriter<'a, REG, 6, Source>;
impl<'a, REG> SourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable_Signal"]
    #[inline(always)]
    pub fn b000000(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B000000)
    }
    #[doc = "LPUART0_Rx_Signal"]
    #[inline(always)]
    pub fn b000010(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B000010)
    }
    #[doc = "LPUART0_Tx_Signal"]
    #[inline(always)]
    pub fn b000011(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B000011)
    }
    #[doc = "LPUART1_Rx_Signal"]
    #[inline(always)]
    pub fn b000100(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B000100)
    }
    #[doc = "LPUART1_Tx_Signal"]
    #[inline(always)]
    pub fn b000101(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B000101)
    }
    #[doc = "UART2_Rx_Signal"]
    #[inline(always)]
    pub fn b000110(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B000110)
    }
    #[doc = "UART2_Tx_Signal"]
    #[inline(always)]
    pub fn b000111(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B000111)
    }
    #[doc = "FlexIO_Channel0_Signal"]
    #[inline(always)]
    pub fn b001010(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B001010)
    }
    #[doc = "FlexIO_Channel1_Signal"]
    #[inline(always)]
    pub fn b001011(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B001011)
    }
    #[doc = "FlexIO_Channel2_Signal"]
    #[inline(always)]
    pub fn b001100(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B001100)
    }
    #[doc = "FlexIO_Channel3_Signal"]
    #[inline(always)]
    pub fn b001101(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B001101)
    }
    #[doc = "I2S0_Rx_Signal"]
    #[inline(always)]
    pub fn b001110(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B001110)
    }
    #[doc = "I2S0_Tx_Signal"]
    #[inline(always)]
    pub fn b001111(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B001111)
    }
    #[doc = "SPI0_Rx_Signal"]
    #[inline(always)]
    pub fn b010000(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B010000)
    }
    #[doc = "SPI0_Tx_Signal"]
    #[inline(always)]
    pub fn b010001(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B010001)
    }
    #[doc = "SPI1_Rx_Signal"]
    #[inline(always)]
    pub fn b010010(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B010010)
    }
    #[doc = "SPI1_Tx_Signal"]
    #[inline(always)]
    pub fn b010011(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B010011)
    }
    #[doc = "I2C0_Signal"]
    #[inline(always)]
    pub fn b010110(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B010110)
    }
    #[doc = "I2C1_Signal"]
    #[inline(always)]
    pub fn b010111(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B010111)
    }
    #[doc = "TPM0_Channel0_Signal"]
    #[inline(always)]
    pub fn b011000(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B011000)
    }
    #[doc = "TPM0_Channel1_Signal"]
    #[inline(always)]
    pub fn b011001(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B011001)
    }
    #[doc = "TPM0_Channel2_Signal"]
    #[inline(always)]
    pub fn b011010(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B011010)
    }
    #[doc = "TPM0_Channel3_Signal"]
    #[inline(always)]
    pub fn b011011(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B011011)
    }
    #[doc = "TPM0_Channel4_Signal"]
    #[inline(always)]
    pub fn b011100(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B011100)
    }
    #[doc = "TPM0_Channel5_Signal"]
    #[inline(always)]
    pub fn b011101(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B011101)
    }
    #[doc = "TPM1_Channel0_Signal"]
    #[inline(always)]
    pub fn b100000(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B100000)
    }
    #[doc = "TPM1_Channel1_Signal"]
    #[inline(always)]
    pub fn b100001(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B100001)
    }
    #[doc = "TPM2_Channel0_Signal"]
    #[inline(always)]
    pub fn b100010(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B100010)
    }
    #[doc = "TPM2_Channel1_Signal"]
    #[inline(always)]
    pub fn b100011(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B100011)
    }
    #[doc = "ADC0_Signal"]
    #[inline(always)]
    pub fn b101000(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B101000)
    }
    #[doc = "CMP0_Signal"]
    #[inline(always)]
    pub fn b101010(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B101010)
    }
    #[doc = "DAC0_Signal"]
    #[inline(always)]
    pub fn b101101(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B101101)
    }
    #[doc = "Port_A_Signal"]
    #[inline(always)]
    pub fn b110001(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B110001)
    }
    #[doc = "Port_C_Signal"]
    #[inline(always)]
    pub fn b110011(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B110011)
    }
    #[doc = "Port_D_Signal"]
    #[inline(always)]
    pub fn b110100(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B110100)
    }
    #[doc = "TPM0_Overflow_Signal"]
    #[inline(always)]
    pub fn b110110(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B110110)
    }
    #[doc = "TPM1_Overflow_Signal"]
    #[inline(always)]
    pub fn b110111(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B110111)
    }
    #[doc = "TPM2_Overflow_Signal"]
    #[inline(always)]
    pub fn b111000(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B111000)
    }
    #[doc = "AlwaysOn60_Signal"]
    #[inline(always)]
    pub fn b111100(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B111100)
    }
    #[doc = "AlwaysOn61_Signal"]
    #[inline(always)]
    pub fn b111101(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B111101)
    }
    #[doc = "AlwaysOn62_Signal"]
    #[inline(always)]
    pub fn b111110(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B111110)
    }
    #[doc = "AlwaysOn63_Signal"]
    #[inline(always)]
    pub fn b111111(self) -> &'a mut crate::W<REG> {
        self.variant(Source::B111111)
    }
}
#[doc = "DMA Channel Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trig {
    #[doc = "0: Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    B0 = 0,
    #[doc = "1: Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    B1 = 1,
}
impl From<Trig> for bool {
    #[inline(always)]
    fn from(variant: Trig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIG` reader - DMA Channel Trigger Enable"]
pub type TrigR = crate::BitReader<Trig>;
impl TrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trig {
        match self.bits {
            false => Trig::B0,
            true => Trig::B1,
        }
    }
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Trig::B0
    }
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Trig::B1
    }
}
#[doc = "Field `TRIG` writer - DMA Channel Trigger Enable"]
pub type TrigW<'a, REG> = crate::BitWriter<'a, REG, Trig>;
impl<'a, REG> TrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::B0)
    }
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Trig::B1)
    }
}
#[doc = "DMA Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enbl {
    #[doc = "0: DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    B0 = 0,
    #[doc = "1: DMA channel is enabled"]
    B1 = 1,
}
impl From<Enbl> for bool {
    #[inline(always)]
    fn from(variant: Enbl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENBL` reader - DMA Channel Enable"]
pub type EnblR = crate::BitReader<Enbl>;
impl EnblR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enbl {
        match self.bits {
            false => Enbl::B0,
            true => Enbl::B1,
        }
    }
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Enbl::B0
    }
    #[doc = "DMA channel is enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Enbl::B1
    }
}
#[doc = "Field `ENBL` writer - DMA Channel Enable"]
pub type EnblW<'a, REG> = crate::BitWriter<'a, REG, Enbl>;
impl<'a, REG> EnblW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Enbl::B0)
    }
    #[doc = "DMA channel is enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Enbl::B1)
    }
}
impl R {
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline(always)]
    pub fn source(&self) -> SourceR {
        SourceR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline(always)]
    pub fn trig(&self) -> TrigR {
        TrigR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline(always)]
    pub fn enbl(&self) -> EnblR {
        EnblR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SourceW<ChcfgSpec> {
        SourceW::new(self, 0)
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TrigW<ChcfgSpec> {
        TrigW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbl(&mut self) -> EnblW<ChcfgSpec> {
        EnblW::new(self, 7)
    }
}
#[doc = "Channel Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`chcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChcfgSpec;
impl crate::RegisterSpec for ChcfgSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`chcfg::R`](R) reader structure"]
impl crate::Readable for ChcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`chcfg::W`](W) writer structure"]
impl crate::Writable for ChcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CHCFG%s to value 0"]
impl crate::Resettable for ChcfgSpec {
    const RESET_VALUE: u8 = 0;
}
