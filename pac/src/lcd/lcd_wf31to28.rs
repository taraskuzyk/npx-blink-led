#[doc = "Register `WF31TO28` reader"]
pub type R = crate::R<LcdWf31to28Spec>;
#[doc = "Register `WF31TO28` writer"]
pub type W = crate::W<LcdWf31to28Spec>;
#[doc = "Field `WF28` reader - Controls segments or phases connected to LCD_P28 as described above for WF3TO0\\[WF3\\]."]
pub type Wf28R = crate::FieldReader;
#[doc = "Field `WF28` writer - Controls segments or phases connected to LCD_P28 as described above for WF3TO0\\[WF3\\]."]
pub type Wf28W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF29` reader - Controls segments or phases connected to LCD_P29 as described above for WF3TO0\\[WF3\\]."]
pub type Wf29R = crate::FieldReader;
#[doc = "Field `WF29` writer - Controls segments or phases connected to LCD_P29 as described above for WF3TO0\\[WF3\\]."]
pub type Wf29W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF30` reader - Controls segments or phases connected to LCD_P30 as described above for WF3TO0\\[WF3\\]."]
pub type Wf30R = crate::FieldReader;
#[doc = "Field `WF30` writer - Controls segments or phases connected to LCD_P30 as described above for WF3TO0\\[WF3\\]."]
pub type Wf30W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF31` reader - Controls segments or phases connected to LCD_P31 as described above for WF3TO0\\[WF3\\]."]
pub type Wf31R = crate::FieldReader;
#[doc = "Field `WF31` writer - Controls segments or phases connected to LCD_P31 as described above for WF3TO0\\[WF3\\]."]
pub type Wf31W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P28 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf28(&self) -> Wf28R {
        Wf28R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P29 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf29(&self) -> Wf29R {
        Wf29R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P30 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf30(&self) -> Wf30R {
        Wf30R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P31 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    pub fn wf31(&self) -> Wf31R {
        Wf31R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P28 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf28(&mut self) -> Wf28W<LcdWf31to28Spec> {
        Wf28W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P29 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf29(&mut self) -> Wf29W<LcdWf31to28Spec> {
        Wf29W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P30 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf30(&mut self) -> Wf30W<LcdWf31to28Spec> {
        Wf30W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Controls segments or phases connected to LCD_P31 as described above for WF3TO0\\[WF3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn wf31(&mut self) -> Wf31W<LcdWf31to28Spec> {
        Wf31W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf31to28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf31to28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf31to28Spec;
impl crate::RegisterSpec for LcdWf31to28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf31to28::R`](R) reader structure"]
impl crate::Readable for LcdWf31to28Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf31to28::W`](W) writer structure"]
impl crate::Writable for LcdWf31to28Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF31TO28 to value 0"]
impl crate::Resettable for LcdWf31to28Spec {
    const RESET_VALUE: u32 = 0;
}
