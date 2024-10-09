#[doc = "Register `BPEN%s` reader"]
pub type R = crate::R<BpenSpec>;
#[doc = "Register `BPEN%s` writer"]
pub type W = crate::W<BpenSpec>;
#[doc = "Back Plane Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Bpen {
    #[doc = "0: Front plane operation enabled on LCD_Pn."]
    B00000000000000000000000000000000 = 0,
    #[doc = "1: Back plane operation enabled on LCD_Pn."]
    B00000000000000000000000000000001 = 1,
}
impl From<Bpen> for u32 {
    #[inline(always)]
    fn from(variant: Bpen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bpen {
    type Ux = u32;
}
impl crate::IsEnum for Bpen {}
#[doc = "Field `BPEN` reader - Back Plane Enable"]
pub type BpenR = crate::FieldReader<Bpen>;
impl BpenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bpen> {
        match self.bits {
            0 => Some(Bpen::B00000000000000000000000000000000),
            1 => Some(Bpen::B00000000000000000000000000000001),
            _ => None,
        }
    }
    #[doc = "Front plane operation enabled on LCD_Pn."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000000(&self) -> bool {
        *self == Bpen::B00000000000000000000000000000000
    }
    #[doc = "Back plane operation enabled on LCD_Pn."]
    #[inline(always)]
    pub fn is_b00000000000000000000000000000001(&self) -> bool {
        *self == Bpen::B00000000000000000000000000000001
    }
}
#[doc = "Field `BPEN` writer - Back Plane Enable"]
pub type BpenW<'a, REG> = crate::FieldWriter<'a, REG, 32, Bpen>;
impl<'a, REG> BpenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Front plane operation enabled on LCD_Pn."]
    #[inline(always)]
    pub fn b00000000000000000000000000000000(self) -> &'a mut crate::W<REG> {
        self.variant(Bpen::B00000000000000000000000000000000)
    }
    #[doc = "Back plane operation enabled on LCD_Pn."]
    #[inline(always)]
    pub fn b00000000000000000000000000000001(self) -> &'a mut crate::W<REG> {
        self.variant(Bpen::B00000000000000000000000000000001)
    }
}
impl R {
    #[doc = "Bits 0:31 - Back Plane Enable"]
    #[inline(always)]
    pub fn bpen(&self) -> BpenR {
        BpenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Back Plane Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bpen(&mut self) -> BpenW<BpenSpec> {
        BpenW::new(self, 0)
    }
}
#[doc = "LCD Back Plane Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`bpen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BpenSpec;
impl crate::RegisterSpec for BpenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bpen::R`](R) reader structure"]
impl crate::Readable for BpenSpec {}
#[doc = "`write(|w| ..)` method takes [`bpen::W`](W) writer structure"]
impl crate::Writable for BpenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BPEN%s to value 0"]
impl crate::Resettable for BpenSpec {
    const RESET_VALUE: u32 = 0;
}
