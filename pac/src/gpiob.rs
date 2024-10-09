#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pdor: Pdor,
    psor: Psor,
    pcor: Pcor,
    ptor: Ptor,
    pdir: Pdir,
    pddr: Pddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Port Data Output Register"]
    #[inline(always)]
    pub const fn pdor(&self) -> &Pdor {
        &self.pdor
    }
    #[doc = "0x04 - Port Set Output Register"]
    #[inline(always)]
    pub const fn psor(&self) -> &Psor {
        &self.psor
    }
    #[doc = "0x08 - Port Clear Output Register"]
    #[inline(always)]
    pub const fn pcor(&self) -> &Pcor {
        &self.pcor
    }
    #[doc = "0x0c - Port Toggle Output Register"]
    #[inline(always)]
    pub const fn ptor(&self) -> &Ptor {
        &self.ptor
    }
    #[doc = "0x10 - Port Data Input Register"]
    #[inline(always)]
    pub const fn pdir(&self) -> &Pdir {
        &self.pdir
    }
    #[doc = "0x14 - Port Data Direction Register"]
    #[inline(always)]
    pub const fn pddr(&self) -> &Pddr {
        &self.pddr
    }
}
#[doc = "PDOR (rw) register accessor: Port Data Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdor`]
module"]
#[doc(alias = "PDOR")]
pub type Pdor = crate::Reg<pdor::PdorSpec>;
#[doc = "Port Data Output Register"]
pub mod pdor;
#[doc = "PSOR (rw) register accessor: Port Set Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`psor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psor`]
module"]
#[doc(alias = "PSOR")]
pub type Psor = crate::Reg<psor::PsorSpec>;
#[doc = "Port Set Output Register"]
pub mod psor;
#[doc = "PCOR (rw) register accessor: Port Clear Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcor`]
module"]
#[doc(alias = "PCOR")]
pub type Pcor = crate::Reg<pcor::PcorSpec>;
#[doc = "Port Clear Output Register"]
pub mod pcor;
#[doc = "PTOR (rw) register accessor: Port Toggle Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ptor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ptor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ptor`]
module"]
#[doc(alias = "PTOR")]
pub type Ptor = crate::Reg<ptor::PtorSpec>;
#[doc = "Port Toggle Output Register"]
pub mod ptor;
#[doc = "PDIR (r) register accessor: Port Data Input Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdir`]
module"]
#[doc(alias = "PDIR")]
pub type Pdir = crate::Reg<pdir::PdirSpec>;
#[doc = "Port Data Input Register"]
pub mod pdir;
#[doc = "PDDR (rw) register accessor: Port Data Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pddr`]
module"]
#[doc(alias = "PDDR")]
pub type Pddr = crate::Reg<pddr::PddrSpec>;
#[doc = "Port Data Direction Register"]
pub mod pddr;
