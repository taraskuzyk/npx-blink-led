#[doc = "Register `CONF` reader"]
pub type R = crate::R<ConfSpec>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<ConfSpec>;
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dozeen {
    #[doc = "0: Internal TPM counter continues in Doze mode."]
    B0 = 0,
    #[doc = "1: Internal TPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    B1 = 1,
}
impl From<Dozeen> for bool {
    #[inline(always)]
    fn from(variant: Dozeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOZEEN` reader - Doze Enable"]
pub type DozeenR = crate::BitReader<Dozeen>;
impl DozeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dozeen {
        match self.bits {
            false => Dozeen::B0,
            true => Dozeen::B1,
        }
    }
    #[doc = "Internal TPM counter continues in Doze mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dozeen::B0
    }
    #[doc = "Internal TPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dozeen::B1
    }
}
#[doc = "Field `DOZEEN` writer - Doze Enable"]
pub type DozeenW<'a, REG> = crate::BitWriter<'a, REG, Dozeen>;
impl<'a, REG> DozeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal TPM counter continues in Doze mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dozeen::B0)
    }
    #[doc = "Internal TPM counter is paused and does not increment during Doze mode. Trigger inputs and input capture events are also ignored."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dozeen::B1)
    }
}
#[doc = "Debug Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dbgmode {
    #[doc = "0: TPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    B00 = 0,
    #[doc = "3: TPM counter continues in debug mode."]
    B11 = 3,
}
impl From<Dbgmode> for u8 {
    #[inline(always)]
    fn from(variant: Dbgmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dbgmode {
    type Ux = u8;
}
impl crate::IsEnum for Dbgmode {}
#[doc = "Field `DBGMODE` reader - Debug Mode"]
pub type DbgmodeR = crate::FieldReader<Dbgmode>;
impl DbgmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dbgmode> {
        match self.bits {
            0 => Some(Dbgmode::B00),
            3 => Some(Dbgmode::B11),
            _ => None,
        }
    }
    #[doc = "TPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Dbgmode::B00
    }
    #[doc = "TPM counter continues in debug mode."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Dbgmode::B11
    }
}
#[doc = "Field `DBGMODE` writer - Debug Mode"]
pub type DbgmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dbgmode>;
impl<'a, REG> DbgmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TPM counter is paused and does not increment during debug mode. Trigger inputs and input capture events are also ignored."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgmode::B00)
    }
    #[doc = "TPM counter continues in debug mode."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Dbgmode::B11)
    }
}
#[doc = "Global Time Base Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gtbsync {
    #[doc = "0: Global timebase synchronization disabled."]
    B0 = 0,
    #[doc = "1: Global timebase synchronization enabled."]
    B1 = 1,
}
impl From<Gtbsync> for bool {
    #[inline(always)]
    fn from(variant: Gtbsync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTBSYNC` reader - Global Time Base Synchronization"]
pub type GtbsyncR = crate::BitReader<Gtbsync>;
impl GtbsyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gtbsync {
        match self.bits {
            false => Gtbsync::B0,
            true => Gtbsync::B1,
        }
    }
    #[doc = "Global timebase synchronization disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gtbsync::B0
    }
    #[doc = "Global timebase synchronization enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gtbsync::B1
    }
}
#[doc = "Field `GTBSYNC` writer - Global Time Base Synchronization"]
pub type GtbsyncW<'a, REG> = crate::BitWriter<'a, REG, Gtbsync>;
impl<'a, REG> GtbsyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Global timebase synchronization disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gtbsync::B0)
    }
    #[doc = "Global timebase synchronization enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gtbsync::B1)
    }
}
#[doc = "Global time base enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gtbeen {
    #[doc = "0: All channels use the internally generated TPM counter as their timebase"]
    B0 = 0,
    #[doc = "1: All channels use an externally generated global timebase as their timebase"]
    B1 = 1,
}
impl From<Gtbeen> for bool {
    #[inline(always)]
    fn from(variant: Gtbeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GTBEEN` reader - Global time base enable"]
pub type GtbeenR = crate::BitReader<Gtbeen>;
impl GtbeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gtbeen {
        match self.bits {
            false => Gtbeen::B0,
            true => Gtbeen::B1,
        }
    }
    #[doc = "All channels use the internally generated TPM counter as their timebase"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Gtbeen::B0
    }
    #[doc = "All channels use an externally generated global timebase as their timebase"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Gtbeen::B1
    }
}
#[doc = "Field `GTBEEN` writer - Global time base enable"]
pub type GtbeenW<'a, REG> = crate::BitWriter<'a, REG, Gtbeen>;
impl<'a, REG> GtbeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All channels use the internally generated TPM counter as their timebase"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Gtbeen::B0)
    }
    #[doc = "All channels use an externally generated global timebase as their timebase"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Gtbeen::B1)
    }
}
#[doc = "Counter Start on Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csot {
    #[doc = "0: TPM counter starts to increment immediately, once it is enabled."]
    B0 = 0,
    #[doc = "1: TPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    B1 = 1,
}
impl From<Csot> for bool {
    #[inline(always)]
    fn from(variant: Csot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSOT` reader - Counter Start on Trigger"]
pub type CsotR = crate::BitReader<Csot>;
impl CsotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csot {
        match self.bits {
            false => Csot::B0,
            true => Csot::B1,
        }
    }
    #[doc = "TPM counter starts to increment immediately, once it is enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Csot::B0
    }
    #[doc = "TPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Csot::B1
    }
}
#[doc = "Field `CSOT` writer - Counter Start on Trigger"]
pub type CsotW<'a, REG> = crate::BitWriter<'a, REG, Csot>;
impl<'a, REG> CsotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TPM counter starts to increment immediately, once it is enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Csot::B0)
    }
    #[doc = "TPM counter only starts to increment when it a rising edge on the selected input trigger is detected, after it has been enabled or after it has stopped due to overflow."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Csot::B1)
    }
}
#[doc = "Counter Stop On Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csoo {
    #[doc = "0: TPM counter continues incrementing or decrementing after overflow"]
    B0 = 0,
    #[doc = "1: TPM counter stops incrementing or decrementing after overflow."]
    B1 = 1,
}
impl From<Csoo> for bool {
    #[inline(always)]
    fn from(variant: Csoo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSOO` reader - Counter Stop On Overflow"]
pub type CsooR = crate::BitReader<Csoo>;
impl CsooR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csoo {
        match self.bits {
            false => Csoo::B0,
            true => Csoo::B1,
        }
    }
    #[doc = "TPM counter continues incrementing or decrementing after overflow"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Csoo::B0
    }
    #[doc = "TPM counter stops incrementing or decrementing after overflow."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Csoo::B1
    }
}
#[doc = "Field `CSOO` writer - Counter Stop On Overflow"]
pub type CsooW<'a, REG> = crate::BitWriter<'a, REG, Csoo>;
impl<'a, REG> CsooW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TPM counter continues incrementing or decrementing after overflow"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Csoo::B0)
    }
    #[doc = "TPM counter stops incrementing or decrementing after overflow."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Csoo::B1)
    }
}
#[doc = "Counter Reload On Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Crot {
    #[doc = "0: Counter is not reloaded due to a rising edge on the selected input trigger"]
    B0 = 0,
    #[doc = "1: Counter is reloaded when a rising edge is detected on the selected input trigger"]
    B1 = 1,
}
impl From<Crot> for bool {
    #[inline(always)]
    fn from(variant: Crot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CROT` reader - Counter Reload On Trigger"]
pub type CrotR = crate::BitReader<Crot>;
impl CrotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crot {
        match self.bits {
            false => Crot::B0,
            true => Crot::B1,
        }
    }
    #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Crot::B0
    }
    #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Crot::B1
    }
}
#[doc = "Field `CROT` writer - Counter Reload On Trigger"]
pub type CrotW<'a, REG> = crate::BitWriter<'a, REG, Crot>;
impl<'a, REG> CrotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Counter is not reloaded due to a rising edge on the selected input trigger"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Crot::B0)
    }
    #[doc = "Counter is reloaded when a rising edge is detected on the selected input trigger"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Crot::B1)
    }
}
#[doc = "Field `CPOT` reader - Counter Pause On Trigger"]
pub type CpotR = crate::BitReader;
#[doc = "Field `CPOT` writer - Counter Pause On Trigger"]
pub type CpotW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Trigger Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgpol {
    #[doc = "0: Trigger is active high."]
    B0 = 0,
    #[doc = "1: Trigger is active low."]
    B1 = 1,
}
impl From<Trgpol> for bool {
    #[inline(always)]
    fn from(variant: Trgpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGPOL` reader - Trigger Polarity"]
pub type TrgpolR = crate::BitReader<Trgpol>;
impl TrgpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgpol {
        match self.bits {
            false => Trgpol::B0,
            true => Trgpol::B1,
        }
    }
    #[doc = "Trigger is active high."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Trgpol::B0
    }
    #[doc = "Trigger is active low."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Trgpol::B1
    }
}
#[doc = "Field `TRGPOL` writer - Trigger Polarity"]
pub type TrgpolW<'a, REG> = crate::BitWriter<'a, REG, Trgpol>;
impl<'a, REG> TrgpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger is active high."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgpol::B0)
    }
    #[doc = "Trigger is active low."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgpol::B1)
    }
}
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trgsrc {
    #[doc = "0: Trigger source selected by TRGSEL is external."]
    B0 = 0,
    #[doc = "1: Trigger source selected by TRGSEL is internal (channel pin input capture)."]
    B1 = 1,
}
impl From<Trgsrc> for bool {
    #[inline(always)]
    fn from(variant: Trgsrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRGSRC` reader - Trigger Source"]
pub type TrgsrcR = crate::BitReader<Trgsrc>;
impl TrgsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trgsrc {
        match self.bits {
            false => Trgsrc::B0,
            true => Trgsrc::B1,
        }
    }
    #[doc = "Trigger source selected by TRGSEL is external."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Trgsrc::B0
    }
    #[doc = "Trigger source selected by TRGSEL is internal (channel pin input capture)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Trgsrc::B1
    }
}
#[doc = "Field `TRGSRC` writer - Trigger Source"]
pub type TrgsrcW<'a, REG> = crate::BitWriter<'a, REG, Trgsrc>;
impl<'a, REG> TrgsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger source selected by TRGSEL is external."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc::B0)
    }
    #[doc = "Trigger source selected by TRGSEL is internal (channel pin input capture)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsrc::B1)
    }
}
#[doc = "Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trgsel {
    #[doc = "1: Channel 0 pin input capture"]
    B0001 = 1,
    #[doc = "2: Channel 1 pin input capture"]
    B0010 = 2,
    #[doc = "3: Channel 0 or Channel 1 pin input capture"]
    B0011 = 3,
    #[doc = "4: Channel 2 pin input capture"]
    B0100 = 4,
    #[doc = "5: Channel 0 or Channel 2 pin input capture"]
    B0101 = 5,
    #[doc = "6: Channel 1 or Channel 2 pin input capture"]
    B0110 = 6,
    #[doc = "7: Channel 0 or Channel 1 or Channel 2 pin input capture"]
    B0111 = 7,
    #[doc = "8: Channel 3 pin input capture"]
    B1000 = 8,
    #[doc = "9: Channel 0 or Channel 3 pin input capture"]
    B1001 = 9,
    #[doc = "10: Channel 1 or Channel 3 pin input capture"]
    B1010 = 10,
    #[doc = "11: Channel 0 or Channel 1 or Channel 3 pin input capture"]
    B1011 = 11,
    #[doc = "12: Channel 2 or Channel 3 pin input capture"]
    B1100 = 12,
    #[doc = "13: Channel 0 or Channel 2 or Channel 3 pin input capture"]
    B1101 = 13,
    #[doc = "14: Channel 1 or Channel 2 or Channel 3 pin input capture"]
    B1110 = 14,
    #[doc = "15: Channel 0 or Channel 1 or Channel 2 or Channel 3 pin input capture"]
    B1111 = 15,
}
impl From<Trgsel> for u8 {
    #[inline(always)]
    fn from(variant: Trgsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trgsel {
    type Ux = u8;
}
impl crate::IsEnum for Trgsel {}
#[doc = "Field `TRGSEL` reader - Trigger Select"]
pub type TrgselR = crate::FieldReader<Trgsel>;
impl TrgselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trgsel> {
        match self.bits {
            1 => Some(Trgsel::B0001),
            2 => Some(Trgsel::B0010),
            3 => Some(Trgsel::B0011),
            4 => Some(Trgsel::B0100),
            5 => Some(Trgsel::B0101),
            6 => Some(Trgsel::B0110),
            7 => Some(Trgsel::B0111),
            8 => Some(Trgsel::B1000),
            9 => Some(Trgsel::B1001),
            10 => Some(Trgsel::B1010),
            11 => Some(Trgsel::B1011),
            12 => Some(Trgsel::B1100),
            13 => Some(Trgsel::B1101),
            14 => Some(Trgsel::B1110),
            15 => Some(Trgsel::B1111),
            _ => None,
        }
    }
    #[doc = "Channel 0 pin input capture"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Trgsel::B0001
    }
    #[doc = "Channel 1 pin input capture"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Trgsel::B0010
    }
    #[doc = "Channel 0 or Channel 1 pin input capture"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Trgsel::B0011
    }
    #[doc = "Channel 2 pin input capture"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == Trgsel::B0100
    }
    #[doc = "Channel 0 or Channel 2 pin input capture"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == Trgsel::B0101
    }
    #[doc = "Channel 1 or Channel 2 pin input capture"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == Trgsel::B0110
    }
    #[doc = "Channel 0 or Channel 1 or Channel 2 pin input capture"]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == Trgsel::B0111
    }
    #[doc = "Channel 3 pin input capture"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Trgsel::B1000
    }
    #[doc = "Channel 0 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Trgsel::B1001
    }
    #[doc = "Channel 1 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == Trgsel::B1010
    }
    #[doc = "Channel 0 or Channel 1 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == Trgsel::B1011
    }
    #[doc = "Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == Trgsel::B1100
    }
    #[doc = "Channel 0 or Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == Trgsel::B1101
    }
    #[doc = "Channel 1 or Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == Trgsel::B1110
    }
    #[doc = "Channel 0 or Channel 1 or Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == Trgsel::B1111
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Select"]
pub type TrgselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Trgsel>;
impl<'a, REG> TrgselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel 0 pin input capture"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B0001)
    }
    #[doc = "Channel 1 pin input capture"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B0010)
    }
    #[doc = "Channel 0 or Channel 1 pin input capture"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B0011)
    }
    #[doc = "Channel 2 pin input capture"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B0100)
    }
    #[doc = "Channel 0 or Channel 2 pin input capture"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B0101)
    }
    #[doc = "Channel 1 or Channel 2 pin input capture"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B0110)
    }
    #[doc = "Channel 0 or Channel 1 or Channel 2 pin input capture"]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B0111)
    }
    #[doc = "Channel 3 pin input capture"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B1000)
    }
    #[doc = "Channel 0 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B1001)
    }
    #[doc = "Channel 1 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B1010)
    }
    #[doc = "Channel 0 or Channel 1 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B1011)
    }
    #[doc = "Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B1100)
    }
    #[doc = "Channel 0 or Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B1101)
    }
    #[doc = "Channel 1 or Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B1110)
    }
    #[doc = "Channel 0 or Channel 1 or Channel 2 or Channel 3 pin input capture"]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(Trgsel::B1111)
    }
}
impl R {
    #[doc = "Bit 5 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&self) -> DozeenR {
        DozeenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    pub fn dbgmode(&self) -> DbgmodeR {
        DbgmodeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Global Time Base Synchronization"]
    #[inline(always)]
    pub fn gtbsync(&self) -> GtbsyncR {
        GtbsyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global time base enable"]
    #[inline(always)]
    pub fn gtbeen(&self) -> GtbeenR {
        GtbeenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Counter Start on Trigger"]
    #[inline(always)]
    pub fn csot(&self) -> CsotR {
        CsotR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Counter Stop On Overflow"]
    #[inline(always)]
    pub fn csoo(&self) -> CsooR {
        CsooR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Counter Reload On Trigger"]
    #[inline(always)]
    pub fn crot(&self) -> CrotR {
        CrotR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Counter Pause On Trigger"]
    #[inline(always)]
    pub fn cpot(&self) -> CpotR {
        CpotR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Trigger Polarity"]
    #[inline(always)]
    pub fn trgpol(&self) -> TrgpolR {
        TrgpolR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TrgsrcR {
        TrgsrcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TrgselR {
        TrgselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - Doze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dozeen(&mut self) -> DozeenW<ConfSpec> {
        DozeenW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Debug Mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbgmode(&mut self) -> DbgmodeW<ConfSpec> {
        DbgmodeW::new(self, 6)
    }
    #[doc = "Bit 8 - Global Time Base Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn gtbsync(&mut self) -> GtbsyncW<ConfSpec> {
        GtbsyncW::new(self, 8)
    }
    #[doc = "Bit 9 - Global time base enable"]
    #[inline(always)]
    #[must_use]
    pub fn gtbeen(&mut self) -> GtbeenW<ConfSpec> {
        GtbeenW::new(self, 9)
    }
    #[doc = "Bit 16 - Counter Start on Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn csot(&mut self) -> CsotW<ConfSpec> {
        CsotW::new(self, 16)
    }
    #[doc = "Bit 17 - Counter Stop On Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn csoo(&mut self) -> CsooW<ConfSpec> {
        CsooW::new(self, 17)
    }
    #[doc = "Bit 18 - Counter Reload On Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn crot(&mut self) -> CrotW<ConfSpec> {
        CrotW::new(self, 18)
    }
    #[doc = "Bit 19 - Counter Pause On Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn cpot(&mut self) -> CpotW<ConfSpec> {
        CpotW::new(self, 19)
    }
    #[doc = "Bit 22 - Trigger Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn trgpol(&mut self) -> TrgpolW<ConfSpec> {
        TrgpolW::new(self, 22)
    }
    #[doc = "Bit 23 - Trigger Source"]
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TrgsrcW<ConfSpec> {
        TrgsrcW::new(self, 23)
    }
    #[doc = "Bits 24:27 - Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TrgselW<ConfSpec> {
        TrgselW::new(self, 24)
    }
}
#[doc = "Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfSpec;
impl crate::RegisterSpec for ConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for ConfSpec {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for ConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for ConfSpec {
    const RESET_VALUE: u32 = 0;
}
