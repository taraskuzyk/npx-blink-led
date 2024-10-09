#[doc = "Register `WF63TO60` reader"]
pub type R = crate::R<LcdWf63to60Spec>;
#[doc = "Register `WF63TO60` writer"]
pub type W = crate::W<LcdWf63to60Spec>;
#[doc = "Field `WF60` reader - Controls segments or phases connected to LCD_P60 as described above for WF3TO0\\[WF3\\]."]
pub type Wf60R = crate::FieldReader;
#[doc = "Field `WF60` writer - Controls segments or phases connected to LCD_P60 as described above for WF3TO0\\[WF3\\]."]
pub type Wf60W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF61` reader - Controls segments or phases connected to LCD_P61 as described above for WF3TO0\\[WF3\\]."]
pub type Wf61R = crate::FieldReader;
#[doc = "Field `WF61` writer - Controls segments or phases connected to LCD_P61 as described above for WF3TO0\\[WF3\\]."]
pub type Wf61W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF62` reader - Controls segments or phases connected to LCD_P62 as described above for WF3TO0\\[WF3\\]."]
pub type Wf62R = crate::FieldReader;
#[doc = "Field `WF62` writer - Controls segments or phases connected to LCD_P62 as described above for WF3TO0\\[WF3\\]."]
pub type Wf62W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF63` reader - Controls segments or phases connected to LCD_P63 as described above for WF3TO0\\[WF3\\]."]
pub type Wf63R = crate::FieldReader;
#[doc = "Field `WF63` writer - Controls segments or phases connected to LCD_P63 as described above for WF3TO0\\[WF3\\]."]
pub type Wf63W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P60 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf60(&self) -> Wf60R {
        Wf60R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P61 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf61(&self) -> Wf61R {
        Wf61R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P62 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf62(&self) -> Wf62R {
        Wf62R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P63 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf63(&self) -> Wf63R {
        Wf63R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P60 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf60(&mut self) -> Wf60W<LcdWf63to60Spec> {
        Wf60W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P61 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf61(&mut self) -> Wf61W<LcdWf63to60Spec> {
        Wf61W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P62 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf62(&mut self) -> Wf62W<LcdWf63to60Spec> {
        Wf62W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P63 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf63(&mut self) -> Wf63W<LcdWf63to60Spec> {
        Wf63W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf63to60::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf63to60::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf63to60Spec;
impl crate::RegisterSpec for LcdWf63to60Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf63to60::R`](R) reader structure"]
impl crate::Readable for LcdWf63to60Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf63to60::W`](W) writer structure"]
impl crate::Writable for LcdWf63to60Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF63TO60 to value 0"]
impl crate::Resettable for LcdWf63to60Spec {
    const RESET_VALUE: u32 = 0;
}
