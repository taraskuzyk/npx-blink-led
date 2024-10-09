#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    c1: C1,
    c2: C2,
    _reserved2: [u8; 0x04],
    s: S,
    _reserved3: [u8; 0x01],
    sc: Sc,
    _reserved4: [u8; 0x0b],
    hctrim: Hctrim,
    httrim: Httrim,
    hftrim: Hftrim,
    _reserved7: [u8; 0x01],
    mc: Mc,
    ltrimrng: Ltrimrng,
    lftrim: Lftrim,
    lstrim: Lstrim,
}
impl RegisterBlock {
    #[doc = "0x00 - MCG Control Register 1"]
    #[inline(always)]
    pub const fn c1(&self) -> &C1 {
        &self.c1
    }
    #[doc = "0x01 - MCG Control Register 2"]
    #[inline(always)]
    pub const fn c2(&self) -> &C2 {
        &self.c2
    }
    #[doc = "0x06 - MCG Status Register"]
    #[inline(always)]
    pub const fn s(&self) -> &S {
        &self.s
    }
    #[doc = "0x08 - MCG Status and Control Register"]
    #[inline(always)]
    pub const fn sc(&self) -> &Sc {
        &self.sc
    }
    #[doc = "0x14 - MCG High-frequency IRC Coarse Trim Register"]
    #[inline(always)]
    pub const fn hctrim(&self) -> &Hctrim {
        &self.hctrim
    }
    #[doc = "0x15 - MCG High-frequency IRC Tempco (Temperature Coefficient) Trim Register"]
    #[inline(always)]
    pub const fn httrim(&self) -> &Httrim {
        &self.httrim
    }
    #[doc = "0x16 - MCG High-frequency IRC Fine Trim Register"]
    #[inline(always)]
    pub const fn hftrim(&self) -> &Hftrim {
        &self.hftrim
    }
    #[doc = "0x18 - MCG Miscellaneous Control Register"]
    #[inline(always)]
    pub const fn mc(&self) -> &Mc {
        &self.mc
    }
    #[doc = "0x19 - MCG Low-frequency IRC Trim Range Register"]
    #[inline(always)]
    pub const fn ltrimrng(&self) -> &Ltrimrng {
        &self.ltrimrng
    }
    #[doc = "0x1a - MCG Low-frequency IRC8M Trim Register"]
    #[inline(always)]
    pub const fn lftrim(&self) -> &Lftrim {
        &self.lftrim
    }
    #[doc = "0x1b - MCG Low-frequency IRC2M Trim Register"]
    #[inline(always)]
    pub const fn lstrim(&self) -> &Lstrim {
        &self.lstrim
    }
}
#[doc = "C1 (rw) register accessor: MCG Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`]
module"]
pub type C1 = crate::Reg<c1::C1Spec>;
#[doc = "MCG Control Register 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: MCG Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`]
module"]
pub type C2 = crate::Reg<c2::C2Spec>;
#[doc = "MCG Control Register 2"]
pub mod c2;
#[doc = "S (r) register accessor: MCG Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`s::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s`]
module"]
pub type S = crate::Reg<s::SSpec>;
#[doc = "MCG Status Register"]
pub mod s;
#[doc = "SC (rw) register accessor: MCG Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sc`]
module"]
#[doc(alias = "SC")]
pub type Sc = crate::Reg<sc::ScSpec>;
#[doc = "MCG Status and Control Register"]
pub mod sc;
#[doc = "HCTRIM (r) register accessor: MCG High-frequency IRC Coarse Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hctrim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctrim`]
module"]
#[doc(alias = "HCTRIM")]
pub type Hctrim = crate::Reg<hctrim::HctrimSpec>;
#[doc = "MCG High-frequency IRC Coarse Trim Register"]
pub mod hctrim;
#[doc = "HTTRIM (r) register accessor: MCG High-frequency IRC Tempco (Temperature Coefficient) Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`httrim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@httrim`]
module"]
#[doc(alias = "HTTRIM")]
pub type Httrim = crate::Reg<httrim::HttrimSpec>;
#[doc = "MCG High-frequency IRC Tempco (Temperature Coefficient) Trim Register"]
pub mod httrim;
#[doc = "HFTRIM (r) register accessor: MCG High-frequency IRC Fine Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hftrim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hftrim`]
module"]
#[doc(alias = "HFTRIM")]
pub type Hftrim = crate::Reg<hftrim::HftrimSpec>;
#[doc = "MCG High-frequency IRC Fine Trim Register"]
pub mod hftrim;
#[doc = "MC (rw) register accessor: MCG Miscellaneous Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mc`]
module"]
#[doc(alias = "MC")]
pub type Mc = crate::Reg<mc::McSpec>;
#[doc = "MCG Miscellaneous Control Register"]
pub mod mc;
#[doc = "LTRIMRNG (r) register accessor: MCG Low-frequency IRC Trim Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ltrimrng::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltrimrng`]
module"]
#[doc(alias = "LTRIMRNG")]
pub type Ltrimrng = crate::Reg<ltrimrng::LtrimrngSpec>;
#[doc = "MCG Low-frequency IRC Trim Range Register"]
pub mod ltrimrng;
#[doc = "LFTRIM (r) register accessor: MCG Low-frequency IRC8M Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lftrim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lftrim`]
module"]
#[doc(alias = "LFTRIM")]
pub type Lftrim = crate::Reg<lftrim::LftrimSpec>;
#[doc = "MCG Low-frequency IRC8M Trim Register"]
pub mod lftrim;
#[doc = "LSTRIM (r) register accessor: MCG Low-frequency IRC2M Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lstrim::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lstrim`]
module"]
#[doc(alias = "LSTRIM")]
pub type Lstrim = crate::Reg<lstrim::LstrimSpec>;
#[doc = "MCG Low-frequency IRC2M Trim Register"]
pub mod lstrim;
