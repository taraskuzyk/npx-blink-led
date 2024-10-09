#[doc = "Register `SC2` reader"]
pub type R = crate::R<Sc2Spec>;
#[doc = "Register `SC2` writer"]
pub type W = crate::W<Sc2Spec>;
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    B00 = 0,
    #[doc = "1: Alternate reference pair, that is, VALTH and VALTL . This pair may be additional external pins or internal sources depending on the MCU configuration. See the chip configuration information for details specific to this MCU"]
    B01 = 1,
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsel {
    type Ux = u8;
}
impl crate::IsEnum for Refsel {}
#[doc = "Field `REFSEL` reader - Voltage Reference Selection"]
pub type RefselR = crate::FieldReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refsel> {
        match self.bits {
            0 => Some(Refsel::B00),
            1 => Some(Refsel::B01),
            _ => None,
        }
    }
    #[doc = "Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Refsel::B00
    }
    #[doc = "Alternate reference pair, that is, VALTH and VALTL . This pair may be additional external pins or internal sources depending on the MCU configuration. See the chip configuration information for details specific to this MCU"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Refsel::B01
    }
}
#[doc = "Field `REFSEL` writer - Voltage Reference Selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refsel>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::B00)
    }
    #[doc = "Alternate reference pair, that is, VALTH and VALTL . This pair may be additional external pins or internal sources depending on the MCU configuration. See the chip configuration information for details specific to this MCU"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::B01)
    }
}
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaen {
    #[doc = "0: DMA is disabled."]
    B0 = 0,
    #[doc = "1: DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event noted when any of the SC1n\\[COCO\\]
flags is asserted."]
    B1 = 1,
}
impl From<Dmaen> for bool {
    #[inline(always)]
    fn from(variant: Dmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DmaenR = crate::BitReader<Dmaen>;
impl DmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaen {
        match self.bits {
            false => Dmaen::B0,
            true => Dmaen::B1,
        }
    }
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Dmaen::B0
    }
    #[doc = "DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event noted when any of the SC1n\\[COCO\\]
flags is asserted."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Dmaen::B1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG, Dmaen>;
impl<'a, REG> DmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::B0)
    }
    #[doc = "DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event noted when any of the SC1n\\[COCO\\]
