#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    _reserved1: [u8; 0x1c],
    comp: (),
    _reserved2: [u8; 0x04],
    mask: (),
    _reserved3: [u8; 0x04],
    fct0: Fct0,
    _reserved4: [u8; 0x0c],
    fct1: Fct1,
    _reserved5: [u8; 0x01c4],
    tbctrl: Tbctrl,
    _reserved6: [u8; 0x0dc4],
    devicecfg: Devicecfg,
    devicetypid: Devicetypid,
    periphid: [Periphid; 8],
    compid: [Compid; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - MTB DWT Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x20..0x28 - MTB_DWT Comparator Register"]
    #[inline(always)]
    pub const fn comp(&self, n: usize) -> &Comp {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - MTB_DWT Comparator Register"]
    #[inline(always)]
    pub fn comp_iter(&self) -> impl Iterator<Item = &Comp> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(32)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x24..0x2c - MTB_DWT Comparator Mask Register"]
    #[inline(always)]
    pub const fn mask(&self, n: usize) -> &Mask {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(36)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x2c - MTB_DWT Comparator Mask Register"]
    #[inline(always)]
    pub fn mask_iter(&self) -> impl Iterator<Item = &Mask> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(36)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x28 - MTB_DWT Comparator Function Register 0"]
    #[inline(always)]
    pub const fn fct0(&self) -> &Fct0 {
        &self.fct0
    }
    #[doc = "0x38 - MTB_DWT Comparator Function Register 1"]
    #[inline(always)]
    pub const fn fct1(&self) -> &Fct1 {
        &self.fct1
    }
    #[doc = "0x200 - MTB_DWT Trace Buffer Control Register"]
    #[inline(always)]
    pub const fn tbctrl(&self) -> &Tbctrl {
        &self.tbctrl
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
#[doc = "CTRL (r) register accessor: MTB DWT Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "MTB DWT Control Register"]
pub mod ctrl;
#[doc = "COMP (rw) register accessor: MTB_DWT Comparator Register\n\nYou can [`read`](crate::Reg::read) this register and get [`comp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp`]
module"]
#[doc(alias = "COMP")]
pub type Comp = crate::Reg<comp::CompSpec>;
#[doc = "MTB_DWT Comparator Register"]
pub mod comp;
#[doc = "MASK (rw) register accessor: MTB_DWT Comparator Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask`]
module"]
#[doc(alias = "MASK")]
pub type Mask = crate::Reg<mask::MaskSpec>;
#[doc = "MTB_DWT Comparator Mask Register"]
pub mod mask;
#[doc = "FCT0 (rw) register accessor: MTB_DWT Comparator Function Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`fct0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fct0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fct0`]
module"]
#[doc(alias = "FCT0")]
pub type Fct0 = crate::Reg<fct0::Fct0Spec>;
#[doc = "MTB_DWT Comparator Function Register 0"]
pub mod fct0;
#[doc = "FCT1 (rw) register accessor: MTB_DWT Comparator Function Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fct1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fct1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fct1`]
module"]
#[doc(alias = "FCT1")]
pub type Fct1 = crate::Reg<fct1::Fct1Spec>;
#[doc = "MTB_DWT Comparator Function Register 1"]
pub mod fct1;
#[doc = "TBCTRL (rw) register accessor: MTB_DWT Trace Buffer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tbctrl`]
module"]
#[doc(alias = "TBCTRL")]
pub type Tbctrl = crate::Reg<tbctrl::TbctrlSpec>;
#[doc = "MTB_DWT Trace Buffer Control Register"]
pub mod tbctrl;
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
