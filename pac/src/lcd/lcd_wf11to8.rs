#[doc = "Register `WF11TO8` reader"]
pub type R = crate::R<LcdWf11to8Spec>;
#[doc = "Register `WF11TO8` writer"]
pub type W = crate::W<LcdWf11to8Spec>;
#[doc = "Field `WF8` reader - Controls segments or phases connected to LCD_P8 as described above for WF3TO0\\[WF3\\]."]
pub type Wf8R = crate::FieldReader;
#[doc = "Field `WF8` writer - Controls segments or phases connected to LCD_P8 as described above for WF3TO0\\[WF3\\]."]
pub type Wf8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF9` reader - Controls segments or phases connected to LCD_P9 as described above for WF3TO0\\[WF3\\]."]
pub type Wf9R = crate::FieldReader;
#[doc = "Field `WF9` writer - Controls segments or phases connected to LCD_P9 as described above for WF3TO0\\[WF3\\]."]
pub type Wf9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF10` reader - Controls segments or phases connected to LCD_P10 as described above for WF3TO0\\[WF3\\]."]
pub type Wf10R = crate::FieldReader;
#[doc = "Field `WF10` writer - Controls segments or phases connected to LCD_P10 as described above for WF3TO0\\[WF3\\]."]
pub type Wf10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF11` reader - Controls segments or phases connected to LCD_P11 as described above for WF3TO0\\[WF3\\]."]
pub type Wf11R = crate::FieldReader;
#[doc = "Field `WF11` writer - Controls segments or phases connected to LCD_P11 as described above for WF3TO0\\[WF3\\]."]
pub type Wf11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P8 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf8(&self) -> Wf8R {
        Wf8R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P9 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf9(&self) -> Wf9R {
        Wf9R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P10 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf10(&self) -> Wf10R {
        Wf10R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P11 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf11(&self) -> Wf11R {
        Wf11R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P8 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf8(&mut self) -> Wf8W<LcdWf11to8Spec> {
        Wf8W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P9 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf9(&mut self) -> Wf9W<LcdWf11to8Spec> {
        Wf9W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P10 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf10(&mut self) -> Wf10W<LcdWf11to8Spec> {
        Wf10W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P11 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf11(&mut self) -> Wf11W<LcdWf11to8Spec> {
        Wf11W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf11to8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf11to8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf11to8Spec;
impl crate::RegisterSpec for LcdWf11to8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf11to8::R`](R) reader structure"]
impl crate::Readable for LcdWf11to8Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf11to8::W`](W) writer structure"]
impl crate::Writable for LcdWf11to8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF11TO8 to value 0"]
impl crate::Resettable for LcdWf11to8Spec {
    const RESET_VALUE: u32 = 0;
}
