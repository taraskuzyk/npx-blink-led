#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    lvdsc1: Lvdsc1,
    lvdsc2: Lvdsc2,
    regsc: Regsc,
}
impl RegisterBlock {
    #[doc = "0x00 - Low Voltage Detect Status And Control 1 register"]
    #[inline(always)]
    pub const fn lvdsc1(&self) -> &Lvdsc1 {
        &self.lvdsc1
    }
    #[doc = "0x01 - Low Voltage Detect Status And Control 2 register"]
    #[inline(always)]
    pub const fn lvdsc2(&self) -> &Lvdsc2 {
        &self.lvdsc2
    }
    #[doc = "0x02 - Regulator Status And Control register"]
    #[inline(always)]
    pub const fn regsc(&self) -> &Regsc {
        &self.regsc
    }
}
#[doc = "LVDSC1 (rw) register accessor: Low Voltage Detect Status And Control 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdsc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdsc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdsc1`]
module"]
#[doc(alias = "LVDSC1")]
pub type Lvdsc1 = crate::Reg<lvdsc1::Lvdsc1Spec>;
#[doc = "Low Voltage Detect Status And Control 1 register"]
pub mod lvdsc1;
#[doc = "LVDSC2 (rw) register accessor: Low Voltage Detect Status And Control 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`lvdsc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdsc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lvdsc2`]
module"]
#[doc(alias = "LVDSC2")]
pub type Lvdsc2 = crate::Reg<lvdsc2::Lvdsc2Spec>;
#[doc = "Low Voltage Detect Status And Control 2 register"]
pub mod lvdsc2;
#[doc = "REGSC (rw) register accessor: Regulator Status And Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`regsc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regsc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@regsc`]
module"]
#[doc(alias = "REGSC")]
pub type Regsc = crate::Reg<regsc::RegscSpec>;
#[doc = "Regulator Status And Control register"]
pub mod regsc;
