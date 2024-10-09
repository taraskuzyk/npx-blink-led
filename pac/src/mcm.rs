#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    plasc: Plasc,
    plamc: Plamc,
    placr: Placr,
    _reserved3: [u8; 0x30],
    cpo: Cpo,
}
impl RegisterBlock {
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    #[inline(always)]
    pub const fn plasc(&self) -> &Plasc {
        &self.plasc
    }
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    #[inline(always)]
    pub const fn plamc(&self) -> &Plamc {
        &self.plamc
    }
    #[doc = "0x0c - Platform Control Register"]
    #[inline(always)]
    pub const fn placr(&self) -> &Placr {
        &self.placr
    }
    #[doc = "0x40 - Compute Operation Control Register"]
    #[inline(always)]
    pub const fn cpo(&self) -> &Cpo {
        &self.cpo
    }
}
#[doc = "PLASC (r) register accessor: Crossbar Switch (AXBS) Slave Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`plasc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plasc`]
module"]
#[doc(alias = "PLASC")]
pub type Plasc = crate::Reg<plasc::PlascSpec>;
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "PLAMC (r) register accessor: Crossbar Switch (AXBS) Master Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`plamc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@plamc`]
module"]
#[doc(alias = "PLAMC")]
pub type Plamc = crate::Reg<plamc::PlamcSpec>;
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "PLACR (rw) register accessor: Platform Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`placr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`placr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@placr`]
module"]
#[doc(alias = "PLACR")]
pub type Placr = crate::Reg<placr::PlacrSpec>;
#[doc = "Platform Control Register"]
pub mod placr;
#[doc = "CPO (rw) register accessor: Compute Operation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpo`]
module"]
#[doc(alias = "CPO")]
pub type Cpo = crate::Reg<cpo::CpoSpec>;
#[doc = "Compute Operation Control Register"]
pub mod cpo;
