#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    verid: Verid,
    param: Param,
    ctrl: Ctrl,
    _reserved3: [u8; 0x04],
    shiftstat: Shiftstat,
    shifterr: Shifterr,
    timstat: Timstat,
    _reserved6: [u8; 0x04],
    shiftsien: Shiftsien,
    shifteien: Shifteien,
    timien: Timien,
    _reserved9: [u8; 0x04],
    shiftsden: Shiftsden,
    _reserved10: [u8; 0x4c],
    shiftctl: [Shiftctl; 4],
    _reserved11: [u8; 0x70],
    shiftcfg: [Shiftcfg; 4],
    _reserved12: [u8; 0xf0],
    shiftbuf: [Shiftbuf; 4],
    _reserved13: [u8; 0x70],
    shiftbufbis: [Shiftbufbis; 4],
    _reserved14: [u8; 0x70],
    shiftbufbys: [Shiftbufbys; 4],
    _reserved15: [u8; 0x70],
    shiftbufbbs: [Shiftbufbbs; 4],
    _reserved16: [u8; 0x70],
    timctl: [Timctl; 4],
    _reserved17: [u8; 0x70],
    timcfg: [Timcfg; 4],
    _reserved18: [u8; 0x70],
    timcmp: [Timcmp; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Version ID Register"]
    #[inline(always)]
    pub const fn verid(&self) -> &Verid {
        &self.verid
    }
    #[doc = "0x04 - Parameter Register"]
    #[inline(always)]
    pub const fn param(&self) -> &Param {
        &self.param
    }
    #[doc = "0x08 - FlexIO Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x10 - Shifter Status Register"]
    #[inline(always)]
    pub const fn shiftstat(&self) -> &Shiftstat {
        &self.shiftstat
    }
    #[doc = "0x14 - Shifter Error Register"]
    #[inline(always)]
    pub const fn shifterr(&self) -> &Shifterr {
        &self.shifterr
    }
    #[doc = "0x18 - Timer Status Register"]
    #[inline(always)]
    pub const fn timstat(&self) -> &Timstat {
        &self.timstat
    }
    #[doc = "0x20 - Shifter Status Interrupt Enable"]
    #[inline(always)]
    pub const fn shiftsien(&self) -> &Shiftsien {
        &self.shiftsien
    }
    #[doc = "0x24 - Shifter Error Interrupt Enable"]
    #[inline(always)]
    pub const fn shifteien(&self) -> &Shifteien {
        &self.shifteien
    }
    #[doc = "0x28 - Timer Interrupt Enable Register"]
    #[inline(always)]
    pub const fn timien(&self) -> &Timien {
        &self.timien
    }
    #[doc = "0x30 - Shifter Status DMA Enable"]
    #[inline(always)]
    pub const fn shiftsden(&self) -> &Shiftsden {
        &self.shiftsden
    }
    #[doc = "0x80..0x90 - Shifter Control N Register"]
    #[inline(always)]
    pub const fn shiftctl(&self, n: usize) -> &Shiftctl {
        &self.shiftctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x90 - Shifter Control N Register"]
    #[inline(always)]
    pub fn shiftctl_iter(&self) -> impl Iterator<Item = &Shiftctl> {
        self.shiftctl.iter()
    }
    #[doc = "0x100..0x110 - Shifter Configuration N Register"]
    #[inline(always)]
    pub const fn shiftcfg(&self, n: usize) -> &Shiftcfg {
        &self.shiftcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x110 - Shifter Configuration N Register"]
    #[inline(always)]
    pub fn shiftcfg_iter(&self) -> impl Iterator<Item = &Shiftcfg> {
        self.shiftcfg.iter()
    }
    #[doc = "0x200..0x210 - Shifter Buffer N Register"]
    #[inline(always)]
    pub const fn shiftbuf(&self, n: usize) -> &Shiftbuf {
        &self.shiftbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x210 - Shifter Buffer N Register"]
    #[inline(always)]
    pub fn shiftbuf_iter(&self) -> impl Iterator<Item = &Shiftbuf> {
        self.shiftbuf.iter()
    }
    #[doc = "0x280..0x290 - Shifter Buffer N Bit Swapped Register"]
    #[inline(always)]
    pub const fn shiftbufbis(&self, n: usize) -> &Shiftbufbis {
        &self.shiftbufbis[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x280..0x290 - Shifter Buffer N Bit Swapped Register"]
    #[inline(always)]
    pub fn shiftbufbis_iter(&self) -> impl Iterator<Item = &Shiftbufbis> {
        self.shiftbufbis.iter()
    }
    #[doc = "0x300..0x310 - Shifter Buffer N Byte Swapped Register"]
    #[inline(always)]
    pub const fn shiftbufbys(&self, n: usize) -> &Shiftbufbys {
        &self.shiftbufbys[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x310 - Shifter Buffer N Byte Swapped Register"]
    #[inline(always)]
    pub fn shiftbufbys_iter(&self) -> impl Iterator<Item = &Shiftbufbys> {
        self.shiftbufbys.iter()
    }
    #[doc = "0x380..0x390 - Shifter Buffer N Bit Byte Swapped Register"]
    #[inline(always)]
    pub const fn shiftbufbbs(&self, n: usize) -> &Shiftbufbbs {
        &self.shiftbufbbs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x380..0x390 - Shifter Buffer N Bit Byte Swapped Register"]
    #[inline(always)]
    pub fn shiftbufbbs_iter(&self) -> impl Iterator<Item = &Shiftbufbbs> {
        self.shiftbufbbs.iter()
    }
    #[doc = "0x400..0x410 - Timer Control N Register"]
    #[inline(always)]
    pub const fn timctl(&self, n: usize) -> &Timctl {
        &self.timctl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x410 - Timer Control N Register"]
    #[inline(always)]
    pub fn timctl_iter(&self) -> impl Iterator<Item = &Timctl> {
        self.timctl.iter()
    }
    #[doc = "0x480..0x490 - Timer Configuration N Register"]
    #[inline(always)]
    pub const fn timcfg(&self, n: usize) -> &Timcfg {
        &self.timcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x480..0x490 - Timer Configuration N Register"]
    #[inline(always)]
    pub fn timcfg_iter(&self) -> impl Iterator<Item = &Timcfg> {
        self.timcfg.iter()
    }
    #[doc = "0x500..0x510 - Timer Compare N Register"]
    #[inline(always)]
    pub const fn timcmp(&self, n: usize) -> &Timcmp {
        &self.timcmp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x500..0x510 - Timer Compare N Register"]
    #[inline(always)]
    pub fn timcmp_iter(&self) -> impl Iterator<Item = &Timcmp> {
        self.timcmp.iter()
    }
}
#[doc = "VERID (r) register accessor: Version ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`verid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verid`]
module"]
#[doc(alias = "VERID")]
pub type Verid = crate::Reg<verid::VeridSpec>;
#[doc = "Version ID Register"]
pub mod verid;
#[doc = "PARAM (r) register accessor: Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`param::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@param`]
module"]
#[doc(alias = "PARAM")]
pub type Param = crate::Reg<param::ParamSpec>;
#[doc = "Parameter Register"]
pub mod param;
#[doc = "CTRL (rw) register accessor: FlexIO Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "FlexIO Control Register"]
pub mod ctrl;
#[doc = "SHIFTSTAT (rw) register accessor: Shifter Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftstat`]
module"]
#[doc(alias = "SHIFTSTAT")]
pub type Shiftstat = crate::Reg<shiftstat::ShiftstatSpec>;
#[doc = "Shifter Status Register"]
pub mod shiftstat;
#[doc = "SHIFTERR (rw) register accessor: Shifter Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shifterr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shifterr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shifterr`]
module"]
#[doc(alias = "SHIFTERR")]
pub type Shifterr = crate::Reg<shifterr::ShifterrSpec>;
#[doc = "Shifter Error Register"]
pub mod shifterr;
#[doc = "TIMSTAT (rw) register accessor: Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timstat`]
module"]
#[doc(alias = "TIMSTAT")]
pub type Timstat = crate::Reg<timstat::TimstatSpec>;
#[doc = "Timer Status Register"]
pub mod timstat;
#[doc = "SHIFTSIEN (rw) register accessor: Shifter Status Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftsien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftsien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftsien`]
module"]
#[doc(alias = "SHIFTSIEN")]
pub type Shiftsien = crate::Reg<shiftsien::ShiftsienSpec>;
#[doc = "Shifter Status Interrupt Enable"]
pub mod shiftsien;
#[doc = "SHIFTEIEN (rw) register accessor: Shifter Error Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`shifteien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shifteien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shifteien`]
module"]
#[doc(alias = "SHIFTEIEN")]
pub type Shifteien = crate::Reg<shifteien::ShifteienSpec>;
#[doc = "Shifter Error Interrupt Enable"]
pub mod shifteien;
#[doc = "TIMIEN (rw) register accessor: Timer Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timien`]
module"]
#[doc(alias = "TIMIEN")]
pub type Timien = crate::Reg<timien::TimienSpec>;
#[doc = "Timer Interrupt Enable Register"]
pub mod timien;
#[doc = "SHIFTSDEN (rw) register accessor: Shifter Status DMA Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftsden::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftsden::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftsden`]
module"]
#[doc(alias = "SHIFTSDEN")]
pub type Shiftsden = crate::Reg<shiftsden::ShiftsdenSpec>;
#[doc = "Shifter Status DMA Enable"]
pub mod shiftsden;
#[doc = "SHIFTCTL (rw) register accessor: Shifter Control N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftctl`]
module"]
#[doc(alias = "SHIFTCTL")]
pub type Shiftctl = crate::Reg<shiftctl::ShiftctlSpec>;
#[doc = "Shifter Control N Register"]
pub mod shiftctl;
#[doc = "SHIFTCFG (rw) register accessor: Shifter Configuration N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftcfg`]
module"]
#[doc(alias = "SHIFTCFG")]
pub type Shiftcfg = crate::Reg<shiftcfg::ShiftcfgSpec>;
#[doc = "Shifter Configuration N Register"]
pub mod shiftcfg;
#[doc = "SHIFTBUF (rw) register accessor: Shifter Buffer N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftbuf`]
module"]
#[doc(alias = "SHIFTBUF")]
pub type Shiftbuf = crate::Reg<shiftbuf::ShiftbufSpec>;
#[doc = "Shifter Buffer N Register"]
pub mod shiftbuf;
#[doc = "SHIFTBUFBIS (rw) register accessor: Shifter Buffer N Bit Swapped Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftbufbis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftbufbis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftbufbis`]
module"]
#[doc(alias = "SHIFTBUFBIS")]
pub type Shiftbufbis = crate::Reg<shiftbufbis::ShiftbufbisSpec>;
#[doc = "Shifter Buffer N Bit Swapped Register"]
pub mod shiftbufbis;
#[doc = "SHIFTBUFBYS (rw) register accessor: Shifter Buffer N Byte Swapped Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftbufbys::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftbufbys::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftbufbys`]
module"]
#[doc(alias = "SHIFTBUFBYS")]
pub type Shiftbufbys = crate::Reg<shiftbufbys::ShiftbufbysSpec>;
#[doc = "Shifter Buffer N Byte Swapped Register"]
pub mod shiftbufbys;
#[doc = "SHIFTBUFBBS (rw) register accessor: Shifter Buffer N Bit Byte Swapped Register\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftbufbbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftbufbbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftbufbbs`]
module"]
#[doc(alias = "SHIFTBUFBBS")]
pub type Shiftbufbbs = crate::Reg<shiftbufbbs::ShiftbufbbsSpec>;
#[doc = "Shifter Buffer N Bit Byte Swapped Register"]
pub mod shiftbufbbs;
#[doc = "TIMCTL (rw) register accessor: Timer Control N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timctl`]
module"]
#[doc(alias = "TIMCTL")]
pub type Timctl = crate::Reg<timctl::TimctlSpec>;
#[doc = "Timer Control N Register"]
pub mod timctl;
#[doc = "TIMCFG (rw) register accessor: Timer Configuration N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timcfg`]
module"]
#[doc(alias = "TIMCFG")]
pub type Timcfg = crate::Reg<timcfg::TimcfgSpec>;
#[doc = "Timer Configuration N Register"]
pub mod timcfg;
#[doc = "TIMCMP (rw) register accessor: Timer Compare N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timcmp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timcmp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timcmp`]
module"]
#[doc(alias = "TIMCMP")]
pub type Timcmp = crate::Reg<timcmp::TimcmpSpec>;
#[doc = "Timer Compare N Register"]
pub mod timcmp;
