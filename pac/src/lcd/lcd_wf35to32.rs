#[doc = "Register `WF35TO32` reader"]
pub type R = crate::R<LcdWf35to32Spec>;
#[doc = "Register `WF35TO32` writer"]
pub type W = crate::W<LcdWf35to32Spec>;
#[doc = "Field `WF32` reader - Controls segments or phases connected to LCD_P32 as described above for WF3TO0\\[WF3\\]."]
pub type Wf32R = crate::FieldReader;
#[doc = "Field `WF32` writer - Controls segments or phases connected to LCD_P32 as described above for WF3TO0\\[WF3\\]."]
pub type Wf32W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF33` reader - Controls segments or phases connected to LCD_P33 as described above for WF3TO0\\[WF3\\]."]
pub type Wf33R = crate::FieldReader;
#[doc = "Field `WF33` writer - Controls segments or phases connected to LCD_P33 as described above for WF3TO0\\[WF3\\]."]
pub type Wf33W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF34` reader - Controls segments or phases connected to LCD_P34 as described above for WF3TO0\\[WF3\\]."]
pub type Wf34R = crate::FieldReader;
#[doc = "Field `WF34` writer - Controls segments or phases connected to LCD_P34 as described above for WF3TO0\\[WF3\\]."]
pub type Wf34W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF35` reader - Controls segments or phases connected to LCD_P35 as described above for WF3TO0\\[WF3\\]."]
pub type Wf35R = crate::FieldReader;
#[doc = "Field `WF35` writer - Controls segments or phases connected to LCD_P35 as described above for WF3TO0\\[WF3\\]."]
pub type Wf35W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P32 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf32(&self) -> Wf32R {
        Wf32R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P33 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf33(&self) -> Wf33R {
        Wf33R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P34 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf34(&self) -> Wf34R {
        Wf34R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P35 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf35(&self) -> Wf35R {
        Wf35R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P32 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf32(&mut self) -> Wf32W<LcdWf35to32Spec> {
        Wf32W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P33 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf33(&mut self) -> Wf33W<LcdWf35to32Spec> {
        Wf33W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P34 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf34(&mut self) -> Wf34W<LcdWf35to32Spec> {
        Wf34W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P35 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf35(&mut self) -> Wf35W<LcdWf35to32Spec> {
        Wf35W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf35to32::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf35to32::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf35to32Spec;
impl crate::RegisterSpec for LcdWf35to32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf35to32::R`](R) reader structure"]
impl crate::Readable for LcdWf35to32Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf35to32::W`](W) writer structure"]
impl crate::Writable for LcdWf35to32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF35TO32 to value 0"]
impl crate::Resettable for LcdWf35to32Spec {
    const RESET_VALUE: u32 = 0;
}
