#[doc = "Register `WF7TO4` reader"]
pub type R = crate::R<LcdWf7to4Spec>;
#[doc = "Register `WF7TO4` writer"]
pub type W = crate::W<LcdWf7to4Spec>;
#[doc = "Field `WF4` reader - Controls segments or phases connected to LCD_P4 as described above for WF3TO0\\[WF3\\]."]
pub type Wf4R = crate::FieldReader;
#[doc = "Field `WF4` writer - Controls segments or phases connected to LCD_P4 as described above for WF3TO0\\[WF3\\]."]
pub type Wf4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF5` reader - Controls segments or phases connected to LCD_P5 as described above for WF3TO0\\[WF3\\]."]
pub type Wf5R = crate::FieldReader;
#[doc = "Field `WF5` writer - Controls segments or phases connected to LCD_P5 as described above for WF3TO0\\[WF3\\]."]
pub type Wf5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF6` reader - Controls segments or phases connected to LCD_P6 as described above for WF3TO0\\[WF3\\]."]
pub type Wf6R = crate::FieldReader;
#[doc = "Field `WF6` writer - Controls segments or phases connected to LCD_P6 as described above for WF3TO0\\[WF3\\]."]
pub type Wf6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF7` reader - Controls segments or phases connected to LCD_P7 as described above for WF3TO0\\[WF3\\]."]
pub type Wf7R = crate::FieldReader;
#[doc = "Field `WF7` writer - Controls segments or phases connected to LCD_P7 as described above for WF3TO0\\[WF3\\]."]
pub type Wf7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P4 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf4(&self) -> Wf4R {
        Wf4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P5 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf5(&self) -> Wf5R {
        Wf5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P6 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf6(&self) -> Wf6R {
        Wf6R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P7 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf7(&self) -> Wf7R {
        Wf7R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P4 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf4(&mut self) -> Wf4W<LcdWf7to4Spec> {
        Wf4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P5 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf5(&mut self) -> Wf5W<LcdWf7to4Spec> {
        Wf5W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P6 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf6(&mut self) -> Wf6W<LcdWf7to4Spec> {
        Wf6W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P7 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf7(&mut self) -> Wf7W<LcdWf7to4Spec> {
        Wf7W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf7to4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf7to4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf7to4Spec;
impl crate::RegisterSpec for LcdWf7to4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf7to4::R`](R) reader structure"]
impl crate::Readable for LcdWf7to4Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf7to4::W`](W) writer structure"]
impl crate::Writable for LcdWf7to4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF7TO4 to value 0"]
impl crate::Resettable for LcdWf7to4Spec {
    const RESET_VALUE: u32 = 0;
}
