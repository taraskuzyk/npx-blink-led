#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    srs0: Srs0,
    srs1: Srs1,
    _reserved2: [u8; 0x02],
    rpfc: Rpfc,
    rpfw: Rpfw,
    fm: Fm,
    mr: Mr,
    ssrs0: Ssrs0,
    ssrs1: Ssrs1,
}
impl RegisterBlock {
    #[doc = "0x00 - System Reset Status Register 0"]
    #[inline(always)]
    pub const fn srs0(&self) -> &Srs0 {
        &self.srs0
    }
    #[doc = "0x01 - System Reset Status Register 1"]
    #[inline(always)]
    pub const fn srs1(&self) -> &Srs1 {
        &self.srs1
    }
    #[doc = "0x04 - Reset Pin Filter Control register"]
    #[inline(always)]
    pub const fn rpfc(&self) -> &Rpfc {
        &self.rpfc
    }
    #[doc = "0x05 - Reset Pin Filter Width register"]
    #[inline(always)]
    pub const fn rpfw(&self) -> &Rpfw {
        &self.rpfw
    }
    #[doc = "0x06 - Force Mode Register"]
    #[inline(always)]
    pub const fn fm(&self) -> &Fm {
        &self.fm
    }
    #[doc = "0x07 - Mode Register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x08 - Sticky System Reset Status Register 0"]
    #[inline(always)]
    pub const fn ssrs0(&self) -> &Ssrs0 {
        &self.ssrs0
    }
    #[doc = "0x09 - Sticky System Reset Status Register 1"]
    #[inline(always)]
    pub const fn ssrs1(&self) -> &Ssrs1 {
        &self.ssrs1
    }
}
#[doc = "SRS0 (r) register accessor: System Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`srs0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srs0`]
module"]
#[doc(alias = "SRS0")]
pub type Srs0 = crate::Reg<srs0::Srs0Spec>;
#[doc = "System Reset Status Register 0"]
pub mod srs0;
#[doc = "SRS1 (r) register accessor: System Reset Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`srs1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srs1`]
module"]
#[doc(alias = "SRS1")]
pub type Srs1 = crate::Reg<srs1::Srs1Spec>;
#[doc = "System Reset Status Register 1"]
pub mod srs1;
#[doc = "RPFC (rw) register accessor: Reset Pin Filter Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpfc`]
module"]
#[doc(alias = "RPFC")]
pub type Rpfc = crate::Reg<rpfc::RpfcSpec>;
#[doc = "Reset Pin Filter Control register"]
pub mod rpfc;
#[doc = "RPFW (rw) register accessor: Reset Pin Filter Width register\n\nYou can [`read`](crate::Reg::read) this register and get [`rpfw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpfw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rpfw`]
module"]
#[doc(alias = "RPFW")]
pub type Rpfw = crate::Reg<rpfw::RpfwSpec>;
#[doc = "Reset Pin Filter Width register"]
pub mod rpfw;
#[doc = "FM (rw) register accessor: Force Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fm`]
module"]
#[doc(alias = "FM")]
pub type Fm = crate::Reg<fm::FmSpec>;
#[doc = "Force Mode Register"]
pub mod fm;
#[doc = "MR (rw) register accessor: Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Mode Register"]
pub mod mr;
#[doc = "SSRS0 (rw) register accessor: Sticky System Reset Status Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ssrs0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrs0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssrs0`]
module"]
#[doc(alias = "SSRS0")]
pub type Ssrs0 = crate::Reg<ssrs0::Ssrs0Spec>;
#[doc = "Sticky System Reset Status Register 0"]
pub mod ssrs0;
#[doc = "SSRS1 (rw) register accessor: Sticky System Reset Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ssrs1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssrs1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssrs1`]
module"]
#[doc(alias = "SSRS1")]
pub type Ssrs1 = crate::Reg<ssrs1::Ssrs1Spec>;
#[doc = "Sticky System Reset Status Register 1"]
pub mod ssrs1;
