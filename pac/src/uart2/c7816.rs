#[doc = "Register `C7816` reader"]
pub type R = crate::R<C7816Spec>;
#[doc = "Register `C7816` writer"]
pub type W = crate::W<C7816Spec>;
#[doc = "ISO-7816 Functionality Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso7816e {
    #[doc = "0: ISO-7816 functionality is turned off/not enabled."]
    B0 = 0,
    #[doc = "1: ISO-7816 functionality is turned on/enabled."]
    B1 = 1,
}
impl From<Iso7816e> for bool {
    #[inline(always)]
    fn from(variant: Iso7816e) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISO_7816E` reader - ISO-7816 Functionality Enabled"]
pub type Iso7816eR = crate::BitReader<Iso7816e>;
impl Iso7816eR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iso7816e {
        match self.bits {
            false => Iso7816e::B0,
            true => Iso7816e::B1,
        }
    }
    #[doc = "ISO-7816 functionality is turned off/not enabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Iso7816e::B0
    }
    #[doc = "ISO-7816 functionality is turned on/enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Iso7816e::B1
    }
}
#[doc = "Field `ISO_7816E` writer - ISO-7816 Functionality Enabled"]
pub type Iso7816eW<'a, REG> = crate::BitWriter<'a, REG, Iso7816e>;
impl<'a, REG> Iso7816eW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ISO-7816 functionality is turned off/not enabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Iso7816e::B0)
    }
    #[doc = "ISO-7816 functionality is turned on/enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Iso7816e::B1)
    }
}
#[doc = "Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ttype {
    #[doc = "0: T = 0 per the ISO-7816 specification."]
    B0 = 0,
    #[doc = "1: T = 1 per the ISO-7816 specification."]
    B1 = 1,
}
impl From<Ttype> for bool {
    #[inline(always)]
    fn from(variant: Ttype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TTYPE` reader - Transfer Type"]
pub type TtypeR = crate::BitReader<Ttype>;
impl TtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ttype {
        match self.bits {
            false => Ttype::B0,
            true => Ttype::B1,
        }
    }
    #[doc = "T = 0 per the ISO-7816 specification."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ttype::B0
    }
    #[doc = "T = 1 per the ISO-7816 specification."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ttype::B1
    }
}
#[doc = "Field `TTYPE` writer - Transfer Type"]
pub type TtypeW<'a, REG> = crate::BitWriter<'a, REG, Ttype>;
impl<'a, REG> TtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "T = 0 per the ISO-7816 specification."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ttype::B0)
    }
    #[doc = "T = 1 per the ISO-7816 specification."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ttype::B1)
    }
}
#[doc = "Detect Initial Character\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Init {
    #[doc = "0: Normal operating mode. Receiver does not seek to identify initial character."]
    B0 = 0,
    #[doc = "1: Receiver searches for initial character."]
    B1 = 1,
}
impl From<Init> for bool {
    #[inline(always)]
    fn from(variant: Init) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INIT` reader - Detect Initial Character"]
pub type InitR = crate::BitReader<Init>;
impl InitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Init {
        match self.bits {
            false => Init::B0,
            true => Init::B1,
        }
    }
    #[doc = "Normal operating mode. Receiver does not seek to identify initial character."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Init::B0
    }
    #[doc = "Receiver searches for initial character."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Init::B1
    }
}
#[doc = "Field `INIT` writer - Detect Initial Character"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG, Init>;
impl<'a, REG> InitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal operating mode. Receiver does not seek to identify initial character."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Init::B0)
    }
    #[doc = "Receiver searches for initial character."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Init::B1)
    }
}
#[doc = "Generate NACK on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Anack {
    #[doc = "0: No NACK is automatically generated."]
    B0 = 0,
    #[doc = "1: A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected."]
    B1 = 1,
}
impl From<Anack> for bool {
    #[inline(always)]
    fn from(variant: Anack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ANACK` reader - Generate NACK on Error"]
pub type AnackR = crate::BitReader<Anack>;
impl AnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Anack {
        match self.bits {
            false => Anack::B0,
            true => Anack::B1,
        }
    }
    #[doc = "No NACK is automatically generated."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Anack::B0
    }
    #[doc = "A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Anack::B1
    }
}
#[doc = "Field `ANACK` writer - Generate NACK on Error"]
pub type AnackW<'a, REG> = crate::BitWriter<'a, REG, Anack>;
impl<'a, REG> AnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No NACK is automatically generated."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Anack::B0)
    }
    #[doc = "A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Anack::B1)
    }
}
#[doc = "Generate NACK on Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Onack {
    #[doc = "0: The received data does not generate a NACK when the receipt of the data results in an overflow event."]
    B0 = 0,
    #[doc = "1: If the receiver buffer overflows, a NACK is automatically sent on a received character."]
    B1 = 1,
}
impl From<Onack> for bool {
    #[inline(always)]
    fn from(variant: Onack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONACK` reader - Generate NACK on Overflow"]
pub type OnackR = crate::BitReader<Onack>;
impl OnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Onack {
        match self.bits {
            false => Onack::B0,
            true => Onack::B1,
        }
    }
    #[doc = "The received data does not generate a NACK when the receipt of the data results in an overflow event."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Onack::B0
    }
    #[doc = "If the receiver buffer overflows, a NACK is automatically sent on a received character."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Onack::B1
    }
}
#[doc = "Field `ONACK` writer - Generate NACK on Overflow"]
pub type OnackW<'a, REG> = crate::BitWriter<'a, REG, Onack>;
impl<'a, REG> OnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The received data does not generate a NACK when the receipt of the data results in an overflow event."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Onack::B0)
    }
    #[doc = "If the receiver buffer overflows, a NACK is automatically sent on a received character."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Onack::B1)
    }
}
impl R {
    #[doc = "Bit 0 - ISO-7816 Functionality Enabled"]
    #[inline(always)]
    pub fn iso_7816e(&self) -> Iso7816eR {
        Iso7816eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Type"]
    #[inline(always)]
    pub fn ttype(&self) -> TtypeR {
        TtypeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Detect Initial Character"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generate NACK on Error"]
    #[inline(always)]
    pub fn anack(&self) -> AnackR {
        AnackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generate NACK on Overflow"]
    #[inline(always)]
    pub fn onack(&self) -> OnackR {
        OnackR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ISO-7816 Functionality Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn iso_7816e(&mut self) -> Iso7816eW<C7816Spec> {
        Iso7816eW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn ttype(&mut self) -> TtypeW<C7816Spec> {
        TtypeW::new(self, 1)
    }
    #[doc = "Bit 2 - Detect Initial Character"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> InitW<C7816Spec> {
        InitW::new(self, 2)
    }
    #[doc = "Bit 3 - Generate NACK on Error"]
    #[inline(always)]
    #[must_use]
    pub fn anack(&mut self) -> AnackW<C7816Spec> {
        AnackW::new(self, 3)
    }
    #[doc = "Bit 4 - Generate NACK on Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn onack(&mut self) -> OnackW<C7816Spec> {
        OnackW::new(self, 4)
    }
}
#[doc = "UART 7816 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c7816::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7816::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C7816Spec;
impl crate::RegisterSpec for C7816Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c7816::R`](R) reader structure"]
impl crate::Readable for C7816Spec {}
#[doc = "`write(|w| ..)` method takes [`c7816::W`](W) writer structure"]
impl crate::Writable for C7816Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C7816 to value 0"]
impl crate::Resettable for C7816Spec {
    const RESET_VALUE: u8 = 0;
}
