#[doc = "Register `WF43TO40` reader"]
pub type R = crate::R<LcdWf43to40Spec>;
#[doc = "Register `WF43TO40` writer"]
pub type W = crate::W<LcdWf43to40Spec>;
#[doc = "Field `WF40` reader - Controls segments or phases connected to LCD_P40 as described above for WF3TO0\\[WF3\\]."]
pub type Wf40R = crate::FieldReader;
#[doc = "Field `WF40` writer - Controls segments or phases connected to LCD_P40 as described above for WF3TO0\\[WF3\\]."]
pub type Wf40W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF41` reader - Controls segments or phases connected to LCD_P41 as described above for WF3TO0\\[WF3\\]."]
pub type Wf41R = crate::FieldReader;
#[doc = "Field `WF41` writer - Controls segments or phases connected to LCD_P41 as described above for WF3TO0\\[WF3\\]."]
pub type Wf41W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF42` reader - Controls segments or phases connected to LCD_P42 as described above for WF3TO0\\[WF3\\]."]
pub type Wf42R = crate::FieldReader;
#[doc = "Field `WF42` writer - Controls segments or phases connected to LCD_P42 as described above for WF3TO0\\[WF3\\]."]
pub type Wf42W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF43` reader - Controls segments or phases connected to LCD_P43 as described above for WF3TO0\\[WF3\\]."]
pub type Wf43R = crate::FieldReader;
#[doc = "Field `WF43` writer - Controls segments or phases connected to LCD_P43 as described above for WF3TO0\\[WF3\\]."]
pub type Wf43W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P40 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf40(&self) -> Wf40R {
        Wf40R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P41 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf41(&self) -> Wf41R {
        Wf41R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P42 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf42(&self) -> Wf42R {
        Wf42R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P43 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf43(&self) -> Wf43R {
        Wf43R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P40 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf40(&mut self) -> Wf40W<LcdWf43to40Spec> {
        Wf40W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P41 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf41(&mut self) -> Wf41W<LcdWf43to40Spec> {
        Wf41W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P42 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf42(&mut self) -> Wf42W<LcdWf43to40Spec> {
        Wf42W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P43 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf43(&mut self) -> Wf43W<LcdWf43to40Spec> {
        Wf43W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf43to40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf43to40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf43to40Spec;
impl crate::RegisterSpec for LcdWf43to40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf43to40::R`](R) reader structure"]
impl crate::Readable for LcdWf43to40Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf43to40::W`](W) writer structure"]
impl crate::Writable for LcdWf43to40Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF43TO40 to value 0"]
impl crate::Resettable for LcdWf43to40Spec {
    const RESET_VALUE: u32 = 0;
}
