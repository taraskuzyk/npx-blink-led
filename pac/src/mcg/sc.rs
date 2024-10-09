#[doc = "Register `SC` reader"]
pub type R = crate::R<ScSpec>;
#[doc = "Register `SC` writer"]
pub type W = crate::W<ScSpec>;
#[doc = "Low-frequency Internal Reference Clock Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fcrdiv {
    #[doc = "0: Division factor is 1."]
    B000 = 0,
    #[doc = "1: Division factor is 2."]
    B001 = 1,
    #[doc = "2: Division factor is 4."]
    B010 = 2,
    #[doc = "3: Division factor is 8."]
    B011 = 3,
    #[doc = "4: Division factor is 16."]
    B100 = 4,
    #[doc = "5: Division factor is 32."]
    B101 = 5,
    #[doc = "6: Division factor is 64."]
    B110 = 6,
    #[doc = "7: Division factor is 128."]
    B111 = 7,
}
impl From<Fcrdiv> for u8 {
    #[inline(always)]
    fn from(variant: Fcrdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fcrdiv {
    type Ux = u8;
}
impl crate::IsEnum for Fcrdiv {}
#[doc = "Field `FCRDIV` reader - Low-frequency Internal Reference Clock Divider"]
pub type FcrdivR = crate::FieldReader<Fcrdiv>;
impl FcrdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fcrdiv {
        match self.bits {
            0 => Fcrdiv::B000,
            1 => Fcrdiv::B001,
            2 => Fcrdiv::B010,
            3 => Fcrdiv::B011,
            4 => Fcrdiv::B100,
            5 => Fcrdiv::B101,
            6 => Fcrdiv::B110,
            7 => Fcrdiv::B111,
            _ => unreachable!(),
        }
    }
    #[doc = "Division factor is 1."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Fcrdiv::B000
    }
    #[doc = "Division factor is 2."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Fcrdiv::B001
    }
    #[doc = "Division factor is 4."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Fcrdiv::B010
    }
    #[doc = "Division factor is 8."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Fcrdiv::B011
    }
    #[doc = "Division factor is 16."]
    #[inline(always)]
    pub fn is_b100(&self) -> bool {
        *self == Fcrdiv::B100
    }
    #[doc = "Division factor is 32."]
    #[inline(always)]
    pub fn is_b101(&self) -> bool {
        *self == Fcrdiv::B101
    }
    #[doc = "Division factor is 64."]
    #[inline(always)]
    pub fn is_b110(&self) -> bool {
        *self == Fcrdiv::B110
    }
    #[doc = "Division factor is 128."]
    #[inline(always)]
    pub fn is_b111(&self) -> bool {
        *self == Fcrdiv::B111
    }
}
#[doc = "Field `FCRDIV` writer - Low-frequency Internal Reference Clock Divider"]
pub type FcrdivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Fcrdiv, crate::Safe>;
impl<'a, REG> FcrdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Division factor is 1."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Fcrdiv::B000)
    }
    #[doc = "Division factor is 2."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Fcrdiv::B001)
    }
    #[doc = "Division factor is 4."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Fcrdiv::B010)
    }
    #[doc = "Division factor is 8."]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Fcrdiv::B011)
    }
    #[doc = "Division factor is 16."]
    #[inline(always)]
    pub fn b100(self) -> &'a mut crate::W<REG> {
        self.variant(Fcrdiv::B100)
    }
    #[doc = "Division factor is 32."]
    #[inline(always)]
    pub fn b101(self) -> &'a mut crate::W<REG> {
        self.variant(Fcrdiv::B101)
    }
    #[doc = "Division factor is 64."]
    #[inline(always)]
    pub fn b110(self) -> &'a mut crate::W<REG> {
        self.variant(Fcrdiv::B110)
    }
    #[doc = "Division factor is 128."]
    #[inline(always)]
    pub fn b111(self) -> &'a mut crate::W<REG> {
        self.variant(Fcrdiv::B111)
    }
}
impl R {
    #[doc = "Bits 1:3 - Low-frequency Internal Reference Clock Divider"]
    #[inline(always)]
    pub fn fcrdiv(&self) -> FcrdivR {
        FcrdivR::new((self.bits >> 1) & 7)
    }
}
impl W {
    #[doc = "Bits 1:3 - Low-frequency Internal Reference Clock Divider"]
    #[inline(always)]
    #[must_use]
    pub fn fcrdiv(&mut self) -> FcrdivW<ScSpec> {
        FcrdivW::new(self, 1)
    }
}
#[doc = "MCG Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScSpec;
impl crate::RegisterSpec for ScSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sc::R`](R) reader structure"]
impl crate::Readable for ScSpec {}
#[doc = "`write(|w| ..)` method takes [`sc::W`](W) writer structure"]
impl crate::Writable for ScSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for ScSpec {
    const RESET_VALUE: u8 = 0;
}
