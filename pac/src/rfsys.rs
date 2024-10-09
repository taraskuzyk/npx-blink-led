#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    reg: [Reg; 8],
}
impl RegisterBlock {
    #[doc = "0x00..0x20 - Register file register"]
    #[inline(always)]
    pub const fn reg(&self, n: usize) -> &Reg {
        &self.reg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Register file register"]
    #[inline(always)]
    pub fn reg_iter(&self) -> impl Iterator<Item = &Reg> {
        self.reg.iter()
    }
}
#[doc = "REG (rw) register accessor: Register file register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reg`]
module"]
#[doc(alias = "REG")]
pub type Reg = crate::Reg<reg::RegSpec>;
#[doc = "Register file register"]
pub mod reg;
