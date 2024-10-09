#[doc = "Register `WF19TO16` reader"]
pub type R = crate::R<LcdWf19to16Spec>;
#[doc = "Register `WF19TO16` writer"]
pub type W = crate::W<LcdWf19to16Spec>;
#[doc = "Field `WF16` reader - Controls segments or phases connected to LCD_P16 as described above for WF3TO0\\[WF3\\]."]
pub type Wf16R = crate::FieldReader;
#[doc = "Field `WF16` writer - Controls segments or phases connected to LCD_P16 as described above for WF3TO0\\[WF3\\]."]
pub type Wf16W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF17` reader - Controls segments or phases connected to LCD_P17 as described above for WF3TO0\\[WF3\\]."]
pub type Wf17R = crate::FieldReader;
#[doc = "Field `WF17` writer - Controls segments or phases connected to LCD_P17 as described above for WF3TO0\\[WF3\\]."]
pub type Wf17W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF18` reader - Controls segments or phases connected to LCD_P18 as described above for WF3TO0\\[WF3\\]."]
pub type Wf18R = crate::FieldReader;
#[doc = "Field `WF18` writer - Controls segments or phases connected to LCD_P18 as described above for WF3TO0\\[WF3\\]."]
pub type Wf18W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF19` reader - Controls segments or phases connected to LCD_P19 as described above for WF3TO0\\[WF3\\]."]
pub type Wf19R = crate::FieldReader;
#[doc = "Field `WF19` writer - Controls segments or phases connected to LCD_P19 as described above for WF3TO0\\[WF3\\]."]
pub type Wf19W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P16 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf16(&self) -> Wf16R {
        Wf16R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P17 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf17(&self) -> Wf17R {
        Wf17R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P18 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf18(&self) -> Wf18R {
        Wf18R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P19 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf19(&self) -> Wf19R {
        Wf19R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P16 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf16(&mut self) -> Wf16W<LcdWf19to16Spec> {
        Wf16W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P17 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf17(&mut self) -> Wf17W<LcdWf19to16Spec> {
        Wf17W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P18 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf18(&mut self) -> Wf18W<LcdWf19to16Spec> {
        Wf18W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P19 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf19(&mut self) -> Wf19W<LcdWf19to16Spec> {
        Wf19W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf19to16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf19to16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf19to16Spec;
impl crate::RegisterSpec for LcdWf19to16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf19to16::R`](R) reader structure"]
impl crate::Readable for LcdWf19to16Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf19to16::W`](W) writer structure"]
impl crate::Writable for LcdWf19to16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF19TO16 to value 0"]
impl crate::Resettable for LcdWf19to16Spec {
    const RESET_VALUE: u32 = 0;
}
