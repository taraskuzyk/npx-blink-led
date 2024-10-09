#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Channel 0 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch0f {
    #[doc = "0: No channel event has occurred."]
    B0 = 0,
    #[doc = "1: A channel event has occurred."]
    B1 = 1,
}
impl From<Ch0f> for bool {
    #[inline(always)]
    fn from(variant: Ch0f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH0F` reader - Channel 0 Flag"]
pub type Ch0fR = crate::BitReader<Ch0f>;
impl Ch0fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch0f {
        match self.bits {
            false => Ch0f::B0,
            true => Ch0f::B1,
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch0f::B0
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch0f::B1
    }
}
#[doc = "Field `CH0F` writer - Channel 0 Flag"]
pub type Ch0fW<'a, REG> = crate::BitWriter<'a, REG, Ch0f>;
impl<'a, REG> Ch0fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0f::B0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch0f::B1)
    }
}
#[doc = "Channel 1 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch1f {
    #[doc = "0: No channel event has occurred."]
    B0 = 0,
    #[doc = "1: A channel event has occurred."]
    B1 = 1,
}
impl From<Ch1f> for bool {
    #[inline(always)]
    fn from(variant: Ch1f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH1F` reader - Channel 1 Flag"]
pub type Ch1fR = crate::BitReader<Ch1f>;
impl Ch1fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch1f {
        match self.bits {
            false => Ch1f::B0,
            true => Ch1f::B1,
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch1f::B0
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch1f::B1
    }
}
#[doc = "Field `CH1F` writer - Channel 1 Flag"]
pub type Ch1fW<'a, REG> = crate::BitWriter<'a, REG, Ch1f>;
impl<'a, REG> Ch1fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1f::B0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch1f::B1)
    }
}
#[doc = "Channel 2 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch2f {
    #[doc = "0: No channel event has occurred."]
    B0 = 0,
    #[doc = "1: A channel event has occurred."]
    B1 = 1,
}
impl From<Ch2f> for bool {
    #[inline(always)]
    fn from(variant: Ch2f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH2F` reader - Channel 2 Flag"]
pub type Ch2fR = crate::BitReader<Ch2f>;
impl Ch2fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch2f {
        match self.bits {
            false => Ch2f::B0,
            true => Ch2f::B1,
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch2f::B0
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch2f::B1
    }
}
#[doc = "Field `CH2F` writer - Channel 2 Flag"]
pub type Ch2fW<'a, REG> = crate::BitWriter<'a, REG, Ch2f>;
impl<'a, REG> Ch2fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2f::B0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch2f::B1)
    }
}
#[doc = "Channel 3 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch3f {
    #[doc = "0: No channel event has occurred."]
    B0 = 0,
    #[doc = "1: A channel event has occurred."]
    B1 = 1,
}
impl From<Ch3f> for bool {
    #[inline(always)]
    fn from(variant: Ch3f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH3F` reader - Channel 3 Flag"]
pub type Ch3fR = crate::BitReader<Ch3f>;
impl Ch3fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch3f {
        match self.bits {
            false => Ch3f::B0,
            true => Ch3f::B1,
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch3f::B0
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch3f::B1
    }
}
#[doc = "Field `CH3F` writer - Channel 3 Flag"]
pub type Ch3fW<'a, REG> = crate::BitWriter<'a, REG, Ch3f>;
impl<'a, REG> Ch3fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3f::B0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch3f::B1)
    }
}
#[doc = "Channel 4 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch4f {
    #[doc = "0: No channel event has occurred."]
    B0 = 0,
    #[doc = "1: A channel event has occurred."]
    B1 = 1,
}
impl From<Ch4f> for bool {
    #[inline(always)]
    fn from(variant: Ch4f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH4F` reader - Channel 4 Flag"]
pub type Ch4fR = crate::BitReader<Ch4f>;
impl Ch4fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch4f {
        match self.bits {
            false => Ch4f::B0,
            true => Ch4f::B1,
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch4f::B0
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch4f::B1
    }
}
#[doc = "Field `CH4F` writer - Channel 4 Flag"]
pub type Ch4fW<'a, REG> = crate::BitWriter<'a, REG, Ch4f>;
impl<'a, REG> Ch4fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4f::B0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch4f::B1)
    }
}
#[doc = "Channel 5 Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ch5f {
    #[doc = "0: No channel event has occurred."]
    B0 = 0,
    #[doc = "1: A channel event has occurred."]
    B1 = 1,
}
impl From<Ch5f> for bool {
    #[inline(always)]
    fn from(variant: Ch5f) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CH5F` reader - Channel 5 Flag"]
pub type Ch5fR = crate::BitReader<Ch5f>;
impl Ch5fR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ch5f {
        match self.bits {
            false => Ch5f::B0,
            true => Ch5f::B1,
        }
    }
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ch5f::B0
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ch5f::B1
    }
}
#[doc = "Field `CH5F` writer - Channel 5 Flag"]
pub type Ch5fW<'a, REG> = crate::BitWriter<'a, REG, Ch5f>;
impl<'a, REG> Ch5fW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5f::B0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ch5f::B1)
    }
}
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tof {
    #[doc = "0: TPM counter has not overflowed."]
    B0 = 0,
    #[doc = "1: TPM counter has overflowed."]
    B1 = 1,
}
impl From<Tof> for bool {
    #[inline(always)]
    fn from(variant: Tof) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TOF` reader - Timer Overflow Flag"]
pub type TofR = crate::BitReader<Tof>;
impl TofR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tof {
        match self.bits {
            false => Tof::B0,
            true => Tof::B1,
        }
    }
    #[doc = "TPM counter has not overflowed."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Tof::B0
    }
    #[doc = "TPM counter has overflowed."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Tof::B1
    }
}
#[doc = "Field `TOF` writer - Timer Overflow Flag"]
pub type TofW<'a, REG> = crate::BitWriter<'a, REG, Tof>;
impl<'a, REG> TofW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TPM counter has not overflowed."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Tof::B0)
    }
    #[doc = "TPM counter has overflowed."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Tof::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline(always)]
    pub fn ch0f(&self) -> Ch0fR {
        Ch0fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline(always)]
    pub fn ch1f(&self) -> Ch1fR {
        Ch1fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline(always)]
    pub fn ch2f(&self) -> Ch2fR {
        Ch2fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline(always)]
    pub fn ch3f(&self) -> Ch3fR {
        Ch3fR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline(always)]
    pub fn ch4f(&self) -> Ch4fR {
        Ch4fR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline(always)]
    pub fn ch5f(&self) -> Ch5fR {
        Ch5fR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TofR {
        TofR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0f(&mut self) -> Ch0fW<StatusSpec> {
        Ch0fW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1f(&mut self) -> Ch1fW<StatusSpec> {
        Ch1fW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2f(&mut self) -> Ch2fW<StatusSpec> {
        Ch2fW::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3f(&mut self) -> Ch3fW<StatusSpec> {
        Ch3fW::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch4f(&mut self) -> Ch4fW<StatusSpec> {
        Ch4fW::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch5f(&mut self) -> Ch5fW<StatusSpec> {
        Ch5fW::new(self, 5)
    }
    #[doc = "Bit 8 - Timer Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TofW<StatusSpec> {
        TofW::new(self, 8)
    }
}
#[doc = "Capture and Compare Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