flags is asserted."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaen::B1)
    }
}
#[doc = "Compare Function Range Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acren {
    #[doc = "0: Range function disabled. Only CV1 is compared."]
    B0 = 0,
    #[doc = "1: Range function enabled. Both CV1 and CV2 are compared."]
    B1 = 1,
}
impl From<Acren> for bool {
    #[inline(always)]
    fn from(variant: Acren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACREN` reader - Compare Function Range Enable"]
pub type AcrenR = crate::BitReader<Acren>;
impl AcrenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acren {
        match self.bits {
            false => Acren::B0,
            true => Acren::B1,
        }
    }
    #[doc = "Range function disabled. Only CV1 is compared."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acren::B0
    }
    #[doc = "Range function enabled. Both CV1 and CV2 are compared."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acren::B1
    }
}
#[doc = "Field `ACREN` writer - Compare Function Range Enable"]
pub type AcrenW<'a, REG> = crate::BitWriter<'a, REG, Acren>;
impl<'a, REG> AcrenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Range function disabled. Only CV1 is compared."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Acren::B0)
    }
    #[doc = "Range function enabled. Both CV1 and CV2 are compared."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Acren::B1)
    }
}
#[doc = "Compare Function Greater Than Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acfgt {
    #[doc = "0: Configures less than threshold, outside range not inclusive and inside range not inclusive; functionality based on the values placed in CV1 and CV2."]
    B0 = 0,
    #[doc = "1: Configures greater than or equal to threshold, outside and inside ranges inclusive; functionality based on the values placed in CV1 and CV2."]
    B1 = 1,
}
impl From<Acfgt> for bool {
    #[inline(always)]
    fn from(variant: Acfgt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACFGT` reader - Compare Function Greater Than Enable"]
pub type AcfgtR = crate::BitReader<Acfgt>;
impl AcfgtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acfgt {
        match self.bits {
            false => Acfgt::B0,
            true => Acfgt::B1,
        }
    }
    #[doc = "Configures less than threshold, outside range not inclusive and inside range not inclusive; functionality based on the values placed in CV1 and CV2."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acfgt::B0
    }
    #[doc = "Configures greater than or equal to threshold, outside and inside ranges inclusive; functionality based on the values placed in CV1 and CV2."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acfgt::B1
    }
}
#[doc = "Field `ACFGT` writer - Compare Function Greater Than Enable"]
pub type AcfgtW<'a, REG> = crate::BitWriter<'a, REG, Acfgt>;
impl<'a, REG> AcfgtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Configures less than threshold, outside range not inclusive and inside range not inclusive; functionality based on the values placed in CV1 and CV2."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Acfgt::B0)
    }
    #[doc = "Configures greater than or equal to threshold, outside and inside ranges inclusive; functionality based on the values placed in CV1 and CV2."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Acfgt::B1)
    }
}
#[doc = "Compare Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acfe {
    #[doc = "0: Compare function disabled."]
    B0 = 0,
    #[doc = "1: Compare function enabled."]
    B1 = 1,
}
impl From<Acfe> for bool {
    #[inline(always)]
    fn from(variant: Acfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACFE` reader - Compare Function Enable"]
pub type AcfeR = crate::BitReader<Acfe>;
impl AcfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acfe {
        match self.bits {
            false => Acfe::B0,
            true => Acfe::B1,
        }
    }
    #[doc = "Compare function disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Acfe::B0
    }
    #[doc = "Compare function enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Acfe::B1
    }
}
#[doc = "Field `ACFE` writer - Compare Function Enable"]
pub type AcfeW<'a, REG> = crate::BitWriter<'a, REG, Acfe>;
impl<'a, REG> AcfeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Compare function disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Acfe::B0)
    }
    #[doc = "Compare function enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Acfe::B1)
    }
}
#[doc = "Conversion Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adtrg {
    #[doc = "0: Software trigger selected."]
    B0 = 0,
    #[doc = "1: Hardware trigger selected."]
    B1 = 1,
}
impl From<Adtrg> for bool {
    #[inline(always)]
    fn from(variant: Adtrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADTRG` reader - Conversion Trigger Select"]
pub type AdtrgR = crate::BitReader<Adtrg>;
impl AdtrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adtrg {
        match self.bits {
            false => Adtrg::B0,
            true => Adtrg::B1,
        }
    }
    #[doc = "Software trigger selected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adtrg::B0
    }
    #[doc = "Hardware trigger selected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adtrg::B1
    }
}
#[doc = "Field `ADTRG` writer - Conversion Trigger Select"]
pub type AdtrgW<'a, REG> = crate::BitWriter<'a, REG, Adtrg>;
impl<'a, REG> AdtrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger selected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adtrg::B0)
    }
    #[doc = "Hardware trigger selected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adtrg::B1)
    }
}
#[doc = "Conversion Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adact {
    #[doc = "0: Conversion not in progress."]
    B0 = 0,
    #[doc = "1: Conversion in progress."]
    B1 = 1,
}
impl From<Adact> for bool {
    #[inline(always)]
    fn from(variant: Adact) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADACT` reader - Conversion Active"]
pub type AdactR = crate::BitReader<Adact>;
impl AdactR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adact {
        match self.bits {
            false => Adact::B0,
            true => Adact::B1,
        }
    }
    #[doc = "Conversion not in progress."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adact::B0
    }
    #[doc = "Conversion in progress."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adact::B1
    }
}
impl R {
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Function Range Enable"]
    #[inline(always)]
    pub fn acren(&self) -> AcrenR {
        AcrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&self) -> AcfgtR {
        AcfgtR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&self) -> AcfeR {
        AcfeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&self) -> AdtrgR {
        AdtrgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Conversion Active"]
    #[inline(always)]
    pub fn adact(&self) -> AdactR {
        AdactR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> RefselW<Sc2Spec> {
        RefselW::new(self, 0)
    }
    #[doc = "Bit 2 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DmaenW<Sc2Spec> {
        DmaenW::new(self, 2)
    }
    #[doc = "Bit 3 - Compare Function Range Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acren(&mut self) -> AcrenW<Sc2Spec> {
        AcrenW::new(self, 3)
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acfgt(&mut self) -> AcfgtW<Sc2Spec> {
        AcfgtW::new(self, 4)
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acfe(&mut self) -> AcfeW<Sc2Spec> {
        AcfeW::new(self, 5)
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn adtrg(&mut self) -> AdtrgW<Sc2Spec> {
        AdtrgW::new(self, 6)
    }
}
#[doc = "Status and Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sc2Spec;
impl crate::RegisterSpec for Sc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sc2::R`](R) reader structure"]
impl crate::Readable for Sc2Spec {}
#[doc = "`write(|w| ..)` method takes [`sc2::W`](W) writer structure"]
impl crate::Writable for Sc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SC2 to value 0"]
impl crate::Resettable for Sc2Spec {
    const RESET_VALUE: u32 = 0;
}
