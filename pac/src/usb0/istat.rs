#[doc = "Register `ISTAT` reader"]
pub type R = crate::R<IstatSpec>;
#[doc = "Register `ISTAT` writer"]
pub type W = crate::W<IstatSpec>;
#[doc = "Field `USBRST` reader - This bit is set when the USB Module has decoded a valid USB reset"]
pub type UsbrstR = crate::BitReader;
#[doc = "Field `USBRST` writer - This bit is set when the USB Module has decoded a valid USB reset"]
pub type UsbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR` reader - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTOK` reader - This bit is set when the USB Module receives a Start Of Frame (SOF) token."]
pub type SoftokR = crate::BitReader;
#[doc = "Field `SOFTOK` writer - This bit is set when the USB Module receives a Start Of Frame (SOF) token."]
pub type SoftokW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOKDNE` reader - This bit is set when the current token being processed has completed"]
pub type TokdneR = crate::BitReader;
#[doc = "Field `TOKDNE` writer - This bit is set when the current token being processed has completed"]
pub type TokdneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP` reader - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
pub type SleepR = crate::BitReader;
#[doc = "Field `SLEEP` writer - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
pub type SleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUME` reader - This bit is set when a K-state is observed on the DP/DM signals for 2"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - This bit is set when a K-state is observed on the DP/DM signals for 2"]
pub type ResumeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - Stall Interrupt"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - Stall Interrupt"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This bit is set when the USB Module has decoded a valid USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> UsbrstR {
        UsbrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set when the USB Module receives a Start Of Frame (SOF) token."]
    #[inline(always)]
    pub fn softok(&self) -> SoftokR {
        SoftokR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the current token being processed has completed"]
    #[inline(always)]
    pub fn tokdne(&self) -> TokdneR {
        TokdneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
    #[inline(always)]
    pub fn sleep(&self) -> SleepR {
        SleepR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set when a K-state is observed on the DP/DM signals for 2"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Stall Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when the USB Module has decoded a valid USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> UsbrstW<IstatSpec> {
        UsbrstW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<IstatSpec> {
        ErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is set when the USB Module receives a Start Of Frame (SOF) token."]
    #[inline(always)]
    #[must_use]
    pub fn softok(&mut self) -> SoftokW<IstatSpec> {
        SoftokW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is set when the current token being processed has completed"]
    #[inline(always)]
    #[must_use]
    pub fn tokdne(&mut self) -> TokdneW<IstatSpec> {
        TokdneW::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SleepW<IstatSpec> {
        SleepW::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is set when a K-state is observed on the DP/DM signals for 2"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> ResumeW<IstatSpec> {
        ResumeW::new(self, 5)
    }
    #[doc = "Bit 7 - Stall Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<IstatSpec> {
        StallW::new(self, 7)
    }
}
#[doc = "Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`istat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`istat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IstatSpec;
impl crate::RegisterSpec for IstatSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`istat::R`](R) reader structure"]
impl crate::Readable for IstatSpec {}
#[doc = "`write(|w| ..)` method takes [`istat::W`](W) writer structure"]
impl crate::Writable for IstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ISTAT to value 0"]
impl crate::Resettable for IstatSpec {
    const RESET_VALUE: u8 = 0;
}
