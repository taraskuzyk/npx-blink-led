#[doc = "Register `PDOR` reader"]
pub type R = crate::R<PdorSpec>;
#[doc = "Register `PDOR` writer"]
pub type W = crate::W<PdorSpec>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pdo {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    B00000000000000000000000000000001 = 1,
}
impl From<Pdo> for u32 {
    #[inline(always)]
    fn from(variant: Pdo) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pdo {
    type Ux = u32;
}
impl crate::IsEnum for Pdo {}
#[doc = "Field `PDO` reader - Port Data Output"]
pub type PdoR = crate::FieldReader<Pdo>;
impl PdoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pdo> {
        match self.bits {
            0 => Some(Pdo::B00000000000000000000000000000000),
            1 => Some(Pdo::B00000000000000000000000000000001),
            _ => None,
        }
    }
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Pdo::B00000000000000000000000000000000
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Pdo::B00000000000000000000000000000001
    }
}
#[doc = "Field `PDO` writer - Port Data Output"]
pub type PdoW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pdo>;
impl<'a, REG> PdoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Pdo::B00000000000000000000000000000000)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Pdo::B00000000000000000000000000000001)
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo(&self) -> PdoR {
        PdoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo(&mut self) -> PdoW<PdorSpec> {
        PdoW::new(self, 0)
    }
}
#[doc = "Port Data Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdorSpec;
impl crate::RegisterSpec for PdorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdor::R`](R) reader structure"]
impl crate::Readable for PdorSpec {}
#[doc = "`write(|w| ..)` method takes [`pdor::W`](W) writer structure"]
impl crate::Writable for PdorSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDOR to value 0"]
impl crate::Resettable for PdorSpec {
    const RESET_VALUE: u32 = 0;
}
