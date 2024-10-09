#[doc = "Register `WF23TO20` reader"]
pub type R = crate::R<LcdWf23to20Spec>;
#[doc = "Register `WF23TO20` writer"]
pub type W = crate::W<LcdWf23to20Spec>;
#[doc = "Field `WF20` reader - Controls segments or phases connected to LCD_P20 as described above for WF3TO0\\[WF3\\]."]
pub type Wf20R = crate::FieldReader;
#[doc = "Field `WF20` writer - Controls segments or phases connected to LCD_P20 as described above for WF3TO0\\[WF3\\]."]
pub type Wf20W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF21` reader - Controls segments or phases connected to LCD_P21 as described above for WF3TO0\\[WF3\\]."]
pub type Wf21R = crate::FieldReader;
#[doc = "Field `WF21` writer - Controls segments or phases connected to LCD_P21 as described above for WF3TO0\\[WF3\\]."]
pub type Wf21W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF22` reader - Controls segments or phases connected to LCD_P22 as described above for WF3TO0\\[WF3\\]."]
pub type Wf22R = crate::FieldReader;
#[doc = "Field `WF22` writer - Controls segments or phases connected to LCD_P22 as described above for WF3TO0\\[WF3\\]."]
pub type Wf22W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF23` reader - Controls segments or phases connected to LCD_P23 as described above for WF3TO0\\[WF3\\]."]
pub type Wf23R = crate::FieldReader;
#[doc = "Field `WF23` writer - Controls segments or phases connected to LCD_P23 as described above for WF3TO0\\[WF3\\]."]
pub type Wf23W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P20 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf20(&self) -> Wf20R {
        Wf20R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P21 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf21(&self) -> Wf21R {
        Wf21R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P22 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf22(&self) -> Wf22R {
        Wf22R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P23 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf23(&self) -> Wf23R {
        Wf23R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P20 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf20(&mut self) -> Wf20W<LcdWf23to20Spec> {
        Wf20W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P21 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf21(&mut self) -> Wf21W<LcdWf23to20Spec> {
        Wf21W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P22 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf22(&mut self) -> Wf22W<LcdWf23to20Spec> {
        Wf22W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P23 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf23(&mut self) -> Wf23W<LcdWf23to20Spec> {
        Wf23W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf23to20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf23to20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf23to20Spec;
impl crate::RegisterSpec for LcdWf23to20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf23to20::R`](R) reader structure"]
impl crate::Readable for LcdWf23to20Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf23to20::W`](W) writer structure"]
impl crate::Writable for LcdWf23to20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF23TO20 to value 0"]
impl crate::Resettable for LcdWf23to20Spec {
    const RESET_VALUE: u32 = 0;
}
