#[doc = "Register `S2` reader"]
pub type R = crate::R<S2Spec>;
#[doc = "Register `S2` writer"]
pub type W = crate::W<S2Spec>;
#[doc = "Empty flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Empty {
    #[doc = "0: Tx or Rx buffer is not empty and cannot be written to, that is new data cannot be loaded into the buffer."]
    B0 = 0,
    #[doc = "1: Tx or Rx buffer is empty and can be written to, that is new data can be loaded into the buffer."]
    B1 = 1,
}
impl From<Empty> for bool {
    #[inline(always)]
    fn from(variant: Empty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMPTY` reader - Empty flag"]
pub type EmptyR = crate::BitReader<Empty>;
impl EmptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Empty {
        match self.bits {
            false => Empty::B0,
            true => Empty::B1,
        }
    }
    #[doc = "Tx or Rx buffer is not empty and cannot be written to, that is new data cannot be loaded into the buffer."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Empty::B0
    }
    #[doc = "Tx or Rx buffer is empty and can be written to, that is new data can be loaded into the buffer."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Empty::B1
    }
}
#[doc = "Error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: The buffer is not full and all write/read operations have no errors."]
    B0 = 0,
    #[doc = "1: There are 3 or more write/read errors during the data transfer phase (when the Empty flag is not set and the buffer is busy)."]
    B1 = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Error flag"]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::B0,
            true => Error::B1,
        }
    }
    #[doc = "The buffer is not full and all write/read operations have no errors."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Error::B0
    }
    #[doc = "There are 3 or more write/read errors during the data transfer phase (when the Empty flag is not set and the buffer is busy)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Error::B1
    }
}
#[doc = "Field `ERROR` writer - Error flag"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, Error>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The buffer is not full and all write/read operations have no errors."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Error::B0)
    }
    #[doc = "There are 3 or more write/read errors during the data transfer phase (when the Empty flag is not set and the buffer is busy)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Error::B1)
    }
}
impl R {
    #[doc = "Bit 0 - Empty flag"]
    #[inline(always)]
    pub fn empty(&self) -> EmptyR {
        EmptyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error flag"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Error flag"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ErrorW<S2Spec> {
        ErrorW::new(self, 1)
    }
}
#[doc = "I2C Status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`s2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2Spec;
impl crate::RegisterSpec for S2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`s2::R`](R) reader structure"]
impl crate::Readable for S2Spec {}
#[doc = "`write(|w| ..)` method takes [`s2::W`](W) writer structure"]
impl crate::Writable for S2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets S2 to value 0x01"]
impl crate::Resettable for S2Spec {
    const RESET_VALUE: u8 = 0x01;
}
