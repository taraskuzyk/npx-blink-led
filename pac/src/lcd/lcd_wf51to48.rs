#[doc = "Register `WF51TO48` reader"]
pub type R = crate::R<LcdWf51to48Spec>;
#[doc = "Register `WF51TO48` writer"]
pub type W = crate::W<LcdWf51to48Spec>;
#[doc = "Field `WF48` reader - Controls segments or phases connected to LCD_P48 as described above for WF3TO0\\[WF3\\]."]
pub type Wf48R = crate::FieldReader;
#[doc = "Field `WF48` writer - Controls segments or phases connected to LCD_P48 as described above for WF3TO0\\[WF3\\]."]
pub type Wf48W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF49` reader - Controls segments or phases connected to LCD_P49 as described above for WF3TO0\\[WF3\\]."]
pub type Wf49R = crate::FieldReader;
#[doc = "Field `WF49` writer - Controls segments or phases connected to LCD_P49 as described above for WF3TO0\\[WF3\\]."]
pub type Wf49W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF50` reader - Controls segments or phases connected to LCD_P50 as described above for WF3TO0\\[WF3\\]."]
pub type Wf50R = crate::FieldReader;
#[doc = "Field `WF50` writer - Controls segments or phases connected to LCD_P50 as described above for WF3TO0\\[WF3\\]."]
pub type Wf50W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF51` reader - Controls segments or phases connected to LCD_P51 as described above for WF3TO0\\[WF3\\]."]
pub type Wf51R = crate::FieldReader;
#[doc = "Field `WF51` writer - Controls segments or phases connected to LCD_P51 as described above for WF3TO0\\[WF3\\]."]
pub type Wf51W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P48 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf48(&self) -> Wf48R {
        Wf48R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P49 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf49(&self) -> Wf49R {
        Wf49R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P50 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf50(&self) -> Wf50R {
        Wf50R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P51 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf51(&self) -> Wf51R {
        Wf51R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P48 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf48(&mut self) -> Wf48W<LcdWf51to48Spec> {
        Wf48W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P49 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf49(&mut self) -> Wf49W<LcdWf51to48Spec> {
        Wf49W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P50 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf50(&mut self) -> Wf50W<LcdWf51to48Spec> {
        Wf50W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P51 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf51(&mut self) -> Wf51W<LcdWf51to48Spec> {
        Wf51W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf51to48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf51to48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf51to48Spec;
impl crate::RegisterSpec for LcdWf51to48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf51to48::R`](R) reader structure"]
impl crate::Readable for LcdWf51to48Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf51to48::W`](W) writer structure"]
impl crate::Writable for LcdWf51to48Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF51TO48 to value 0"]
impl crate::Resettable for LcdWf51to48Spec {
    const RESET_VALUE: u32 = 0;
}
