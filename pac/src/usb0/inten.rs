#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "USBRST Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usbrsten {
    #[doc = "0: Disables the USBRST interrupt."]
    B0 = 0,
    #[doc = "1: Enables the USBRST interrupt."]
    B1 = 1,
}
impl From<Usbrsten> for bool {
    #[inline(always)]
    fn from(variant: Usbrsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `USBRSTEN` reader - USBRST Interrupt Enable"]
pub type UsbrstenR = crate::BitReader<Usbrsten>;
impl UsbrstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usbrsten {
        match self.bits {
            false => Usbrsten::B0,
            true => Usbrsten::B1,
        }
    }
    #[doc = "Disables the USBRST interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Usbrsten::B0
    }
    #[doc = "Enables the USBRST interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Usbrsten::B1
    }
}
#[doc = "Field `USBRSTEN` writer - USBRST Interrupt Enable"]
pub type UsbrstenW<'a, REG> = crate::BitWriter<'a, REG, Usbrsten>;
impl<'a, REG> UsbrstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the USBRST interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrsten::B0)
    }
    #[doc = "Enables the USBRST interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Usbrsten::B1)
    }
}
#[doc = "ERROR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Erroren {
    #[doc = "0: Disables the ERROR interrupt."]
    B0 = 0,
    #[doc = "1: Enables the ERROR interrupt."]
    B1 = 1,
}
impl From<Erroren> for bool {
    #[inline(always)]
    fn from(variant: Erroren) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROREN` reader - ERROR Interrupt Enable"]
pub type ErrorenR = crate::BitReader<Erroren>;
impl ErrorenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Erroren {
        match self.bits {
            false => Erroren::B0,
            true => Erroren::B1,
        }
    }
    #[doc = "Disables the ERROR interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Erroren::B0
    }
    #[doc = "Enables the ERROR interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Erroren::B1
    }
}
#[doc = "Field `ERROREN` writer - ERROR Interrupt Enable"]
pub type ErrorenW<'a, REG> = crate::BitWriter<'a, REG, Erroren>;
impl<'a, REG> ErrorenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the ERROR interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Erroren::B0)
    }
    #[doc = "Enables the ERROR interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Erroren::B1)
    }
}
#[doc = "SOFTOK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Softoken {
    #[doc = "0: Disbles the SOFTOK interrupt."]
    B0 = 0,
    #[doc = "1: Enables the SOFTOK interrupt."]
    B1 = 1,
}
impl From<Softoken> for bool {
    #[inline(always)]
    fn from(variant: Softoken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTOKEN` reader - SOFTOK Interrupt Enable"]
pub type SoftokenR = crate::BitReader<Softoken>;
impl SoftokenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Softoken {
        match self.bits {
            false => Softoken::B0,
            true => Softoken::B1,
        }
    }
    #[doc = "Disbles the SOFTOK interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Softoken::B0
    }
    #[doc = "Enables the SOFTOK interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Softoken::B1
    }
}
#[doc = "Field `SOFTOKEN` writer - SOFTOK Interrupt Enable"]
pub type SoftokenW<'a, REG> = crate::BitWriter<'a, REG, Softoken>;
impl<'a, REG> SoftokenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disbles the SOFTOK interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Softoken::B0)
    }
    #[doc = "Enables the SOFTOK interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Softoken::B1)
    }
}
#[doc = "TOKDNE Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tokdneen {
    #[doc = "0: Disables the TOKDNE interrupt."]
    B0 = 0,
    #[doc = "1: Enables the TOKDNE interrupt."]
    B1 = 1,
}
impl From<Tokdneen> for bool {
    #[inline(always)]
    fn from(variant: Tokdneen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOKDNEEN` reader - TOKDNE Interrupt Enable"]
pub type TokdneenR = crate::BitReader<Tokdneen>;
impl TokdneenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tokdneen {
        match self.bits {
            false => Tokdneen::B0,
            true => Tokdneen::B1,
        }
    }
    #[doc = "Disables the TOKDNE interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tokdneen::B0
    }
    #[doc = "Enables the TOKDNE interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tokdneen::B1
    }
}
#[doc = "Field `TOKDNEEN` writer - TOKDNE Interrupt Enable"]
pub type TokdneenW<'a, REG> = crate::BitWriter<'a, REG, Tokdneen>;
impl<'a, REG> TokdneenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the TOKDNE interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tokdneen::B0)
    }
    #[doc = "Enables the TOKDNE interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tokdneen::B1)
    }
}
#[doc = "SLEEP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sleepen {
    #[doc = "0: Disables the SLEEP interrupt."]
    B0 = 0,
    #[doc = "1: Enables the SLEEP interrupt."]
    B1 = 1,
}
impl From<Sleepen> for bool {
    #[inline(always)]
    fn from(variant: Sleepen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEPEN` reader - SLEEP Interrupt Enable"]
pub type SleepenR = crate::BitReader<Sleepen>;
impl SleepenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sleepen {
        match self.bits {
            false => Sleepen::B0,
            true => Sleepen::B1,
        }
    }
    #[doc = "Disables the SLEEP interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Sleepen::B0
    }
    #[doc = "Enables the SLEEP interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Sleepen::B1
    }
}
#[doc = "Field `SLEEPEN` writer - SLEEP Interrupt Enable"]
pub type SleepenW<'a, REG> = crate::BitWriter<'a, REG, Sleepen>;
impl<'a, REG> SleepenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the SLEEP interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepen::B0)
    }
    #[doc = "Enables the SLEEP interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Sleepen::B1)
    }
}
#[doc = "RESUME Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Resumeen {
    #[doc = "0: Disables the RESUME interrupt."]
    B0 = 0,
    #[doc = "1: Enables the RESUME interrupt."]
    B1 = 1,
}
impl From<Resumeen> for bool {
    #[inline(always)]
    fn from(variant: Resumeen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESUMEEN` reader - RESUME Interrupt Enable"]
pub type ResumeenR = crate::BitReader<Resumeen>;
impl ResumeenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resumeen {
        match self.bits {
            false => Resumeen::B0,
            true => Resumeen::B1,
        }
    }
    #[doc = "Disables the RESUME interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Resumeen::B0
    }
    #[doc = "Enables the RESUME interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Resumeen::B1
    }
}
#[doc = "Field `RESUMEEN` writer - RESUME Interrupt Enable"]
pub type ResumeenW<'a, REG> = crate::BitWriter<'a, REG, Resumeen>;
impl<'a, REG> ResumeenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disables the RESUME interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Resumeen::B0)
    }
    #[doc = "Enables the RESUME interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Resumeen::B1)
    }
}
#[doc = "STALL Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stallen {
    #[doc = "0: Diasbles the STALL interrupt."]
    B0 = 0,
    #[doc = "1: Enables the STALL interrupt."]
    B1 = 1,
}
impl From<Stallen> for bool {
    #[inline(always)]
    fn from(variant: Stallen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STALLEN` reader - STALL Interrupt Enable"]
pub type StallenR = crate::BitReader<Stallen>;
impl StallenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stallen {
        match self.bits {
            false => Stallen::B0,
            true => Stallen::B1,
        }
    }
    #[doc = "Diasbles the STALL interrupt."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Stallen::B0
    }
    #[doc = "Enables the STALL interrupt."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Stallen::B1
    }
}
#[doc = "Field `STALLEN` writer - STALL Interrupt Enable"]
pub type StallenW<'a, REG> = crate::BitWriter<'a, REG, Stallen>;
impl<'a, REG> StallenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Diasbles the STALL interrupt."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Stallen::B0)
    }
    #[doc = "Enables the STALL interrupt."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Stallen::B1)
    }
}
impl R {
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline(always)]
    pub fn usbrsten(&self) -> UsbrstenR {
        UsbrstenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline(always)]
    pub fn erroren(&self) -> ErrorenR {
        ErrorenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline(always)]
    pub fn softoken(&self) -> SoftokenR {
        SoftokenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline(always)]
    pub fn tokdneen(&self) -> TokdneenR {
        TokdneenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline(always)]
    pub fn sleepen(&self) -> SleepenR {
        SleepenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline(always)]
    pub fn resumeen(&self) -> ResumeenR {
        ResumeenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline(always)]
    pub fn stallen(&self) -> StallenR {
        StallenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbrsten(&mut self) -> UsbrstenW<IntenSpec> {
        UsbrstenW::new(self, 0)
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erroren(&mut self) -> ErrorenW<IntenSpec> {
        ErrorenW::new(self, 1)
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn softoken(&mut self) -> SoftokenW<IntenSpec> {
        SoftokenW::new(self, 2)
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tokdneen(&mut self) -> TokdneenW<IntenSpec> {
        TokdneenW::new(self, 3)
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sleepen(&mut self) -> SleepenW<IntenSpec> {
        SleepenW::new(self, 4)
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resumeen(&mut self) -> ResumeenW<IntenSpec> {
        ResumeenW::new(self, 5)
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallen(&mut self) -> StallenW<IntenSpec> {
        StallenW::new(self, 7)
    }
}
#[doc = "Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u8 = 0;
}
