#[doc = "Register `WF39TO36` reader"]
pub type R = crate::R<LcdWf39to36Spec>;
#[doc = "Register `WF39TO36` writer"]
pub type W = crate::W<LcdWf39to36Spec>;
#[doc = "Field `WF36` reader - Controls segments or phases connected to LCD_P36 as described above for WF3TO0\\[WF3\\]."]
pub type Wf36R = crate::FieldReader;
#[doc = "Field `WF36` writer - Controls segments or phases connected to LCD_P36 as described above for WF3TO0\\[WF3\\]."]
pub type Wf36W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF37` reader - Controls segments or phases connected to LCD_P37 as described above for WF3TO0\\[WF3\\]."]
pub type Wf37R = crate::FieldReader;
#[doc = "Field `WF37` writer - Controls segments or phases connected to LCD_P37 as described above for WF3TO0\\[WF3\\]."]
pub type Wf37W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF38` reader - Controls segments or phases connected to LCD_P38 as described above for WF3TO0\\[WF3\\]."]
pub type Wf38R = crate::FieldReader;
#[doc = "Field `WF38` writer - Controls segments or phases connected to LCD_P38 as described above for WF3TO0\\[WF3\\]."]
pub type Wf38W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF39` reader - Controls segments or phases connected to LCD_P39 as described above for WF3TO0\\[WF3\\]."]
pub type Wf39R = crate::FieldReader;
#[doc = "Field `WF39` writer - Controls segments or phases connected to LCD_P39 as described above for WF3TO0\\[WF3\\]."]
pub type Wf39W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P36 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf36(&self) -> Wf36R {
        Wf36R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P37 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf37(&self) -> Wf37R {
        Wf37R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P38 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf38(&self) -> Wf38R {
        Wf38R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P39 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf39(&self) -> Wf39R {
        Wf39R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P36 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf36(&mut self) -> Wf36W<LcdWf39to36Spec> {
        Wf36W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P37 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf37(&mut self) -> Wf37W<LcdWf39to36Spec> {
        Wf37W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P38 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf38(&mut self) -> Wf38W<LcdWf39to36Spec> {
        Wf38W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P39 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf39(&mut self) -> Wf39W<LcdWf39to36Spec> {
        Wf39W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf39to36::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf39to36::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf39to36Spec;
impl crate::RegisterSpec for LcdWf39to36Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf39to36::R`](R) reader structure"]
impl crate::Readable for LcdWf39to36Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf39to36::W`](W) writer structure"]
impl crate::Writable for LcdWf39to36Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF39TO36 to value 0"]
impl crate::Resettable for LcdWf39to36Spec {
    const RESET_VALUE: u32 = 0;
}
