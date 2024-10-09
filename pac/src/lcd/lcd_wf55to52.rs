#[doc = "Register `WF55TO52` reader"]
pub type R = crate::R<LcdWf55to52Spec>;
#[doc = "Register `WF55TO52` writer"]
pub type W = crate::W<LcdWf55to52Spec>;
#[doc = "Field `WF52` reader - Controls segments or phases connected to LCD_P52 as described above for WF3TO0\\[WF3\\]."]
pub type Wf52R = crate::FieldReader;
#[doc = "Field `WF52` writer - Controls segments or phases connected to LCD_P52 as described above for WF3TO0\\[WF3\\]."]
pub type Wf52W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF53` reader - Controls segments or phases connected to LCD_P53 as described above for WF3TO0\\[WF3\\]."]
pub type Wf53R = crate::FieldReader;
#[doc = "Field `WF53` writer - Controls segments or phases connected to LCD_P53 as described above for WF3TO0\\[WF3\\]."]
pub type Wf53W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF54` reader - Controls segments or phases connected to LCD_P54 as described above for WF3TO0\\[WF3\\]."]
pub type Wf54R = crate::FieldReader;
#[doc = "Field `WF54` writer - Controls segments or phases connected to LCD_P54 as described above for WF3TO0\\[WF3\\]."]
pub type Wf54W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF55` reader - Controls segments or phases connected to LCD_P55 as described above for WF3TO0\\[WF3\\]."]
pub type Wf55R = crate::FieldReader;
#[doc = "Field `WF55` writer - Controls segments or phases connected to LCD_P55 as described above for WF3TO0\\[WF3\\]."]
pub type Wf55W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P52 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf52(&self) -> Wf52R {
        Wf52R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P53 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf53(&self) -> Wf53R {
        Wf53R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P54 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf54(&self) -> Wf54R {
        Wf54R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P55 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf55(&self) -> Wf55R {
        Wf55R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P52 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf52(&mut self) -> Wf52W<LcdWf55to52Spec> {
        Wf52W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P53 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf53(&mut self) -> Wf53W<LcdWf55to52Spec> {
        Wf53W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P54 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf54(&mut self) -> Wf54W<LcdWf55to52Spec> {
        Wf54W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P55 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf55(&mut self) -> Wf55W<LcdWf55to52Spec> {
        Wf55W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf55to52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf55to52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf55to52Spec;
impl crate::RegisterSpec for LcdWf55to52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf55to52::R`](R) reader structure"]
impl crate::Readable for LcdWf55to52Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf55to52::W`](W) writer structure"]
impl crate::Writable for LcdWf55to52Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF55TO52 to value 0"]
impl crate::Resettable for LcdWf55to52Spec {
    const RESET_VALUE: u32 = 0;
}
