# [doc = "Register `CLK_RECOVER_INT_STATUS` reader"] pub type R = crate :: R < ClkRecoverIntStatusSpec > ; # [doc = "Register `CLK_RECOVER_INT_STATUS` writer"] pub type W = crate :: W < ClkRecoverIntStatusSpec > ; # [doc = "Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum OvfError { # [doc = "0: No interrupt is reported"] B0 = 0 , # [doc = "1: Unmasked interrupt has been generated"] B1 = 1 , } impl From < OvfError > for bool { # [inline (always)] fn from (variant : OvfError) -> Self { variant as u8 != 0 } } # [doc = "Field `OVF_ERROR` reader - Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module"] pub type OvfErrorR = crate :: BitReader < OvfError > ; impl OvfErrorR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> OvfError { match self . bits { false => OvfError :: B0 , true => OvfError :: B1 , } } # [doc = "No interrupt is reported"] # [inline (always)] pub fn is_b0 (& self) -> bool { * self == OvfError :: B0 } # [doc = "Unmasked interrupt has been generated"] # [inline (always)] pub fn is_b1 (& self) -> bool { * self == OvfError :: B1 } } # [doc = "Field `OVF_ERROR` writer - Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module"] pub type OvfErrorW < 'a , REG > = crate :: BitWriter < 'a , REG , OvfError > ; impl < 'a , REG > OvfErrorW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "No interrupt is reported"] # [inline (always)] pub fn b0 (self) -> & 'a mut crate :: W < REG > { self . variant (OvfError :: B0) } # [doc = "Unmasked interrupt has been generated"] # [inline (always)] pub fn b1 (self) -> & 'a mut crate :: W < REG > { self . variant (OvfError :: B1) } } impl R { # [doc = "Bit 4 - Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module"] # [inline (always)] pub fn ovf_error (& self) -> OvfErrorR { OvfErrorR :: new (((self . bits >> 4) & 1) != 0) } } impl W { # [doc = "Bit 4 - Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module"] # [inline (always)] # [must_use] pub fn ovf_error (& mut self) -> OvfErrorW < ClkRecoverIntStatusSpec > { OvfErrorW :: new (self , 4) } } # [doc = "Clock recovery separated interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_recover_int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_recover_int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct ClkRecoverIntStatusSpec ; impl crate :: RegisterSpec for ClkRecoverIntStatusSpec { type Ux = u8 ; } # [doc = "`read()` method returns [`clk_recover_int_status::R`](R) reader structure"] impl crate :: Readable for ClkRecoverIntStatusSpec { } # [doc = "`write(|w| ..)` method takes [`clk_recover_int_status::W`](W) writer structure"] impl crate :: Writable for ClkRecoverIntStatusSpec { type Safety = crate :: Unsafe ; const ZERO_TO_MODIFY_FIELDS_BITMAP : u8 = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : u8 = 0 ; } # [doc = "`reset()` method sets CLK_RECOVER_INT_STATUS to value 0"] impl crate :: Resettable for ClkRecoverIntStatusSpec { const RESET_VALUE : u8 = 0 ; }