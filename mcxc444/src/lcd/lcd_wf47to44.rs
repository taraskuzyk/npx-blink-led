# [doc = "Register `WF47TO44` reader"] pub type R = crate :: R < LcdWf47to44Spec > ; # [doc = "Register `WF47TO44` writer"] pub type W = crate :: W < LcdWf47to44Spec > ; # [doc = "Field `WF44` reader - Controls segments or phases connected to LCD_P44 as described above for WF3TO0\\[WF3\\]."] pub type Wf44R = crate :: FieldReader ; # [doc = "Field `WF44` writer - Controls segments or phases connected to LCD_P44 as described above for WF3TO0\\[WF3\\]."] pub type Wf44W < 'a , REG > = crate :: FieldWriter < 'a , REG , 8 > ; # [doc = "Field `WF45` reader - Controls segments or phases connected to LCD_P45 as described above for WF3TO0\\[WF3\\]."] pub type Wf45R = crate :: FieldReader ; # [doc = "Field `WF45` writer - Controls segments or phases connected to LCD_P45 as described above for WF3TO0\\[WF3\\]."] pub type Wf45W < 'a , REG > = crate :: FieldWriter < 'a , REG , 8 > ; # [doc = "Field `WF46` reader - Controls segments or phases connected to LCD_P46 as described above for WF3TO0\\[WF3\\]."] pub type Wf46R = crate :: FieldReader ; # [doc = "Field `WF46` writer - Controls segments or phases connected to LCD_P46 as described above for WF3TO0\\[WF3\\]."] pub type Wf46W < 'a , REG > = crate :: FieldWriter < 'a , REG , 8 > ; # [doc = "Field `WF47` reader - Controls segments or phases connected to LCD_P47 as described above for WF3TO0\\[WF3\\]."] pub type Wf47R = crate :: FieldReader ; # [doc = "Field `WF47` writer - Controls segments or phases connected to LCD_P47 as described above for WF3TO0\\[WF3\\]."] pub type Wf47W < 'a , REG > = crate :: FieldWriter < 'a , REG , 8 > ; impl R { # [doc = "Bits 0:7 - Controls segments or phases connected to LCD_P44 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] pub fn wf44 (& self) -> Wf44R { Wf44R :: new ((self . bits & 0xff) as u8) } # [doc = "Bits 8:15 - Controls segments or phases connected to LCD_P45 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] pub fn wf45 (& self) -> Wf45R { Wf45R :: new (((self . bits >> 8) & 0xff) as u8) } # [doc = "Bits 16:23 - Controls segments or phases connected to LCD_P46 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] pub fn wf46 (& self) -> Wf46R { Wf46R :: new (((self . bits >> 16) & 0xff) as u8) } # [doc = "Bits 24:31 - Controls segments or phases connected to LCD_P47 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] pub fn wf47 (& self) -> Wf47R { Wf47R :: new (((self . bits >> 24) & 0xff) as u8) } } impl W { # [doc = "Bits 0:7 - Controls segments or phases connected to LCD_P44 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] # [must_use] pub fn wf44 (& mut self) -> Wf44W < LcdWf47to44Spec > { Wf44W :: new (self , 0) } # [doc = "Bits 8:15 - Controls segments or phases connected to LCD_P45 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] # [must_use] pub fn wf45 (& mut self) -> Wf45W < LcdWf47to44Spec > { Wf45W :: new (self , 8) } # [doc = "Bits 16:23 - Controls segments or phases connected to LCD_P46 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] # [must_use] pub fn wf46 (& mut self) -> Wf46W < LcdWf47to44Spec > { Wf46W :: new (self , 16) } # [doc = "Bits 24:31 - Controls segments or phases connected to LCD_P47 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] # [must_use] pub fn wf47 (& mut self) -> Wf47W < LcdWf47to44Spec > { Wf47W :: new (self , 24) } } # [doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf47to44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf47to44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct LcdWf47to44Spec ; impl crate :: RegisterSpec for LcdWf47to44Spec { type Ux = u32 ; } # [doc = "`read()` method returns [`lcd_wf47to44::R`](R) reader structure"] impl crate :: Readable for LcdWf47to44Spec { } # [doc = "`write(|w| ..)` method takes [`lcd_wf47to44::W`](W) writer structure"] impl crate :: Writable for LcdWf47to44Spec { type Safety = crate :: Unsafe ; const ZERO_TO_MODIFY_FIELDS_BITMAP : u32 = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : u32 = 0 ; } # [doc = "`reset()` method sets WF47TO44 to value 0"] impl crate :: Resettable for LcdWf47to44Spec { const RESET_VALUE : u32 = 0 ; }