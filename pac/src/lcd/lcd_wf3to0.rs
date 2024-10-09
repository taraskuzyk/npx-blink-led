#[doc = "Register `WF3TO0` reader"]
pub type R = crate::R<LcdWf3to0Spec>;
#[doc = "Register `WF3TO0` writer"]
pub type W = crate::W<LcdWf3to0Spec>;
#[doc = "Field `WF0` reader - Controls segments or phases connected to LCD_P0 as described above for WF3."]
pub type Wf0R = crate::FieldReader;
#[doc = "Field `WF0` writer - Controls segments or phases connected to LCD_P0 as described above for WF3."]
pub type Wf0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF1` reader - Controls segments or phases connected to LCD_P1 as described above for WF3."]
pub type Wf1R = crate::FieldReader;
#[doc = "Field `WF1` writer - Controls segments or phases connected to LCD_P1 as described above for WF3."]
pub type Wf1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF2` reader - Controls segments or phases connected to LCD_P2 as described above for WF3."]
pub type Wf2R = crate::FieldReader;
#[doc = "Field `WF2` writer - Controls segments or phases connected to LCD_P2 as described above for WF3."]
pub type Wf2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WF3` reader - Segment-on front plane operation - Each bit turns on or off the segments associated with LCD_P3 in the following pattern: HGFEDCBA (most significant bit controls segment H and least significant bit controls segment A)"]
pub type Wf3R = crate::FieldReader;
#[doc = "Field `WF3` writer - Segment-on front plane operation - Each bit turns on or off the segments associated with LCD_P3 in the following pattern: HGFEDCBA (most significant bit controls segment H and least significant bit controls segment A)"]
pub type Wf3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P0 as described above for WF3."]
    #[inline(always)]
    pub fn wf0(&self) -> Wf0R {
        Wf0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P1 as described above for WF3."]
    #[inline(always)]
    pub fn wf1(&self) -> Wf1R {
        Wf1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P2 as described above for WF3."]
    #[inline(always)]
    pub fn wf2(&self) -> Wf2R {
        Wf2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Segment-on front plane operation - Each bit turns on or off the segments associated with LCD_P3 in the following pattern: HGFEDCBA (most significant bit controls segment H and least significant bit controls segment A)"]
    #[inline(always)]
    pub fn wf3(&self) -> Wf3R {
        Wf3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls segments or phases connected to LCD_P0 as described above for WF3."]
    #[inline(always)]
    #[must_use]
    pub fn wf0(&mut self) -> Wf0W<LcdWf3to0Spec> {
        Wf0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Controls segments or phases connected to LCD_P1 as described above for WF3."]
    #[inline(always)]
    #[must_use]
    pub fn wf1(&mut self) -> Wf1W<LcdWf3to0Spec> {
        Wf1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Controls segments or phases connected to LCD_P2 as described above for WF3."]
    #[inline(always)]
    #[must_use]
    pub fn wf2(&mut self) -> Wf2W<LcdWf3to0Spec> {
        Wf2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Segment-on front plane operation - Each bit turns on or off the segments associated with LCD_P3 in the following pattern: HGFEDCBA (most significant bit controls segment H and least significant bit controls segment A)"]
    #[inline(always)]
    #[must_use]
    pub fn wf3(&mut self) -> Wf3W<LcdWf3to0Spec> {
        Wf3W::new(self, 24)
    }
}
#[doc = "LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf3to0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf3to0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdWf3to0Spec;
impl crate::RegisterSpec for LcdWf3to0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcd_wf3to0::R`](R) reader structure"]
impl crate::Readable for LcdWf3to0Spec {}
#[doc = "`write(|w| ..)` method takes [`lcd_wf3to0::W`](W) writer structure"]
impl crate::Writable for LcdWf3to0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WF3TO0 to value 0"]
impl crate::Resettable for LcdWf3to0Spec {
    const RESET_VALUE: u32 = 0;
}
