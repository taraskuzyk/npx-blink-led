#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    bdh: Bdh,
    bdl: Bdl,
    c1: C1,
    c2: C2,
    s1: S1,
    s2: S2,
    c3: C3,
    d: D,
    ma1: Ma1,
    ma2: Ma2,
    c4: C4,
    c5: C5,
    _reserved12: [u8; 0x0c],
    c7816: C7816,
    ie7816: Ie7816,
    is7816: Is7816,
    wp7816: Wp7816,
    wn7816: Wn7816,
    wf7816: Wf7816,
    et7816: Et7816,
    tl7816: Tl7816,
    _reserved20: [u8; 0x1a],
    ap7816a_t0: Ap7816aT0,
    ap7816b_t0: Ap7816bT0,
    _reserved_22_uart2_wp7816a_t: [u8; 0x01],
    _reserved_23_uart2_wp7816b_t: [u8; 0x01],
    wgp7816_t1: Wgp7816T1,
    wp7816c_t1: Wp7816cT1,
}
impl RegisterBlock {
    #[doc = "0x00 - UART Baud Rate Registers: High"]
    #[inline(always)]
    pub const fn bdh(&self) -> &Bdh {
        &self.bdh
    }
    #[doc = "0x01 - UART Baud Rate Registers: Low"]
    #[inline(always)]
    pub const fn bdl(&self) -> &Bdl {
        &self.bdl
    }
    #[doc = "0x02 - UART Control Register 1"]
    #[inline(always)]
    pub const fn c1(&self) -> &C1 {
        &self.c1
    }
    #[doc = "0x03 - UART Control Register 2"]
    #[inline(always)]
    pub const fn c2(&self) -> &C2 {
        &self.c2
    }
    #[doc = "0x04 - UART Status Register 1"]
    #[inline(always)]
    pub const fn s1(&self) -> &S1 {
        &self.s1
    }
    #[doc = "0x05 - UART Status Register 2"]
    #[inline(always)]
    pub const fn s2(&self) -> &S2 {
        &self.s2
    }
    #[doc = "0x06 - UART Control Register 3"]
    #[inline(always)]
    pub const fn c3(&self) -> &C3 {
        &self.c3
    }
    #[doc = "0x07 - UART Data Register"]
    #[inline(always)]
    pub const fn d(&self) -> &D {
        &self.d
    }
    #[doc = "0x08 - UART Match Address Registers 1"]
    #[inline(always)]
    pub const fn ma1(&self) -> &Ma1 {
        &self.ma1
    }
    #[doc = "0x09 - UART Match Address Registers 2"]
    #[inline(always)]
    pub const fn ma2(&self) -> &Ma2 {
        &self.ma2
    }
    #[doc = "0x0a - UART Control Register 4"]
    #[inline(always)]
    pub const fn c4(&self) -> &C4 {
        &self.c4
    }
    #[doc = "0x0b - UART Control Register 5"]
    #[inline(always)]
    pub const fn c5(&self) -> &C5 {
        &self.c5
    }
    #[doc = "0x18 - UART 7816 Control Register"]
    #[inline(always)]
    pub const fn c7816(&self) -> &C7816 {
        &self.c7816
    }
    #[doc = "0x19 - UART 7816 Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie7816(&self) -> &Ie7816 {
        &self.ie7816
    }
    #[doc = "0x1a - UART 7816 Interrupt Status Register"]
    #[inline(always)]
    pub const fn is7816(&self) -> &Is7816 {
        &self.is7816
    }
    #[doc = "0x1b - UART 7816 Wait Parameter Register"]
    #[inline(always)]
    pub const fn wp7816(&self) -> &Wp7816 {
        &self.wp7816
    }
    #[doc = "0x1c - UART 7816 Wait N Register"]
    #[inline(always)]
    pub const fn wn7816(&self) -> &Wn7816 {
        &self.wn7816
    }
    #[doc = "0x1d - UART 7816 Wait FD Register"]
    #[inline(always)]
    pub const fn wf7816(&self) -> &Wf7816 {
        &self.wf7816
    }
    #[doc = "0x1e - UART 7816 Error Threshold Register"]
    #[inline(always)]
    pub const fn et7816(&self) -> &Et7816 {
        &self.et7816
    }
    #[doc = "0x1f - UART 7816 Transmit Length Register"]
    #[inline(always)]
    pub const fn tl7816(&self) -> &Tl7816 {
        &self.tl7816
    }
    #[doc = "0x3a - UART 7816 ATR Duration Timer Register A"]
    #[inline(always)]
    pub const fn ap7816a_t0(&self) -> &Ap7816aT0 {
        &self.ap7816a_t0
    }
    #[doc = "0x3b - UART 7816 ATR Duration Timer Register B"]
    #[inline(always)]
    pub const fn ap7816b_t0(&self) -> &Ap7816bT0 {
        &self.ap7816b_t0
    }
    #[doc = "0x3c - UART 7816 Wait Parameter Register A"]
    #[inline(always)]
    pub const fn uart2_wp7816a_t1(&self) -> &Uart2Wp7816aT1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - UART 7816 Wait Parameter Register A"]
    #[inline(always)]
    pub const fn uart2_wp7816a_t0(&self) -> &Uart2Wp7816aT0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3d - UART 7816 Wait Parameter Register B"]
    #[inline(always)]
    pub const fn uart2_wp7816b_t1(&self) -> &Uart2Wp7816bT1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(61).cast() }
    }
    #[doc = "0x3d - UART 7816 Wait Parameter Register B"]
    #[inline(always)]
    pub const fn uart2_wp7816b_t0(&self) -> &Uart2Wp7816bT0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(61).cast() }
    }
    #[doc = "0x3e - UART 7816 Wait and Guard Parameter Register"]
    #[inline(always)]
    pub const fn wgp7816_t1(&self) -> &Wgp7816T1 {
        &self.wgp7816_t1
    }
    #[doc = "0x3f - UART 7816 Wait Parameter Register C"]
    #[inline(always)]
    pub const fn wp7816c_t1(&self) -> &Wp7816cT1 {
        &self.wp7816c_t1
    }
}
#[doc = "BDH (rw) register accessor: UART Baud Rate Registers: High\n\nYou can [`read`](crate::Reg::read) this register and get [`bdh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdh`]
module"]
#[doc(alias = "BDH")]
pub type Bdh = crate::Reg<bdh::BdhSpec>;
#[doc = "UART Baud Rate Registers: High"]
pub mod bdh;
#[doc = "BDL (rw) register accessor: UART Baud Rate Registers: Low\n\nYou can [`read`](crate::Reg::read) this register and get [`bdl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdl`]
module"]
#[doc(alias = "BDL")]
pub type Bdl = crate::Reg<bdl::BdlSpec>;
#[doc = "UART Baud Rate Registers: Low"]
pub mod bdl;
#[doc = "C1 (rw) register accessor: UART Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`c1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1`]
module"]
pub type C1 = crate::Reg<c1::C1Spec>;
#[doc = "UART Control Register 1"]
pub mod c1;
#[doc = "C2 (rw) register accessor: UART Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`c2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2`]
module"]
pub type C2 = crate::Reg<c2::C2Spec>;
#[doc = "UART Control Register 2"]
pub mod c2;
#[doc = "S1 (r) register accessor: UART Status Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`s1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s1`]
module"]
pub type S1 = crate::Reg<s1::S1Spec>;
#[doc = "UART Status Register 1"]
pub mod s1;
#[doc = "S2 (rw) register accessor: UART Status Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`s2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s2`]
module"]
pub type S2 = crate::Reg<s2::S2Spec>;
#[doc = "UART Status Register 2"]
pub mod s2;
#[doc = "C3 (rw) register accessor: UART Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`c3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3`]
module"]
pub type C3 = crate::Reg<c3::C3Spec>;
#[doc = "UART Control Register 3"]
pub mod c3;
#[doc = "D (rw) register accessor: UART Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d`]
module"]
pub type D = crate::Reg<d::DSpec>;
#[doc = "UART Data Register"]
pub mod d;
#[doc = "MA1 (rw) register accessor: UART Match Address Registers 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ma1`]
module"]
#[doc(alias = "MA1")]
pub type Ma1 = crate::Reg<ma1::Ma1Spec>;
#[doc = "UART Match Address Registers 1"]
pub mod ma1;
#[doc = "MA2 (rw) register accessor: UART Match Address Registers 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ma2`]
module"]
#[doc(alias = "MA2")]
pub type Ma2 = crate::Reg<ma2::Ma2Spec>;
#[doc = "UART Match Address Registers 2"]
pub mod ma2;
#[doc = "C4 (rw) register accessor: UART Control Register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4`]
module"]
pub type C4 = crate::Reg<c4::C4Spec>;
#[doc = "UART Control Register 4"]
pub mod c4;
#[doc = "C5 (rw) register accessor: UART Control Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`c5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5`]
module"]
pub type C5 = crate::Reg<c5::C5Spec>;
#[doc = "UART Control Register 5"]
pub mod c5;
#[doc = "C7816 (rw) register accessor: UART 7816 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7816`]
module"]
pub type C7816 = crate::Reg<c7816::C7816Spec>;
#[doc = "UART 7816 Control Register"]
pub mod c7816;
#[doc = "IE7816 (rw) register accessor: UART 7816 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie7816`]
module"]
#[doc(alias = "IE7816")]
pub type Ie7816 = crate::Reg<ie7816::Ie7816Spec>;
#[doc = "UART 7816 Interrupt Enable Register"]
pub mod ie7816;
#[doc = "IS7816 (rw) register accessor: UART 7816 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`is7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`is7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@is7816`]
module"]
#[doc(alias = "IS7816")]
pub type Is7816 = crate::Reg<is7816::Is7816Spec>;
#[doc = "UART 7816 Interrupt Status Register"]
pub mod is7816;
#[doc = "WP7816 (rw) register accessor: UART 7816 Wait Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wp7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wp7816`]
module"]
#[doc(alias = "WP7816")]
pub type Wp7816 = crate::Reg<wp7816::Wp7816Spec>;
#[doc = "UART 7816 Wait Parameter Register"]
pub mod wp7816;
#[doc = "WN7816 (rw) register accessor: UART 7816 Wait N Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wn7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wn7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wn7816`]
module"]
#[doc(alias = "WN7816")]
pub type Wn7816 = crate::Reg<wn7816::Wn7816Spec>;
#[doc = "UART 7816 Wait N Register"]
pub mod wn7816;
#[doc = "WF7816 (rw) register accessor: UART 7816 Wait FD Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wf7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf7816`]
module"]
#[doc(alias = "WF7816")]
pub type Wf7816 = crate::Reg<wf7816::Wf7816Spec>;
#[doc = "UART 7816 Wait FD Register"]
pub mod wf7816;
#[doc = "ET7816 (rw) register accessor: UART 7816 Error Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`et7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`et7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@et7816`]
module"]
#[doc(alias = "ET7816")]
pub type Et7816 = crate::Reg<et7816::Et7816Spec>;
#[doc = "UART 7816 Error Threshold Register"]
pub mod et7816;
#[doc = "TL7816 (rw) register accessor: UART 7816 Transmit Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tl7816::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tl7816::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tl7816`]
module"]
#[doc(alias = "TL7816")]
pub type Tl7816 = crate::Reg<tl7816::Tl7816Spec>;
#[doc = "UART 7816 Transmit Length Register"]
pub mod tl7816;
#[doc = "AP7816A_T0 (rw) register accessor: UART 7816 ATR Duration Timer Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`ap7816a_t0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ap7816a_t0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ap7816a_t0`]
module"]
#[doc(alias = "AP7816A_T0")]
pub type Ap7816aT0 = crate::Reg<ap7816a_t0::Ap7816aT0Spec>;
#[doc = "UART 7816 ATR Duration Timer Register A"]
pub mod ap7816a_t0;
#[doc = "AP7816B_T0 (rw) register accessor: UART 7816 ATR Duration Timer Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`ap7816b_t0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ap7816b_t0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ap7816b_t0`]
module"]
#[doc(alias = "AP7816B_T0")]
pub type Ap7816bT0 = crate::Reg<ap7816b_t0::Ap7816bT0Spec>;
#[doc = "UART 7816 ATR Duration Timer Register B"]
pub mod ap7816b_t0;
#[doc = "UART2_WP7816A_T0 (rw) register accessor: UART 7816 Wait Parameter Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_wp7816a_t0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_wp7816a_t0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_wp7816a_t0`]
module"]
#[doc(alias = "UART2_WP7816A_T0")]
pub type Uart2Wp7816aT0 = crate::Reg<uart2_wp7816a_t0::Uart2Wp7816aT0Spec>;
#[doc = "UART 7816 Wait Parameter Register A"]
pub mod uart2_wp7816a_t0;
#[doc = "UART2_WP7816A_T1 (rw) register accessor: UART 7816 Wait Parameter Register A\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_wp7816a_t1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_wp7816a_t1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_wp7816a_t1`]
module"]
#[doc(alias = "UART2_WP7816A_T1")]
pub type Uart2Wp7816aT1 = crate::Reg<uart2_wp7816a_t1::Uart2Wp7816aT1Spec>;
#[doc = "UART 7816 Wait Parameter Register A"]
pub mod uart2_wp7816a_t1;
#[doc = "UART2_WP7816B_T0 (rw) register accessor: UART 7816 Wait Parameter Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_wp7816b_t0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_wp7816b_t0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_wp7816b_t0`]
module"]
#[doc(alias = "UART2_WP7816B_T0")]
pub type Uart2Wp7816bT0 = crate::Reg<uart2_wp7816b_t0::Uart2Wp7816bT0Spec>;
#[doc = "UART 7816 Wait Parameter Register B"]
pub mod uart2_wp7816b_t0;
#[doc = "UART2_WP7816B_T1 (rw) register accessor: UART 7816 Wait Parameter Register B\n\nYou can [`read`](crate::Reg::read) this register and get [`uart2_wp7816b_t1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uart2_wp7816b_t1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@uart2_wp7816b_t1`]
module"]
#[doc(alias = "UART2_WP7816B_T1")]
pub type Uart2Wp7816bT1 = crate::Reg<uart2_wp7816b_t1::Uart2Wp7816bT1Spec>;
#[doc = "UART 7816 Wait Parameter Register B"]
pub mod uart2_wp7816b_t1;
#[doc = "WGP7816_T1 (rw) register accessor: UART 7816 Wait and Guard Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wgp7816_t1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wgp7816_t1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wgp7816_t1`]
module"]
#[doc(alias = "WGP7816_T1")]
pub type Wgp7816T1 = crate::Reg<wgp7816_t1::Wgp7816T1Spec>;
#[doc = "UART 7816 Wait and Guard Parameter Register"]
pub mod wgp7816_t1;
#[doc = "WP7816C_T1 (rw) register accessor: UART 7816 Wait Parameter Register C\n\nYou can [`read`](crate::Reg::read) this register and get [`wp7816c_t1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wp7816c_t1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wp7816c_t1`]
module"]
#[doc(alias = "WP7816C_T1")]
pub type Wp7816cT1 = crate::Reg<wp7816c_t1::Wp7816cT1Spec>;
#[doc = "UART 7816 Wait Parameter Register C"]
pub mod wp7816c_t1;
