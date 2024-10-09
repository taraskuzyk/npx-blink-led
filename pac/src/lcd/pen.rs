#[doc = "Register `PEN%s` reader"]
pub type R = crate::R<PenSpec>;
#[doc = "Register `PEN%s` writer"]
pub type W = crate::W<PenSpec>;
#[doc = "LCD Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Pen {
    #[doc = "0: LCD operation disabled on LCD_Pn."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: LCD operation enabled on LCD_Pn."]
    B00000000000000000000000000000001 = 1,
}
impl From<Pen> for u32 {
    #[inline(always)]
    fn from(variant: Pen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pen {
    type Ux = u32;
}
impl crate::IsEnum for Pen {}
#[doc = "Field `PEN` reader - LCD Pin Enable"]
pub type PenR = crate::FieldReader<Pen>;
impl PenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pen> {
        match self.bits {
            0 => Some(Pen::B00000000000000000000000000000000),
            1 => Some(Pen::B00000000000000000000000000000001),
            _ => None,
        }
    }
    #[doc = "LCD operation disabled on LCD_Pn."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Pen::B00000000000000000000000000000000
    }
    #[doc = "LCD operation enabled on LCD_Pn."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Pen::B00000000000000000000000000000001
    }
}
#[doc = "Field `PEN` writer - LCD Pin Enable"]
pub type PenW<'a, REG> = crate::FieldWriter<'a, REG, 32, Pen>;
impl<'a, REG> PenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "LCD operation disabled on LCD_Pn."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::B00000000000000000000000000000000)
    }
    #[doc = "LCD operation enabled on LCD_Pn."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Pen::B00000000000000000000000000000001)
    }
}
impl R {
    #[doc = "Bits 0:31 - LCD Pin Enable"]
    #[inline(always)]
    pub fn pen(&self) -> PenR {
        PenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LCD Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pen(&mut self) -> PenW<PenSpec> {
        PenW::new(self, 0)
    }
}
#[doc = "LCD Pin Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`pen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PenSpec;
impl crate::RegisterSpec for PenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pen::R`](R) reader structure"]
impl crate::Readable for PenSpec {}
#[doc = "`write(|w| ..)` method takes [`pen::W`](W) writer structure"]
impl crate::Writable for PenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PEN%s to value 0"]
impl crate::Resettable for PenSpec {
    const RESET_VALUE: u32 = 0;
}
