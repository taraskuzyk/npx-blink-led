#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    chcfg: [Chcfg; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Channel Configuration register"]
    #[inline(always)]
    pub const fn chcfg(&self, n: usize) -> &Chcfg {
        &self.chcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00 - Channel Configuration register"]
    #[inline(always)]
    pub fn chcfg_iter(&self) -> impl Iterator<Item = &Chcfg> {
        self.chcfg.iter()
    }
}
#[doc = "CHCFG (rw) register accessor: Channel Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`chcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chcfg`]
module"]
#[doc(alias = "CHCFG")]
pub type Chcfg = crate::Reg<chcfg::ChcfgSpec>;
#[doc = "Channel Configuration register"]
pub mod chcfg;
