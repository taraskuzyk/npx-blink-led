#[doc = "Register `WF15TO12` reader"]
pub type R = crate::R<LcdWf15to12Spec>;
#[doc = "Register `WF15TO12` writer"]
pub type W = crate::W<LcdWf15to12Spec>;
#[doc = "Field `WF12` reader - Controls segments or phases connected to LCD_P12 as described above for WF3TO0\\[WF3\\]."]
pub type Wf12R = crate::FieldReader;
#[doc = "Field `WF12` writer - Controls segments or phases connected to LCD_P12 as described above for WF3TO0\\[WF3\\]."]
pub type Wf12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF13` reader - Controls segments or phases connected to LCD_P13 as described above for WF3TO0\\[WF3\\]."]
pub type Wf13R = crate::FieldReader;
#[doc = "Field `WF13` writer - Controls segments or phases connected to LCD_P13 as described above for WF3TO0\\[WF3\\]."]
pub type Wf13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF14` reader - Controls segments or phases connected to LCD_P14 as described above for WF3TO0\\[WF3\\]."]
pub type Wf14R = crate::FieldReader;
#[doc = "Field `WF14` writer - Controls segments or phases connected to LCD_P14 as described above for WF3TO0\\[WF3\\]."]
pub type Wf14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF15` reader - Controls segments or phases connected to LCD_P15 as described above for WF3TO0\\[WF3\\]."]
pub type Wf15R = crate::FieldReader;
#[doc = "Field `WF15` writer - Controls segments or phases connected to LCD_P15 as described above for WF3TO0\\[WF3\\]."]
pub type Wf15W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P12 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf12(&self) -> Wf12R {
        Wf12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P13 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf13(&self) -> Wf13R {
        Wf13R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P14 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf14(&self) -> Wf14R {
        Wf14R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P15 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf15(&self) -> Wf15R {
        Wf15R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P12 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf12(&mut self) -> Wf12W<LcdWf15to12Spec> {
        Wf12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P13 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf13(&mut self) -> Wf13W<LcdWf15to12Spec> {
        Wf13W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P14 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf14(&mut self) -> Wf14W<LcdWf15to12Spec> {
        Wf14W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P15 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf15(&mut self) -> Wf15W<LcdWf15to12Spec> {
        Wf15W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf15to12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf15to12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf15to12Spec;
impl crate::RegisterSpec for LcdWf15to12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf15to12::R`](R) reader structure"]
impl crate::Readable for LcdWf15to12Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf15to12::W`](W) writer structure"]
impl crate::Writable for LcdWf15to12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF15TO12 to value 0"]
impl crate::Resettable for LcdWf15to12Spec {
    const RESET_VALUE: u32 = 0;
}
