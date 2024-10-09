#[doc = "Register `DCR1` reader"]
pub type R = crate::R<Dcr1Spec>;
#[doc = "Register `DCR1` writer"]
pub type W = crate::W<Dcr1Spec>;
#[doc = "Link Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lch2 {
    #[doc = "0: DMA Channel 0"]
    B00 = 0,
    #[doc = "1: DMA Channel 1"]
    B01 = 1,
    #[doc = "2: DMA Channel 2"]
    B10 = 2,
    #[doc = "3: DMA Channel 3"]
    B11 = 3,
}
impl From<Lch2> for u8 {
    #[inline(always)]
    fn from(variant: Lch2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lch2 {
    type Ux = u8;
}
impl crate::IsEnum for Lch2 {}
#[doc = "Field `LCH2` reader - Link Channel 2"]
pub type Lch2R = crate::FieldReader<Lch2>;
impl Lch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lch2 {
        match self.bits {
            0 => Lch2::B00,
            1 => Lch2::B01,
            2 => Lch2::B10,
            3 => Lch2::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lch2::B00
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lch2::B01
    }
    #[doc = "DMA Channel 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lch2::B10
    }
    #[doc = "DMA Channel 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Lch2::B11
    }
}
#[doc = "Field `LCH2` writer - Link Channel 2"]
pub type Lch2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Lch2, crate::Safe>;
impl<'a, REG> Lch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lch2::B00)
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lch2::B01)
    }
    #[doc = "DMA Channel 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lch2::B10)
    }
    #[doc = "DMA Channel 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Lch2::B11)
    }
}
#[doc = "Link Channel 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lch1 {
    #[doc = "0: DMA Channel 0"]
    B00 = 0,
    #[doc = "1: DMA Channel 1"]
    B01 = 1,
    #[doc = "2: DMA Channel 2"]
    B10 = 2,
    #[doc = "3: DMA Channel 3"]
    B11 = 3,
}
impl From<Lch1> for u8 {
    #[inline(always)]
    fn from(variant: Lch1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lch1 {
    type Ux = u8;
}
impl crate::IsEnum for Lch1 {}
#[doc = "Field `LCH1` reader - Link Channel 1"]
pub type Lch1R = crate::FieldReader<Lch1>;
impl Lch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lch1 {
        match self.bits {
            0 => Lch1::B00,
            1 => Lch1::B01,
            2 => Lch1::B10,
            3 => Lch1::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Lch1::B00
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Lch1::B01
    }
    #[doc = "DMA Channel 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Lch1::B10
    }
    #[doc = "DMA Channel 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Lch1::B11
    }
}
#[doc = "Field `LCH1` writer - Link Channel 1"]
pub type Lch1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Lch1, crate::Safe>;
impl<'a, REG> Lch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA Channel 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Lch1::B00)
    }
    #[doc = "DMA Channel 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Lch1::B01)
    }
    #[doc = "DMA Channel 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Lch1::B10)
    }
    #[doc = "DMA Channel 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Lch1::B11)
    }
}
#[doc = "Link Channel Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Linkcc {
    #[doc = "0: No channel-to-channel linking"]
    B00 = 0,
    #[doc = "1: Perform a link to channel LCH1 after each cycle-steal transfer followed by a link to LCH2 after the BCR decrements to 0."]
    B01 = 1,
    #[doc = "2: Perform a link to channel LCH1 after each cycle-steal transfer"]
    B10 = 2,
    #[doc = "3: Perform a link to channel LCH1 after the BCR decrements to 0."]
    B11 = 3,
}
impl From<Linkcc> for u8 {
    #[inline(always)]
    fn from(variant: Linkcc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Linkcc {
    type Ux = u8;
}
impl crate::IsEnum for Linkcc {}
#[doc = "Field `LINKCC` reader - Link Channel Control"]
pub type LinkccR = crate::FieldReader<Linkcc>;
impl LinkccR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Linkcc {
        match self.bits {
            0 => Linkcc::B00,
            1 => Linkcc::B01,
            2 => Linkcc::B10,
            3 => Linkcc::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "No channel-to-channel linking"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Linkcc::B00
    }
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer followed by a link to LCH2 after the BCR decrements to 0."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Linkcc::B01
    }
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Linkcc::B10
    }
    #[doc = "Perform a link to channel LCH1 after the BCR decrements to 0."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Linkcc::B11
    }
}
#[doc = "Field `LINKCC` writer - Link Channel Control"]
pub type LinkccW<'a, REG> = crate::FieldWriter<'a, REG, 2, Linkcc, crate::Safe>;
impl<'a, REG> LinkccW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No channel-to-channel linking"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Linkcc::B00)
    }
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer followed by a link to LCH2 after the BCR decrements to 0."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Linkcc::B01)
    }
    #[doc = "Perform a link to channel LCH1 after each cycle-steal transfer"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Linkcc::B10)
    }
    #[doc = "Perform a link to channel LCH1 after the BCR decrements to 0."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Linkcc::B11)
    }
}
#[doc = "Disable Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DReq {
    #[doc = "0: ERQ bit is not affected."]
    B0 = 0,
    #[doc = "1: ERQ bit is cleared when the BCR is exhausted."]
    B1 = 1,
}
impl From<DReq> for bool {
    #[inline(always)]
    fn from(variant: DReq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D_REQ` reader - Disable Request"]
pub type DReqR = crate::BitReader<DReq>;
impl DReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DReq {
        match self.bits {
            false => DReq::B0,
            true => DReq::B1,
        }
    }
    #[doc = "ERQ bit is not affected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DReq::B0
    }
    #[doc = "ERQ bit is cleared when the BCR is exhausted."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DReq::B1
    }
}
#[doc = "Field `D_REQ` writer - Disable Request"]
pub type DReqW<'a, REG> = crate::BitWriter<'a, REG, DReq>;
impl<'a, REG> DReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ERQ bit is not affected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DReq::B0)
    }
    #[doc = "ERQ bit is cleared when the BCR is exhausted."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DReq::B1)
    }
}
#[doc = "Destination Address Modulo\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmod {
    #[doc = "0: Buffer disabled"]
    B0000 = 0,
    #[doc = "1: Circular buffer size is 16 bytes"]
    B0001 = 1,
    #[doc = "2: Circular buffer size is 32 bytes"]
    B0010 = 2,
    #[doc = "3: Circular buffer size is 64 bytes"]
    B0011 = 3,
    #[doc = "4: Circular buffer size is 128 bytes"]
    B0100 = 4,
    #[doc = "5: Circular buffer size is 256 bytes"]
    B0101 = 5,
    #[doc = "6: Circular buffer size is 512 bytes"]
    B0110 = 6,
    #[doc = "7: Circular buffer size is 1 KB"]
    B0111 = 7,
    #[doc = "8: Circular buffer size is 2 KB"]
    B1000 = 8,
    #[doc = "9: Circular buffer size is 4 KB"]
    B1001 = 9,
    #[doc = "10: Circular buffer size is 8 KB"]
    B1010 = 10,
    #[doc = "11: Circular buffer size is 16 KB"]
    B1011 = 11,
    #[doc = "12: Circular buffer size is 32 KB"]
    B1100 = 12,
    #[doc = "13: Circular buffer size is 64 KB"]
    B1101 = 13,
    #[doc = "14: Circular buffer size is 128 KB"]
    B1110 = 14,
    #[doc = "15: Circular buffer size is 256 KB"]
    B1111 = 15,
}
impl From<Dmod> for u8 {
    #[inline(always)]
    fn from(variant: Dmod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmod {
    type Ux = u8;
}
impl crate::IsEnum for Dmod {}
#[doc = "Field `DMOD` reader - Destination Address Modulo"]
pub type DmodR = crate::FieldReader<Dmod>;
impl DmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmod {
        match self.bits {
            0 => Dmod::B0000,
            1 => Dmod::B0001,
            2 => Dmod::B0010,
            3 => Dmod::B0011,
            4 => Dmod::B0100,
            5 => Dmod::B0101,
            6 => Dmod::B0110,
            7 => Dmod::B0111,
            8 => Dmod::B1000,
            9 => Dmod::B1001,
            10 => Dmod::B1010,
            11 => Dmod::B1011,
            12 => Dmod::B1100,
            13 => Dmod::B1101,
            14 => Dmod::B1110,
            15 => Dmod::B1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Buffer disabled"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Dmod::B0000
    }
    #[doc = "Circular buffer size is 16 bytes"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Dmod::B0001
    }
    #[doc = "Circular buffer size is 32 bytes"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Dmod::B0010
    }
    #[doc = "Circular buffer size is 64 bytes"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Dmod::B0011
    }
    #[doc = "Circular buffer size is 128 bytes"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Dmod::B0100
    }
    #[doc = "Circular buffer size is 256 bytes"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Dmod::B0101
    }
    #[doc = "Circular buffer size is 512 bytes"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Dmod::B0110
    }
    #[doc = "Circular buffer size is 1 KB"]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == Dmod::B0111
    }
    #[doc = "Circular buffer size is 2 KB"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Dmod::B1000
    }
    #[doc = "Circular buffer size is 4 KB"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Dmod::B1001
    }
    #[doc = "Circular buffer size is 8 KB"]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == Dmod::B1010
    }
    #[doc = "Circular buffer size is 16 KB"]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == Dmod::B1011
    }
    #[doc = "Circular buffer size is 32 KB"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == Dmod::B1100
    }
    #[doc = "Circular buffer size is 64 KB"]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == Dmod::B1101
    }
    #[doc = "Circular buffer size is 128 KB"]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == Dmod::B1110
    }
    #[doc = "Circular buffer size is 256 KB"]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == Dmod::B1111
    }
}
#[doc = "Field `DMOD` writer - Destination Address Modulo"]
pub type DmodW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dmod, crate::Safe>;
impl<'a, REG> DmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Buffer disabled"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B0000)
    }
    #[doc = "Circular buffer size is 16 bytes"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B0001)
    }
    #[doc = "Circular buffer size is 32 bytes"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B0010)
    }
    #[doc = "Circular buffer size is 64 bytes"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B0011)
    }
    #[doc = "Circular buffer size is 128 bytes"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B0100)
    }
    #[doc = "Circular buffer size is 256 bytes"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B0101)
    }
    #[doc = "Circular buffer size is 512 bytes"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B0110)
    }
    #[doc = "Circular buffer size is 1 KB"]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B0111)
    }
    #[doc = "Circular buffer size is 2 KB"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B1000)
    }
    #[doc = "Circular buffer size is 4 KB"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B1001)
    }
    #[doc = "Circular buffer size is 8 KB"]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B1010)
    }
    #[doc = "Circular buffer size is 16 KB"]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B1011)
    }
    #[doc = "Circular buffer size is 32 KB"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B1100)
    }
    #[doc = "Circular buffer size is 64 KB"]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B1101)
    }
    #[doc = "Circular buffer size is 128 KB"]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B1110)
    }
    #[doc = "Circular buffer size is 256 KB"]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(Dmod::B1111)
    }
}
#[doc = "Source Address Modulo\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Smod {
    #[doc = "0: Buffer disabled"]
    B0000 = 0,
    #[doc = "1: Circular buffer size is 16 bytes."]
    B0001 = 1,
    #[doc = "2: Circular buffer size is 32 bytes."]
    B0010 = 2,
    #[doc = "3: Circular buffer size is 64 bytes."]
    B0011 = 3,
    #[doc = "4: Circular buffer size is 128 bytes."]
    B0100 = 4,
    #[doc = "5: Circular buffer size is 256 bytes."]
    B0101 = 5,
    #[doc = "6: Circular buffer size is 512 bytes."]
    B0110 = 6,
    #[doc = "7: Circular buffer size is 1 KB."]
    B0111 = 7,
    #[doc = "8: Circular buffer size is 2 KB."]
    B1000 = 8,
    #[doc = "9: Circular buffer size is 4 KB."]
    B1001 = 9,
    #[doc = "10: Circular buffer size is 8 KB."]
    B1010 = 10,
    #[doc = "11: Circular buffer size is 16 KB."]
    B1011 = 11,
    #[doc = "12: Circular buffer size is 32 KB."]
    B1100 = 12,
    #[doc = "13: Circular buffer size is 64 KB."]
    B1101 = 13,
    #[doc = "14: Circular buffer size is 128 KB."]
    B1110 = 14,
    #[doc = "15: Circular buffer size is 256 KB."]
    B1111 = 15,
}
impl From<Smod> for u8 {
    #[inline(always)]
    fn from(variant: Smod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Smod {
    type Ux = u8;
}
impl crate::IsEnum for Smod {}
#[doc = "Field `SMOD` reader - Source Address Modulo"]
pub type SmodR = crate::FieldReader<Smod>;
impl SmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smod {
        match self.bits {
            0 => Smod::B0000,
            1 => Smod::B0001,
            2 => Smod::B0010,
            3 => Smod::B0011,
            4 => Smod::B0100,
            5 => Smod::B0101,
            6 => Smod::B0110,
            7 => Smod::B0111,
            8 => Smod::B1000,
            9 => Smod::B1001,
            10 => Smod::B1010,
            11 => Smod::B1011,
            12 => Smod::B1100,
            13 => Smod::B1101,
            14 => Smod::B1110,
            15 => Smod::B1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Buffer disabled"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Smod::B0000
    }
    #[doc = "Circular buffer size is 16 bytes."]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Smod::B0001
    }
    #[doc = "Circular buffer size is 32 bytes."]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Smod::B0010
    }
    #[doc = "Circular buffer size is 64 bytes."]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Smod::B0011
    }
    #[doc = "Circular buffer size is 128 bytes."]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Smod::B0100
    }
    #[doc = "Circular buffer size is 256 bytes."]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Smod::B0101
    }
    #[doc = "Circular buffer size is 512 bytes."]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Smod::B0110
    }
    #[doc = "Circular buffer size is 1 KB."]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == Smod::B0111
    }
    #[doc = "Circular buffer size is 2 KB."]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Smod::B1000
    }
    #[doc = "Circular buffer size is 4 KB."]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Smod::B1001
    }
    #[doc = "Circular buffer size is 8 KB."]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == Smod::B1010
    }
    #[doc = "Circular buffer size is 16 KB."]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == Smod::B1011
    }
    #[doc = "Circular buffer size is 32 KB."]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == Smod::B1100
    }
    #[doc = "Circular buffer size is 64 KB."]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == Smod::B1101
    }
    #[doc = "Circular buffer size is 128 KB."]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == Smod::B1110
    }
    #[doc = "Circular buffer size is 256 KB."]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == Smod::B1111
    }
}
#[doc = "Field `SMOD` writer - Source Address Modulo"]
pub type SmodW<'a, REG> = crate::FieldWriter<'a, REG, 4, Smod, crate::Safe>;
impl<'a, REG> SmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Buffer disabled"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B0000)
    }
    #[doc = "Circular buffer size is 16 bytes."]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B0001)
    }
    #[doc = "Circular buffer size is 32 bytes."]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B0010)
    }
    #[doc = "Circular buffer size is 64 bytes."]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B0011)
    }
    #[doc = "Circular buffer size is 128 bytes."]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B0100)
    }
    #[doc = "Circular buffer size is 256 bytes."]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B0101)
    }
    #[doc = "Circular buffer size is 512 bytes."]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B0110)
    }
    #[doc = "Circular buffer size is 1 KB."]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B0111)
    }
    #[doc = "Circular buffer size is 2 KB."]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B1000)
    }
    #[doc = "Circular buffer size is 4 KB."]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B1001)
    }
    #[doc = "Circular buffer size is 8 KB."]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B1010)
    }
    #[doc = "Circular buffer size is 16 KB."]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B1011)
    }
    #[doc = "Circular buffer size is 32 KB."]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B1100)
    }
    #[doc = "Circular buffer size is 64 KB."]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B1101)
    }
    #[doc = "Circular buffer size is 128 KB."]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B1110)
    }
    #[doc = "Circular buffer size is 256 KB."]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(Smod::B1111)
    }
}
#[doc = "Start Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: DMA inactive"]
    B0 = 0,
    #[doc = "1: The DMA begins the transfer in accordance to the values in the TCDn. START is cleared automatically after one module clock and always reads as logic 0."]
    B1 = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start Transfer"]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::B0,
            true => Start::B1,
        }
    }
    #[doc = "DMA inactive"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Start::B0
    }
    #[doc = "The DMA begins the transfer in accordance to the values in the TCDn. START is cleared automatically after one module clock and always reads as logic 0."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Start::B1
    }
}
#[doc = "Field `START` writer - Start Transfer"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA inactive"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::B0)
    }
    #[doc = "The DMA begins the transfer in accordance to the values in the TCDn. START is cleared automatically after one module clock and always reads as logic 0."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::B1)
    }
}
#[doc = "Destination Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dsize {
    #[doc = "0: 32-bit"]
    B00 = 0,
    #[doc = "1: 8-bit"]
    B01 = 1,
    #[doc = "2: 16-bit"]
    B10 = 2,
    #[doc = "3: Reserved (generates a configuration error (DSRn\\[CE\\]) if incorrectly specified at time of channel activation)"]
    B11 = 3,
}
impl From<Dsize> for u8 {
    #[inline(always)]
    fn from(variant: Dsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dsize {
    type Ux = u8;
}
impl crate::IsEnum for Dsize {}
#[doc = "Field `DSIZE` reader - Destination Size"]
pub type DsizeR = crate::FieldReader<Dsize>;
impl DsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dsize {
        match self.bits {
            0 => Dsize::B00,
            1 => Dsize::B01,
            2 => Dsize::B10,
            3 => Dsize::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Dsize::B00
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Dsize::B01
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Dsize::B10
    }
    #[doc = "Reserved (generates a configuration error (DSRn\\[CE\\]) if incorrectly specified at time of channel activation)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Dsize::B11
    }
}
#[doc = "Field `DSIZE` writer - Destination Size"]
pub type DsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dsize, crate::Safe>;
impl<'a, REG> DsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::B00)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::B01)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::B10)
    }
    #[doc = "Reserved (generates a configuration error (DSRn\\[CE\\]) if incorrectly specified at time of channel activation)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Dsize::B11)
    }
}
#[doc = "Destination Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dinc {
    #[doc = "0: No change to the DAR after a successful transfer."]
    B0 = 0,
    #[doc = "1: The DAR increments by 1, 2, 4 depending upon the size of the transfer."]
    B1 = 1,
}
impl From<Dinc> for bool {
    #[inline(always)]
    fn from(variant: Dinc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DINC` reader - Destination Increment"]
pub type DincR = crate::BitReader<Dinc>;
impl DincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dinc {
        match self.bits {
            false => Dinc::B0,
            true => Dinc::B1,
        }
    }
    #[doc = "No change to the DAR after a successful transfer."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dinc::B0
    }
    #[doc = "The DAR increments by 1, 2, 4 depending upon the size of the transfer."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dinc::B1
    }
}
#[doc = "Field `DINC` writer - Destination Increment"]
pub type DincW<'a, REG> = crate::BitWriter<'a, REG, Dinc>;
impl<'a, REG> DincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change to the DAR after a successful transfer."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dinc::B0)
    }
    #[doc = "The DAR increments by 1, 2, 4 depending upon the size of the transfer."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dinc::B1)
    }
}
#[doc = "Source Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ssize {
    #[doc = "0: 32-bit"]
    B00 = 0,
    #[doc = "1: 8-bit"]
    B01 = 1,
    #[doc = "2: 16-bit"]
    B10 = 2,
    #[doc = "3: Reserved (generates a configuration error (DSRn\\[CE\\]) if incorrectly specified at time of channel activation)"]
    B11 = 3,
}
impl From<Ssize> for u8 {
    #[inline(always)]
    fn from(variant: Ssize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ssize {
    type Ux = u8;
}
impl crate::IsEnum for Ssize {}
#[doc = "Field `SSIZE` reader - Source Size"]
pub type SsizeR = crate::FieldReader<Ssize>;
impl SsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ssize {
        match self.bits {
            0 => Ssize::B00,
            1 => Ssize::B01,
            2 => Ssize::B10,
            3 => Ssize::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Ssize::B00
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Ssize::B01
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Ssize::B10
    }
    #[doc = "Reserved (generates a configuration error (DSRn\\[CE\\]) if incorrectly specified at time of channel activation)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Ssize::B11
    }
}
#[doc = "Field `SSIZE` writer - Source Size"]
pub type SsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ssize, crate::Safe>;
impl<'a, REG> SsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Ssize::B00)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Ssize::B01)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Ssize::B10)
    }
    #[doc = "Reserved (generates a configuration error (DSRn\\[CE\\]) if incorrectly specified at time of channel activation)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Ssize::B11)
    }
}
#[doc = "Source Increment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sinc {
    #[doc = "0: No change to SAR after a successful transfer."]
    B0 = 0,
    #[doc = "1: The SAR increments by 1, 2, 4 as determined by the transfer size."]
    B1 = 1,
}
impl From<Sinc> for bool {
    #[inline(always)]
    fn from(variant: Sinc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINC` reader - Source Increment"]
pub type SincR = crate::BitReader<Sinc>;
impl SincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sinc {
        match self.bits {
            false => Sinc::B0,
            true => Sinc::B1,
        }
    }
    #[doc = "No change to SAR after a successful transfer."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sinc::B0
    }
    #[doc = "The SAR increments by 1, 2, 4 as determined by the transfer size."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sinc::B1
    }
}
#[doc = "Field `SINC` writer - Source Increment"]
pub type SincW<'a, REG> = crate::BitWriter<'a, REG, Sinc>;
impl<'a, REG> SincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No change to SAR after a successful transfer."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sinc::B0)
    }
    #[doc = "The SAR increments by 1, 2, 4 as determined by the transfer size."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sinc::B1)
    }
}
#[doc = "Enable asynchronous DMA requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eadreq {
    #[doc = "0: Disabled"]
    B0 = 0,
    #[doc = "1: Enabled"]
    B1 = 1,
}
impl From<Eadreq> for bool {
    #[inline(always)]
    fn from(variant: Eadreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EADREQ` reader - Enable asynchronous DMA requests"]
pub type EadreqR = crate::BitReader<Eadreq>;
impl EadreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eadreq {
        match self.bits {
            false => Eadreq::B0,
            true => Eadreq::B1,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Eadreq::B0
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Eadreq::B1
    }
}
#[doc = "Field `EADREQ` writer - Enable asynchronous DMA requests"]
pub type EadreqW<'a, REG> = crate::BitWriter<'a, REG, Eadreq>;
impl<'a, REG> EadreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Eadreq::B0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Eadreq::B1)
    }
}
#[doc = "Auto-align\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aa {
    #[doc = "0: Auto-align disabled"]
    B0 = 0,
    #[doc = "1: If SSIZE indicates a transfer no smaller than DSIZE, source accesses are auto-aligned; otherwise, destination accesses are auto-aligned. Source alignment takes precedence over destination alignment. If auto-alignment is enabled, the appropriate address register increments, regardless of DINC or SINC."]
    B1 = 1,
}
impl From<Aa> for bool {
    #[inline(always)]
    fn from(variant: Aa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AA` reader - Auto-align"]
pub type AaR = crate::BitReader<Aa>;
impl AaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aa {
        match self.bits {
            false => Aa::B0,
            true => Aa::B1,
        }
    }
    #[doc = "Auto-align disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Aa::B0
    }
    #[doc = "If SSIZE indicates a transfer no smaller than DSIZE, source accesses are auto-aligned; otherwise, destination accesses are auto-aligned. Source alignment takes precedence over destination alignment. If auto-alignment is enabled, the appropriate address register increments, regardless of DINC or SINC."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Aa::B1
    }
}
#[doc = "Field `AA` writer - Auto-align"]
pub type AaW<'a, REG> = crate::BitWriter<'a, REG, Aa>;
impl<'a, REG> AaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto-align disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Aa::B0)
    }
    #[doc = "If SSIZE indicates a transfer no smaller than DSIZE, source accesses are auto-aligned; otherwise, destination accesses are auto-aligned. Source alignment takes precedence over destination alignment. If auto-alignment is enabled, the appropriate address register increments, regardless of DINC or SINC."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Aa::B1)
    }
}
#[doc = "Cycle Steal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cs {
    #[doc = "0: DMA continuously makes read/write transfers until the BCR decrements to 0."]
    B0 = 0,
    #[doc = "1: Forces a single read/write transfer per request."]
    B1 = 1,
}
impl From<Cs> for bool {
    #[inline(always)]
    fn from(variant: Cs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS` reader - Cycle Steal"]
pub type CsR = crate::BitReader<Cs>;
impl CsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cs {
        match self.bits {
            false => Cs::B0,
            true => Cs::B1,
        }
    }
    #[doc = "DMA continuously makes read/write transfers until the BCR decrements to 0."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cs::B0
    }
    #[doc = "Forces a single read/write transfer per request."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cs::B1
    }
}
#[doc = "Field `CS` writer - Cycle Steal"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG, Cs>;
impl<'a, REG> CsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA continuously makes read/write transfers until the BCR decrements to 0."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::B0)
    }
    #[doc = "Forces a single read/write transfer per request."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::B1)
    }
}
#[doc = "Enable Peripheral Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erq {
    #[doc = "0: Peripheral request is ignored."]
    B0 = 0,
    #[doc = "1: Enables peripheral request to initiate transfer. A software-initiated request (setting START) is always enabled."]
    B1 = 1,
}
impl From<Erq> for bool {
    #[inline(always)]
    fn from(variant: Erq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERQ` reader - Enable Peripheral Request"]
pub type ErqR = crate::BitReader<Erq>;
impl ErqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erq {
        match self.bits {
            false => Erq::B0,
            true => Erq::B1,
        }
    }
    #[doc = "Peripheral request is ignored."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Erq::B0
    }
    #[doc = "Enables peripheral request to initiate transfer. A software-initiated request (setting START) is always enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Erq::B1
    }
}
#[doc = "Field `ERQ` writer - Enable Peripheral Request"]
pub type ErqW<'a, REG> = crate::BitWriter<'a, REG, Erq>;
impl<'a, REG> ErqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral request is ignored."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Erq::B0)
    }
    #[doc = "Enables peripheral request to initiate transfer. A software-initiated request (setting START) is always enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Erq::B1)
    }
}
#[doc = "Enable Interrupt on Completion of Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eint {
    #[doc = "0: No interrupt is generated."]
    B0 = 0,
    #[doc = "1: Interrupt signal is enabled."]
    B1 = 1,
}
impl From<Eint> for bool {
    #[inline(always)]
    fn from(variant: Eint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EINT` reader - Enable Interrupt on Completion of Transfer"]
pub type EintR = crate::BitReader<Eint>;
impl EintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eint {
        match self.bits {
            false => Eint::B0,
            true => Eint::B1,
        }
    }
    #[doc = "No interrupt is generated."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Eint::B0
    }
    #[doc = "Interrupt signal is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Eint::B1
    }
}
#[doc = "Field `EINT` writer - Enable Interrupt on Completion of Transfer"]
pub type EintW<'a, REG> = crate::BitWriter<'a, REG, Eint>;
impl<'a, REG> EintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt is generated."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Eint::B0)
    }
    #[doc = "Interrupt signal is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Eint::B1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Link Channel 2"]
    #[inline(always)]
    pub fn lch2(&self) -> Lch2R {
        Lch2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Link Channel 1"]
    #[inline(always)]
    pub fn lch1(&self) -> Lch1R {
        Lch1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Link Channel Control"]
    #[inline(always)]
    pub fn linkcc(&self) -> LinkccR {
        LinkccR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable Request"]
    #[inline(always)]
    pub fn d_req(&self) -> DReqR {
        DReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Destination Address Modulo"]
    #[inline(always)]
    pub fn dmod(&self) -> DmodR {
        DmodR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Source Address Modulo"]
    #[inline(always)]
    pub fn smod(&self) -> SmodR {
        SmodR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Start Transfer"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Destination Size"]
    #[inline(always)]
    pub fn dsize(&self) -> DsizeR {
        DsizeR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Destination Increment"]
    #[inline(always)]
    pub fn dinc(&self) -> DincR {
        DincR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Source Size"]
    #[inline(always)]
    pub fn ssize(&self) -> SsizeR {
        SsizeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Source Increment"]
    #[inline(always)]
    pub fn sinc(&self) -> SincR {
        SincR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable asynchronous DMA requests"]
    #[inline(always)]
    pub fn eadreq(&self) -> EadreqR {
        EadreqR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - Auto-align"]
    #[inline(always)]
    pub fn aa(&self) -> AaR {
        AaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Cycle Steal"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable Peripheral Request"]
    #[inline(always)]
    pub fn erq(&self) -> ErqR {
        ErqR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Interrupt on Completion of Transfer"]
    #[inline(always)]
    pub fn eint(&self) -> EintR {
        EintR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Link Channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn lch2(&mut self) -> Lch2W<Dcr1Spec> {
        Lch2W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Link Channel 1"]
    #[inline(always)]
    #[must_use]
    pub fn lch1(&mut self) -> Lch1W<Dcr1Spec> {
        Lch1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Link Channel Control"]
    #[inline(always)]
    #[must_use]
    pub fn linkcc(&mut self) -> LinkccW<Dcr1Spec> {
        LinkccW::new(self, 4)
    }
    #[doc = "Bit 7 - Disable Request"]
    #[inline(always)]
    #[must_use]
    pub fn d_req(&mut self) -> DReqW<Dcr1Spec> {
        DReqW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Destination Address Modulo"]
    #[inline(always)]
    #[must_use]
    pub fn dmod(&mut self) -> DmodW<Dcr1Spec> {
        DmodW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Source Address Modulo"]
    #[inline(always)]
    #[must_use]
    pub fn smod(&mut self) -> SmodW<Dcr1Spec> {
        SmodW::new(self, 12)
    }
    #[doc = "Bit 16 - Start Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<Dcr1Spec> {
        StartW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Destination Size"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DsizeW<Dcr1Spec> {
        DsizeW::new(self, 17)
    }
    #[doc = "Bit 19 - Destination Increment"]
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DincW<Dcr1Spec> {
        DincW::new(self, 19)
    }
    #[doc = "Bits 20:21 - Source Size"]
    #[inline(always)]
    #[must_use]
    pub fn ssize(&mut self) -> SsizeW<Dcr1Spec> {
        SsizeW::new(self, 20)
    }
    #[doc = "Bit 22 - Source Increment"]
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SincW<Dcr1Spec> {
        SincW::new(self, 22)
    }
    #[doc = "Bit 23 - Enable asynchronous DMA requests"]
    #[inline(always)]
    #[must_use]
    pub fn eadreq(&mut self) -> EadreqW<Dcr1Spec> {
        EadreqW::new(self, 23)
    }
    #[doc = "Bit 28 - Auto-align"]
    #[inline(always)]
    #[must_use]
    pub fn aa(&mut self) -> AaW<Dcr1Spec> {
        AaW::new(self, 28)
    }
    #[doc = "Bit 29 - Cycle Steal"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CsW<Dcr1Spec> {
        CsW::new(self, 29)
    }
    #[doc = "Bit 30 - Enable Peripheral Request"]
    #[inline(always)]
    #[must_use]
    pub fn erq(&mut self) -> ErqW<Dcr1Spec> {
        ErqW::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Interrupt on Completion of Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn eint(&mut self) -> EintW<Dcr1Spec> {
        EintW::new(self, 31)
    }
}
#[doc = "DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcr1Spec;
impl crate::RegisterSpec for Dcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcr1::R`](R) reader structure"]
impl crate::Readable for Dcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dcr1::W`](W) writer structure"]
impl crate::Writable for Dcr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCR1 to value 0"]
impl crate::Resettable for Dcr1Spec {
    const RESET_VALUE: u32 = 0;
}
