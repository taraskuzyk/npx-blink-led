#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fstat: Fstat,
    fcnfg: Fcnfg,
    fsec: Fsec,
    fopt: Fopt,
    fccob: [Fccob; 12],
    fprot: [Fprot; 4],
}
impl RegisterBlock {
    #[doc = "0x00 - Flash Status Register"]
    #[inline(always)]
    pub const fn fstat(&self) -> &Fstat {
        &self.fstat
    }
    #[doc = "0x01 - Flash Configuration Register"]
    #[inline(always)]
    pub const fn fcnfg(&self) -> &Fcnfg {
        &self.fcnfg
    }
    #[doc = "0x02 - Flash Security Register"]
    #[inline(always)]
    pub const fn fsec(&self) -> &Fsec {
        &self.fsec
    }
    #[doc = "0x03 - Flash Option Register"]
    #[inline(always)]
    pub const fn fopt(&self) -> &Fopt {
        &self.fopt
    }
    #[doc = "0x04..0x10 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob(&self, n: usize) -> &Fccob {
        &self.fccob[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x10 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob_iter(&self) -> impl Iterator<Item = &Fccob> {
        self.fccob.iter()
    }
    #[doc = "0x04 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob3(&self) -> &Fccob {
        self.fccob(0)
    }
    #[doc = "0x05 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob2(&self) -> &Fccob {
        self.fccob(1)
    }
    #[doc = "0x06 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob1(&self) -> &Fccob {
        self.fccob(2)
    }
    #[doc = "0x07 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob0(&self) -> &Fccob {
        self.fccob(3)
    }
    #[doc = "0x08 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob7(&self) -> &Fccob {
        self.fccob(4)
    }
    #[doc = "0x09 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob6(&self) -> &Fccob {
        self.fccob(5)
    }
    #[doc = "0x0a - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob5(&self) -> &Fccob {
        self.fccob(6)
    }
    #[doc = "0x0b - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob4(&self) -> &Fccob {
        self.fccob(7)
    }
    #[doc = "0x0c - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccobb(&self) -> &Fccob {
        self.fccob(8)
    }
    #[doc = "0x0d - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccoba(&self) -> &Fccob {
        self.fccob(9)
    }
    #[doc = "0x0e - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob9(&self) -> &Fccob {
        self.fccob(10)
    }
    #[doc = "0x0f - Flash Common Command Object Registers"]
    #[inline(always)]
    pub const fn fccob8(&self) -> &Fccob {
        self.fccob(11)
    }
    #[doc = "0x10 - Program Flash Protection Registers"]
    #[inline(always)]
    pub const fn fprot(&self, n: usize) -> &Fprot {
        &self.fprot[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10 - Program Flash Protection Registers"]
    #[inline(always)]
    pub fn fprot_iter(&self) -> impl Iterator<Item = &Fprot> {
        self.fprot.iter()
    }
    #[doc = "0x10 - Program Flash Protection Registers"]
    #[inline(always)]
    pub const fn fprot3(&self) -> &Fprot {
        self.fprot(0)
    }
    #[doc = "0x11 - Program Flash Protection Registers"]
    #[inline(always)]
    pub const fn fprot2(&self) -> &Fprot {
        self.fprot(1)
    }
    #[doc = "0x12 - Program Flash Protection Registers"]
    #[inline(always)]
    pub const fn fprot1(&self) -> &Fprot {
        self.fprot(2)
    }
    #[doc = "0x13 - Program Flash Protection Registers"]
    #[inline(always)]
    pub const fn fprot0(&self) -> &Fprot {
        self.fprot(3)
    }
}
#[doc = "FSTAT (rw) register accessor: Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fstat`]
module"]
#[doc(alias = "FSTAT")]
pub type Fstat = crate::Reg<fstat::FstatSpec>;
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "FCNFG (rw) register accessor: Flash Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcnfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcnfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcnfg`]
module"]
#[doc(alias = "FCNFG")]
pub type Fcnfg = crate::Reg<fcnfg::FcnfgSpec>;
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "FSEC (r) register accessor: Flash Security Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsec::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsec`]
module"]
#[doc(alias = "FSEC")]
pub type Fsec = crate::Reg<fsec::FsecSpec>;
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "FOPT (r) register accessor: Flash Option Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fopt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fopt`]
module"]
#[doc(alias = "FOPT")]
pub type Fopt = crate::Reg<fopt::FoptSpec>;
#[doc = "Flash Option Register"]
pub mod fopt;
#[doc = "FCCOB (rw) register accessor: Flash Common Command Object Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`fccob::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccob::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fccob`]
module"]
#[doc(alias = "FCCOB")]
pub type Fccob = crate::Reg<fccob::FccobSpec>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob;
#[doc = "FPROT (rw) register accessor: Program Flash Protection Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`fprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fprot`]
module"]
#[doc(alias = "FPROT")]
pub type Fprot = crate::Reg<fprot::FprotSpec>;
#[doc = "Program Flash Protection Registers"]
pub mod fprot;
