#[doc = "Register `TIMCFG%s` reader"]
pub type R = crate::R<TimcfgSpec>;
#[doc = "Register `TIMCFG%s` writer"]
pub type W = crate::W<TimcfgSpec>;
#[doc = "Timer Start Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tstart {
    #[doc = "0: Start bit disabled"]
    B0 = 0,
    #[doc = "1: Start bit enabled"]
    B1 = 1,
}
impl From<Tstart> for bool {
    #[inline(always)]
    fn from(variant: Tstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSTART` reader - Timer Start Bit"]
pub type TstartR = crate::BitReader<Tstart>;
impl TstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstart {
        match self.bits {
            false => Tstart::B0,
            true => Tstart::B1,
        }
    }
    #[doc = "Start bit disabled"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tstart::B0
    }
    #[doc = "Start bit enabled"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tstart::B1
    }
}
#[doc = "Field `TSTART` writer - Timer Start Bit"]
pub type TstartW<'a, REG> = crate::BitWriter<'a, REG, Tstart>;
impl<'a, REG> TstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start bit disabled"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::B0)
    }
    #[doc = "Start bit enabled"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::B1)
    }
}
#[doc = "Timer Stop Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstop {
    #[doc = "0: Stop bit disabled"]
    B00 = 0,
    #[doc = "1: Stop bit is enabled on timer compare"]
    B01 = 1,
    #[doc = "2: Stop bit is enabled on timer disable"]
    B10 = 2,
    #[doc = "3: Stop bit is enabled on timer compare and timer disable"]
    B11 = 3,
}
impl From<Tstop> for u8 {
    #[inline(always)]
    fn from(variant: Tstop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstop {
    type Ux = u8;
}
impl crate::IsEnum for Tstop {}
#[doc = "Field `TSTOP` reader - Timer Stop Bit"]
pub type TstopR = crate::FieldReader<Tstop>;
impl TstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tstop {
        match self.bits {
            0 => Tstop::B00,
            1 => Tstop::B01,
            2 => Tstop::B10,
            3 => Tstop::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Stop bit disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Tstop::B00
    }
    #[doc = "Stop bit is enabled on timer compare"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Tstop::B01
    }
    #[doc = "Stop bit is enabled on timer disable"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Tstop::B10
    }
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Tstop::B11
    }
}
#[doc = "Field `TSTOP` writer - Timer Stop Bit"]
pub type TstopW<'a, REG> = crate::FieldWriter<'a, REG, 2, Tstop, crate::Safe>;
impl<'a, REG> TstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop bit disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::B00)
    }
    #[doc = "Stop bit is enabled on timer compare"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::B01)
    }
    #[doc = "Stop bit is enabled on timer disable"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::B10)
    }
    #[doc = "Stop bit is enabled on timer compare and timer disable"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::B11)
    }
}
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timena {
    #[doc = "0: Timer always enabled"]
    B000 = 0,
    #[doc = "1: Timer enabled on Timer N-1 enable"]
    B001 = 1,
    #[doc = "2: Timer enabled on Trigger high"]
    B010 = 2,
    #[doc = "3: Timer enabled on Trigger high and Pin high"]
    B011 = 3,
    #[doc = "4: Timer enabled on Pin rising edge"]
    B100 = 4,
    #[doc = "5: Timer enabled on Pin rising edge and Trigger high"]
    B101 = 5,
    #[doc = "6: Timer enabled on Trigger rising edge"]
    B110 = 6,
    #[doc = "7: Timer enabled on Trigger rising or falling edge"]
    B111 = 7,
}
impl From<Timena> for u8 {
    #[inline(always)]
    fn from(variant: Timena) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timena {
    type Ux = u8;
}
impl crate::IsEnum for Timena {}
#[doc = "Field `TIMENA` reader - Timer Enable"]
pub type TimenaR = crate::FieldReader<Timena>;
impl TimenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timena {
        match self.bits {
            0 => Timena::B000,
            1 => Timena::B001,
            2 => Timena::B010,
            3 => Timena::B011,
            4 => Timena::B100,
            5 => Timena::B101,
            6 => Timena::B110,
            7 => Timena::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer always enabled"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Timena::B000
    }
    #[doc = "Timer enabled on Timer N-1 enable"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Timena::B001
    }
    #[doc = "Timer enabled on Trigger high"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Timena::B010
    }
    #[doc = "Timer enabled on Trigger high and Pin high"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Timena::B011
    }
    #[doc = "Timer enabled on Pin rising edge"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Timena::B100
    }
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Timena::B101
    }
    #[doc = "Timer enabled on Trigger rising edge"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Timena::B110
    }
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Timena::B111
    }
}
#[doc = "Field `TIMENA` writer - Timer Enable"]
pub type TimenaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Timena, crate::Safe>;
impl<'a, REG> TimenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer always enabled"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Timena::B000)
    }
    #[doc = "Timer enabled on Timer N-1 enable"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Timena::B001)
    }
    #[doc = "Timer enabled on Trigger high"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Timena::B010)
    }
    #[doc = "Timer enabled on Trigger high and Pin high"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Timena::B011)
    }
    #[doc = "Timer enabled on Pin rising edge"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Timena::B100)
    }
    #[doc = "Timer enabled on Pin rising edge and Trigger high"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Timena::B101)
    }
    #[doc = "Timer enabled on Trigger rising edge"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Timena::B110)
    }
    #[doc = "Timer enabled on Trigger rising or falling edge"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Timena::B111)
    }
}
#[doc = "Timer Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timdis {
    #[doc = "0: Timer never disabled"]
    B000 = 0,
    #[doc = "1: Timer disabled on Timer N-1 disable"]
    B001 = 1,
    #[doc = "2: Timer disabled on Timer compare"]
    B010 = 2,
    #[doc = "3: Timer disabled on Timer compare and Trigger Low"]
    B011 = 3,
    #[doc = "4: Timer disabled on Pin rising or falling edge"]
    B100 = 4,
    #[doc = "5: Timer disabled on Pin rising or falling edge provided Trigger is high"]
    B101 = 5,
    #[doc = "6: Timer disabled on Trigger falling edge"]
    B110 = 6,
}
impl From<Timdis> for u8 {
    #[inline(always)]
    fn from(variant: Timdis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timdis {
    type Ux = u8;
}
impl crate::IsEnum for Timdis {}
#[doc = "Field `TIMDIS` reader - Timer Disable"]
pub type TimdisR = crate::FieldReader<Timdis>;
impl TimdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timdis> {
        match self.bits {
            0 => Some(Timdis::B000),
            1 => Some(Timdis::B001),
            2 => Some(Timdis::B010),
            3 => Some(Timdis::B011),
            4 => Some(Timdis::B100),
            5 => Some(Timdis::B101),
            6 => Some(Timdis::B110),
            _ => None,
        }
    }
    #[doc = "Timer never disabled"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Timdis::B000
    }
    #[doc = "Timer disabled on Timer N-1 disable"]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Timdis::B001
    }
    #[doc = "Timer disabled on Timer compare"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Timdis::B010
    }
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Timdis::B011
    }
    #[doc = "Timer disabled on Pin rising or falling edge"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Timdis::B100
    }
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Timdis::B101
    }
    #[doc = "Timer disabled on Trigger falling edge"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Timdis::B110
    }
}
#[doc = "Field `TIMDIS` writer - Timer Disable"]
pub type TimdisW<'a, REG> = crate::FieldWriter<'a, REG, 3, Timdis>;
impl<'a, REG> TimdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer never disabled"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Timdis::B000)
    }
    #[doc = "Timer disabled on Timer N-1 disable"]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Timdis::B001)
    }
    #[doc = "Timer disabled on Timer compare"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Timdis::B010)
    }
    #[doc = "Timer disabled on Timer compare and Trigger Low"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Timdis::B011)
    }
    #[doc = "Timer disabled on Pin rising or falling edge"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Timdis::B100)
    }
    #[doc = "Timer disabled on Pin rising or falling edge provided Trigger is high"]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Timdis::B101)
    }
    #[doc = "Timer disabled on Trigger falling edge"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Timdis::B110)
    }
}
#[doc = "Timer Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timrst {
    #[doc = "0: Timer never reset"]
    B000 = 0,
    #[doc = "2: Timer reset on Timer Pin equal to Timer Output"]
    B010 = 2,
    #[doc = "3: Timer reset on Timer Trigger equal to Timer Output"]
    B011 = 3,
    #[doc = "4: Timer reset on Timer Pin rising edge"]
    B100 = 4,
    #[doc = "6: Timer reset on Trigger rising edge"]
    B110 = 6,
    #[doc = "7: Timer reset on Trigger rising or falling edge"]
    B111 = 7,
}
impl From<Timrst> for u8 {
    #[inline(always)]
    fn from(variant: Timrst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timrst {
    type Ux = u8;
}
impl crate::IsEnum for Timrst {}
#[doc = "Field `TIMRST` reader - Timer Reset"]
pub type TimrstR = crate::FieldReader<Timrst>;
impl TimrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timrst> {
        match self.bits {
            0 => Some(Timrst::B000),
            2 => Some(Timrst::B010),
            3 => Some(Timrst::B011),
            4 => Some(Timrst::B100),
            6 => Some(Timrst::B110),
            7 => Some(Timrst::B111),
            _ => None,
        }
    }
    #[doc = "Timer never reset"]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Timrst::B000
    }
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Timrst::B010
    }
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Timrst::B011
    }
    #[doc = "Timer reset on Timer Pin rising edge"]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Timrst::B100
    }
    #[doc = "Timer reset on Trigger rising edge"]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Timrst::B110
    }
    #[doc = "Timer reset on Trigger rising or falling edge"]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Timrst::B111
    }
}
#[doc = "Field `TIMRST` writer - Timer Reset"]
pub type TimrstW<'a, REG> = crate::FieldWriter<'a, REG, 3, Timrst>;
impl<'a, REG> TimrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer never reset"]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Timrst::B000)
    }
    #[doc = "Timer reset on Timer Pin equal to Timer Output"]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Timrst::B010)
    }
    #[doc = "Timer reset on Timer Trigger equal to Timer Output"]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Timrst::B011)
    }
    #[doc = "Timer reset on Timer Pin rising edge"]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Timrst::B100)
    }
    #[doc = "Timer reset on Trigger rising edge"]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Timrst::B110)
    }
    #[doc = "Timer reset on Trigger rising or falling edge"]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Timrst::B111)
    }
}
#[doc = "Timer Decrement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timdec {
    #[doc = "0: Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    B00 = 0,
    #[doc = "1: Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    B01 = 1,
    #[doc = "2: Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    B10 = 2,
    #[doc = "3: Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    B11 = 3,
}
impl From<Timdec> for u8 {
    #[inline(always)]
    fn from(variant: Timdec) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timdec {
    type Ux = u8;
}
impl crate::IsEnum for Timdec {}
#[doc = "Field `TIMDEC` reader - Timer Decrement"]
pub type TimdecR = crate::FieldReader<Timdec>;
impl TimdecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timdec {
        match self.bits {
            0 => Timdec::B00,
            1 => Timdec::B01,
            2 => Timdec::B10,
            3 => Timdec::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Timdec::B00
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Timdec::B01
    }
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Timdec::B10
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Timdec::B11
    }
}
#[doc = "Field `TIMDEC` writer - Timer Decrement"]
pub type TimdecW<'a, REG> = crate::FieldWriter<'a, REG, 2, Timdec, crate::Safe>;
impl<'a, REG> TimdecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Decrement counter on FlexIO clock, Shift clock equals Timer output."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Timdec::B00)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Timer output."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Timdec::B01)
    }
    #[doc = "Decrement counter on Pin input (both edges), Shift clock equals Pin input."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Timdec::B10)
    }
    #[doc = "Decrement counter on Trigger input (both edges), Shift clock equals Trigger input."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Timdec::B11)
    }
}
#[doc = "Timer Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Timout {
    #[doc = "0: Timer output is logic one when enabled and is not affected by timer reset"]
    B00 = 0,
    #[doc = "1: Timer output is logic zero when enabled and is not affected by timer reset"]
    B01 = 1,
    #[doc = "2: Timer output is logic one when enabled and on timer reset"]
    B10 = 2,
    #[doc = "3: Timer output is logic zero when enabled and on timer reset"]
    B11 = 3,
}
impl From<Timout> for u8 {
    #[inline(always)]
    fn from(variant: Timout) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timout {
    type Ux = u8;
}
impl crate::IsEnum for Timout {}
#[doc = "Field `TIMOUT` reader - Timer Output"]
pub type TimoutR = crate::FieldReader<Timout>;
impl TimoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timout {
        match self.bits {
            0 => Timout::B00,
            1 => Timout::B01,
            2 => Timout::B10,
            3 => Timout::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Timout::B00
    }
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Timout::B01
    }
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Timout::B10
    }
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Timout::B11
    }
}
#[doc = "Field `TIMOUT` writer - Timer Output"]
pub type TimoutW<'a, REG> = crate::FieldWriter<'a, REG, 2, Timout, crate::Safe>;
impl<'a, REG> TimoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer output is logic one when enabled and is not affected by timer reset"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Timout::B00)
    }
    #[doc = "Timer output is logic zero when enabled and is not affected by timer reset"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Timout::B01)
    }
    #[doc = "Timer output is logic one when enabled and on timer reset"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Timout::B10)
    }
    #[doc = "Timer output is logic zero when enabled and on timer reset"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Timout::B11)
    }
}
impl R {
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline(always)]
    pub fn tstop(&self) -> TstopR {
        TstopR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline(always)]
    pub fn timena(&self) -> TimenaR {
        TimenaR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline(always)]
    pub fn timdis(&self) -> TimdisR {
        TimdisR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline(always)]
    pub fn timrst(&self) -> TimrstR {
        TimrstR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline(always)]
    pub fn timdec(&self) -> TimdecR {
        TimdecR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline(always)]
    pub fn timout(&self) -> TimoutR {
        TimoutR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Timer Start Bit"]
    #[inline(always)]
    #[must_use]
    pub fn tstart(&mut self) -> TstartW<TimcfgSpec> {
        TstartW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Timer Stop Bit"]
    #[inline(always)]
    #[must_use]
    pub fn tstop(&mut self) -> TstopW<TimcfgSpec> {
        TstopW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn timena(&mut self) -> TimenaW<TimcfgSpec> {
        TimenaW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Timer Disable"]
    #[inline(always)]
    #[must_use]
    pub fn timdis(&mut self) -> TimdisW<TimcfgSpec> {
        TimdisW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Timer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn timrst(&mut self) -> TimrstW<TimcfgSpec> {
        TimrstW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Timer Decrement"]
    #[inline(always)]
    #[must_use]
    pub fn timdec(&mut self) -> TimdecW<TimcfgSpec> {
        TimdecW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Timer Output"]
    #[inline(always)]
    #[must_use]
    pub fn timout(&mut self) -> TimoutW<TimcfgSpec> {
        TimoutW::new(self, 24)
    }
}
#[doc = "Timer Configuration N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimcfgSpec;
impl crate::RegisterSpec for TimcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timcfg::R`](R) reader structure"]
impl crate::Readable for TimcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`timcfg::W`](W) writer structure"]
impl crate::Writable for TimcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMCFG%s to value 0"]
impl crate::Resettable for TimcfgSpec {
    const RESET_VALUE: u32 = 0;
}
