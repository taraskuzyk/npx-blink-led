# [repr (C)] # [doc = "Register block"] pub struct RegisterBlock { datl : () , _reserved1 : [u8 ; 0x01] , dath : () , _reserved2 : [u8 ; 0x1f] , sr : Sr , c0 : C0 , c1 : C1 , c2 : C2 , } impl RegisterBlock { # [doc = "0x00 - DAC Data Low Register"] # [inline (always)] pub const fn datl (& self , n : usize) -> & Datl { # [allow (clippy :: no_effect)] [() ; 2] [n] ; unsafe { & * (self as * const Self) . cast :: < u8 > () . add (0) . add (2 * n) . cast () } } # [doc = "Iterator for array of:"] # [doc = "0x00 - DAC Data Low Register"] # [inline (always)] pub fn datl_iter (& self) -> impl Iterator < Item = & Datl > { (0 .. 2) . map (move | n | unsafe { & * (self as * const Self) . cast :: < u8 > () . add (0) . add (2 * n) . cast () }) } # [doc = "0x00 - DAC Data Low Register"] # [inline (always)] pub const fn dat0l (& self) -> & Datl { self . datl (0) } # [doc = "0x02 - DAC Data Low Register"] # [inline (always)] pub const fn dat1l (& self) -> & Datl { self . datl (1) } # [doc = "0x01 - DAC Data High Register"] # [inline (always)] pub const fn dath (& self , n : usize) -> & Dath { # [allow (clippy :: no_effect)] [() ; 2] [n] ; unsafe { & * (self as * const Self) . cast :: < u8 > () . add (1) . add (2 * n) . cast () } } # [doc = "Iterator for array of:"] # [doc = "0x01 - DAC Data High Register"] # [inline (always)] pub fn dath_iter (& self) -> impl Iterator < Item = & Dath > { (0 .. 2) . map (move | n | unsafe { & * (self as * const Self) . cast :: < u8 > () . add (1) . add (2 * n) . cast () }) } # [doc = "0x01 - DAC Data High Register"] # [inline (always)] pub const fn dat0h (& self) -> & Dath { self . dath (0) } # [doc = "0x03 - DAC Data High Register"] # [inline (always)] pub const fn dat1h (& self) -> & Dath { self . dath (1) } # [doc = "0x20 - DAC Status Register"] # [inline (always)] pub const fn sr (& self) -> & Sr { & self . sr } # [doc = "0x21 - DAC Control Register"] # [inline (always)] pub const fn c0 (& self) -> & C0 { & self . c0 } # [doc = "0x22 - DAC Control Register 1"] # [inline (always)] pub const fn c1 (& self) -> & C1 { & self . c1 } # [doc = "0x23 - DAC Control Register 2"] # [inline (always)] pub const fn c2 (& self) -> & C2 { & self . c2 } } # [doc = "DATL (rw) register accessor: DAC Data Low Register\n\nYou can [`read`](crate::Reg::read) this register and get [`datl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datl`]
module"] # [doc (alias = "DATL")] pub type Datl = crate :: Reg < datl :: DatlSpec > ; # [doc = "DAC Data Low Register"] pub mod datl ; # [doc = "DATH (rw) register accessor: DAC Data High Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dath::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dath::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dath`]
module"] # [doc (alias = "DATH")] pub type Dath = crate :: Reg < dath :: DathSpec > ; # [doc = "DAC Data High Register"] pub mod dath ; # [doc = "SR (rw) register accessor: DAC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"] # [doc (alias = "SR")] pub type Sr = crate :: Reg < sr :: SrSpec > ; # [doc = "DAC Status Register"] pub mod sr ; # [doc = "C0 (rw) register accessor: DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c0`]
module"] pub type C0 = crate :: Reg < c0 :: C0Spec > ; # [doc = "DAC Control Register"] pub mod c0 ; # [doc = "C1 (rw) register accessor: DAC Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`]
module"] pub type C1 = crate :: Reg < c1 :: C1Spec > ; # [doc = "DAC Control Register 1"] pub mod c1 ; # [doc = "C2 (rw) register accessor: DAC Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`]
module"] pub type C2 = crate :: Reg < c2 :: C2Spec > ; # [doc = "DAC Control Register 2"] pub mod c2 ;