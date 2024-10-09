#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    baud: Baud,
    stat: Stat,
    ctrl: Ctrl,
    data: Data,
    match_: Match,
}
impl RegisterBlock {
    #[doc = "0x00 - LPUART Baud Rate Register"]
    #[inline(always)]
    pub const fn baud(&self) -> &Baud {
        &self.baud
    }
    #[doc = "0x04 - LPUART Status Register"]
    #[inline(always)]
    pub const fn stat(&self) -> &Stat {
        &self.stat
    }
    #[doc = "0x08 - LPUART Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - LPUART Data Register"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x10 - LPUART Match Address Register"]
    #[inline(always)]
    pub const fn match_(&self) -> &Match {
        &self.match_
    }
}
#[doc = "BAUD (rw) register accessor: LPUART Baud Rate Register\n\nYou can [`read`](crate::Reg::read) this register and get [`baud::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`baud::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@baud`]
module"]
#[doc(alias = "BAUD")]
pub type Baud = crate::Reg<baud::BaudSpec>;
#[doc = "LPUART Baud Rate Register"]
pub mod baud;
#[doc = "STAT (rw) register accessor: LPUART Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "STAT")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "LPUART Status Register"]
pub mod stat;
#[doc = "CTRL (rw) register accessor: LPUART Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "LPUART Control Register"]
pub mod ctrl;
#[doc = "DATA (rw) register accessor: LPUART Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`]
module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "LPUART Data Register"]
pub mod data;
#[doc = "MATCH (rw) register accessor: LPUART Match Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`match_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`match_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@match_`]
module"]
#[doc(alias = "MATCH")]
pub type Match = crate::Reg<match_::MatchSpec>;
#[doc = "LPUART Match Address Register"]
pub mod match_;
