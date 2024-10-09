#[doc = "Register `CLK_RECOVER_IRC_EN` reader"]
pub type R = crate::R<ClkRecoverIrcEnSpec>;
#[doc = "Register `CLK_RECOVER_IRC_EN` writer"]
pub type W = crate::W<ClkRecoverIrcEnSpec>;
#[doc = "IRC48M enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrcEn {
    #[doc = "0: Disable the IRC48M module (default)"]
    B0 = 0,
    #[doc = "1: Enable the IRC48M module"]
    B1 = 1,
}
impl From<IrcEn> for bool {
    #[inline(always)]
    fn from(variant: IrcEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRC_EN` reader - IRC48M enable"]
pub type IrcEnR = crate::BitReader<IrcEn>;
impl IrcEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrcEn {
        match self.bits {
            false => IrcEn::B0,
            true => IrcEn::B1,
        }
    }
    #[doc = "Disable the IRC48M module (default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IrcEn::B0
    }
    #[doc = "Enable the IRC48M module"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IrcEn::B1
    }
}
#[doc = "Field `IRC_EN` writer - IRC48M enable"]
pub type IrcEnW<'a, REG> = crate::BitWriter<'a, REG, IrcEn>;
impl<'a, REG> IrcEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the IRC48M module (default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IrcEn::B0)
    }
    #[doc = "Enable the IRC48M module"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IrcEn::B1)
    }
}
impl R {
    #[doc = "Bit 1 - IRC48M enable"]
    #[inline(always)]
    pub fn irc_en(&self) -> IrcEnR {
        IrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - IRC48M enable"]
    #[inline(always)]
    #[must_use]
    pub fn irc_en(&mut self) -> IrcEnW<ClkRecoverIrcEnSpec> {
        IrcEnW::new(self, 1)
    }
}
#[doc = "IRC48M oscillator enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_recover_irc_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_recover_irc_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkRecoverIrcEnSpec;
impl crate::RegisterSpec for ClkRecoverIrcEnSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`clk_recover_irc_en::R`](R) reader structure"]
impl crate::Readable for ClkRecoverIrcEnSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_recover_irc_en::W`](W) writer structure"]
impl crate::Writable for ClkRecoverIrcEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CLK_RECOVER_IRC_EN to value 0x01"]
impl crate::Resettable for ClkRecoverIrcEnSpec {
    const RESET_VALUE: u8 = 0x01;
}
