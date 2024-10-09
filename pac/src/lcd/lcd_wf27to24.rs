#[doc = "Register `WF27TO24` reader"]
pub type R = crate::R<LcdWf27to24Spec>;
#[doc = "Register `WF27TO24` writer"]
pub type W = crate::W<LcdWf27to24Spec>;
#[doc = "Field `WF24` reader - Controls segments or phases connected to LCD_P24 as described above for WF3TO0\\[WF3\\]."]
pub type Wf24R = crate::FieldReader;
#[doc = "Field `WF24` writer - Controls segments or phases connected to LCD_P24 as described above for WF3TO0\\[WF3\\]."]
pub type Wf24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF25` reader - Controls segments or phases connected to LCD_P25 as described above for WF3TO0\\[WF3\\]."]
pub type Wf25R = crate::FieldReader;
#[doc = "Field `WF25` writer - Controls segments or phases connected to LCD_P25 as described above for WF3TO0\\[WF3\\]."]
pub type Wf25W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF26` reader - Controls segments or phases connected to LCD_P26 as described above for WF3TO0\\[WF3\\]."]
pub type Wf26R = crate::FieldReader;
#[doc = "Field `WF26` writer - Controls segments or phases connected to LCD_P26 as described above for WF3TO0\\[WF3\\]."]
pub type Wf26W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF27` reader - Controls segments or phases connected to LCD_P27 as described above for WF3TO0\\[WF3\\]."]
pub type Wf27R = crate::FieldReader;
#[doc = "Field `WF27` writer - Controls segments or phases connected to LCD_P27 as described above for WF3TO0\\[WF3\\]."]
pub type Wf27W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P24 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf24(&self) -> Wf24R {
        Wf24R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P25 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf25(&self) -> Wf25R {
        Wf25R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P26 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf26(&self) -> Wf26R {
        Wf26R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P27 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf27(&self) -> Wf27R {
        Wf27R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P24 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf24(&mut self) -> Wf24W<LcdWf27to24Spec> {
        Wf24W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P25 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf25(&mut self) -> Wf25W<LcdWf27to24Spec> {
        Wf25W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P26 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf26(&mut self) -> Wf26W<LcdWf27to24Spec> {
        Wf26W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P27 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf27(&mut self) -> Wf27W<LcdWf27to24Spec> {
        Wf27W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf27to24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf27to24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf27to24Spec;
impl crate::RegisterSpec for LcdWf27to24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf27to24::R`](R) reader structure"]
impl crate::Readable for LcdWf27to24Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf27to24::W`](W) writer structure"]
impl crate::Writable for LcdWf27to24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF27TO24 to value 0"]
impl crate::Resettable for LcdWf27to24Spec {
    const RESET_VALUE: u32 = 0;
}
