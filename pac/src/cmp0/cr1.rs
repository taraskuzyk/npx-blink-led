#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Comparator Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum En {
    #[doc = "0: Analog Comparator is disabled."]
    B0 = 0,
    #[doc = "1: Analog Comparator is enabled."]
    B1 = 1,
}
impl From<En> for bool {
    #[inline(always)]
    fn from(variant: En) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Comparator Module Enable"]
pub type EnR = crate::BitReader<En>;
impl EnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> En {
        match self.bits {
            false => En::B0,
            true => En::B1,
        }
    }
    #[doc = "Analog Comparator is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == En::B0
    }
    #[doc = "Analog Comparator is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == En::B1
    }
}
#[doc = "Field `EN` writer - Comparator Module Enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG, En>;
impl<'a, REG> EnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog Comparator is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(En::B0)
    }
    #[doc = "Analog Comparator is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(En::B1)
    }
}
#[doc = "Comparator Output Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ope {
    #[doc = "0: CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    B0 = 0,
    #[doc = "1: CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    B1 = 1,
}
impl From<Ope> for bool {
    #[inline(always)]
    fn from(variant: Ope) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OPE` reader - Comparator Output Pin Enable"]
pub type OpeR = crate::BitReader<Ope>;
impl OpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ope {
        match self.bits {
            false => Ope::B0,
            true => Ope::B1,
        }
    }
    #[doc = "CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ope::B0
    }
    #[doc = "CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ope::B1
    }
}
#[doc = "Field `OPE` writer - Comparator Output Pin Enable"]
pub type OpeW<'a, REG> = crate::BitWriter<'a, REG, Ope>;
impl<'a, REG> OpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ope::B0)
    }
    #[doc = "CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ope::B1)
    }
}
#[doc = "Comparator Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cos {
    #[doc = "0: Set the filtered comparator output (CMPO) to equal COUT."]
    B0 = 0,
    #[doc = "1: Set the unfiltered comparator output (CMPO) to equal COUTA."]
    B1 = 1,
}
impl From<Cos> for bool {
    #[inline(always)]
    fn from(variant: Cos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COS` reader - Comparator Output Select"]
pub type CosR = crate::BitReader<Cos>;
impl CosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cos {
        match self.bits {
            false => Cos::B0,
            true => Cos::B1,
        }
    }
    #[doc = "Set the filtered comparator output (CMPO) to equal COUT."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Cos::B0
    }
    #[doc = "Set the unfiltered comparator output (CMPO) to equal COUTA."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Cos::B1
    }
}
#[doc = "Field `COS` writer - Comparator Output Select"]
pub type CosW<'a, REG> = crate::BitWriter<'a, REG, Cos>;
impl<'a, REG> CosW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set the filtered comparator output (CMPO) to equal COUT."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Cos::B0)
    }
    #[doc = "Set the unfiltered comparator output (CMPO) to equal COUTA."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Cos::B1)
    }
}
#[doc = "Comparator INVERT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    #[doc = "0: Does not invert the comparator output."]
    B0 = 0,
    #[doc = "1: Inverts the comparator output."]
    B1 = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Comparator INVERT"]
