#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pmprot: Pmprot,
    pmctrl: Pmctrl,
    stopctrl: Stopctrl,
    pmstat: Pmstat,
}
impl RegisterBlock {
    #[doc = "0x00 - Power Mode Protection register"]
    #[inline(always)]
    pub const fn pmprot(&self) -> &Pmprot {
        &self.pmprot
    }
    #[doc = "0x01 - Power Mode Control register"]
    #[inline(always)]
    pub const fn pmctrl(&self) -> &Pmctrl {
        &self.pmctrl
    }
    #[doc = "0x02 - Stop Control Register"]
    #[inline(always)]
    pub const fn stopctrl(&self) -> &Stopctrl {
        &self.stopctrl
    }
    #[doc = "0x03 - Power Mode Status register"]
    #[inline(always)]
    pub const fn pmstat(&self) -> &Pmstat {
        &self.pmstat
    }
}
#[doc = "PMPROT (rw) register accessor: Power Mode Protection register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmprot`]
module"]
#[doc(alias = "PMPROT")]
pub type Pmprot = crate::Reg<pmprot::PmprotSpec>;
#[doc = "Power Mode Protection register"]
pub mod pmprot;
#[doc = "PMCTRL (rw) register accessor: Power Mode Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmctrl`]
module"]
#[doc(alias = "PMCTRL")]
pub type Pmctrl = crate::Reg<pmctrl::PmctrlSpec>;
#[doc = "Power Mode Control register"]
pub mod pmctrl;
#[doc = "STOPCTRL (rw) register accessor: Stop Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stopctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stopctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stopctrl`]
module"]
#[doc(alias = "STOPCTRL")]
pub type Stopctrl = crate::Reg<stopctrl::StopctrlSpec>;
#[doc = "Stop Control Register"]
pub mod stopctrl;
#[doc = "PMSTAT (r) register accessor: Power Mode Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmstat`]
module"]
#[doc(alias = "PMSTAT")]
pub type Pmstat = crate::Reg<pmstat::PmstatSpec>;
#[doc = "Power Mode Status register"]
pub mod pmstat;
