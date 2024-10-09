#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gcr: Gcr,
    ar: Ar,
    fdcr: Fdcr,
    fdsr: Fdsr,
    pen: [Pen; 2],
    bpen: [Bpen; 2],
    _reserved_6_wf1: [u8; 0x04],
    _reserved_7_wf5: [u8; 0x04],
    _reserved_8_wf9: [u8; 0x04],
    _reserved_9_wf13: [u8; 0x04],
    _reserved_10_wf17: [u8; 0x04],
    _reserved_11_wf21: [u8; 0x04],
    _reserved_12_wf25: [u8; 0x04],
    _reserved_13_wf29: [u8; 0x04],
    _reserved_14_wf33: [u8; 0x04],
    _reserved_15_wf37: [u8; 0x04],
    _reserved_16_wf41: [u8; 0x04],
    _reserved_17_wf45: [u8; 0x04],
    _reserved_18_wf49: [u8; 0x04],
    _reserved_19_wf53: [u8; 0x04],
    _reserved_20_wf57: [u8; 0x04],
    _reserved_21_wf61: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - LCD General Control Register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &Gcr {
        &self.gcr
    }
    #[doc = "0x04 - LCD Auxiliary Register"]
    #[inline(always)]
    pub const fn ar(&self) -> &Ar {
        &self.ar
    }
    #[doc = "0x08 - LCD Fault Detect Control Register"]
    #[inline(always)]
    pub const fn fdcr(&self) -> &Fdcr {
        &self.fdcr
    }
    #[doc = "0x0c - LCD Fault Detect Status Register"]
    #[inline(always)]
    pub const fn fdsr(&self) -> &Fdsr {
        &self.fdsr
    }
    #[doc = "0x10..0x18 - LCD Pin Enable register"]
    #[inline(always)]
    pub const fn pen(&self, n: usize) -> &Pen {
        &self.pen[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - LCD Pin Enable register"]
    #[inline(always)]
    pub fn pen_iter(&self) -> impl Iterator<Item = &Pen> {
        self.pen.iter()
    }
    #[doc = "0x10 - LCD Pin Enable register"]
    #[inline(always)]
    pub const fn penl(&self) -> &Pen {
        self.pen(0)
    }
    #[doc = "0x14 - LCD Pin Enable register"]
    #[inline(always)]
    pub const fn penh(&self) -> &Pen {
        self.pen(1)
    }
    #[doc = "0x18..0x20 - LCD Back Plane Enable register"]
    #[inline(always)]
    pub const fn bpen(&self, n: usize) -> &Bpen {
        &self.bpen[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x20 - LCD Back Plane Enable register"]
    #[inline(always)]
    pub fn bpen_iter(&self) -> impl Iterator<Item = &Bpen> {
        self.bpen.iter()
    }
    #[doc = "0x18 - LCD Back Plane Enable register"]
    #[inline(always)]
    pub const fn bpenl(&self) -> &Bpen {
        self.bpen(0)
    }
    #[doc = "0x1c - LCD Back Plane Enable register"]
    #[inline(always)]
    pub const fn bpenh(&self) -> &Bpen {
        self.bpen(1)
    }
    #[doc = "0x20 - LCD Waveform Register 0."]
    #[inline(always)]
    pub const fn lcd_wf0(&self) -> &LcdWf0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf3to0(&self) -> &LcdWf3to0 {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x21 - LCD Waveform Register 1."]
    #[inline(always)]
    pub const fn wf1(&self) -> &Wf1 {
        unsafe { &*(self as *const Self).cast::<u8>().add(33).cast() }
    }
    #[doc = "0x22 - LCD Waveform Register 2."]
    #[inline(always)]
    pub const fn wf2(&self) -> &Wf2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(34).cast() }
    }
    #[doc = "0x23 - LCD Waveform Register 3."]
    #[inline(always)]
    pub const fn wf3(&self) -> &Wf3 {
        unsafe { &*(self as *const Self).cast::<u8>().add(35).cast() }
    }
    #[doc = "0x24 - LCD Waveform Register 4."]
    #[inline(always)]
    pub const fn lcd_wf4(&self) -> &LcdWf4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x24 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf7to4(&self) -> &LcdWf7to4 {
        unsafe { &*(self as *const Self).cast::<u8>().add(36).cast() }
    }
    #[doc = "0x25 - LCD Waveform Register 5."]
    #[inline(always)]
    pub const fn wf5(&self) -> &Wf5 {
        unsafe { &*(self as *const Self).cast::<u8>().add(37).cast() }
    }
    #[doc = "0x26 - LCD Waveform Register 6."]
    #[inline(always)]
    pub const fn wf6(&self) -> &Wf6 {
        unsafe { &*(self as *const Self).cast::<u8>().add(38).cast() }
    }
    #[doc = "0x27 - LCD Waveform Register 7."]
    #[inline(always)]
    pub const fn wf7(&self) -> &Wf7 {
        unsafe { &*(self as *const Self).cast::<u8>().add(39).cast() }
    }
    #[doc = "0x28 - LCD Waveform Register 8."]
    #[inline(always)]
    pub const fn lcd_wf8(&self) -> &LcdWf8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x28 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf11to8(&self) -> &LcdWf11to8 {
        unsafe { &*(self as *const Self).cast::<u8>().add(40).cast() }
    }
    #[doc = "0x29 - LCD Waveform Register 9."]
    #[inline(always)]
    pub const fn wf9(&self) -> &Wf9 {
        unsafe { &*(self as *const Self).cast::<u8>().add(41).cast() }
    }
    #[doc = "0x2a - LCD Waveform Register 10."]
    #[inline(always)]
    pub const fn wf10(&self) -> &Wf10 {
        unsafe { &*(self as *const Self).cast::<u8>().add(42).cast() }
    }
    #[doc = "0x2b - LCD Waveform Register 11."]
    #[inline(always)]
    pub const fn wf11(&self) -> &Wf11 {
        unsafe { &*(self as *const Self).cast::<u8>().add(43).cast() }
    }
    #[doc = "0x2c - LCD Waveform Register 12."]
    #[inline(always)]
    pub const fn lcd_wf12(&self) -> &LcdWf12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf15to12(&self) -> &LcdWf15to12 {
        unsafe { &*(self as *const Self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2d - LCD Waveform Register 13."]
    #[inline(always)]
    pub const fn wf13(&self) -> &Wf13 {
        unsafe { &*(self as *const Self).cast::<u8>().add(45).cast() }
    }
    #[doc = "0x2e - LCD Waveform Register 14."]
    #[inline(always)]
    pub const fn wf14(&self) -> &Wf14 {
        unsafe { &*(self as *const Self).cast::<u8>().add(46).cast() }
    }
    #[doc = "0x2f - LCD Waveform Register 15."]
    #[inline(always)]
    pub const fn wf15(&self) -> &Wf15 {
        unsafe { &*(self as *const Self).cast::<u8>().add(47).cast() }
    }
    #[doc = "0x30 - LCD Waveform Register 16."]
    #[inline(always)]
    pub const fn lcd_wf16(&self) -> &LcdWf16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf19to16(&self) -> &LcdWf19to16 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x31 - LCD Waveform Register 17."]
    #[inline(always)]
    pub const fn wf17(&self) -> &Wf17 {
        unsafe { &*(self as *const Self).cast::<u8>().add(49).cast() }
    }
    #[doc = "0x32 - LCD Waveform Register 18."]
    #[inline(always)]
    pub const fn wf18(&self) -> &Wf18 {
        unsafe { &*(self as *const Self).cast::<u8>().add(50).cast() }
    }
    #[doc = "0x33 - LCD Waveform Register 19."]
    #[inline(always)]
    pub const fn wf19(&self) -> &Wf19 {
        unsafe { &*(self as *const Self).cast::<u8>().add(51).cast() }
    }
    #[doc = "0x34 - LCD Waveform Register 20."]
    #[inline(always)]
    pub const fn lcd_wf20(&self) -> &LcdWf20 {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x34 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf23to20(&self) -> &LcdWf23to20 {
        unsafe { &*(self as *const Self).cast::<u8>().add(52).cast() }
    }
    #[doc = "0x35 - LCD Waveform Register 21."]
    #[inline(always)]
    pub const fn wf21(&self) -> &Wf21 {
        unsafe { &*(self as *const Self).cast::<u8>().add(53).cast() }
    }
    #[doc = "0x36 - LCD Waveform Register 22."]
    #[inline(always)]
    pub const fn wf22(&self) -> &Wf22 {
        unsafe { &*(self as *const Self).cast::<u8>().add(54).cast() }
    }
    #[doc = "0x37 - LCD Waveform Register 23."]
    #[inline(always)]
    pub const fn wf23(&self) -> &Wf23 {
        unsafe { &*(self as *const Self).cast::<u8>().add(55).cast() }
    }
    #[doc = "0x38 - LCD Waveform Register 24."]
    #[inline(always)]
    pub const fn lcd_wf24(&self) -> &LcdWf24 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x38 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf27to24(&self) -> &LcdWf27to24 {
        unsafe { &*(self as *const Self).cast::<u8>().add(56).cast() }
    }
    #[doc = "0x39 - LCD Waveform Register 25."]
    #[inline(always)]
    pub const fn wf25(&self) -> &Wf25 {
        unsafe { &*(self as *const Self).cast::<u8>().add(57).cast() }
    }
    #[doc = "0x3a - LCD Waveform Register 26."]
    #[inline(always)]
    pub const fn wf26(&self) -> &Wf26 {
        unsafe { &*(self as *const Self).cast::<u8>().add(58).cast() }
    }
    #[doc = "0x3b - LCD Waveform Register 27."]
    #[inline(always)]
    pub const fn wf27(&self) -> &Wf27 {
        unsafe { &*(self as *const Self).cast::<u8>().add(59).cast() }
    }
    #[doc = "0x3c - LCD Waveform Register 28."]
    #[inline(always)]
    pub const fn lcd_wf28(&self) -> &LcdWf28 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3c - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf31to28(&self) -> &LcdWf31to28 {
        unsafe { &*(self as *const Self).cast::<u8>().add(60).cast() }
    }
    #[doc = "0x3d - LCD Waveform Register 29."]
    #[inline(always)]
    pub const fn wf29(&self) -> &Wf29 {
        unsafe { &*(self as *const Self).cast::<u8>().add(61).cast() }
    }
    #[doc = "0x3e - LCD Waveform Register 30."]
    #[inline(always)]
    pub const fn wf30(&self) -> &Wf30 {
        unsafe { &*(self as *const Self).cast::<u8>().add(62).cast() }
    }
    #[doc = "0x3f - LCD Waveform Register 31."]
    #[inline(always)]
    pub const fn wf31(&self) -> &Wf31 {
        unsafe { &*(self as *const Self).cast::<u8>().add(63).cast() }
    }
    #[doc = "0x40 - LCD Waveform Register 32."]
    #[inline(always)]
    pub const fn lcd_wf32(&self) -> &LcdWf32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x40 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf35to32(&self) -> &LcdWf35to32 {
        unsafe { &*(self as *const Self).cast::<u8>().add(64).cast() }
    }
    #[doc = "0x41 - LCD Waveform Register 33."]
    #[inline(always)]
    pub const fn wf33(&self) -> &Wf33 {
        unsafe { &*(self as *const Self).cast::<u8>().add(65).cast() }
    }
    #[doc = "0x42 - LCD Waveform Register 34."]
    #[inline(always)]
    pub const fn wf34(&self) -> &Wf34 {
        unsafe { &*(self as *const Self).cast::<u8>().add(66).cast() }
    }
    #[doc = "0x43 - LCD Waveform Register 35."]
    #[inline(always)]
    pub const fn wf35(&self) -> &Wf35 {
        unsafe { &*(self as *const Self).cast::<u8>().add(67).cast() }
    }
    #[doc = "0x44 - LCD Waveform Register 36."]
    #[inline(always)]
    pub const fn lcd_wf36(&self) -> &LcdWf36 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x44 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf39to36(&self) -> &LcdWf39to36 {
        unsafe { &*(self as *const Self).cast::<u8>().add(68).cast() }
    }
    #[doc = "0x45 - LCD Waveform Register 37."]
    #[inline(always)]
    pub const fn wf37(&self) -> &Wf37 {
        unsafe { &*(self as *const Self).cast::<u8>().add(69).cast() }
    }
    #[doc = "0x46 - LCD Waveform Register 38."]
    #[inline(always)]
    pub const fn wf38(&self) -> &Wf38 {
        unsafe { &*(self as *const Self).cast::<u8>().add(70).cast() }
    }
    #[doc = "0x47 - LCD Waveform Register 39."]
    #[inline(always)]
    pub const fn wf39(&self) -> &Wf39 {
        unsafe { &*(self as *const Self).cast::<u8>().add(71).cast() }
    }
    #[doc = "0x48 - LCD Waveform Register 40."]
    #[inline(always)]
    pub const fn lcd_wf40(&self) -> &LcdWf40 {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x48 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf43to40(&self) -> &LcdWf43to40 {
        unsafe { &*(self as *const Self).cast::<u8>().add(72).cast() }
    }
    #[doc = "0x49 - LCD Waveform Register 41."]
    #[inline(always)]
    pub const fn wf41(&self) -> &Wf41 {
        unsafe { &*(self as *const Self).cast::<u8>().add(73).cast() }
    }
    #[doc = "0x4a - LCD Waveform Register 42."]
    #[inline(always)]
    pub const fn wf42(&self) -> &Wf42 {
        unsafe { &*(self as *const Self).cast::<u8>().add(74).cast() }
    }
    #[doc = "0x4b - LCD Waveform Register 43."]
    #[inline(always)]
    pub const fn wf43(&self) -> &Wf43 {
        unsafe { &*(self as *const Self).cast::<u8>().add(75).cast() }
    }
    #[doc = "0x4c - LCD Waveform Register 44."]
    #[inline(always)]
    pub const fn lcd_wf44(&self) -> &LcdWf44 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4c - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf47to44(&self) -> &LcdWf47to44 {
        unsafe { &*(self as *const Self).cast::<u8>().add(76).cast() }
    }
    #[doc = "0x4d - LCD Waveform Register 45."]
    #[inline(always)]
    pub const fn wf45(&self) -> &Wf45 {
        unsafe { &*(self as *const Self).cast::<u8>().add(77).cast() }
    }
    #[doc = "0x4e - LCD Waveform Register 46."]
    #[inline(always)]
    pub const fn wf46(&self) -> &Wf46 {
        unsafe { &*(self as *const Self).cast::<u8>().add(78).cast() }
    }
    #[doc = "0x4f - LCD Waveform Register 47."]
    #[inline(always)]
    pub const fn wf47(&self) -> &Wf47 {
        unsafe { &*(self as *const Self).cast::<u8>().add(79).cast() }
    }
    #[doc = "0x50 - LCD Waveform Register 48."]
    #[inline(always)]
    pub const fn lcd_wf48(&self) -> &LcdWf48 {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x50 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf51to48(&self) -> &LcdWf51to48 {
        unsafe { &*(self as *const Self).cast::<u8>().add(80).cast() }
    }
    #[doc = "0x51 - LCD Waveform Register 49."]
    #[inline(always)]
    pub const fn wf49(&self) -> &Wf49 {
        unsafe { &*(self as *const Self).cast::<u8>().add(81).cast() }
    }
    #[doc = "0x52 - LCD Waveform Register 50."]
    #[inline(always)]
    pub const fn wf50(&self) -> &Wf50 {
        unsafe { &*(self as *const Self).cast::<u8>().add(82).cast() }
    }
    #[doc = "0x53 - LCD Waveform Register 51."]
    #[inline(always)]
    pub const fn wf51(&self) -> &Wf51 {
        unsafe { &*(self as *const Self).cast::<u8>().add(83).cast() }
    }
    #[doc = "0x54 - LCD Waveform Register 52."]
    #[inline(always)]
    pub const fn lcd_wf52(&self) -> &LcdWf52 {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x54 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf55to52(&self) -> &LcdWf55to52 {
        unsafe { &*(self as *const Self).cast::<u8>().add(84).cast() }
    }
    #[doc = "0x55 - LCD Waveform Register 53."]
    #[inline(always)]
    pub const fn wf53(&self) -> &Wf53 {
        unsafe { &*(self as *const Self).cast::<u8>().add(85).cast() }
    }
    #[doc = "0x56 - LCD Waveform Register 54."]
    #[inline(always)]
    pub const fn wf54(&self) -> &Wf54 {
        unsafe { &*(self as *const Self).cast::<u8>().add(86).cast() }
    }
    #[doc = "0x57 - LCD Waveform Register 55."]
    #[inline(always)]
    pub const fn wf55(&self) -> &Wf55 {
        unsafe { &*(self as *const Self).cast::<u8>().add(87).cast() }
    }
    #[doc = "0x58 - LCD Waveform Register 56."]
    #[inline(always)]
    pub const fn lcd_wf56(&self) -> &LcdWf56 {
        unsafe { &*(self as *const Self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x58 - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf59to56(&self) -> &LcdWf59to56 {
        unsafe { &*(self as *const Self).cast::<u8>().add(88).cast() }
    }
    #[doc = "0x59 - LCD Waveform Register 57."]
    #[inline(always)]
    pub const fn wf57(&self) -> &Wf57 {
        unsafe { &*(self as *const Self).cast::<u8>().add(89).cast() }
    }
    #[doc = "0x5a - LCD Waveform Register 58."]
    #[inline(always)]
    pub const fn wf58(&self) -> &Wf58 {
        unsafe { &*(self as *const Self).cast::<u8>().add(90).cast() }
    }
    #[doc = "0x5b - LCD Waveform Register 59."]
    #[inline(always)]
    pub const fn wf59(&self) -> &Wf59 {
        unsafe { &*(self as *const Self).cast::<u8>().add(91).cast() }
    }
    #[doc = "0x5c - LCD Waveform Register 60."]
    #[inline(always)]
    pub const fn lcd_wf60(&self) -> &LcdWf60 {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x5c - LCD Waveform register"]
    #[inline(always)]
    pub const fn lcd_wf63to60(&self) -> &LcdWf63to60 {
        unsafe { &*(self as *const Self).cast::<u8>().add(92).cast() }
    }
    #[doc = "0x5d - LCD Waveform Register 61."]
    #[inline(always)]
    pub const fn wf61(&self) -> &Wf61 {
        unsafe { &*(self as *const Self).cast::<u8>().add(93).cast() }
    }
    #[doc = "0x5e - LCD Waveform Register 62."]
    #[inline(always)]
    pub const fn wf62(&self) -> &Wf62 {
        unsafe { &*(self as *const Self).cast::<u8>().add(94).cast() }
    }
    #[doc = "0x5f - LCD Waveform Register 63."]
    #[inline(always)]
    pub const fn wf63(&self) -> &Wf63 {
        unsafe { &*(self as *const Self).cast::<u8>().add(95).cast() }
    }
}
#[doc = "GCR (rw) register accessor: LCD General Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`]
module"]
#[doc(alias = "GCR")]
pub type Gcr = crate::Reg<gcr::GcrSpec>;
#[doc = "LCD General Control Register"]
pub mod gcr;
#[doc = "AR (rw) register accessor: LCD Auxiliary Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`]
module"]
#[doc(alias = "AR")]
pub type Ar = crate::Reg<ar::ArSpec>;
#[doc = "LCD Auxiliary Register"]
pub mod ar;
#[doc = "FDCR (rw) register accessor: LCD Fault Detect Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcr`]
module"]
#[doc(alias = "FDCR")]
pub type Fdcr = crate::Reg<fdcr::FdcrSpec>;
#[doc = "LCD Fault Detect Control Register"]
pub mod fdcr;
#[doc = "FDSR (rw) register accessor: LCD Fault Detect Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdsr`]
module"]
#[doc(alias = "FDSR")]
pub type Fdsr = crate::Reg<fdsr::FdsrSpec>;
#[doc = "LCD Fault Detect Status Register"]
pub mod fdsr;
#[doc = "PEN (rw) register accessor: LCD Pin Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`pen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pen`]
module"]
#[doc(alias = "PEN")]
pub type Pen = crate::Reg<pen::PenSpec>;
#[doc = "LCD Pin Enable register"]
pub mod pen;
#[doc = "BPEN (rw) register accessor: LCD Back Plane Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`bpen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bpen`]
module"]
#[doc(alias = "BPEN")]
pub type Bpen = crate::Reg<bpen::BpenSpec>;
#[doc = "LCD Back Plane Enable register"]
pub mod bpen;
#[doc = "LCD_WF3TO0 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf3to0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf3to0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf3to0`]
module"]
#[doc(alias = "LCD_WF3TO0")]
pub type LcdWf3to0 = crate::Reg<lcd_wf3to0::LcdWf3to0Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf3to0;
#[doc = "LCD_WF0 (rw) register accessor: LCD Waveform Register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf0`]
module"]
#[doc(alias = "LCD_WF0")]
pub type LcdWf0 = crate::Reg<lcd_wf0::LcdWf0Spec>;
#[doc = "LCD Waveform Register 0."]
pub mod lcd_wf0;
#[doc = "WF1 (rw) register accessor: LCD Waveform Register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf1`]
module"]
#[doc(alias = "WF1")]
pub type Wf1 = crate::Reg<wf1::Wf1Spec>;
#[doc = "LCD Waveform Register 1."]
pub mod wf1;
#[doc = "WF2 (rw) register accessor: LCD Waveform Register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf2`]
module"]
#[doc(alias = "WF2")]
pub type Wf2 = crate::Reg<wf2::Wf2Spec>;
#[doc = "LCD Waveform Register 2."]
pub mod wf2;
#[doc = "WF3 (rw) register accessor: LCD Waveform Register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf3`]
module"]
#[doc(alias = "WF3")]
pub type Wf3 = crate::Reg<wf3::Wf3Spec>;
#[doc = "LCD Waveform Register 3."]
pub mod wf3;
#[doc = "LCD_WF7TO4 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf7to4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf7to4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf7to4`]
module"]
#[doc(alias = "LCD_WF7TO4")]
pub type LcdWf7to4 = crate::Reg<lcd_wf7to4::LcdWf7to4Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf7to4;
#[doc = "LCD_WF4 (rw) register accessor: LCD Waveform Register 4.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf4`]
module"]
#[doc(alias = "LCD_WF4")]
pub type LcdWf4 = crate::Reg<lcd_wf4::LcdWf4Spec>;
#[doc = "LCD Waveform Register 4."]
pub mod lcd_wf4;
#[doc = "WF5 (rw) register accessor: LCD Waveform Register 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf5`]
module"]
#[doc(alias = "WF5")]
pub type Wf5 = crate::Reg<wf5::Wf5Spec>;
#[doc = "LCD Waveform Register 5."]
pub mod wf5;
#[doc = "WF6 (rw) register accessor: LCD Waveform Register 6.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf6`]
module"]
#[doc(alias = "WF6")]
pub type Wf6 = crate::Reg<wf6::Wf6Spec>;
#[doc = "LCD Waveform Register 6."]
pub mod wf6;
#[doc = "WF7 (rw) register accessor: LCD Waveform Register 7.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf7`]
module"]
#[doc(alias = "WF7")]
pub type Wf7 = crate::Reg<wf7::Wf7Spec>;
#[doc = "LCD Waveform Register 7."]
pub mod wf7;
#[doc = "LCD_WF11TO8 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf11to8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf11to8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf11to8`]
module"]
#[doc(alias = "LCD_WF11TO8")]
pub type LcdWf11to8 = crate::Reg<lcd_wf11to8::LcdWf11to8Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf11to8;
#[doc = "LCD_WF8 (rw) register accessor: LCD Waveform Register 8.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf8`]
module"]
#[doc(alias = "LCD_WF8")]
pub type LcdWf8 = crate::Reg<lcd_wf8::LcdWf8Spec>;
#[doc = "LCD Waveform Register 8."]
pub mod lcd_wf8;
#[doc = "WF9 (rw) register accessor: LCD Waveform Register 9.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf9`]
module"]
#[doc(alias = "WF9")]
pub type Wf9 = crate::Reg<wf9::Wf9Spec>;
#[doc = "LCD Waveform Register 9."]
pub mod wf9;
#[doc = "WF10 (rw) register accessor: LCD Waveform Register 10.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf10`]
module"]
#[doc(alias = "WF10")]
pub type Wf10 = crate::Reg<wf10::Wf10Spec>;
#[doc = "LCD Waveform Register 10."]
pub mod wf10;
#[doc = "WF11 (rw) register accessor: LCD Waveform Register 11.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf11`]
module"]
#[doc(alias = "WF11")]
pub type Wf11 = crate::Reg<wf11::Wf11Spec>;
#[doc = "LCD Waveform Register 11."]
pub mod wf11;
#[doc = "LCD_WF15TO12 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf15to12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf15to12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf15to12`]
module"]
#[doc(alias = "LCD_WF15TO12")]
pub type LcdWf15to12 = crate::Reg<lcd_wf15to12::LcdWf15to12Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf15to12;
#[doc = "LCD_WF12 (rw) register accessor: LCD Waveform Register 12.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf12`]
module"]
#[doc(alias = "LCD_WF12")]
pub type LcdWf12 = crate::Reg<lcd_wf12::LcdWf12Spec>;
#[doc = "LCD Waveform Register 12."]
pub mod lcd_wf12;
#[doc = "WF13 (rw) register accessor: LCD Waveform Register 13.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf13`]
module"]
#[doc(alias = "WF13")]
pub type Wf13 = crate::Reg<wf13::Wf13Spec>;
#[doc = "LCD Waveform Register 13."]
pub mod wf13;
#[doc = "WF14 (rw) register accessor: LCD Waveform Register 14.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf14`]
module"]
#[doc(alias = "WF14")]
pub type Wf14 = crate::Reg<wf14::Wf14Spec>;
#[doc = "LCD Waveform Register 14."]
pub mod wf14;
#[doc = "WF15 (rw) register accessor: LCD Waveform Register 15.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf15`]
module"]
#[doc(alias = "WF15")]
pub type Wf15 = crate::Reg<wf15::Wf15Spec>;
#[doc = "LCD Waveform Register 15."]
pub mod wf15;
#[doc = "LCD_WF19TO16 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf19to16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf19to16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf19to16`]
module"]
#[doc(alias = "LCD_WF19TO16")]
pub type LcdWf19to16 = crate::Reg<lcd_wf19to16::LcdWf19to16Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf19to16;
#[doc = "LCD_WF16 (rw) register accessor: LCD Waveform Register 16.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf16`]
module"]
#[doc(alias = "LCD_WF16")]
pub type LcdWf16 = crate::Reg<lcd_wf16::LcdWf16Spec>;
#[doc = "LCD Waveform Register 16."]
pub mod lcd_wf16;
#[doc = "WF17 (rw) register accessor: LCD Waveform Register 17.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf17`]
module"]
#[doc(alias = "WF17")]
pub type Wf17 = crate::Reg<wf17::Wf17Spec>;
#[doc = "LCD Waveform Register 17."]
pub mod wf17;
#[doc = "WF18 (rw) register accessor: LCD Waveform Register 18.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf18`]
module"]
#[doc(alias = "WF18")]
pub type Wf18 = crate::Reg<wf18::Wf18Spec>;
#[doc = "LCD Waveform Register 18."]
pub mod wf18;
#[doc = "WF19 (rw) register accessor: LCD Waveform Register 19.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf19`]
module"]
#[doc(alias = "WF19")]
pub type Wf19 = crate::Reg<wf19::Wf19Spec>;
#[doc = "LCD Waveform Register 19."]
pub mod wf19;
#[doc = "LCD_WF23TO20 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf23to20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf23to20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf23to20`]
module"]
#[doc(alias = "LCD_WF23TO20")]
pub type LcdWf23to20 = crate::Reg<lcd_wf23to20::LcdWf23to20Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf23to20;
#[doc = "LCD_WF20 (rw) register accessor: LCD Waveform Register 20.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf20`]
module"]
#[doc(alias = "LCD_WF20")]
pub type LcdWf20 = crate::Reg<lcd_wf20::LcdWf20Spec>;
#[doc = "LCD Waveform Register 20."]
pub mod lcd_wf20;
#[doc = "WF21 (rw) register accessor: LCD Waveform Register 21.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf21`]
module"]
#[doc(alias = "WF21")]
pub type Wf21 = crate::Reg<wf21::Wf21Spec>;
#[doc = "LCD Waveform Register 21."]
pub mod wf21;
#[doc = "WF22 (rw) register accessor: LCD Waveform Register 22.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf22`]
module"]
#[doc(alias = "WF22")]
pub type Wf22 = crate::Reg<wf22::Wf22Spec>;
#[doc = "LCD Waveform Register 22."]
pub mod wf22;
#[doc = "WF23 (rw) register accessor: LCD Waveform Register 23.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf23`]
module"]
#[doc(alias = "WF23")]
pub type Wf23 = crate::Reg<wf23::Wf23Spec>;
#[doc = "LCD Waveform Register 23."]
pub mod wf23;
#[doc = "LCD_WF27TO24 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf27to24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf27to24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf27to24`]
module"]
#[doc(alias = "LCD_WF27TO24")]
pub type LcdWf27to24 = crate::Reg<lcd_wf27to24::LcdWf27to24Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf27to24;
#[doc = "LCD_WF24 (rw) register accessor: LCD Waveform Register 24.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf24`]
module"]
#[doc(alias = "LCD_WF24")]
pub type LcdWf24 = crate::Reg<lcd_wf24::LcdWf24Spec>;
#[doc = "LCD Waveform Register 24."]
pub mod lcd_wf24;
#[doc = "WF25 (rw) register accessor: LCD Waveform Register 25.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf25`]
module"]
#[doc(alias = "WF25")]
pub type Wf25 = crate::Reg<wf25::Wf25Spec>;
#[doc = "LCD Waveform Register 25."]
pub mod wf25;
#[doc = "WF26 (rw) register accessor: LCD Waveform Register 26.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf26`]
module"]
#[doc(alias = "WF26")]
pub type Wf26 = crate::Reg<wf26::Wf26Spec>;
#[doc = "LCD Waveform Register 26."]
pub mod wf26;
#[doc = "WF27 (rw) register accessor: LCD Waveform Register 27.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf27`]
module"]
#[doc(alias = "WF27")]
pub type Wf27 = crate::Reg<wf27::Wf27Spec>;
#[doc = "LCD Waveform Register 27."]
pub mod wf27;
#[doc = "LCD_WF31TO28 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf31to28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf31to28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf31to28`]
module"]
#[doc(alias = "LCD_WF31TO28")]
pub type LcdWf31to28 = crate::Reg<lcd_wf31to28::LcdWf31to28Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf31to28;
#[doc = "LCD_WF28 (rw) register accessor: LCD Waveform Register 28.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf28`]
module"]
#[doc(alias = "LCD_WF28")]
pub type LcdWf28 = crate::Reg<lcd_wf28::LcdWf28Spec>;
#[doc = "LCD Waveform Register 28."]
pub mod lcd_wf28;
#[doc = "WF29 (rw) register accessor: LCD Waveform Register 29.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf29`]
module"]
#[doc(alias = "WF29")]
pub type Wf29 = crate::Reg<wf29::Wf29Spec>;
#[doc = "LCD Waveform Register 29."]
pub mod wf29;
#[doc = "WF30 (rw) register accessor: LCD Waveform Register 30.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf30`]
module"]
#[doc(alias = "WF30")]
pub type Wf30 = crate::Reg<wf30::Wf30Spec>;
#[doc = "LCD Waveform Register 30."]
pub mod wf30;
#[doc = "WF31 (rw) register accessor: LCD Waveform Register 31.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf31`]
module"]
#[doc(alias = "WF31")]
pub type Wf31 = crate::Reg<wf31::Wf31Spec>;
#[doc = "LCD Waveform Register 31."]
pub mod wf31;
#[doc = "LCD_WF35TO32 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf35to32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf35to32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf35to32`]
module"]
#[doc(alias = "LCD_WF35TO32")]
pub type LcdWf35to32 = crate::Reg<lcd_wf35to32::LcdWf35to32Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf35to32;
#[doc = "LCD_WF32 (rw) register accessor: LCD Waveform Register 32.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf32`]
module"]
#[doc(alias = "LCD_WF32")]
pub type LcdWf32 = crate::Reg<lcd_wf32::LcdWf32Spec>;
#[doc = "LCD Waveform Register 32."]
pub mod lcd_wf32;
#[doc = "WF33 (rw) register accessor: LCD Waveform Register 33.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf33`]
module"]
#[doc(alias = "WF33")]
pub type Wf33 = crate::Reg<wf33::Wf33Spec>;
#[doc = "LCD Waveform Register 33."]
pub mod wf33;
#[doc = "WF34 (rw) register accessor: LCD Waveform Register 34.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf34`]
module"]
#[doc(alias = "WF34")]
pub type Wf34 = crate::Reg<wf34::Wf34Spec>;
#[doc = "LCD Waveform Register 34."]
pub mod wf34;
#[doc = "WF35 (rw) register accessor: LCD Waveform Register 35.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf35`]
module"]
#[doc(alias = "WF35")]
pub type Wf35 = crate::Reg<wf35::Wf35Spec>;
#[doc = "LCD Waveform Register 35."]
pub mod wf35;
#[doc = "LCD_WF39TO36 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf39to36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf39to36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf39to36`]
module"]
#[doc(alias = "LCD_WF39TO36")]
pub type LcdWf39to36 = crate::Reg<lcd_wf39to36::LcdWf39to36Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf39to36;
#[doc = "LCD_WF36 (rw) register accessor: LCD Waveform Register 36.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf36`]
module"]
#[doc(alias = "LCD_WF36")]
pub type LcdWf36 = crate::Reg<lcd_wf36::LcdWf36Spec>;
#[doc = "LCD Waveform Register 36."]
pub mod lcd_wf36;
#[doc = "WF37 (rw) register accessor: LCD Waveform Register 37.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf37`]
module"]
#[doc(alias = "WF37")]
pub type Wf37 = crate::Reg<wf37::Wf37Spec>;
#[doc = "LCD Waveform Register 37."]
pub mod wf37;
#[doc = "WF38 (rw) register accessor: LCD Waveform Register 38.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf38`]
module"]
#[doc(alias = "WF38")]
pub type Wf38 = crate::Reg<wf38::Wf38Spec>;
#[doc = "LCD Waveform Register 38."]
pub mod wf38;
#[doc = "WF39 (rw) register accessor: LCD Waveform Register 39.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf39`]
module"]
#[doc(alias = "WF39")]
pub type Wf39 = crate::Reg<wf39::Wf39Spec>;
#[doc = "LCD Waveform Register 39."]
pub mod wf39;
#[doc = "LCD_WF43TO40 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf43to40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf43to40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf43to40`]
module"]
#[doc(alias = "LCD_WF43TO40")]
pub type LcdWf43to40 = crate::Reg<lcd_wf43to40::LcdWf43to40Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf43to40;
#[doc = "LCD_WF40 (rw) register accessor: LCD Waveform Register 40.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf40`]
module"]
#[doc(alias = "LCD_WF40")]
pub type LcdWf40 = crate::Reg<lcd_wf40::LcdWf40Spec>;
#[doc = "LCD Waveform Register 40."]
pub mod lcd_wf40;
#[doc = "WF41 (rw) register accessor: LCD Waveform Register 41.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf41`]
module"]
#[doc(alias = "WF41")]
pub type Wf41 = crate::Reg<wf41::Wf41Spec>;
#[doc = "LCD Waveform Register 41."]
pub mod wf41;
#[doc = "WF42 (rw) register accessor: LCD Waveform Register 42.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf42`]
module"]
#[doc(alias = "WF42")]
pub type Wf42 = crate::Reg<wf42::Wf42Spec>;
#[doc = "LCD Waveform Register 42."]
pub mod wf42;
#[doc = "WF43 (rw) register accessor: LCD Waveform Register 43.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf43`]
module"]
#[doc(alias = "WF43")]
pub type Wf43 = crate::Reg<wf43::Wf43Spec>;
#[doc = "LCD Waveform Register 43."]
pub mod wf43;
#[doc = "LCD_WF47TO44 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf47to44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf47to44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf47to44`]
module"]
#[doc(alias = "LCD_WF47TO44")]
pub type LcdWf47to44 = crate::Reg<lcd_wf47to44::LcdWf47to44Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf47to44;
#[doc = "LCD_WF44 (rw) register accessor: LCD Waveform Register 44.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf44`]
module"]
#[doc(alias = "LCD_WF44")]
pub type LcdWf44 = crate::Reg<lcd_wf44::LcdWf44Spec>;
#[doc = "LCD Waveform Register 44."]
pub mod lcd_wf44;
#[doc = "WF45 (rw) register accessor: LCD Waveform Register 45.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf45`]
module"]
#[doc(alias = "WF45")]
pub type Wf45 = crate::Reg<wf45::Wf45Spec>;
#[doc = "LCD Waveform Register 45."]
pub mod wf45;
#[doc = "WF46 (rw) register accessor: LCD Waveform Register 46.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf46`]
module"]
#[doc(alias = "WF46")]
pub type Wf46 = crate::Reg<wf46::Wf46Spec>;
#[doc = "LCD Waveform Register 46."]
pub mod wf46;
#[doc = "WF47 (rw) register accessor: LCD Waveform Register 47.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf47`]
module"]
#[doc(alias = "WF47")]
pub type Wf47 = crate::Reg<wf47::Wf47Spec>;
#[doc = "LCD Waveform Register 47."]
pub mod wf47;
#[doc = "LCD_WF51TO48 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf51to48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf51to48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf51to48`]
module"]
#[doc(alias = "LCD_WF51TO48")]
pub type LcdWf51to48 = crate::Reg<lcd_wf51to48::LcdWf51to48Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf51to48;
#[doc = "LCD_WF48 (rw) register accessor: LCD Waveform Register 48.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf48`]
module"]
#[doc(alias = "LCD_WF48")]
pub type LcdWf48 = crate::Reg<lcd_wf48::LcdWf48Spec>;
#[doc = "LCD Waveform Register 48."]
pub mod lcd_wf48;
#[doc = "WF49 (rw) register accessor: LCD Waveform Register 49.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf49`]
module"]
#[doc(alias = "WF49")]
pub type Wf49 = crate::Reg<wf49::Wf49Spec>;
#[doc = "LCD Waveform Register 49."]
pub mod wf49;
#[doc = "WF50 (rw) register accessor: LCD Waveform Register 50.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf50`]
module"]
#[doc(alias = "WF50")]
pub type Wf50 = crate::Reg<wf50::Wf50Spec>;
#[doc = "LCD Waveform Register 50."]
pub mod wf50;
#[doc = "WF51 (rw) register accessor: LCD Waveform Register 51.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf51`]
module"]
#[doc(alias = "WF51")]
pub type Wf51 = crate::Reg<wf51::Wf51Spec>;
#[doc = "LCD Waveform Register 51."]
pub mod wf51;
#[doc = "LCD_WF55TO52 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf55to52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf55to52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf55to52`]
module"]
#[doc(alias = "LCD_WF55TO52")]
pub type LcdWf55to52 = crate::Reg<lcd_wf55to52::LcdWf55to52Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf55to52;
#[doc = "LCD_WF52 (rw) register accessor: LCD Waveform Register 52.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf52`]
module"]
#[doc(alias = "LCD_WF52")]
pub type LcdWf52 = crate::Reg<lcd_wf52::LcdWf52Spec>;
#[doc = "LCD Waveform Register 52."]
pub mod lcd_wf52;
#[doc = "WF53 (rw) register accessor: LCD Waveform Register 53.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf53`]
module"]
#[doc(alias = "WF53")]
pub type Wf53 = crate::Reg<wf53::Wf53Spec>;
#[doc = "LCD Waveform Register 53."]
pub mod wf53;
#[doc = "WF54 (rw) register accessor: LCD Waveform Register 54.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf54`]
module"]
#[doc(alias = "WF54")]
pub type Wf54 = crate::Reg<wf54::Wf54Spec>;
#[doc = "LCD Waveform Register 54."]
pub mod wf54;
#[doc = "WF55 (rw) register accessor: LCD Waveform Register 55.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf55`]
module"]
#[doc(alias = "WF55")]
pub type Wf55 = crate::Reg<wf55::Wf55Spec>;
#[doc = "LCD Waveform Register 55."]
pub mod wf55;
#[doc = "LCD_WF59TO56 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf59to56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf59to56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf59to56`]
module"]
#[doc(alias = "LCD_WF59TO56")]
pub type LcdWf59to56 = crate::Reg<lcd_wf59to56::LcdWf59to56Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf59to56;
#[doc = "LCD_WF56 (rw) register accessor: LCD Waveform Register 56.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf56`]
module"]
#[doc(alias = "LCD_WF56")]
pub type LcdWf56 = crate::Reg<lcd_wf56::LcdWf56Spec>;
#[doc = "LCD Waveform Register 56."]
pub mod lcd_wf56;
#[doc = "WF57 (rw) register accessor: LCD Waveform Register 57.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf57`]
module"]
#[doc(alias = "WF57")]
pub type Wf57 = crate::Reg<wf57::Wf57Spec>;
#[doc = "LCD Waveform Register 57."]
pub mod wf57;
#[doc = "WF58 (rw) register accessor: LCD Waveform Register 58.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf58`]
module"]
#[doc(alias = "WF58")]
pub type Wf58 = crate::Reg<wf58::Wf58Spec>;
#[doc = "LCD Waveform Register 58."]
pub mod wf58;
#[doc = "WF59 (rw) register accessor: LCD Waveform Register 59.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf59`]
module"]
#[doc(alias = "WF59")]
pub type Wf59 = crate::Reg<wf59::Wf59Spec>;
#[doc = "LCD Waveform Register 59."]
pub mod wf59;
#[doc = "LCD_WF63TO60 (rw) register accessor: LCD Waveform register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf63to60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf63to60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf63to60`]
module"]
#[doc(alias = "LCD_WF63TO60")]
pub type LcdWf63to60 = crate::Reg<lcd_wf63to60::LcdWf63to60Spec>;
#[doc = "LCD Waveform register"]
pub mod lcd_wf63to60;
#[doc = "LCD_WF60 (rw) register accessor: LCD Waveform Register 60.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_wf60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_wf60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_wf60`]
module"]
#[doc(alias = "LCD_WF60")]
pub type LcdWf60 = crate::Reg<lcd_wf60::LcdWf60Spec>;
#[doc = "LCD Waveform Register 60."]
pub mod lcd_wf60;
#[doc = "WF61 (rw) register accessor: LCD Waveform Register 61.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf61`]
module"]
#[doc(alias = "WF61")]
pub type Wf61 = crate::Reg<wf61::Wf61Spec>;
#[doc = "LCD Waveform Register 61."]
pub mod wf61;
#[doc = "WF62 (rw) register accessor: LCD Waveform Register 62.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf62`]
module"]
#[doc(alias = "WF62")]
pub type Wf62 = crate::Reg<wf62::Wf62Spec>;
#[doc = "LCD Waveform Register 62."]
pub mod wf62;
#[doc = "WF63 (rw) register accessor: LCD Waveform Register 63.\n\nYou can [`read`](crate::Reg::read) this register and get [`wf63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wf63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wf63`]
module"]
#[doc(alias = "WF63")]
pub type Wf63 = crate::Reg<wf63::Wf63Spec>;
#[doc = "LCD Waveform Register 63."]
pub mod wf63;
