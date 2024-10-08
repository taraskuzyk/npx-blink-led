#[doc = "Register `PCR4` reader"]
pub type R = crate::R<Pcr4Spec>;
#[doc = "Register `PCR4` writer"]
pub type W = crate::W<Pcr4Spec>;
#[doc = "Pull Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ps {
    #[doc = "0: Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    B0 = 0,
    #[doc = "1: Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    B1 = 1,
}
impl From<Ps> for bool {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PS` reader - Pull Select"]
pub type PsR = crate::BitReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            false => Ps::B0,
            true => Ps::B1,
        }
    }
    #[doc = "Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ps::B0
    }
    #[doc = "Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ps::B1
    }
}
#[doc = "Field `PS` writer - Pull Select"]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG, Ps>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal pulldown resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B0)
    }
    #[doc = "Internal pullup resistor is enabled on the corresponding pin, if the corresponding PE field is set."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::B1)
    }
}
#[doc = "Pull Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Internal pullup or pulldown resistor is not enabled on the corresponding pin."]
    B0 = 0,
    #[doc = "1: Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    B1 = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Pull Enable"]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::B0,
            true => Pe::B1,
        }
    }
    #[doc = "Internal pullup or pulldown resistor is not enabled on the corresponding pin."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pe::B0
    }
    #[doc = "Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pe::B1
    }
}
#[doc = "Field `PE` writer - Pull Enable"]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal pullup or pulldown resistor is not enabled on the corresponding pin."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::B0)
    }
    #[doc = "Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::B1)
    }
}
#[doc = "Slew Rate Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sre {
    #[doc = "0: Fast slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
    B0 = 0,
    #[doc = "1: Slow slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
    B1 = 1,
}
impl From<Sre> for bool {
    #[inline(always)]
    fn from(variant: Sre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRE` reader - Slew Rate Enable"]
pub type SreR = crate::BitReader<Sre>;
impl SreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sre {
        match self.bits {
            false => Sre::B0,
            true => Sre::B1,
        }
    }
    #[doc = "Fast slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sre::B0
    }
    #[doc = "Slow slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sre::B1
    }
}
#[doc = "Field `SRE` writer - Slew Rate Enable"]
pub type SreW<'a, REG> = crate::BitWriter<'a, REG, Sre>;
impl<'a, REG> SreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Fast slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sre::B0)
    }
    #[doc = "Slow slew rate is configured on the corresponding pin, if the pin is configured as a digital output."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sre::B1)
    }
}
#[doc = "Passive Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pfe {
    #[doc = "0: Passive input filter is disabled on the corresponding pin."]
    B0 = 0,
    #[doc = "1: Passive input filter is enabled on the corresponding pin, if the pin is configured as a digital input. Refer to the device data sheet for filter characteristics."]
    B1 = 1,
}
impl From<Pfe> for bool {
    #[inline(always)]
    fn from(variant: Pfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFE` reader - Passive Filter Enable"]
pub type PfeR = crate::BitReader<Pfe>;
impl PfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pfe {
        match self.bits {
            false => Pfe::B0,
            true => Pfe::B1,
        }
    }
    #[doc = "Passive input filter is disabled on the corresponding pin."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pfe::B0
    }
    #[doc = "Passive input filter is enabled on the corresponding pin, if the pin is configured as a digital input. Refer to the device data sheet for filter characteristics."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pfe::B1
    }
}
#[doc = "Drive Strength Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dse {
    #[doc = "0: Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    B0 = 0,
    #[doc = "1: High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    B1 = 1,
}
impl From<Dse> for bool {
    #[inline(always)]
    fn from(variant: Dse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSE` reader - Drive Strength Enable"]
pub type DseR = crate::BitReader<Dse>;
impl DseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dse {
        match self.bits {
            false => Dse::B0,
            true => Dse::B1,
        }
    }
    #[doc = "Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dse::B0
    }
    #[doc = "High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dse::B1
    }
}
#[doc = "Field `DSE` writer - Drive Strength Enable"]
pub type DseW<'a, REG> = crate::BitWriter<'a, REG, Dse>;
impl<'a, REG> DseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dse::B0)
    }
    #[doc = "High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dse::B1)
    }
}
#[doc = "Pin Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mux {
    #[doc = "0: Pin disabled (analog)."]
    B000 = 0,
    #[doc = "1: Alternative 1 (GPIO)."]
    B001 = 1,
    #[doc = "2: Alternative 2 (chip-specific)."]
    B010 = 2,
    #[doc = "3: Alternative 3 (chip-specific)."]
    B011 = 3,
    #[doc = "4: Alternative 4 (chip-specific)."]
    B100 = 4,
    #[doc = "5: Alternative 5 (chip-specific)."]
    B101 = 5,
    #[doc = "6: Alternative 6 (chip-specific)."]
    B110 = 6,
    #[doc = "7: Alternative 7 (chip-specific)."]
    B111 = 7,
}
impl From<Mux> for u8 {
    #[inline(always)]
    fn from(variant: Mux) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mux {
    type Ux = u8;
}
impl crate::IsEnum for Mux {}
#[doc = "Field `MUX` reader - Pin Mux Control"]
pub type MuxR = crate::FieldReader<Mux>;
impl MuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mux {
        match self.bits {
            0 => Mux::B000,
            1 => Mux::B001,
            2 => Mux::B010,
            3 => Mux::B011,
            4 => Mux::B100,
            5 => Mux::B101,
            6 => Mux::B110,
            7 => Mux::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Pin disabled (analog)."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Mux::B000
    }
    #[doc = "Alternative 1 (GPIO)."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Mux::B001
    }
    #[doc = "Alternative 2 (chip-specific)."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Mux::B010
    }
    #[doc = "Alternative 3 (chip-specific)."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Mux::B011
    }
    #[doc = "Alternative 4 (chip-specific)."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Mux::B100
    }
    #[doc = "Alternative 5 (chip-specific)."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Mux::B101
    }
    #[doc = "Alternative 6 (chip-specific)."]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Mux::B110
    }
    #[doc = "Alternative 7 (chip-specific)."]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Mux::B111
    }
}
#[doc = "Field `MUX` writer - Pin Mux Control"]
pub type MuxW<'a, REG> = crate::FieldWriter<'a, REG, 3, Mux, crate::Safe>;
impl<'a, REG> MuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Pin disabled (analog)."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::B000)
    }
    #[doc = "Alternative 1 (GPIO)."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::B001)
    }
    #[doc = "Alternative 2 (chip-specific)."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::B010)
    }
    #[doc = "Alternative 3 (chip-specific)."]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::B011)
    }
    #[doc = "Alternative 4 (chip-specific)."]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::B100)
    }
    #[doc = "Alternative 5 (chip-specific)."]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::B101)
    }
    #[doc = "Alternative 6 (chip-specific)."]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::B110)
    }
    #[doc = "Alternative 7 (chip-specific)."]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Mux::B111)
    }
}
#[doc = "Interrupt Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Irqc {
    #[doc = "0: Interrupt/DMA request disabled."]
    B0000 = 0,
    #[doc = "1: DMA request on rising edge."]
    B0001 = 1,
    #[doc = "2: DMA request on falling edge."]
    B0010 = 2,
    #[doc = "3: DMA request on either edge."]
    B0011 = 3,
    #[doc = "8: Interrupt when logic 0."]
    B1000 = 8,
    #[doc = "9: Interrupt on rising-edge."]
    B1001 = 9,
    #[doc = "10: Interrupt on falling-edge."]
    B1010 = 10,
    #[doc = "11: Interrupt on either edge."]
    B1011 = 11,
    #[doc = "12: Interrupt when logic 1."]
    B1100 = 12,
}
impl From<Irqc> for u8 {
    #[inline(always)]
    fn from(variant: Irqc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Irqc {
    type Ux = u8;
}
impl crate::IsEnum for Irqc {}
#[doc = "Field `IRQC` reader - Interrupt Configuration"]
pub type IrqcR = crate::FieldReader<Irqc>;
impl IrqcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Irqc> {
        match self.bits {
            0 => Some(Irqc::B0000),
            1 => Some(Irqc::B0001),
            2 => Some(Irqc::B0010),
            3 => Some(Irqc::B0011),
            8 => Some(Irqc::B1000),
            9 => Some(Irqc::B1001),
            10 => Some(Irqc::B1010),
            11 => Some(Irqc::B1011),
            12 => Some(Irqc::B1100),
            _ => None,
        }
    }
    #[doc = "Interrupt/DMA request disabled."]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == Irqc::B0000
    }
    #[doc = "DMA request on rising edge."]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == Irqc::B0001
    }
    #[doc = "DMA request on falling edge."]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == Irqc::B0010
    }
    #[doc = "DMA request on either edge."]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == Irqc::B0011
    }
    #[doc = "Interrupt when logic 0."]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == Irqc::B1000
    }
    #[doc = "Interrupt on rising-edge."]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == Irqc::B1001
    }
    #[doc = "Interrupt on falling-edge."]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == Irqc::B1010
    }
    #[doc = "Interrupt on either edge."]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == Irqc::B1011
    }
    #[doc = "Interrupt when logic 1."]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == Irqc::B1100
    }
}
#[doc = "Field `IRQC` writer - Interrupt Configuration"]
pub type IrqcW<'a, REG> = crate::FieldWriter<'a, REG, 4, Irqc>;
impl<'a, REG> IrqcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Interrupt/DMA request disabled."]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::B0000)
    }
    #[doc = "DMA request on rising edge."]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::B0001)
    }
    #[doc = "DMA request on falling edge."]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::B0010)
    }
    #[doc = "DMA request on either edge."]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::B0011)
    }
    #[doc = "Interrupt when logic 0."]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::B1000)
    }
    #[doc = "Interrupt on rising-edge."]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::B1001)
    }
    #[doc = "Interrupt on falling-edge."]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::B1010)
    }
    #[doc = "Interrupt on either edge."]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::B1011)
    }
    #[doc = "Interrupt when logic 1."]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(Irqc::B1100)
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Isf {
    #[doc = "0: Configured interrupt is not detected."]
    B0 = 0,
    #[doc = "1: Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    B1 = 1,
}
impl From<Isf> for bool {
    #[inline(always)]
    fn from(variant: Isf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISF` reader - Interrupt Status Flag"]
pub type IsfR = crate::BitReader<Isf>;
impl IsfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Isf {
        match self.bits {
            false => Isf::B0,
            true => Isf::B1,
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Isf::B0
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Isf::B1
    }
}
#[doc = "Field `ISF` writer - Interrupt Status Flag"]
pub type IsfW<'a, REG> = crate::BitWriter<'a, REG, Isf>;
impl<'a, REG> IsfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Isf::B0)
    }
    #[doc = "Configured interrupt is detected. If the pin is configured to generate a DMA request, then the corresponding flag will be cleared automatically at the completion of the requested DMA transfer. Otherwise, the flag remains set until a logic 1 is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Isf::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Pull Select"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slew Rate Enable"]
    #[inline(always)]
    pub fn sre(&self) -> SreR {
        SreR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Passive Filter Enable"]
    #[inline(always)]
    pub fn pfe(&self) -> PfeR {
        PfeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    pub fn dse(&self) -> DseR {
        DseR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    pub fn mux(&self) -> MuxR {
        MuxR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    pub fn irqc(&self) -> IrqcR {
        IrqcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&self) -> IsfR {
        IsfR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Pull Select"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<Pcr4Spec> {
        PsW::new(self, 0)
    }
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<Pcr4Spec> {
        PeW::new(self, 1)
    }
    #[doc = "Bit 2 - Slew Rate Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sre(&mut self) -> SreW<Pcr4Spec> {
        SreW::new(self, 2)
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dse(&mut self) -> DseW<Pcr4Spec> {
        DseW::new(self, 6)
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    #[must_use]
    pub fn mux(&mut self) -> MuxW<Pcr4Spec> {
        MuxW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn irqc(&mut self) -> IrqcW<Pcr4Spec> {
        IrqcW::new(self, 16)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn isf(&mut self) -> IsfW<Pcr4Spec> {
        IsfW::new(self, 24)
    }
}
#[doc = "Pin Control Register n\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcr4Spec;
impl crate::RegisterSpec for Pcr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr4::R`](R) reader structure"]
impl crate::Readable for Pcr4Spec {}
#[doc = "`write(|w| ..)` method takes [`pcr4::W`](W) writer structure"]
impl crate::Writable for Pcr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR4 to value 0x01"]
impl crate::Resettable for Pcr4Spec {
    const RESET_VALUE: u32 = 0x01;
}
