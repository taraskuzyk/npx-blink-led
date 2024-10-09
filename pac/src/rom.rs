#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    entry: [Entry; 3],
    tablemark: Tablemark,
    _reserved2: [u8; 0x0fbc],
    sysaccess: Sysaccess,
    periphid: [Periphid; 8],
    compid: [Compid; 4],
}
impl RegisterBlock {
    #[doc = "0x00..0x0c - Entry"]
    #[inline(always)]
    pub const fn entry(&self, n: usize) -> &Entry {
        &self.entry[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x0c - Entry"]
    #[inline(always)]
    pub fn entry_iter(&self) -> impl Iterator<Item = &Entry> {
        self.entry.iter()
    }
    #[doc = "0x0c - End of Table Marker Register"]
    #[inline(always)]
    pub const fn tablemark(&self) -> &Tablemark {
        &self.tablemark
    }
    #[doc = "0xfcc - System Access Register"]
    #[inline(always)]
    pub const fn sysaccess(&self) -> &Sysaccess {
        &self.sysaccess
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
#[doc = "ENTRY (r) register accessor: Entry\n\nYou can [`read`](crate::Reg::read) this register and get [`entry::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@entry`]
module"]
#[doc(alias = "ENTRY")]
pub type Entry = crate::Reg<entry::EntrySpec>;
#[doc = "Entry"]
pub mod entry;
#[doc = "TABLEMARK (r) register accessor: End of Table Marker Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tablemark::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tablemark`]
module"]
#[doc(alias = "TABLEMARK")]
pub type Tablemark = crate::Reg<tablemark::TablemarkSpec>;
#[doc = "End of Table Marker Register"]
pub mod tablemark;
#[doc = "SYSACCESS (r) register accessor: System Access Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sysaccess::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysaccess`]
module"]
#[doc(alias = "SYSACCESS")]
pub type Sysaccess = crate::Reg<sysaccess::SysaccessSpec>;
#[doc = "System Access Register"]
pub mod sysaccess;
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
