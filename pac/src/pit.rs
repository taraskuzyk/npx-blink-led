#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcr: Mcr,
    _reserved1: [u8; 0xdc],
    ltmr64h: Ltmr64h,
    ltmr64l: Ltmr64l,
    _reserved3: [u8; 0x18],
    ldval: (),
    _reserved4: [u8; 0x04],
    cval: (),
    _reserved5: [u8; 0x04],
    tctrl: (),
    _reserved6: [u8; 0x04],
    tflg: (),
}
impl RegisterBlock {
    #[doc = "0x00 - PIT Module Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0xe0 - PIT Upper Lifetime Timer Register"]
    #[inline(always)]
    pub const fn ltmr64h(&self) -> &Ltmr64h {
        &self.ltmr64h
    }
    #[doc = "0xe4 - PIT Lower Lifetime Timer Register"]
    #[inline(always)]
    pub const fn ltmr64l(&self) -> &Ltmr64l {
        &self.ltmr64l
    }
    #[doc = "0x100..0x108 - Timer Load Value Register"]
    #[inline(always)]
    pub const fn ldval(&self, n: usize) -> &Ldval {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x108 - Timer Load Value Register"]
    #[inline(always)]
    pub fn ldval_iter(&self) -> impl Iterator<Item = &Ldval> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(256)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x104..0x10c - Current Timer Value Register"]
    #[inline(always)]
    pub const fn cval(&self, n: usize) -> &Cval {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x10c - Current Timer Value Register"]
    #[inline(always)]
    pub fn cval_iter(&self) -> impl Iterator<Item = &Cval> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(260)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x108..0x110 - Timer Control Register"]
    #[inline(always)]
    pub const fn tctrl(&self, n: usize) -> &Tctrl {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(264)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x108..0x110 - Timer Control Register"]
    #[inline(always)]
    pub fn tctrl_iter(&self) -> impl Iterator<Item = &Tctrl> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(264)
                .add(16 * n)
                .cast()
        })
    }
    #[doc = "0x10c..0x114 - Timer Flag Register"]
    #[inline(always)]
    pub const fn tflg(&self, n: usize) -> &Tflg {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(268)
                .add(16 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10c..0x114 - Timer Flag Register"]
    #[inline(always)]
    pub fn tflg_iter(&self) -> impl Iterator<Item = &Tflg> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(268)
                .add(16 * n)
                .cast()
        })
    }
}
#[doc = "MCR (rw) register accessor: PIT Module Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "PIT Module Control Register"]
pub mod mcr;
#[doc = "LTMR64H (r) register accessor: PIT Upper Lifetime Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ltmr64h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltmr64h`]
module"]
#[doc(alias = "LTMR64H")]
pub type Ltmr64h = crate::Reg<ltmr64h::Ltmr64hSpec>;
#[doc = "PIT Upper Lifetime Timer Register"]
pub mod ltmr64h;
#[doc = "LTMR64L (r) register accessor: PIT Lower Lifetime Timer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ltmr64l::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltmr64l`]
module"]
#[doc(alias = "LTMR64L")]
pub type Ltmr64l = crate::Reg<ltmr64l::Ltmr64lSpec>;
#[doc = "PIT Lower Lifetime Timer Register"]
pub mod ltmr64l;
#[doc = "LDVAL (rw) register accessor: Timer Load Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ldval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ldval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ldval`]
module"]
#[doc(alias = "LDVAL")]
pub type Ldval = crate::Reg<ldval::LdvalSpec>;
#[doc = "Timer Load Value Register"]
pub mod ldval;
#[doc = "CVAL (r) register accessor: Current Timer Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cval::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cval`]
module"]
#[doc(alias = "CVAL")]
pub type Cval = crate::Reg<cval::CvalSpec>;
#[doc = "Current Timer Value Register"]
pub mod cval;
#[doc = "TCTRL (rw) register accessor: Timer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tctrl`]
module"]
#[doc(alias = "TCTRL")]
pub type Tctrl = crate::Reg<tctrl::TctrlSpec>;
#[doc = "Timer Control Register"]
pub mod tctrl;
#[doc = "TFLG (rw) register accessor: Timer Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tflg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tflg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tflg`]
module"]
#[doc(alias = "TFLG")]
pub type Tflg = crate::Reg<tflg::TflgSpec>;
#[doc = "Timer Flag Register"]
pub mod tflg;
