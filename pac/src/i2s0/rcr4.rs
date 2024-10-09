#[doc = "Register `RCR4` reader"]
pub type R = crate::R<Rcr4Spec>;
#[doc = "Register `RCR4` writer"]
pub type W = crate::W<Rcr4Spec>;
#[doc = "Frame Sync Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsd {
    #[doc = "0: Frame Sync is generated externally in Slave mode."]
    B0 = 0,
    #[doc = "1: Frame Sync is generated internally in Master mode."]
    B1 = 1,
}
impl From<Fsd> for bool {
    #[inline(always)]
    fn from(variant: Fsd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSD` reader - Frame Sync Direction"]
pub type FsdR = crate::BitReader<Fsd>;
impl FsdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsd {
        match self.bits {
            false => Fsd::B0,
            true => Fsd::B1,
        }
    }
    #[doc = "Frame Sync is generated externally in Slave mode."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fsd::B0
    }
    #[doc = "Frame Sync is generated internally in Master mode."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fsd::B1
    }
}
#[doc = "Field `FSD` writer - Frame Sync Direction"]
pub type FsdW<'a, REG> = crate::BitWriter<'a, REG, Fsd>;
impl<'a, REG> FsdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frame Sync is generated externally in Slave mode."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsd::B0)
    }
    #[doc = "Frame Sync is generated internally in Master mode."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsd::B1)
    }
}
#[doc = "Frame Sync Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fsp {
    #[doc = "0: Frame sync is active high."]
    B0 = 0,
    #[doc = "1: Frame sync is active low."]
    B1 = 1,
}
impl From<Fsp> for bool {
    #[inline(always)]
    fn from(variant: Fsp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSP` reader - Frame Sync Polarity"]
pub type FspR = crate::BitReader<Fsp>;
impl FspR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsp {
        match self.bits {
            false => Fsp::B0,
            true => Fsp::B1,
        }
    }
    #[doc = "Frame sync is active high."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fsp::B0
    }
    #[doc = "Frame sync is active low."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fsp::B1
    }
}
#[doc = "Field `FSP` writer - Frame Sync Polarity"]
pub type FspW<'a, REG> = crate::BitWriter<'a, REG, Fsp>;
impl<'a, REG> FspW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frame sync is active high."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsp::B0)
    }
    #[doc = "Frame sync is active low."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsp::B1)
    }
}
#[doc = "On Demand Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ondem {
    #[doc = "0: Internal frame sync is generated continuously."]
    B0 = 0,
    #[doc = "1: Internal frame sync is generated when the FIFO warning flag is clear."]
    B1 = 1,
}
impl From<Ondem> for bool {
    #[inline(always)]
    fn from(variant: Ondem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONDEM` reader - On Demand Mode"]
pub type OndemR = crate::BitReader<Ondem>;
impl OndemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ondem {
        match self.bits {
            false => Ondem::B0,
            true => Ondem::B1,
        }
    }
    #[doc = "Internal frame sync is generated continuously."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Ondem::B0
    }
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Ondem::B1
    }
}
#[doc = "Field `ONDEM` writer - On Demand Mode"]
pub type OndemW<'a, REG> = crate::BitWriter<'a, REG, Ondem>;
impl<'a, REG> OndemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal frame sync is generated continuously."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Ondem::B0)
    }
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Ondem::B1)
    }
}
#[doc = "Frame Sync Early\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fse {
    #[doc = "0: Frame sync asserts with the first bit of the frame."]
    B0 = 0,
    #[doc = "1: Frame sync asserts one bit before the first bit of the frame."]
    B1 = 1,
}
impl From<Fse> for bool {
    #[inline(always)]
    fn from(variant: Fse) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FSE` reader - Frame Sync Early"]
pub type FseR = crate::BitReader<Fse>;
impl FseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fse {
        match self.bits {
            false => Fse::B0,
            true => Fse::B1,
        }
    }
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fse::B0
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fse::B1
    }
}
#[doc = "Field `FSE` writer - Frame Sync Early"]
pub type FseW<'a, REG> = crate::BitWriter<'a, REG, Fse>;
impl<'a, REG> FseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fse::B0)
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fse::B1)
    }
}
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mf {
    #[doc = "0: LSB is received first."]
    B0 = 0,
    #[doc = "1: MSB is received first."]
    B1 = 1,
}
impl From<Mf> for bool {
    #[inline(always)]
    fn from(variant: Mf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MF` reader - MSB First"]
pub type MfR = crate::BitReader<Mf>;
impl MfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mf {
        match self.bits {
            false => Mf::B0,
            true => Mf::B1,
        }
    }
    #[doc = "LSB is received first."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Mf::B0
    }
    #[doc = "MSB is received first."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Mf::B1
    }
}
#[doc = "Field `MF` writer - MSB First"]
pub type MfW<'a, REG> = crate::BitWriter<'a, REG, Mf>;
impl<'a, REG> MfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LSB is received first."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Mf::B0)
    }
    #[doc = "MSB is received first."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Mf::B1)
    }
}
#[doc = "Field `SYWD` reader - Sync Width"]
pub type SywdR = crate::FieldReader;
#[doc = "Field `SYWD` writer - Sync Width"]
pub type SywdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FRSZ` reader - Frame Size"]
pub type FrszR = crate::BitReader;
#[doc = "Field `FRSZ` writer - Frame Size"]
pub type FrszW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FIFO Packing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fpack {
    #[doc = "0: FIFO packing is disabled"]
    B00 = 0,
    #[doc = "2: 8-bit FIFO packing is enabled"]
    B10 = 2,
    #[doc = "3: 16-bit FIFO packing is enabled"]
    B11 = 3,
}
impl From<Fpack> for u8 {
    #[inline(always)]
    fn from(variant: Fpack) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fpack {
    type Ux = u8;
}
impl crate::IsEnum for Fpack {}
#[doc = "Field `FPACK` reader - FIFO Packing Mode"]
pub type FpackR = crate::FieldReader<Fpack>;
impl FpackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Fpack> {
        match self.bits {
            0 => Some(Fpack::B00),
            2 => Some(Fpack::B10),
            3 => Some(Fpack::B11),
            _ => None,
        }
    }
    #[doc = "FIFO packing is disabled"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Fpack::B00
    }
    #[doc = "8-bit FIFO packing is enabled"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Fpack::B10
    }
    #[doc = "16-bit FIFO packing is enabled"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Fpack::B11
    }
}
#[doc = "Field `FPACK` writer - FIFO Packing Mode"]
pub type FpackW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fpack>;
impl<'a, REG> FpackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "FIFO packing is disabled"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Fpack::B00)
    }
    #[doc = "8-bit FIFO packing is enabled"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Fpack::B10)
    }
    #[doc = "16-bit FIFO packing is enabled"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Fpack::B11)
    }
}
#[doc = "FIFO Continue on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fcont {
    #[doc = "0: On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    B0 = 0,
    #[doc = "1: On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    B1 = 1,
}
impl From<Fcont> for bool {
    #[inline(always)]
    fn from(variant: Fcont) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FCONT` reader - FIFO Continue on Error"]
pub type FcontR = crate::BitReader<Fcont>;
impl FcontR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcont {
        match self.bits {
            false => Fcont::B0,
            true => Fcont::B1,
        }
    }
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Fcont::B0
    }
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Fcont::B1
    }
}
#[doc = "Field `FCONT` writer - FIFO Continue on Error"]
pub type FcontW<'a, REG> = crate::BitWriter<'a, REG, Fcont>;
impl<'a, REG> FcontW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Fcont::B0)
    }
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Fcont::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&self) -> FsdR {
        FsdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&self) -> FspR {
        FspR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline(always)]
    pub fn ondem(&self) -> OndemR {
        OndemR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&self) -> FseR {
        FseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&self) -> MfR {
        MfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&self) -> SywdR {
        SywdR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Frame Size"]
    #[inline(always)]
    pub fn frsz(&self) -> FrszR {
        FrszR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline(always)]
    pub fn fpack(&self) -> FpackR {
        FpackR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline(always)]
    pub fn fcont(&self) -> FcontR {
        FcontR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    #[must_use]
    pub fn fsd(&mut self) -> FsdW<Rcr4Spec> {
        FsdW::new(self, 0)
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn fsp(&mut self) -> FspW<Rcr4Spec> {
        FspW::new(self, 1)
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ondem(&mut self) -> OndemW<Rcr4Spec> {
        OndemW::new(self, 2)
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    #[must_use]
    pub fn fse(&mut self) -> FseW<Rcr4Spec> {
        FseW::new(self, 3)
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    #[must_use]
    pub fn mf(&mut self) -> MfW<Rcr4Spec> {
        MfW::new(self, 4)
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    #[must_use]
    pub fn sywd(&mut self) -> SywdW<Rcr4Spec> {
        SywdW::new(self, 8)
    }
    #[doc = "Bit 16 - Frame Size"]
    #[inline(always)]
    #[must_use]
    pub fn frsz(&mut self) -> FrszW<Rcr4Spec> {
        FrszW::new(self, 16)
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpack(&mut self) -> FpackW<Rcr4Spec> {
        FpackW::new(self, 24)
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline(always)]
    #[must_use]
    pub fn fcont(&mut self) -> FcontW<Rcr4Spec> {
        FcontW::new(self, 28)
    }
}
#[doc = "SAI Receive Configuration 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rcr4Spec;
impl crate::RegisterSpec for Rcr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr4::R`](R) reader structure"]
impl crate::Readable for Rcr4Spec {}
#[doc = "`write(|w| ..)` method takes [`rcr4::W`](W) writer structure"]
impl crate::Writable for Rcr4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR4 to value 0"]
impl crate::Resettable for Rcr4Spec {
    const RESET_VALUE: u32 = 0;
}
