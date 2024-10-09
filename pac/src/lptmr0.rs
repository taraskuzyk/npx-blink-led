#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: Csr,
    psr: Psr,
    cmr: Cmr,
    cnr: Cnr,
}
impl RegisterBlock {
    #[doc = "0x00 - Low Power Timer Control Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - Low Power Timer Prescale Register"]
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        &self.psr
    }
    #[doc = "0x08 - Low Power Timer Compare Register"]
    #[inline(always)]
    pub const fn cmr(&self) -> &Cmr {
        &self.cmr
    }
    #[doc = "0x0c - Low Power Timer Counter Register"]
    #[inline(always)]
    pub const fn cnr(&self) -> &Cnr {
        &self.cnr
    }
}
#[doc = "CSR (rw) register accessor: Low Power Timer Control Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Low Power Timer Control Status Register"]
pub mod csr;
#[doc = "PSR (rw) register accessor: Low Power Timer Prescale Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psr`]
module"]
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
#[doc = "Low Power Timer Prescale Register"]
pub mod psr;
#[doc = "CMR (rw) register accessor: Low Power Timer Compare Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmr`]
module"]
#[doc(alias = "CMR")]
pub type Cmr = crate::Reg<cmr::CmrSpec>;
#[doc = "Low Power Timer Compare Register"]
pub mod cmr;
#[doc = "CNR (rw) register accessor: Low Power Timer Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnr`]
module"]
#[doc(alias = "CNR")]
pub type Cnr = crate::Reg<cnr::CnrSpec>;
#[doc = "Low Power Timer Counter Register"]
pub mod cnr;
