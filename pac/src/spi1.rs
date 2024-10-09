#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    s: S,
    br: Br,
    c2: C2,
    c1: C1,
    ml: Ml,
    mh: Mh,
    dl: Dl,
    dh: Dh,
    _reserved8: [u8; 0x02],
    ci: Ci,
    c3: C3,
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Status Register"]
    #[inline(always)]
    pub const fn s(&self) -> &S {
        &self.s
    }
    #[doc = "0x01 - SPI Baud Rate Register"]
    #[inline(always)]
    pub const fn br(&self) -> &Br {
        &self.br
    }
    #[doc = "0x02 - SPI Control Register 2"]
    #[inline(always)]
    pub const fn c2(&self) -> &C2 {
        &self.c2
    }
    #[doc = "0x03 - SPI Control Register 1"]
    #[inline(always)]
    pub const fn c1(&self) -> &C1 {
        &self.c1
    }
    #[doc = "0x04 - SPI Match Register low"]
    #[inline(always)]
    pub const fn ml(&self) -> &Ml {
        &self.ml
    }
    #[doc = "0x05 - SPI match register high"]
    #[inline(always)]
    pub const fn mh(&self) -> &Mh {
        &self.mh
    }
    #[doc = "0x06 - SPI Data Register low"]
    #[inline(always)]
    pub const fn dl(&self) -> &Dl {
        &self.dl
    }
    #[doc = "0x07 - SPI data register high"]
    #[inline(always)]
    pub const fn dh(&self) -> &Dh {
        &self.dh
    }
    #[doc = "0x0a - SPI clear interrupt register"]
    #[inline(always)]
    pub const fn ci(&self) -> &Ci {
        &self.ci
    }
    #[doc = "0x0b - SPI control register 3"]
    #[inline(always)]
    pub const fn c3(&self) -> &C3 {
        &self.c3
    }
}
#[doc = "S (rw) register accessor: SPI Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s`]
module"]
pub type S = crate::Reg<s::SSpec>;
#[doc = "SPI Status Register"]
pub mod s;
#[doc = "BR (rw) register accessor: SPI Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`br::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`br::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@br`]
module"]
#[doc(alias = "BR")]
pub type Br = crate::Reg<br::BrSpec>;
#[doc = "SPI Baud Rate Register"]
pub mod br;
#[doc = "C2 (rw) register accessor: SPI Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`]
module"]
pub type C2 = crate::Reg<c2::C2Spec>;
#[doc = "SPI Control Register 2"]
pub mod c2;
#[doc = "C1 (rw) register accessor: SPI Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`]
module"]
pub type C1 = crate::Reg<c1::C1Spec>;
#[doc = "SPI Control Register 1"]
pub mod c1;
#[doc = "ML (rw) register accessor: SPI Match Register low\n\nYou can [`read`](crate::Reg::read) this register and get [`ml::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ml::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ml`]
module"]
#[doc(alias = "ML")]
pub type Ml = crate::Reg<ml::MlSpec>;
#[doc = "SPI Match Register low"]
pub mod ml;
#[doc = "MH (rw) register accessor: SPI match register high\n\nYou can [`read`](crate::Reg::read) this register and get [`mh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mh`]
module"]
#[doc(alias = "MH")]
pub type Mh = crate::Reg<mh::MhSpec>;
#[doc = "SPI match register high"]
pub mod mh;
#[doc = "DL (rw) register accessor: SPI Data Register low\n\nYou can [`read`](crate::Reg::read) this register and get [`dl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dl`]
module"]
#[doc(alias = "DL")]
pub type Dl = crate::Reg<dl::DlSpec>;
#[doc = "SPI Data Register low"]
pub mod dl;
#[doc = "DH (rw) register accessor: SPI data register high\n\nYou can [`read`](crate::Reg::read) this register and get [`dh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dh`]
module"]
#[doc(alias = "DH")]
pub type Dh = crate::Reg<dh::DhSpec>;
#[doc = "SPI data register high"]
pub mod dh;
#[doc = "CI (rw) register accessor: SPI clear interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`ci::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ci::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ci`]
module"]
#[doc(alias = "CI")]
pub type Ci = crate::Reg<ci::CiSpec>;
#[doc = "SPI clear interrupt register"]
pub mod ci;
#[doc = "C3 (rw) register accessor: SPI control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3`]
module"]
pub type C3 = crate::Reg<c3::C3Spec>;
#[doc = "SPI control register 3"]
pub mod c3;
