# [doc = "Register `WF13` reader"] pub type R = crate :: R < Wf13Spec > ; # [doc = "Register `WF13` writer"] pub type W = crate :: W < Wf13Spec > ; # [doc = "no description available\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bpalcd13 { # [doc = "0: LCD segment off or LCD backplane inactive for phase A"] B0 = 0 , # [doc = "1: LCD segment on or LCD backplane active for phase A"] B1 = 1 , } impl From < Bpalcd13 > for bool { # [inline (always)] fn from (variant : Bpalcd13) -> Self { variant as u8 != 0 } } # [doc = "Field `BPALCD13` reader - no description available"] pub type Bpalcd13R = crate :: BitReader < Bpalcd13 > ; impl Bpalcd13R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bpalcd13 { match self . bits { false => Bpalcd13 :: B0 , true => Bpalcd13 :: B1 , } } # [doc = "LCD segment off or LCD backplane inactive for phase A"] # [inline (always)] pub fn is_b0 (& self) -> bool { * self == Bpalcd13 :: B0 } # [doc = "LCD segment on or LCD backplane active for phase A"] # [inline (always)] pub fn is_b1 (& self) -> bool { * self == Bpalcd13 :: B1 } } # [doc = "Field `BPALCD13` writer - no description available"] pub type Bpalcd13W < 'a , REG > = crate :: BitWriter < 'a , REG , Bpalcd13 > ; impl < 'a , REG > Bpalcd13W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LCD segment off or LCD backplane inactive for phase A"] # [inline (always)] pub fn b0 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpalcd13 :: B0) } # [doc = "LCD segment on or LCD backplane active for phase A"] # [inline (always)] pub fn b1 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpalcd13 :: B1) } } # [doc = "no description available\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bpblcd13 { # [doc = "0: LCD segment off or LCD backplane inactive for phase B"] B0 = 0 , # [doc = "1: LCD segment on or LCD backplane active for phase B"] B1 = 1 , } impl From < Bpblcd13 > for bool { # [inline (always)] fn from (variant : Bpblcd13) -> Self { variant as u8 != 0 } } # [doc = "Field `BPBLCD13` reader - no description available"] pub type Bpblcd13R = crate :: BitReader < Bpblcd13 > ; impl Bpblcd13R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bpblcd13 { match self . bits { false => Bpblcd13 :: B0 , true => Bpblcd13 :: B1 , } } # [doc = "LCD segment off or LCD backplane inactive for phase B"] # [inline (always)] pub fn is_b0 (& self) -> bool { * self == Bpblcd13 :: B0 } # [doc = "LCD segment on or LCD backplane active for phase B"] # [inline (always)] pub fn is_b1 (& self) -> bool { * self == Bpblcd13 :: B1 } } # [doc = "Field `BPBLCD13` writer - no description available"] pub type Bpblcd13W < 'a , REG > = crate :: BitWriter < 'a , REG , Bpblcd13 > ; impl < 'a , REG > Bpblcd13W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LCD segment off or LCD backplane inactive for phase B"] # [inline (always)] pub fn b0 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpblcd13 :: B0) } # [doc = "LCD segment on or LCD backplane active for phase B"] # [inline (always)] pub fn b1 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpblcd13 :: B1) } } # [doc = "no description available\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bpclcd13 { # [doc = "0: LCD segment off or LCD backplane inactive for phase C"] B0 = 0 , # [doc = "1: LCD segment on or LCD backplane active for phase C"] B1 = 1 , } impl From < Bpclcd13 > for bool { # [inline (always)] fn from (variant : Bpclcd13) -> Self { variant as u8 != 0 } } # [doc = "Field `BPCLCD13` reader - no description available"] pub type Bpclcd13R = crate :: BitReader < Bpclcd13 > ; impl Bpclcd13R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bpclcd13 { match self . bits { false => Bpclcd13 :: B0 , true => Bpclcd13 :: B1 , } } # [doc = "LCD segment off or LCD backplane inactive for phase C"] # [inline (always)] pub fn is_b0 (& self) -> bool { * self == Bpclcd13 :: B0 } # [doc = "LCD segment on or LCD backplane active for phase C"] # [inline (always)] pub fn is_b1 (& self) -> bool { * self == Bpclcd13 :: B1 } } # [doc = "Field `BPCLCD13` writer - no description available"] pub type Bpclcd13W < 'a , REG > = crate :: BitWriter < 'a , REG , Bpclcd13 > ; impl < 'a , REG > Bpclcd13W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LCD segment off or LCD backplane inactive for phase C"] # [inline (always)] pub fn b0 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpclcd13 :: B0) } # [doc = "LCD segment on or LCD backplane active for phase C"] # [inline (always)] pub fn b1 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpclcd13 :: B1) } } # [doc = "no description available\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bpdlcd13 { # [doc = "0: LCD segment off or LCD backplane inactive for phase D"] B0 = 0 , # [doc = "1: LCD segment on or LCD backplane active for phase D"] B1 = 1 , } impl From < Bpdlcd13 > for bool { # [inline (always)] fn from (variant : Bpdlcd13) -> Self { variant as u8 != 0 } } # [doc = "Field `BPDLCD13` reader - no description available"] pub type Bpdlcd13R = crate :: BitReader < Bpdlcd13 > ; impl Bpdlcd13R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bpdlcd13 { match self . bits { false => Bpdlcd13 :: B0 , true => Bpdlcd13 :: B1 , } } # [doc = "LCD segment off or LCD backplane inactive for phase D"] # [inline (always)] pub fn is_b0 (& self) -> bool { * self == Bpdlcd13 :: B0 } # [doc = "LCD segment on or LCD backplane active for phase D"] # [inline (always)] pub fn is_b1 (& self) -> bool { * self == Bpdlcd13 :: B1 } } # [doc = "Field `BPDLCD13` writer - no description available"] pub type Bpdlcd13W < 'a , REG > = crate :: BitWriter < 'a , REG , Bpdlcd13 > ; impl < 'a , REG > Bpdlcd13W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LCD segment off or LCD backplane inactive for phase D"] # [inline (always)] pub fn b0 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpdlcd13 :: B0) } # [doc = "LCD segment on or LCD backplane active for phase D"] # [inline (always)] pub fn b1 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpdlcd13 :: B1) } } # [doc = "no description available\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bpelcd13 { # [doc = "0: LCD segment off or LCD backplane inactive for phase E"] B0 = 0 , # [doc = "1: LCD segment on or LCD backplane active for phase E"] B1 = 1 , } impl From < Bpelcd13 > for bool { # [inline (always)] fn from (variant : Bpelcd13) -> Self { variant as u8 != 0 } } # [doc = "Field `BPELCD13` reader - no description available"] pub type Bpelcd13R = crate :: BitReader < Bpelcd13 > ; impl Bpelcd13R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bpelcd13 { match self . bits { false => Bpelcd13 :: B0 , true => Bpelcd13 :: B1 , } } # [doc = "LCD segment off or LCD backplane inactive for phase E"] # [inline (always)] pub fn is_b0 (& self) -> bool { * self == Bpelcd13 :: B0 } # [doc = "LCD segment on or LCD backplane active for phase E"] # [inline (always)] pub fn is_b1 (& self) -> bool { * self == Bpelcd13 :: B1 } } # [doc = "Field `BPELCD13` writer - no description available"] pub type Bpelcd13W < 'a , REG > = crate :: BitWriter < 'a , REG , Bpelcd13 > ; impl < 'a , REG > Bpelcd13W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LCD segment off or LCD backplane inactive for phase E"] # [inline (always)] pub fn b0 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpelcd13 :: B0) } # [doc = "LCD segment on or LCD backplane active for phase E"] # [inline (always)] pub fn b1 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpelcd13 :: B1) } } # [doc = "no description available\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bpflcd13 { # [doc = "0: LCD segment off or LCD backplane inactive for phase F"] B0 = 0 , # [doc = "1: LCD segment on or LCD backplane active for phase F"] B1 = 1 , } impl From < Bpflcd13 > for bool { # [inline (always)] fn from (variant : Bpflcd13) -> Self { variant as u8 != 0 } } # [doc = "Field `BPFLCD13` reader - no description available"] pub type Bpflcd13R = crate :: BitReader < Bpflcd13 > ; impl Bpflcd13R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bpflcd13 { match self . bits { false => Bpflcd13 :: B0 , true => Bpflcd13 :: B1 , } } # [doc = "LCD segment off or LCD backplane inactive for phase F"] # [inline (always)] pub fn is_b0 (& self) -> bool { * self == Bpflcd13 :: B0 } # [doc = "LCD segment on or LCD backplane active for phase F"] # [inline (always)] pub fn is_b1 (& self) -> bool { * self == Bpflcd13 :: B1 } } # [doc = "Field `BPFLCD13` writer - no description available"] pub type Bpflcd13W < 'a , REG > = crate :: BitWriter < 'a , REG , Bpflcd13 > ; impl < 'a , REG > Bpflcd13W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LCD segment off or LCD backplane inactive for phase F"] # [inline (always)] pub fn b0 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpflcd13 :: B0) } # [doc = "LCD segment on or LCD backplane active for phase F"] # [inline (always)] pub fn b1 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpflcd13 :: B1) } } # [doc = "no description available\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bpglcd13 { # [doc = "0: LCD segment off or LCD backplane inactive for phase G"] B0 = 0 , # [doc = "1: LCD segment on or LCD backplane active for phase G"] B1 = 1 , } impl From < Bpglcd13 > for bool { # [inline (always)] fn from (variant : Bpglcd13) -> Self { variant as u8 != 0 } } # [doc = "Field `BPGLCD13` reader - no description available"] pub type Bpglcd13R = crate :: BitReader < Bpglcd13 > ; impl Bpglcd13R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bpglcd13 { match self . bits { false => Bpglcd13 :: B0 , true => Bpglcd13 :: B1 , } } # [doc = "LCD segment off or LCD backplane inactive for phase G"] # [inline (always)] pub fn is_b0 (& self) -> bool { * self == Bpglcd13 :: B0 } # [doc = "LCD segment on or LCD backplane active for phase G"] # [inline (always)] pub fn is_b1 (& self) -> bool { * self == Bpglcd13 :: B1 } } # [doc = "Field `BPGLCD13` writer - no description available"] pub type Bpglcd13W < 'a , REG > = crate :: BitWriter < 'a , REG , Bpglcd13 > ; impl < 'a , REG > Bpglcd13W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LCD segment off or LCD backplane inactive for phase G"] # [inline (always)] pub fn b0 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpglcd13 :: B0) } # [doc = "LCD segment on or LCD backplane active for phase G"] # [inline (always)] pub fn b1 (self) -> & 'a mut crate :: W < REG > { self . variant (Bpglcd13 :: B1) } } # [doc = "no description available\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Bphlcd13 { # [doc = "0: LCD segment off or LCD backplane inactive for phase H"] B0 = 0 , # [doc = "1: LCD segment on or LCD backplane active for phase H"] B1 = 1 , } impl From < Bphlcd13 > for bool { # [inline (always)] fn from (variant : Bphlcd13) -> Self { variant as u8 != 0 } } # [doc = "Field `BPHLCD13` reader - no description available"] pub type Bphlcd13R = crate :: BitReader < Bphlcd13 > ; impl Bphlcd13R { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Bphlcd13 { match self . bits { false => Bphlcd13 :: B0 , true => Bphlcd13 :: B1 , } } # [doc = "LCD segment off or LCD backplane inactive for phase H"] # [inline (always)] pub fn is_b0 (& self) -> bool { * self == Bphlcd13 :: B0 } # [doc = "LCD segment on or LCD backplane active for phase H"] # [inline (always)] pub fn is_b1 (& self) -> bool { * self == Bphlcd13 :: B1 } } # [doc = "Field `BPHLCD13` writer - no description available"] pub type Bphlcd13W < 'a , REG > = crate :: BitWriter < 'a , REG , Bphlcd13 > ; impl < 'a , REG > Bphlcd13W < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , { # [doc = "LCD segment off or LCD backplane inactive for phase H"] # [inline (always)] pub fn b0 (self) -> & 'a mut crate :: W < REG > { self . variant (Bphlcd13 :: B0) } # [doc = "LCD segment on or LCD backplane active for phase H"] # [inline (always)] pub fn b1 (self) -> & 'a mut crate :: W < REG > { self . variant (Bphlcd13 :: B1) } } impl R { # [doc = "Bit 0 - no description available"] # [inline (always)] pub fn bpalcd13 (& self) -> Bpalcd13R { Bpalcd13R :: new ((self . bits & 1) != 0) } # [doc = "Bit 1 - no description available"] # [inline (always)] pub fn bpblcd13 (& self) -> Bpblcd13R { Bpblcd13R :: new (((self . bits >> 1) & 1) != 0) } # [doc = "Bit 2 - no description available"] # [inline (always)] pub fn bpclcd13 (& self) -> Bpclcd13R { Bpclcd13R :: new (((self . bits >> 2) & 1) != 0) } # [doc = "Bit 3 - no description available"] # [inline (always)] pub fn bpdlcd13 (& self) -> Bpdlcd13R { Bpdlcd13R :: new (((self . bits >> 3) & 1) != 0) } # [doc = "Bit 4 - no description available"] # [inline (always)] pub fn bpelcd13 (& self) -> Bpelcd13R { Bpelcd13R :: new (((self . bits >> 4) & 1) != 0) } # [doc = "Bit 5 - no description available"] # [inline (always)] pub fn bpflcd13 (& self) -> Bpflcd13R { Bpflcd13R :: new (((self . bits >> 5) & 1) != 0) } # [doc = "Bit 6 - no description available"] # [inline (always)] pub fn bpglcd13 (& self) -> Bpglcd13R { Bpglcd13R :: new (((self . bits >> 6) & 1) != 0) } # [doc = "Bit 7 - no description available"] # [inline (always)] pub fn bphlcd13 (& self) -> Bphlcd13R { Bphlcd13R :: new (((self . bits >> 7) & 1) != 0) } } impl W { # [doc = "Bit 0 - no description available"] # [inline (always)] # [must_use] pub fn bpalcd13 (& mut self) -> Bpalcd13W < Wf13Spec > { Bpalcd13W :: new (self , 0) } # [doc = "Bit 1 - no description available"] # [inline (always)] # [must_use] pub fn bpblcd13 (& mut self) -> Bpblcd13W < Wf13Spec > { Bpblcd13W :: new (self , 1) } # [doc = "Bit 2 - no description available"] # [inline (always)] # [must_use] pub fn bpclcd13 (& mut self) -> Bpclcd13W < Wf13Spec > { Bpclcd13W :: new (self , 2) } # [doc = "Bit 3 - no description available"] # [inline (always)] # [must_use] pub fn bpdlcd13 (& mut self) -> Bpdlcd13W < Wf13Spec > { Bpdlcd13W :: new (self , 3) } # [doc = "Bit 4 - no description available"] # [inline (always)] # [must_use] pub fn bpelcd13 (& mut self) -> Bpelcd13W < Wf13Spec > { Bpelcd13W :: new (self , 4) } # [doc = "Bit 5 - no description available"] # [inline (always)] # [must_use] pub fn bpflcd13 (& mut self) -> Bpflcd13W < Wf13Spec > { Bpflcd13W :: new (self , 5) } # [doc = "Bit 6 - no description available"] # [inline (always)] # [must_use] pub fn bpglcd13 (& mut self) -> Bpglcd13W < Wf13Spec > { Bpglcd13W :: new (self , 6) } # [doc = "Bit 7 - no description available"] # [inline (always)] # [must_use] pub fn bphlcd13 (& mut self) -> Bphlcd13W < Wf13Spec > { Bphlcd13W :: new (self , 7) } } # [doc = "LCD Waveform Register 13.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct Wf13Spec ; impl crate :: RegisterSpec for Wf13Spec { type Ux = u8 ; } # [doc = "`read()` method returns [`wf13::R`](R) reader structure"] impl crate :: Readable for Wf13Spec { } # [doc = "`write(|w| ..)` method takes [`wf13::W`](W) writer structure"] impl crate :: Writable for Wf13Spec { type Safety = crate :: Unsafe ; const ZERO_TO_MODIFY_FIELDS_BITMAP : u8 = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : u8 = 0 ; } # [doc = "`reset()` method sets WF13 to value 0"] impl crate :: Resettable for Wf13Spec { const RESET_VALUE : u8 = 0 ; }