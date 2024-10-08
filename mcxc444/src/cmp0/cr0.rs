# [doc = "Register `CR0` reader"] pub type R = crate :: R < Cr0Spec > ; # [doc = "Register `CR0` writer"] pub type W = crate :: W < Cr0Spec > ; # [doc = "Comparator hard block hysteresis control\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum Hystctr { # [doc = "0: Level 0"] B00 = 0 , # [doc = "1: Level 1"] B01 = 1 , # [doc = "2: Level 2"] B10 = 2 , # [doc = "3: Level 3"] B11 = 3 , } impl From < Hystctr > for u8 { # [inline (always)] fn from (variant : Hystctr) -> Self { variant as _ } } impl crate :: FieldSpec for Hystctr { type Ux = u8 ; } impl crate :: IsEnum for Hystctr { } # [doc = "Field `HYSTCTR` reader - Comparator hard block hysteresis control"] pub type HystctrR = crate :: FieldReader < Hystctr > ; impl HystctrR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> Hystctr { match self . bits { 0 => Hystctr :: B00 , 1 => Hystctr :: B01 , 2 => Hystctr :: B10 , 3 => Hystctr :: B11 , _ => unreachable ! () , } } # [doc = "Level 0"] # [inline (always)] pub fn is_b00 (& self) -> bool { * self == Hystctr :: B00 } # [doc = "Level 1"] # [inline (always)] pub fn is_b01 (& self) -> bool { * self == Hystctr :: B01 } # [doc = "Level 2"] # [inline (always)] pub fn is_b10 (& self) -> bool { * self == Hystctr :: B10 } # [doc = "Level 3"] # [inline (always)] pub fn is_b11 (& self) -> bool { * self == Hystctr :: B11 } } # [doc = "Field `HYSTCTR` writer - Comparator hard block hysteresis control"] pub type HystctrW < 'a , REG > = crate :: FieldWriter < 'a , REG , 2 , Hystctr , crate :: Safe > ; impl < 'a , REG > HystctrW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "Level 0"] # [inline (always)] pub fn b00 (self) -> & 'a mut crate :: W < REG > { self . variant (Hystctr :: B00) } # [doc = "Level 1"] # [inline (always)] pub fn b01 (self) -> & 'a mut crate :: W < REG > { self . variant (Hystctr :: B01) } # [doc = "Level 2"] # [inline (always)] pub fn b10 (self) -> & 'a mut crate :: W < REG > { self . variant (Hystctr :: B10) } # [doc = "Level 3"] # [inline (always)] pub fn b11 (self) -> & 'a mut crate :: W < REG > { self . variant (Hystctr :: B11) } } # [doc = "Filter Sample Count\n\nValue on reset: 0"] # [derive (Clone , Copy , Debug , PartialEq , Eq)] # [repr (u8)] pub enum FilterCnt { # [doc = "0: Filter is disabled. SE = 0, COUT = COUTA."] B000 = 0 , # [doc = "1: One sample must agree. The comparator output is simply sampled."] B001 = 1 , # [doc = "2: 2 consecutive samples must agree."] B010 = 2 , # [doc = "3: 3 consecutive samples must agree."] B011 = 3 , # [doc = "4: 4 consecutive samples must agree."] B100 = 4 , # [doc = "5: 5 consecutive samples must agree."] B101 = 5 , # [doc = "6: 6 consecutive samples must agree."] B110 = 6 , # [doc = "7: 7 consecutive samples must agree."] B111 = 7 , } impl From < FilterCnt > for u8 { # [inline (always)] fn from (variant : FilterCnt) -> Self { variant as _ } } impl crate :: FieldSpec for FilterCnt { type Ux = u8 ; } impl crate :: IsEnum for FilterCnt { } # [doc = "Field `FILTER_CNT` reader - Filter Sample Count"] pub type FilterCntR = crate :: FieldReader < FilterCnt > ; impl FilterCntR { # [doc = "Get enumerated values variant"] # [inline (always)] pub const fn variant (& self) -> FilterCnt { match self . bits { 0 => FilterCnt :: B000 , 1 => FilterCnt :: B001 , 2 => FilterCnt :: B010 , 3 => FilterCnt :: B011 , 4 => FilterCnt :: B100 , 5 => FilterCnt :: B101 , 6 => FilterCnt :: B110 , 7 => FilterCnt :: B111 , _ => unreachable ! () , } } # [doc = "Filter is disabled. SE = 0, COUT = COUTA."] # [inline (always)] pub fn is_b000 (& self) -> bool { * self == FilterCnt :: B000 } # [doc = "One sample must agree. The comparator output is simply sampled."] # [inline (always)] pub fn is_b001 (& self) -> bool { * self == FilterCnt :: B001 } # [doc = "2 consecutive samples must agree."] # [inline (always)] pub fn is_b010 (& self) -> bool { * self == FilterCnt :: B010 } # [doc = "3 consecutive samples must agree."] # [inline (always)] pub fn is_b011 (& self) -> bool { * self == FilterCnt :: B011 } # [doc = "4 consecutive samples must agree."] # [inline (always)] pub fn is_b100 (& self) -> bool { * self == FilterCnt :: B100 } # [doc = "5 consecutive samples must agree."] # [inline (always)] pub fn is_b101 (& self) -> bool { * self == FilterCnt :: B101 } # [doc = "6 consecutive samples must agree."] # [inline (always)] pub fn is_b110 (& self) -> bool { * self == FilterCnt :: B110 } # [doc = "7 consecutive samples must agree."] # [inline (always)] pub fn is_b111 (& self) -> bool { * self == FilterCnt :: B111 } } # [doc = "Field `FILTER_CNT` writer - Filter Sample Count"] pub type FilterCntW < 'a , REG > = crate :: FieldWriter < 'a , REG , 3 , FilterCnt , crate :: Safe > ; impl < 'a , REG > FilterCntW < 'a , REG > where REG : crate :: Writable + crate :: RegisterSpec , REG :: Ux : From < u8 > { # [doc = "Filter is disabled. SE = 0, COUT = COUTA."] # [inline (always)] pub fn b000 (self) -> & 'a mut crate :: W < REG > { self . variant (FilterCnt :: B000) } # [doc = "One sample must agree. The comparator output is simply sampled."] # [inline (always)] pub fn b001 (self) -> & 'a mut crate :: W < REG > { self . variant (FilterCnt :: B001) } # [doc = "2 consecutive samples must agree."] # [inline (always)] pub fn b010 (self) -> & 'a mut crate :: W < REG > { self . variant (FilterCnt :: B010) } # [doc = "3 consecutive samples must agree."] # [inline (always)] pub fn b011 (self) -> & 'a mut crate :: W < REG > { self . variant (FilterCnt :: B011) } # [doc = "4 consecutive samples must agree."] # [inline (always)] pub fn b100 (self) -> & 'a mut crate :: W < REG > { self . variant (FilterCnt :: B100) } # [doc = "5 consecutive samples must agree."] # [inline (always)] pub fn b101 (self) -> & 'a mut crate :: W < REG > { self . variant (FilterCnt :: B101) } # [doc = "6 consecutive samples must agree."] # [inline (always)] pub fn b110 (self) -> & 'a mut crate :: W < REG > { self . variant (FilterCnt :: B110) } # [doc = "7 consecutive samples must agree."] # [inline (always)] pub fn b111 (self) -> & 'a mut crate :: W < REG > { self . variant (FilterCnt :: B111) } } impl R { # [doc = "Bits 0:1 - Comparator hard block hysteresis control"] # [inline (always)] pub fn hystctr (& self) -> HystctrR { HystctrR :: new (self . bits & 3) } # [doc = "Bits 4:6 - Filter Sample Count"] # [inline (always)] pub fn filter_cnt (& self) -> FilterCntR { FilterCntR :: new ((self . bits >> 4) & 7) } } impl W { # [doc = "Bits 0:1 - Comparator hard block hysteresis control"] # [inline (always)] # [must_use] pub fn hystctr (& mut self) -> HystctrW < Cr0Spec > { HystctrW :: new (self , 0) } # [doc = "Bits 4:6 - Filter Sample Count"] # [inline (always)] # [must_use] pub fn filter_cnt (& mut self) -> FilterCntW < Cr0Spec > { FilterCntW :: new (self , 4) } } # [doc = "CMP Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct Cr0Spec ; impl crate :: RegisterSpec for Cr0Spec { type Ux = u8 ; } # [doc = "`read()` method returns [`cr0::R`](R) reader structure"] impl crate :: Readable for Cr0Spec { } # [doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"] impl crate :: Writable for Cr0Spec { type Safety = crate :: Unsafe ; const ZERO_TO_MODIFY_FIELDS_BITMAP : u8 = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : u8 = 0 ; } # [doc = "`reset()` method sets CR0 to value 0"] impl crate :: Resettable for Cr0Spec { const RESET_VALUE : u8 = 0 ; }