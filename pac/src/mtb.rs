#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    position: Position,
    master: Master,
    flow: Flow,
    base: Base,
    _reserved4: [u8; 0x0ef0],
    modectrl: Modectrl,
    _reserved5: [u8; 0x9c],
    tagset: Tagset,
    tagclear: Tagclear,
    _reserved7: [u8; 0x08],
    lockaccess: Lockaccess,
    lockstat: Lockstat,
    authstat: Authstat,
    devicearch: Devicearch,
    _reserved11: [u8; 0x08],
    devicecfg: Devicecfg,
    devicetypid: Devicetypid,
    periphid: [Periphid; 8],
    compid: [Compid; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - MTB Position Register"]
    #[inline(always)]
    pub const fn position(&self) -> &Position {
        &self.position
    }
    #[doc = "0x04 - MTB Master Register"]
    #[inline(always)]
    pub const fn master(&self) -> &Master {
        &self.master
    }
    #[doc = "0x08 - MTB Flow Register"]
    #[inline(always)]
    pub const fn flow(&self) -> &Flow {
        &self.flow
    }
    #[doc = "0x0c - MTB Base Register"]
    #[inline(always)]
    pub const fn base(&self) -> &Base {
        &self.base
    }
    #[doc = "0xf00 - Integration Mode Control Register"]
    #[inline(always)]
    pub const fn modectrl(&self) -> &Modectrl {
        &self.modectrl
    }
    #[doc = "0xfa0 - Claim TAG Set Register"]
    #[inline(always)]
    pub const fn tagset(&self) -> &Tagset {
        &self.tagset
    }
    #[doc = "0xfa4 - Claim TAG Clear Register"]
    #[inline(always)]
    pub const fn tagclear(&self) -> &Tagclear {
        &self.tagclear
    }
    #[doc = "0xfb0 - Lock Access Register"]
    #[inline(always)]
    pub const fn lockaccess(&self) -> &Lockaccess {
        &self.lockaccess
    }
    #[doc = "0xfb4 - Lock Status Register"]
    #[inline(always)]
    pub const fn lockstat(&self) -> &Lockstat {
        &self.lockstat
    }
    #[doc = "0xfb8 - Authentication Status Register"]
    #[inline(always)]
    pub const fn authstat(&self) -> &Authstat {
        &self.authstat
    }
    #[doc = "0xfbc - Device Architecture Register"]
    #[inline(always)]
    pub const fn devicearch(&self) -> &Devicearch {
        &self.devicearch
    }
    #[doc = "0xfc8 - Device Configuration Register"]
    #[inline(always)]
    pub const fn devicecfg(&self) -> &Devicecfg {
        &self.devicecfg
    }
    #[doc = "0xfcc - Device Type Identifier Register"]
    #[inline(always)]
    pub const fn devicetypid(&self) -> &Devicetypid {
        &self.devicetypid
    }
    #[doc = "0xfd0..0xff0 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn periphid(&self, n: usize) -> &Periphid {
        &self.periphid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xfd0..0xff0 - Peripheral ID Register"]
    #[inline(always)]
    pub fn periphid_iter(&self) -> impl Iterator<Item = &Periphid> {
        self.periphid.iter()
    }
    #[doc = "0xfd0 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn periphid4(&self) -> &Periphid {
        self.periphid(0)
    }
    #[doc = "0xfd4 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn periphid5(&self) -> &Periphid {
        self.periphid(1)
    }
    #[doc = "0xfd8 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn periphid6(&self) -> &Periphid {
        self.periphid(2)
    }
    #[doc = "0xfdc - Peripheral ID Register"]
    #[inline(always)]
    pub const fn periphid7(&self) -> &Periphid {
        self.periphid(3)
    }
    #[doc = "0xfe0 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn periphid0(&self) -> &Periphid {
        self.periphid(4)
    }
    #[doc = "0xfe4 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn periphid1(&self) -> &Periphid {
        self.periphid(5)
    }
    #[doc = "0xfe8 - Peripheral ID Register"]
    #[inline(always)]
    pub const fn periphid2(&self) -> &Periphid {
        self.periphid(6)
    }
    #[doc = "0xfec - Peripheral ID Register"]
    #[inline(always)]
    pub const fn periphid3(&self) -> &Periphid {
        self.periphid(7)
    }
    #[doc = "0xff0..0x1000 - Component ID Register"]
    #[inline(always)]
    pub const fn compid(&self, n: usize) -> &Compid {
        &self.compid[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xff0..0x1000 - Component ID Register"]
    #[inline(always)]
    pub fn compid_iter(&self) -> impl Iterator<Item = &Compid> {
        self.compid.iter()
    }
}
#[doc = "POSITION (rw) register accessor: MTB Position Register\n\nYou can [`read`](crate::Reg::read) this register and get [`position::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`position::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@position`]
module"]
#[doc(alias = "POSITION")]
pub type Position = crate::Reg<position::PositionSpec>;
#[doc = "MTB Position Register"]
pub mod position;
#[doc = "MASTER (rw) register accessor: MTB Master Register\n\nYou can [`read`](crate::Reg::read) this register and get [`master::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`master::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@master`]
module"]
#[doc(alias = "MASTER")]
pub type Master = crate::Reg<master::MasterSpec>;
#[doc = "MTB Master Register"]
pub mod master;
#[doc = "FLOW (rw) register accessor: MTB Flow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`flow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flow`]
module"]
#[doc(alias = "FLOW")]
pub type Flow = crate::Reg<flow::FlowSpec>;
#[doc = "MTB Flow Register"]
pub mod flow;
#[doc = "BASE (r) register accessor: MTB Base Register\n\nYou can [`read`](crate::Reg::read) this register and get [`base::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base`]
module"]
#[doc(alias = "BASE")]
pub type Base = crate::Reg<base::BaseSpec>;
#[doc = "MTB Base Register"]
pub mod base;
#[doc = "MODECTRL (r) register accessor: Integration Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`modectrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modectrl`]
module"]
#[doc(alias = "MODECTRL")]
pub type Modectrl = crate::Reg<modectrl::ModectrlSpec>;
#[doc = "Integration Mode Control Register"]
pub mod modectrl;
#[doc = "TAGSET (r) register accessor: Claim TAG Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tagset::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagset`]
module"]
#[doc(alias = "TAGSET")]
pub type Tagset = crate::Reg<tagset::TagsetSpec>;
#[doc = "Claim TAG Set Register"]
pub mod tagset;
#[doc = "TAGCLEAR (r) register accessor: Claim TAG Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tagclear::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tagclear`]
module"]
#[doc(alias = "TAGCLEAR")]
pub type Tagclear = crate::Reg<tagclear::TagclearSpec>;
#[doc = "Claim TAG Clear Register"]
pub mod tagclear;
#[doc = "LOCKACCESS (r) register accessor: Lock Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lockaccess::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockaccess`]
module"]
#[doc(alias = "LOCKACCESS")]
pub type Lockaccess = crate::Reg<lockaccess::LockaccessSpec>;
#[doc = "Lock Access Register"]
pub mod lockaccess;
#[doc = "LOCKSTAT (r) register accessor: Lock Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lockstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockstat`]
module"]
#[doc(alias = "LOCKSTAT")]
pub type Lockstat = crate::Reg<lockstat::LockstatSpec>;
#[doc = "Lock Status Register"]
pub mod lockstat;
#[doc = "AUTHSTAT (r) register accessor: Authentication Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`authstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@authstat`]
module"]
#[doc(alias = "AUTHSTAT")]
pub type Authstat = crate::Reg<authstat::AuthstatSpec>;
#[doc = "Authentication Status Register"]
pub mod authstat;
#[doc = "DEVICEARCH (r) register accessor: Device Architecture Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devicearch::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devicearch`]
module"]
#[doc(alias = "DEVICEARCH")]
pub type Devicearch = crate::Reg<devicearch::DevicearchSpec>;
#[doc = "Device Architecture Register"]
pub mod devicearch;
#[doc = "DEVICECFG (r) register accessor: Device Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devicecfg::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devicecfg`]
module"]
#[doc(alias = "DEVICECFG")]
pub type Devicecfg = crate::Reg<devicecfg::DevicecfgSpec>;
#[doc = "Device Configuration Register"]
pub mod devicecfg;
#[doc = "DEVICETYPID (r) register accessor: Device Type Identifier Register\n\nYou can [`read`](crate::Reg::read) this register and get [`devicetypid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@devicetypid`]
module"]
#[doc(alias = "DEVICETYPID")]
pub type Devicetypid = crate::Reg<devicetypid::DevicetypidSpec>;
#[doc = "Device Type Identifier Register"]
pub mod devicetypid;
#[doc = "PERIPHID (r) register accessor: Peripheral ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`periphid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periphid`]
module"]
#[doc(alias = "PERIPHID")]
pub type Periphid = crate::Reg<periphid::PeriphidSpec>;
#[doc = "Peripheral ID Register"]
pub mod periphid;
#[doc = "COMPID (r) register accessor: Component ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`compid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compid`]
module"]
#[doc(alias = "COMPID")]
pub type Compid = crate::Reg<compid::CompidSpec>;
#[doc = "Component ID Register"]
pub mod compid;
