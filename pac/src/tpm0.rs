#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sc: Sc,
    cnt: Cnt,
    mod_: Mod,
    csc: (),
    _reserved4: [u8; 0x04],
    cv: (),
    _reserved5: [u8; 0x40],
    status: Status,
    _reserved6: [u8; 0x1c],
    pol: Pol,
    _reserved7: [u8; 0x10],
    conf: Conf,
}
impl RegisterBlock {
    #[doc = "0x00 - Status and Control"]
    #[inline(always)]
    pub const fn sc(&self) -> &Sc {
        &self.sc
    }
    #[doc = "0x04 - Counter"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x08 - Modulo"]
    #[inline(always)]
    pub const fn mod_(&self) -> &Mod {
        &self.mod_
    }
    #[doc = "0x0c..0x24 - Channel (n) Status and Control"]
    #[inline(always)]
    pub const fn csc(&self, n: usize) -> &Csc {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(12).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x24 - Channel (n) Status and Control"]
    #[inline(always)]
    pub fn csc_iter(&self) -> impl Iterator<Item = &Csc> {
        (0..6)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(12).add(8 * n).cast() })
    }
    #[doc = "0x0c - Channel (n) Status and Control"]
    #[inline(always)]
    pub const fn c0sc(&self) -> &Csc {
        self.csc(0)
    }
    #[doc = "0x14 - Channel (n) Status and Control"]
    #[inline(always)]
    pub const fn c1sc(&self) -> &Csc {
        self.csc(1)
    }
    #[doc = "0x1c - Channel (n) Status and Control"]
    #[inline(always)]
    pub const fn c2sc(&self) -> &Csc {
        self.csc(2)
    }
    #[doc = "0x24 - Channel (n) Status and Control"]
    #[inline(always)]
    pub const fn c3sc(&self) -> &Csc {
        self.csc(3)
    }
    #[doc = "0x2c - Channel (n) Status and Control"]
    #[inline(always)]
    pub const fn c4sc(&self) -> &Csc {
        self.csc(4)
    }
    #[doc = "0x34 - Channel (n) Status and Control"]
    #[inline(always)]
    pub const fn c5sc(&self) -> &Csc {
        self.csc(5)
    }
    #[doc = "0x10..0x28 - Channel (n) Value"]
    #[inline(always)]
    pub const fn cv(&self, n: usize) -> &Cv {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(16).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x28 - Channel (n) Value"]
    #[inline(always)]
    pub fn cv_iter(&self) -> impl Iterator<Item = &Cv> {
        (0..6)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(16).add(8 * n).cast() })
    }
    #[doc = "0x10 - Channel (n) Value"]
    #[inline(always)]
    pub const fn c0v(&self) -> &Cv {
        self.cv(0)
    }
    #[doc = "0x18 - Channel (n) Value"]
    #[inline(always)]
    pub const fn c1v(&self) -> &Cv {
        self.cv(1)
    }
    #[doc = "0x20 - Channel (n) Value"]
    #[inline(always)]
    pub const fn c2v(&self) -> &Cv {
        self.cv(2)
    }
    #[doc = "0x28 - Channel (n) Value"]
    #[inline(always)]
    pub const fn c3v(&self) -> &Cv {
        self.cv(3)
    }
    #[doc = "0x30 - Channel (n) Value"]
    #[inline(always)]
    pub const fn c4v(&self) -> &Cv {
        self.cv(4)
    }
    #[doc = "0x38 - Channel (n) Value"]
    #[inline(always)]
    pub const fn c5v(&self) -> &Cv {
        self.cv(5)
    }
    #[doc = "0x50 - Capture and Compare Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x70 - Channel Polarity"]
    #[inline(always)]
    pub const fn pol(&self) -> &Pol {
        &self.pol
    }
    #[doc = "0x84 - Configuration"]
    #[inline(always)]
    pub const fn conf(&self) -> &Conf {
        &self.conf
    }
}
#[doc = "SC (rw) register accessor: Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`sc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc`]
module"]
#[doc(alias = "SC")]
pub type Sc = crate::Reg<sc::ScSpec>;
#[doc = "Status and Control"]
pub mod sc;
#[doc = "CNT (rw) register accessor: Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`]
module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "Counter"]
pub mod cnt;
#[doc = "MOD (rw) register accessor: Modulo\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mod_`]
module"]
#[doc(alias = "MOD")]
pub type Mod = crate::Reg<mod_::ModSpec>;
#[doc = "Modulo"]
pub mod mod_;
#[doc = "CSC (rw) register accessor: Channel (n) Status and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`csc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csc`]
module"]
#[doc(alias = "CSC")]
pub type Csc = crate::Reg<csc::CscSpec>;
#[doc = "Channel (n) Status and Control"]
pub mod csc;
#[doc = "CV (rw) register accessor: Channel (n) Value\n\nYou can [`read`](crate::Reg::read) this register and get [`cv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cv`]
module"]
#[doc(alias = "CV")]
pub type Cv = crate::Reg<cv::CvSpec>;
#[doc = "Channel (n) Value"]
pub mod cv;
#[doc = "STATUS (rw) register accessor: Capture and Compare Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Capture and Compare Status"]
pub mod status;
#[doc = "POL (rw) register accessor: Channel Polarity\n\nYou can [`read`](crate::Reg::read) this register and get [`pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pol`]
module"]
#[doc(alias = "POL")]
pub type Pol = crate::Reg<pol::PolSpec>;
#[doc = "Channel Polarity"]
pub mod pol;
#[doc = "CONF (rw) register accessor: Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@conf`]
module"]
#[doc(alias = "CONF")]
pub type Conf = crate::Reg<conf::ConfSpec>;
#[doc = "Configuration"]
pub mod conf;
