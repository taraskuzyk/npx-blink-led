# [doc = "Register `DSR3` reader"] pub type R = crate :: R < Dsr3Spec > ; # [doc = "Register `DSR3` writer"] pub type W = crate :: W < Dsr3Spec > ; impl core :: fmt :: Debug for R { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> core :: fmt :: Result { write ! (f , "{}" , self . bits ()) } } impl W { } # [doc = "DMA_DSR3 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct Dsr3Spec ; impl crate :: RegisterSpec for Dsr3Spec { type Ux = u8 ; } # [doc = "`read()` method returns [`dsr3::R`](R) reader structure"] impl crate :: Readable for Dsr3Spec { } # [doc = "`write(|w| ..)` method takes [`dsr3::W`](W) writer structure"] impl crate :: Writable for Dsr3Spec { type Safety = crate :: Unsafe ; const ZERO_TO_MODIFY_FIELDS_BITMAP : u8 = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : u8 = 0 ; } # [doc = "`reset()` method sets DSR3 to value 0"] impl crate :: Resettable for Dsr3Spec { const RESET_VALUE : u8 = 0 ; }