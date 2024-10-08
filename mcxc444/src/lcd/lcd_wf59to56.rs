# [doc = "Register `WF59TO56` reader"] pub type R = crate :: R < LcdWf59to56Spec > ; # [doc = "Register `WF59TO56` writer"] pub type W = crate :: W < LcdWf59to56Spec > ; # [doc = "Field `WF56` reader - Controls segments or phases connected to LCD_P56 as described above for WF3TO0\\[WF3\\]."] pub type Wf56R = crate :: FieldReader ; # [doc = "Field `WF56` writer - Controls segments or phases connected to LCD_P56 as described above for WF3TO0\\[WF3\\]."] pub type Wf56W < 'a , REG > = crate :: FieldWriter < 'a , REG , 8 > ; # [doc = "Field `WF57` reader - Controls segments or phases connected to LCD_P57 as described above for WF3TO0\\[WF3\\]."] pub type Wf57R = crate :: FieldReader ; # [doc = "Field `WF57` writer - Controls segments or phases connected to LCD_P57 as described above for WF3TO0\\[WF3\\]."] pub type Wf57W < 'a , REG > = crate :: FieldWriter < 'a , REG , 8 > ; # [doc = "Field `WF58` reader - Controls segments or phases connected to LCD_P58 as described above for WF3TO0\\[WF3\\]."] pub type Wf58R = crate :: FieldReader ; # [doc = "Field `WF58` writer - Controls segments or phases connected to LCD_P58 as described above for WF3TO0\\[WF3\\]."] pub type Wf58W < 'a , REG > = crate :: FieldWriter < 'a , REG , 8 > ; # [doc = "Field `WF59` reader - Controls segments or phases connected to LCD_P59 as described above for WF3TO0\\[WF3\\]."] pub type Wf59R = crate :: FieldReader ; # [doc = "Field `WF59` writer - Controls segments or phases connected to LCD_P59 as described above for WF3TO0\\[WF3\\]."] pub type Wf59W < 'a , REG > = crate :: FieldWriter < 'a , REG , 8 > ; impl R { # [doc = "Bits 0:7 - Controls segments or phases connected to LCD_P56 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] pub fn wf56 (& self) -> Wf56R { Wf56R :: new ((self . bits & 0xff) as u8) } # [doc = "Bits 8:15 - Controls segments or phases connected to LCD_P57 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] pub fn wf57 (& self) -> Wf57R { Wf57R :: new (((self . bits >> 8) & 0xff) as u8) } # [doc = "Bits 16:23 - Controls segments or phases connected to LCD_P58 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] pub fn wf58 (& self) -> Wf58R { Wf58R :: new (((self . bits >> 16) & 0xff) as u8) } # [doc = "Bits 24:31 - Controls segments or phases connected to LCD_P59 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] pub fn wf59 (& self) -> Wf59R { Wf59R :: new (((self . bits >> 24) & 0xff) as u8) } } impl W { # [doc = "Bits 0:7 - Controls segments or phases connected to LCD_P56 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] # [must_use] pub fn wf56 (& mut self) -> Wf56W < LcdWf59to56Spec > { Wf56W :: new (self , 0) } # [doc = "Bits 8:15 - Controls segments or phases connected to LCD_P57 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] # [must_use] pub fn wf57 (& mut self) -> Wf57W < LcdWf59to56Spec > { Wf57W :: new (self , 8) } # [doc = "Bits 16:23 - Controls segments or phases connected to LCD_P58 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] # [must_use] pub fn wf58 (& mut self) -> Wf58W < LcdWf59to56Spec > { Wf58W :: new (self , 16) } # [doc = "Bits 24:31 - Controls segments or phases connected to LCD_P59 as described above for WF3TO0\\[WF3\\]."] # [inline (always)] # [must_use] pub fn wf59 (& mut self) -> Wf59W < LcdWf59to56Spec > { Wf59W :: new (self , 24) } } # [doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf59to56::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf59to56::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct LcdWf59to56Spec ; impl crate :: RegisterSpec for LcdWf59to56Spec { type Ux = u32 ; } # [doc = "`read()` method returns [`lcd_wf59to56::R`](R) reader structure"] impl crate :: Readable for LcdWf59to56Spec { } # [doc = "`write(|w| ..)` method takes [`lcd_wf59to56::W`](W) writer structure"] impl crate :: Writable for LcdWf59to56Spec { type Safety = crate :: Unsafe ; const ZERO_TO_MODIFY_FIELDS_BITMAP : u32 = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : u32 = 0 ; } # [doc = "`reset()` method sets WF59TO56 to value 0"] impl crate :: Resettable for LcdWf59to56Spec { const RESET_VALUE : u32 = 0 ; }