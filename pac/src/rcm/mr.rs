#[doc = "Register `MR` reader"]
pub type R = crate::R<MrSpec>;
#[doc = "Register `MR` writer"]
pub type W = crate::W<MrSpec>;
#[doc = "Boot ROM Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bootrom {
    #[doc = "0: Boot from Flash"]
    B00 = 0,
    #[doc = "1: Boot from ROM due to BOOTCFG0 pin assertion"]
    B01 = 1,
    #[doc = "2: Boot form ROM due to FOPT\\[7\\]
configuration"]
    B10 = 2,
    #[doc = "3: Boot from ROM due to both BOOTCFG0 pin assertion and FOPT\\[7\\]
configuration"]
    B11 = 3,
}
impl From<Bootrom> for u8 {
    #[inline(always)]
    fn from(variant: Bootrom) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bootrom {
    type Ux = u8;
}
impl crate::IsEnum for Bootrom {}
#[doc = "Field `BOOTROM` reader - Boot ROM Configuration"]
pub type BootromR = crate::FieldReader<Bootrom>;
impl BootromR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bootrom {
        match self.bits {
            0 => Bootrom::B00,
            1 => Bootrom::B01,
            2 => Bootrom::B10,
            3 => Bootrom::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Boot from Flash"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Bootrom::B00
    }
    #[doc = "Boot from ROM due to BOOTCFG0 pin assertion"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Bootrom::B01
    }
    #[doc = "Boot form ROM due to FOPT\\[7\\]
configuration"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Bootrom::B10
    }
    #[doc = "Boot from ROM due to both BOOTCFG0 pin assertion and FOPT\\[7\\]
configuration"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Bootrom::B11
    }
}
#[doc = "Field `BOOTROM` writer - Boot ROM Configuration"]
pub type BootromW<'a, REG> = crate::FieldWriter<'a, REG, 2, Bootrom, crate::Safe>;
impl<'a, REG> BootromW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Boot from Flash"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Bootrom::B00)
    }
    #[doc = "Boot from ROM due to BOOTCFG0 pin assertion"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Bootrom::B01)
    }
    #[doc = "Boot form ROM due to FOPT\\[7\\]
configuration"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Bootrom::B10)
    }
    #[doc = "Boot from ROM due to both BOOTCFG0 pin assertion and FOPT\\[7\\]
configuration"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Bootrom::B11)
    }
}
impl R {
    #[doc = "Bits 1:2 - Boot ROM Configuration"]
    #[inline(always)]
    pub fn bootrom(&self) -> BootromR {
        BootromR::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bits 1:2 - Boot ROM Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn bootrom(&mut self) -> BootromW<MrSpec> {
        BootromW::new(self, 1)
    }
}
#[doc = "Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrSpec;
impl crate::RegisterSpec for MrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mr::R`](R) reader structure"]
impl crate::Readable for MrSpec {}
#[doc = "`write(|w| ..)` method takes [`mr::W`](W) writer structure"]
impl crate::Writable for MrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MrSpec {
    const RESET_VALUE: u8 = 0;
}
