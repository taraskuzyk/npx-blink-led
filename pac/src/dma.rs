#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    sar0: Sar0,
    dar0: Dar0,
    _reserved_2_dsr0: [u8; 0x04],
    dcr0: Dcr0,
    sar1: Sar1,
    dar1: Dar1,
    _reserved_6_dsr1: [u8; 0x04],
    dcr1: Dcr1,
    sar2: Sar2,
    dar2: Dar2,
    _reserved_10_dsr2: [u8; 0x04],
    dcr2: Dcr2,
    sar3: Sar3,
    dar3: Dar3,
    _reserved_14_dsr3: [u8; 0x04],
    dcr3: Dcr3,
}
impl RegisterBlock {
    #[doc = "0x100 - Source Address Register"]
    #[inline(always)]
    pub const fn sar0(&self) -> &Sar0 {
        &self.sar0
    }
    #[doc = "0x104 - Destination Address Register"]
    #[inline(always)]
    pub const fn dar0(&self) -> &Dar0 {
        &self.dar0
    }
    #[doc = "0x108 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub const fn dsr_bcr0(&self) -> &DsrBcr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(264).cast() }
    }
    #[doc = "0x10b - DMA_DSR0 register."]
    #[inline(always)]
    pub const fn dsr0(&self) -> &Dsr0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(267).cast() }
    }
    #[doc = "0x10c - DMA Control Register"]
    #[inline(always)]
    pub const fn dcr0(&self) -> &Dcr0 {
        &self.dcr0
    }
    #[doc = "0x110 - Source Address Register"]
    #[inline(always)]
    pub const fn sar1(&self) -> &Sar1 {
        &self.sar1
    }
    #[doc = "0x114 - Destination Address Register"]
    #[inline(always)]
    pub const fn dar1(&self) -> &Dar1 {
        &self.dar1
    }
    #[doc = "0x118 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub const fn dsr_bcr1(&self) -> &DsrBcr1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(280).cast() }
    }
    #[doc = "0x11b - DMA_DSR1 register."]
    #[inline(always)]
    pub const fn dsr1(&self) -> &Dsr1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(283).cast() }
    }
    #[doc = "0x11c - DMA Control Register"]
    #[inline(always)]
    pub const fn dcr1(&self) -> &Dcr1 {
        &self.dcr1
    }
    #[doc = "0x120 - Source Address Register"]
    #[inline(always)]
    pub const fn sar2(&self) -> &Sar2 {
        &self.sar2
    }
    #[doc = "0x124 - Destination Address Register"]
    #[inline(always)]
    pub const fn dar2(&self) -> &Dar2 {
        &self.dar2
    }
    #[doc = "0x128 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub const fn dsr_bcr2(&self) -> &DsrBcr2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(296).cast() }
    }
    #[doc = "0x12b - DMA_DSR2 register."]
    #[inline(always)]
    pub const fn dsr2(&self) -> &Dsr2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(299).cast() }
    }
    #[doc = "0x12c - DMA Control Register"]
    #[inline(always)]
    pub const fn dcr2(&self) -> &Dcr2 {
        &self.dcr2
    }
    #[doc = "0x130 - Source Address Register"]
    #[inline(always)]
    pub const fn sar3(&self) -> &Sar3 {
        &self.sar3
    }
    #[doc = "0x134 - Destination Address Register"]
    #[inline(always)]
    pub const fn dar3(&self) -> &Dar3 {
        &self.dar3
    }
    #[doc = "0x138 - DMA Status Register / Byte Count Register"]
    #[inline(always)]
    pub const fn dsr_bcr3(&self) -> &DsrBcr3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(312).cast() }
    }
    #[doc = "0x13b - DMA_DSR3 register."]
    #[inline(always)]
    pub const fn dsr3(&self) -> &Dsr3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(315).cast() }
    }
    #[doc = "0x13c - DMA Control Register"]
    #[inline(always)]
    pub const fn dcr3(&self) -> &Dcr3 {
        &self.dcr3
    }
}
#[doc = "SAR0 (rw) register accessor: Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar0`]
module"]
#[doc(alias = "SAR0")]
pub type Sar0 = crate::Reg<sar0::Sar0Spec>;
#[doc = "Source Address Register"]
pub mod sar0;
#[doc = "DAR0 (rw) register accessor: Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar0`]
module"]
#[doc(alias = "DAR0")]
pub type Dar0 = crate::Reg<dar0::Dar0Spec>;
#[doc = "Destination Address Register"]
pub mod dar0;
#[doc = "DSR_BCR0 (rw) register accessor: DMA Status Register / Byte Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr_bcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr_bcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr_bcr0`]
module"]
#[doc(alias = "DSR_BCR0")]
pub type DsrBcr0 = crate::Reg<dsr_bcr0::DsrBcr0Spec>;
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr0;
#[doc = "DSR0 (rw) register accessor: DMA_DSR0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr0`]
module"]
#[doc(alias = "DSR0")]
pub type Dsr0 = crate::Reg<dsr0::Dsr0Spec>;
#[doc = "DMA_DSR0 register."]
pub mod dsr0;
#[doc = "DCR0 (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr0`]
module"]
#[doc(alias = "DCR0")]
pub type Dcr0 = crate::Reg<dcr0::Dcr0Spec>;
#[doc = "DMA Control Register"]
pub mod dcr0;
#[doc = "SAR1 (rw) register accessor: Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar1`]
module"]
#[doc(alias = "SAR1")]
pub type Sar1 = crate::Reg<sar1::Sar1Spec>;
#[doc = "Source Address Register"]
pub mod sar1;
#[doc = "DAR1 (rw) register accessor: Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar1`]
module"]
#[doc(alias = "DAR1")]
pub type Dar1 = crate::Reg<dar1::Dar1Spec>;
#[doc = "Destination Address Register"]
pub mod dar1;
#[doc = "DSR_BCR1 (rw) register accessor: DMA Status Register / Byte Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr_bcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr_bcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr_bcr1`]
module"]
#[doc(alias = "DSR_BCR1")]
pub type DsrBcr1 = crate::Reg<dsr_bcr1::DsrBcr1Spec>;
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr1;
#[doc = "DSR1 (rw) register accessor: DMA_DSR1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr1`]
module"]
#[doc(alias = "DSR1")]
pub type Dsr1 = crate::Reg<dsr1::Dsr1Spec>;
#[doc = "DMA_DSR1 register."]
pub mod dsr1;
#[doc = "DCR1 (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr1`]
module"]
#[doc(alias = "DCR1")]
pub type Dcr1 = crate::Reg<dcr1::Dcr1Spec>;
#[doc = "DMA Control Register"]
pub mod dcr1;
#[doc = "SAR2 (rw) register accessor: Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar2`]
module"]
#[doc(alias = "SAR2")]
pub type Sar2 = crate::Reg<sar2::Sar2Spec>;
#[doc = "Source Address Register"]
pub mod sar2;
#[doc = "DAR2 (rw) register accessor: Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar2`]
module"]
#[doc(alias = "DAR2")]
pub type Dar2 = crate::Reg<dar2::Dar2Spec>;
#[doc = "Destination Address Register"]
pub mod dar2;
#[doc = "DSR_BCR2 (rw) register accessor: DMA Status Register / Byte Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr_bcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr_bcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr_bcr2`]
module"]
#[doc(alias = "DSR_BCR2")]
pub type DsrBcr2 = crate::Reg<dsr_bcr2::DsrBcr2Spec>;
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr2;
#[doc = "DSR2 (rw) register accessor: DMA_DSR2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr2`]
module"]
#[doc(alias = "DSR2")]
pub type Dsr2 = crate::Reg<dsr2::Dsr2Spec>;
#[doc = "DMA_DSR2 register."]
pub mod dsr2;
#[doc = "DCR2 (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr2`]
module"]
#[doc(alias = "DCR2")]
pub type Dcr2 = crate::Reg<dcr2::Dcr2Spec>;
#[doc = "DMA Control Register"]
pub mod dcr2;
#[doc = "SAR3 (rw) register accessor: Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar3`]
module"]
#[doc(alias = "SAR3")]
pub type Sar3 = crate::Reg<sar3::Sar3Spec>;
#[doc = "Source Address Register"]
pub mod sar3;
#[doc = "DAR3 (rw) register accessor: Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar3`]
module"]
#[doc(alias = "DAR3")]
pub type Dar3 = crate::Reg<dar3::Dar3Spec>;
#[doc = "Destination Address Register"]
pub mod dar3;
#[doc = "DSR_BCR3 (rw) register accessor: DMA Status Register / Byte Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr_bcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr_bcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr_bcr3`]
module"]
#[doc(alias = "DSR_BCR3")]
pub type DsrBcr3 = crate::Reg<dsr_bcr3::DsrBcr3Spec>;
#[doc = "DMA Status Register / Byte Count Register"]
pub mod dsr_bcr3;
#[doc = "DSR3 (rw) register accessor: DMA_DSR3 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsr3`]
module"]
#[doc(alias = "DSR3")]
pub type Dsr3 = crate::Reg<dsr3::Dsr3Spec>;
#[doc = "DMA_DSR3 register."]
pub mod dsr3;
#[doc = "DCR3 (rw) register accessor: DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr3`]
module"]
#[doc(alias = "DCR3")]
pub type Dcr3 = crate::Reg<dcr3::Dcr3Spec>;
#[doc = "DMA Control Register"]
pub mod dcr3;
