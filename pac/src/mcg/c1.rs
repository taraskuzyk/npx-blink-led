#[doc = "Register `C1` reader"]
pub type R = crate::R<C1Spec>;
#[doc = "Register `C1` writer"]
pub type W = crate::W<C1Spec>;
#[doc = "Internal Reference Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irefsten {
    #[doc = "0: LIRC is disabled in Stop mode."]
    B0 = 0,
    #[doc = "1: LIRC is enabled in Stop mode, if IRCLKEN is set."]
    B1 = 1,
}
impl From<Irefsten> for bool {
    #[inline(always)]
    fn from(variant: Irefsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREFSTEN` reader - Internal Reference Stop Enable"]
pub type IrefstenR = crate::BitReader<Irefsten>;
impl IrefstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irefsten {
        match self.bits {
            false => Irefsten::B0,
            true => Irefsten::B1,
        }
    }
    #[doc = "LIRC is disabled in Stop mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Irefsten::B0
    }
    #[doc = "LIRC is enabled in Stop mode, if IRCLKEN is set."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Irefsten::B1
    }
}
#[doc = "Field `IREFSTEN` writer - Internal Reference Stop Enable"]
pub type IrefstenW<'a, REG> = crate::BitWriter<'a, REG, Irefsten>;
impl<'a, REG> IrefstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIRC is disabled in Stop mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Irefsten::B0)
    }
    #[doc = "LIRC is enabled in Stop mode, if IRCLKEN is set."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Irefsten::B1)
    }
}
#[doc = "Internal Reference Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irclken {
    #[doc = "0: LIRC is disabled."]
    B0 = 0,
    #[doc = "1: LIRC is enabled."]
    B1 = 1,
}
impl From<Irclken> for bool {
    #[inline(always)]
    fn from(variant: Irclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRCLKEN` reader - Internal Reference Clock Enable"]
pub type IrclkenR = crate::BitReader<Irclken>;
impl IrclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irclken {
        match self.bits {
            false => Irclken::B0,
            true => Irclken::B1,
        }
    }
    #[doc = "LIRC is disabled."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Irclken::B0
    }
    #[doc = "LIRC is enabled."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Irclken::B1
    }
}
#[doc = "Field `IRCLKEN` writer - Internal Reference Clock Enable"]
pub type IrclkenW<'a, REG> = crate::BitWriter<'a, REG, Irclken>;
impl<'a, REG> IrclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LIRC is disabled."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Irclken::B0)
    }
    #[doc = "LIRC is enabled."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Irclken::B1)
    }
}
#[doc = "Clock Source Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clks {
    #[doc = "0: Selects HIRC clock as the main clock source. This is HIRC mode."]
    B00 = 0,
    #[doc = "1: Selects LIRC clock as the main clock source. This is LIRC2M or LIRC8M mode."]
    B01 = 1,
    #[doc = "2: Selects external clock as the main clock source. This is EXT mode."]
    B10 = 2,
    #[doc = "3: Reserved. Writing 11 takes no effect."]
    B11 = 3,
}
impl From<Clks> for u8 {
    #[inline(always)]
    fn from(variant: Clks) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clks {
    type Ux = u8;
}
impl crate::IsEnum for Clks {}
#[doc = "Field `CLKS` reader - Clock Source Select"]
pub type ClksR = crate::FieldReader<Clks>;
impl ClksR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clks {
        match self.bits {
            0 => Clks::B00,
            1 => Clks::B01,
            2 => Clks::B10,
            3 => Clks::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Selects HIRC clock as the main clock source. This is HIRC mode."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Clks::B00
    }
    #[doc = "Selects LIRC clock as the main clock source. This is LIRC2M or LIRC8M mode."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Clks::B01
    }
    #[doc = "Selects external clock as the main clock source. This is EXT mode."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Clks::B10
    }
    #[doc = "Reserved. Writing 11 takes no effect."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Clks::B11
    }
}
#[doc = "Field `CLKS` writer - Clock Source Select"]
pub type ClksW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clks, crate::Safe>;
impl<'a, REG> ClksW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Selects HIRC clock as the main clock source. This is HIRC mode."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::B00)
    }
    #[doc = "Selects LIRC clock as the main clock source. This is LIRC2M or LIRC8M mode."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::B01)
    }
    #[doc = "Selects external clock as the main clock source. This is EXT mode."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::B10)
    }
    #[doc = "Reserved. Writing 11 takes no effect."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Clks::B11)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    pub fn irefsten(&self) -> IrefstenR {
        IrefstenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    pub fn irclken(&self) -> IrclkenR {
        IrclkenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    pub fn clks(&self) -> ClksR {
        ClksR::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irefsten(&mut self) -> IrefstenW<C1Spec> {
        IrefstenW::new(self, 0)
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irclken(&mut self) -> IrclkenW<C1Spec> {
        IrclkenW::new(self, 1)
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clks(&mut self) -> ClksW<C1Spec> {
        ClksW::new(self, 6)
    }
}
#[doc = "MCG Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Spec;
impl crate::RegisterSpec for C1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`c1::R`](R) reader structure"]
impl crate::Readable for C1Spec {}
#[doc = "`write(|w| ..)` method takes [`c1::W`](W) writer structure"]
impl crate::Writable for C1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets C1 to value 0x40"]
impl crate::Resettable for C1Spec {
    const RESET_VALUE: u8 = 0x40;
}
