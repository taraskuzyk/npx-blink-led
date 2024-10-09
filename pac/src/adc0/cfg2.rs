#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Long Sample Time Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adlsts {
    #[doc = "0: Default longest sample time; 20 extra ADCK cycles; 24 ADCK cycles total."]
    B00 = 0,
    #[doc = "1: 12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    B01 = 1,
    #[doc = "2: 6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    B10 = 2,
    #[doc = "3: 2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    B11 = 3,
}
impl From<Adlsts> for u8 {
    #[inline(always)]
    fn from(variant: Adlsts) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adlsts {
    type Ux = u8;
}
impl crate::IsEnum for Adlsts {}
#[doc = "Field `ADLSTS` reader - Long Sample Time Select"]
pub type AdlstsR = crate::FieldReader<Adlsts>;
impl AdlstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adlsts {
        match self.bits {
            0 => Adlsts::B00,
            1 => Adlsts::B01,
            2 => Adlsts::B10,
            3 => Adlsts::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Default longest sample time; 20 extra ADCK cycles; 24 ADCK cycles total."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Adlsts::B00
    }
    #[doc = "12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Adlsts::B01
    }
    #[doc = "6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Adlsts::B10
    }
    #[doc = "2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Adlsts::B11
    }
}
#[doc = "Field `ADLSTS` writer - Long Sample Time Select"]
pub type AdlstsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adlsts, crate::Safe>;
impl<'a, REG> AdlstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Default longest sample time; 20 extra ADCK cycles; 24 ADCK cycles total."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsts::B00)
    }
    #[doc = "12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsts::B01)
    }
    #[doc = "6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsts::B10)
    }
    #[doc = "2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Adlsts::B11)
    }
}
#[doc = "High-Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adhsc {
    #[doc = "0: Normal conversion sequence selected."]
    B0 = 0,
    #[doc = "1: High-speed conversion sequence selected with 2 additional ADCK cycles to total conversion time."]
    B1 = 1,
}
impl From<Adhsc> for bool {
    #[inline(always)]
    fn from(variant: Adhsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADHSC` reader - High-Speed Configuration"]
pub type AdhscR = crate::BitReader<Adhsc>;
impl AdhscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adhsc {
        match self.bits {
            false => Adhsc::B0,
            true => Adhsc::B1,
        }
    }
    #[doc = "Normal conversion sequence selected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adhsc::B0
    }
    #[doc = "High-speed conversion sequence selected with 2 additional ADCK cycles to total conversion time."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adhsc::B1
    }
}
#[doc = "Field `ADHSC` writer - High-Speed Configuration"]
pub type AdhscW<'a, REG> = crate::BitWriter<'a, REG, Adhsc>;
impl<'a, REG> AdhscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal conversion sequence selected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adhsc::B0)
    }
    #[doc = "High-speed conversion sequence selected with 2 additional ADCK cycles to total conversion time."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adhsc::B1)
    }
}
#[doc = "Asynchronous Clock Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adacken {
    #[doc = "0: Asynchronous clock output disabled; Asynchronous clock is enabled only if selected by ADICLK and a conversion is active."]
    B0 = 0,
    #[doc = "1: Asynchronous clock and clock output is enabled regardless of the state of the ADC."]
    B1 = 1,
}
impl From<Adacken> for bool {
    #[inline(always)]
    fn from(variant: Adacken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADACKEN` reader - Asynchronous Clock Output Enable"]
pub type AdackenR = crate::BitReader<Adacken>;
impl AdackenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adacken {
        match self.bits {
            false => Adacken::B0,
            true => Adacken::B1,
        }
    }
    #[doc = "Asynchronous clock output disabled; Asynchronous clock is enabled only if selected by ADICLK and a conversion is active."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Adacken::B0
    }
    #[doc = "Asynchronous clock and clock output is enabled regardless of the state of the ADC."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Adacken::B1
    }
}
#[doc = "Field `ADACKEN` writer - Asynchronous Clock Output Enable"]
pub type AdackenW<'a, REG> = crate::BitWriter<'a, REG, Adacken>;
impl<'a, REG> AdackenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous clock output disabled; Asynchronous clock is enabled only if selected by ADICLK and a conversion is active."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Adacken::B0)
    }
    #[doc = "Asynchronous clock and clock output is enabled regardless of the state of the ADC."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Adacken::B1)
    }
}
#[doc = "ADC Mux Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Muxsel {
    #[doc = "0: ADxxa channels are selected."]
    B0 = 0,
    #[doc = "1: ADxxb channels are selected."]
    B1 = 1,
}
impl From<Muxsel> for bool {
    #[inline(always)]
    fn from(variant: Muxsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUXSEL` reader - ADC Mux Select"]
pub type MuxselR = crate::BitReader<Muxsel>;
impl MuxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Muxsel {
        match self.bits {
            false => Muxsel::B0,
            true => Muxsel::B1,
        }
    }
    #[doc = "ADxxa channels are selected."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == Muxsel::B0
    }
    #[doc = "ADxxb channels are selected."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == Muxsel::B1
    }
}
#[doc = "Field `MUXSEL` writer - ADC Mux Select"]
pub type MuxselW<'a, REG> = crate::BitWriter<'a, REG, Muxsel>;
impl<'a, REG> MuxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ADxxa channels are selected."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::B0)
    }
    #[doc = "ADxxb channels are selected."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxsel::B1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Long Sample Time Select"]
    #[inline(always)]
    pub fn adlsts(&self) -> AdlstsR {
        AdlstsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - High-Speed Configuration"]
    #[inline(always)]
    pub fn adhsc(&self) -> AdhscR {
        AdhscR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronous Clock Output Enable"]
    #[inline(always)]
    pub fn adacken(&self) -> AdackenR {
        AdackenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Mux Select"]
    #[inline(always)]
    pub fn muxsel(&self) -> MuxselR {
        MuxselR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Long Sample Time Select"]
    #[inline(always)]
    #[must_use]
    pub fn adlsts(&mut self) -> AdlstsW<Cfg2Spec> {
        AdlstsW::new(self, 0)
    }
    #[doc = "Bit 2 - High-Speed Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adhsc(&mut self) -> AdhscW<Cfg2Spec> {
        AdhscW::new(self, 2)
    }
    #[doc = "Bit 3 - Asynchronous Clock Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adacken(&mut self) -> AdackenW<Cfg2Spec> {
        AdackenW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC Mux Select"]
    #[inline(always)]
    #[must_use]
    pub fn muxsel(&mut self) -> MuxselW<Cfg2Spec> {
        MuxselW::new(self, 4)
    }
}
#[doc = "ADC Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for Cfg2Spec {
    const RESET_VALUE: u32 = 0;
}
