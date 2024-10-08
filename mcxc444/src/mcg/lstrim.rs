# [doc = "Register `LSTRIM` reader"] pub type R = crate :: R < LstrimSpec > ; # [doc = "Field `LIRC_STRIM` reader - LIRC2M TRIM"] pub type LircStrimR = crate :: FieldReader ; impl R { # [doc = "Bits 0:6 - LIRC2M TRIM"] # [inline (always)] pub fn lirc_strim (& self) -> LircStrimR { LircStrimR :: new (self . bits & 0x7f) } } # [doc = "MCG Low-frequency IRC2M Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lstrim::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct LstrimSpec ; impl crate :: RegisterSpec for LstrimSpec { type Ux = u8 ; } # [doc = "`read()` method returns [`lstrim::R`](R) reader structure"] impl crate :: Readable for LstrimSpec { } # [doc = "`reset()` method sets LSTRIM to value 0"] impl crate :: Resettable for LstrimSpec { const RESET_VALUE : u8 = 0 ; }