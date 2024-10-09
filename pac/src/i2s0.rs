#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tcsr: Tcsr,
    _reserved1: [u8; 0x04],
    tcr2: Tcr2,
    tcr3: Tcr3,
    tcr4: Tcr4,
    tcr5: Tcr5,
    _reserved5: [u8; 0x08],
    tdr: Tdr,
    _reserved6: [u8; 0x3c],
    tmr: Tmr,
    _reserved7: [u8; 0x1c],
    rcsr: Rcsr,
    _reserved8: [u8; 0x04],
    rcr2: Rcr2,
    rcr3: Rcr3,
    rcr4: Rcr4,
    rcr5: Rcr5,
    _reserved12: [u8; 0x08],
    rdr: Rdr,
    _reserved13: [u8; 0x3c],
    rmr: Rmr,
    _reserved14: [u8; 0x1c],
    mcr: Mcr,
}
impl RegisterBlock {
    #[doc = "0x00 - SAI Transmit Control Register"]
    #[inline(always)]
    pub const fn tcsr(&self) -> &Tcsr {
        &self.tcsr
    }
    #[doc = "0x08 - SAI Transmit Configuration 2 Register"]
    #[inline(always)]
    pub const fn tcr2(&self) -> &Tcr2 {
        &self.tcr2
    }
    #[doc = "0x0c - SAI Transmit Configuration 3 Register"]
    #[inline(always)]
    pub const fn tcr3(&self) -> &Tcr3 {
        &self.tcr3
    }
    #[doc = "0x10 - SAI Transmit Configuration 4 Register"]
    #[inline(always)]
    pub const fn tcr4(&self) -> &Tcr4 {
        &self.tcr4
    }
    #[doc = "0x14 - SAI Transmit Configuration 5 Register"]
    #[inline(always)]
    pub const fn tcr5(&self) -> &Tcr5 {
        &self.tcr5
    }
    #[doc = "0x20 - SAI Transmit Data Register"]
    #[inline(always)]
    pub const fn tdr(&self) -> &Tdr {
        &self.tdr
    }
    #[doc = "0x60 - SAI Transmit Mask Register"]
    #[inline(always)]
    pub const fn tmr(&self) -> &Tmr {
        &self.tmr
    }
    #[doc = "0x80 - SAI Receive Control Register"]
    #[inline(always)]
    pub const fn rcsr(&self) -> &Rcsr {
        &self.rcsr
    }
    #[doc = "0x88 - SAI Receive Configuration 2 Register"]
    #[inline(always)]
    pub const fn rcr2(&self) -> &Rcr2 {
        &self.rcr2
    }
    #[doc = "0x8c - SAI Receive Configuration 3 Register"]
    #[inline(always)]
    pub const fn rcr3(&self) -> &Rcr3 {
        &self.rcr3
    }
    #[doc = "0x90 - SAI Receive Configuration 4 Register"]
    #[inline(always)]
    pub const fn rcr4(&self) -> &Rcr4 {
        &self.rcr4
    }
    #[doc = "0x94 - SAI Receive Configuration 5 Register"]
    #[inline(always)]
    pub const fn rcr5(&self) -> &Rcr5 {
        &self.rcr5
    }
    #[doc = "0xa0 - SAI Receive Data Register"]
    #[inline(always)]
    pub const fn rdr(&self) -> &Rdr {
        &self.rdr
    }
    #[doc = "0xe0 - SAI Receive Mask Register"]
    #[inline(always)]
    pub const fn rmr(&self) -> &Rmr {
        &self.rmr
    }
    #[doc = "0x100 - SAI MCLK Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
}
#[doc = "TCSR (rw) register accessor: SAI Transmit Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcsr`]
module"]
#[doc(alias = "TCSR")]
pub type Tcsr = crate::Reg<tcsr::TcsrSpec>;
#[doc = "SAI Transmit Control Register"]
pub mod tcsr;
#[doc = "TCR2 (rw) register accessor: SAI Transmit Configuration 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr2`]
module"]
#[doc(alias = "TCR2")]
pub type Tcr2 = crate::Reg<tcr2::Tcr2Spec>;
#[doc = "SAI Transmit Configuration 2 Register"]
pub mod tcr2;
#[doc = "TCR3 (rw) register accessor: SAI Transmit Configuration 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr3`]
module"]
#[doc(alias = "TCR3")]
pub type Tcr3 = crate::Reg<tcr3::Tcr3Spec>;
#[doc = "SAI Transmit Configuration 3 Register"]
pub mod tcr3;
#[doc = "TCR4 (rw) register accessor: SAI Transmit Configuration 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr4`]
module"]
#[doc(alias = "TCR4")]
pub type Tcr4 = crate::Reg<tcr4::Tcr4Spec>;
#[doc = "SAI Transmit Configuration 4 Register"]
pub mod tcr4;
#[doc = "TCR5 (rw) register accessor: SAI Transmit Configuration 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr5`]
module"]
#[doc(alias = "TCR5")]
pub type Tcr5 = crate::Reg<tcr5::Tcr5Spec>;
#[doc = "SAI Transmit Configuration 5 Register"]
pub mod tcr5;
#[doc = "TDR (rw) register accessor: SAI Transmit Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdr`]
module"]
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TdrSpec>;
#[doc = "SAI Transmit Data Register"]
pub mod tdr;
#[doc = "TMR (rw) register accessor: SAI Transmit Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tmr`]
module"]
#[doc(alias = "TMR")]
pub type Tmr = crate::Reg<tmr::TmrSpec>;
#[doc = "SAI Transmit Mask Register"]
pub mod tmr;
#[doc = "RCSR (rw) register accessor: SAI Receive Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcsr`]
module"]
#[doc(alias = "RCSR")]
pub type Rcsr = crate::Reg<rcsr::RcsrSpec>;
#[doc = "SAI Receive Control Register"]
pub mod rcsr;
#[doc = "RCR2 (rw) register accessor: SAI Receive Configuration 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr2`]
module"]
#[doc(alias = "RCR2")]
pub type Rcr2 = crate::Reg<rcr2::Rcr2Spec>;
#[doc = "SAI Receive Configuration 2 Register"]
pub mod rcr2;
#[doc = "RCR3 (rw) register accessor: SAI Receive Configuration 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr3`]
module"]
#[doc(alias = "RCR3")]
pub type Rcr3 = crate::Reg<rcr3::Rcr3Spec>;
#[doc = "SAI Receive Configuration 3 Register"]
pub mod rcr3;
#[doc = "RCR4 (rw) register accessor: SAI Receive Configuration 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr4`]
module"]
#[doc(alias = "RCR4")]
pub type Rcr4 = crate::Reg<rcr4::Rcr4Spec>;
#[doc = "SAI Receive Configuration 4 Register"]
pub mod rcr4;
#[doc = "RCR5 (rw) register accessor: SAI Receive Configuration 5 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcr5`]
module"]
#[doc(alias = "RCR5")]
pub type Rcr5 = crate::Reg<rcr5::Rcr5Spec>;
#[doc = "SAI Receive Configuration 5 Register"]
pub mod rcr5;
#[doc = "RDR (r) register accessor: SAI Receive Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
#[doc(alias = "RDR")]
pub type Rdr = crate::Reg<rdr::RdrSpec>;
#[doc = "SAI Receive Data Register"]
pub mod rdr;
#[doc = "RMR (rw) register accessor: SAI Receive Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rmr`]
module"]
#[doc(alias = "RMR")]
pub type Rmr = crate::Reg<rmr::RmrSpec>;
#[doc = "SAI Receive Mask Register"]
pub mod rmr;
#[doc = "MCR (rw) register accessor: SAI MCLK Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "SAI MCLK Control Register"]
pub mod mcr;