pub type InvR = crate::BitReader<Inv>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv {
        match self.bits {
            false => Inv::B0,
            true => Inv::B1,
        }
    }
    #[doc = "Does not invert the comparator output."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Inv::B0
    }
    #[doc = "Inverts the comparator output."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Inv::B1
    }
}
#[doc = "Field `INV` writer - Comparator INVERT"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Does not invert the comparator output."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::B0)
    }
    #[doc = "Inverts the comparator output."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::B1)
    }
}
#[doc = "Power Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmode {
    #[doc = "0: Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    B0 = 0,
    #[doc = "1: High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    B1 = 1,
}
impl From<Pmode> for bool {
    #[inline(always)]
    fn from(variant: Pmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMODE` reader - Power Mode Select"]
pub type PmodeR = crate::BitReader<Pmode>;
impl PmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmode {
        match self.bits {
            false => Pmode::B0,
            true => Pmode::B1,
        }
    }
    #[doc = "Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Pmode::B0
    }
    #[doc = "High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Pmode::B1
    }
}
#[doc = "Field `PMODE` writer - Power Mode Select"]
pub type PmodeW<'a, REG> = crate::BitWriter<'a, REG, Pmode>;
impl<'a, REG> PmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Pmode::B0)
    }
    #[doc = "High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Pmode::B1)
    }
}
#[doc = "Trigger Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trigm {
    #[doc = "0: Trigger mode is disabled."]
    B0 = 0,
    #[doc = "1: Trigger mode is enabled."]
    B1 = 1,
}
impl From<Trigm> for bool {
    #[inline(always)]
    fn from(variant: Trigm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGM` reader - Trigger Mode Enable"]
pub type TrigmR = crate::BitReader<Trigm>;
impl TrigmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Trigm {
        match self.bits {
            false => Trigm::B0,
            true => Trigm::B1,
        }
    }
    #[doc = "Trigger mode is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Trigm::B0
    }
    #[doc = "Trigger mode is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Trigm::B1
    }
}
#[doc = "Field `TRIGM` writer - Trigger Mode Enable"]
pub type TrigmW<'a, REG> = crate::BitWriter<'a, REG, Trigm>;
impl<'a, REG> TrigmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger mode is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Trigm::B0)
    }
    #[doc = "Trigger mode is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Trigm::B1)
    }
}
#[doc = "Windowing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum We {
    #[doc = "0: Windowing mode is not selected."]
    B0 = 0,
    #[doc = "1: Windowing mode is selected."]
    B1 = 1,
}
impl From<We> for bool {
    #[inline(always)]
    fn from(variant: We) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WE` reader - Windowing Enable"]
pub type WeR = crate::BitReader<We>;
impl WeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> We {
        match self.bits {
            false => We::B0,
            true => We::B1,
        }
    }
    #[doc = "Windowing mode is not selected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == We::B0
    }
    #[doc = "Windowing mode is selected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == We::B1
    }
}
#[doc = "Field `WE` writer - Windowing Enable"]
pub type WeW<'a, REG> = crate::BitWriter<'a, REG, We>;
impl<'a, REG> WeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Windowing mode is not selected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(We::B0)
    }
    #[doc = "Windowing mode is selected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(We::B1)
    }
}
#[doc = "Sample Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Se {
    #[doc = "0: Sampling mode is not selected."]
    B0 = 0,
    #[doc = "1: Sampling mode is selected."]
    B1 = 1,
}
impl From<Se> for bool {
    #[inline(always)]
    fn from(variant: Se) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SE` reader - Sample Enable"]
pub type SeR = crate::BitReader<Se>;
impl SeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Se {
        match self.bits {
            false => Se::B0,
            true => Se::B1,
        }
    }
    #[doc = "Sampling mode is not selected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Se::B0
    }
    #[doc = "Sampling mode is selected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Se::B1
    }
}
#[doc = "Field `SE` writer - Sample Enable"]
pub type SeW<'a, REG> = crate::BitWriter<'a, REG, Se>;
impl<'a, REG> SeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sampling mode is not selected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Se::B0)
    }
    #[doc = "Sampling mode is selected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Se::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator Module Enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator Output Pin Enable"]
    #[inline(always)]
    pub fn ope(&self) -> OpeR {
        OpeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator Output Select"]
    #[inline(always)]
    pub fn cos(&self) -> CosR {
        CosR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator INVERT"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Mode Select"]
    #[inline(always)]
    pub fn pmode(&self) -> PmodeR {
        PmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Mode Enable"]
    #[inline(always)]
    pub fn trigm(&self) -> TrigmR {
        TrigmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Windowing Enable"]
    #[inline(always)]
    pub fn we(&self) -> WeR {
        WeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sample Enable"]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<Cr1Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ope(&mut self) -> OpeW<Cr1Spec> {
        OpeW::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn cos(&mut self) -> CosW<Cr1Spec> {
        CosW::new(self, 2)
    }
    #[doc = "Bit 3 - Comparator INVERT"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> InvW<Cr1Spec> {
        InvW::new(self, 3)
    }
    #[doc = "Bit 4 - Power Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PmodeW<Cr1Spec> {
        PmodeW::new(self, 4)
    }
    #[doc = "Bit 5 - Trigger Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trigm(&mut self) -> TrigmW<Cr1Spec> {
        TrigmW::new(self, 5)
    }
    #[doc = "Bit 6 - Windowing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn we(&mut self) -> WeW<Cr1Spec> {
        WeW::new(self, 6)
    }
    #[doc = "Bit 7 - Sample Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SeW<Cr1Spec> {
        SeW::new(self, 7)
    }
}
#[doc = "CMP Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {
    const RESET_VALUE: u8 = 0;
}
