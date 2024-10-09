#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr0: Cr0,
    cr1: Cr1,
    fpr: Fpr,
    scr: Scr,
    daccr: Daccr,
    muxcr: Muxcr,
}
impl RegisterBlock {
    #[doc = "0x00 - CMP Control Register 0"]
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    #[doc = "0x01 - CMP Control Register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x02 - CMP Filter Period Register"]
    #[inline(always)]
    pub const fn fpr(&self) -> &Fpr {
        &self.fpr
    }
    #[doc = "0x03 - CMP Status and Control Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x04 - DAC Control Register"]
    #[inline(always)]
    pub const fn daccr(&self) -> &Daccr {
        &self.daccr
    }
    #[doc = "0x05 - MUX Control Register"]
    #[inline(always)]
    pub const fn muxcr(&self) -> &Muxcr {
        &self.muxcr
    }
}
#[doc = "CR0 (rw) register accessor: CMP Control Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr0`]
module"]
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
#[doc = "CMP Control Register 0"]
pub mod cr0;
#[doc = "CR1 (rw) register accessor: CMP Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "CMP Control Register 1"]
pub mod cr1;
#[doc = "FPR (rw) register accessor: CMP Filter Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpr`]
module"]
#[doc(alias = "FPR")]
pub type Fpr = crate::Reg<fpr::FprSpec>;
#[doc = "CMP Filter Period Register"]
pub mod fpr;
#[doc = "SCR (rw) register accessor: CMP Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "CMP Status and Control Register"]
pub mod scr;
#[doc = "DACCR (rw) register accessor: DAC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`daccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daccr`]
module"]
#[doc(alias = "DACCR")]
pub type Daccr = crate::Reg<daccr::DaccrSpec>;
#[doc = "DAC Control Register"]
pub mod daccr;
#[doc = "MUXCR (rw) register accessor: MUX Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`muxcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`muxcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@muxcr`]
module"]
#[doc(alias = "MUXCR")]
pub type Muxcr = crate::Reg<muxcr::MuxcrSpec>;
#[doc = "MUX Control Register"]
pub mod muxcr;
