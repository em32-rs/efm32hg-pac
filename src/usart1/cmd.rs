#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RXDIS` writer - Receiver Disable"]
pub type RXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TXDIS` writer - Transmitter Disable"]
pub type TXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `MASTEREN` writer - Master Enable"]
pub type MASTEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `MASTERDIS` writer - Master Disable"]
pub type MASTERDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RXBLOCKEN` writer - Receiver Block Enable"]
pub type RXBLOCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `RXBLOCKDIS` writer - Receiver Block Disable"]
pub type RXBLOCKDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TXTRIEN` writer - Transmitter Tristate Enable"]
pub type TXTRIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `TXTRIDIS` writer - Transmitter Tristate Disable"]
pub type TXTRIDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CLEARTX` writer - Clear TX"]
pub type CLEARTX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
#[doc = "Field `CLEARRX` writer - Clear RX"]
pub type CLEARRX_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RXEN_W<0> {
        RXEN_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Disable"]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RXDIS_W<1> {
        RXDIS_W::new(self)
    }
    #[doc = "Bit 2 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TXEN_W<2> {
        TXEN_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Disable"]
    #[inline(always)]
    pub fn txdis(&mut self) -> TXDIS_W<3> {
        TXDIS_W::new(self)
    }
    #[doc = "Bit 4 - Master Enable"]
    #[inline(always)]
    pub fn masteren(&mut self) -> MASTEREN_W<4> {
        MASTEREN_W::new(self)
    }
    #[doc = "Bit 5 - Master Disable"]
    #[inline(always)]
    pub fn masterdis(&mut self) -> MASTERDIS_W<5> {
        MASTERDIS_W::new(self)
    }
    #[doc = "Bit 6 - Receiver Block Enable"]
    #[inline(always)]
    pub fn rxblocken(&mut self) -> RXBLOCKEN_W<6> {
        RXBLOCKEN_W::new(self)
    }
    #[doc = "Bit 7 - Receiver Block Disable"]
    #[inline(always)]
    pub fn rxblockdis(&mut self) -> RXBLOCKDIS_W<7> {
        RXBLOCKDIS_W::new(self)
    }
    #[doc = "Bit 8 - Transmitter Tristate Enable"]
    #[inline(always)]
    pub fn txtrien(&mut self) -> TXTRIEN_W<8> {
        TXTRIEN_W::new(self)
    }
    #[doc = "Bit 9 - Transmitter Tristate Disable"]
    #[inline(always)]
    pub fn txtridis(&mut self) -> TXTRIDIS_W<9> {
        TXTRIDIS_W::new(self)
    }
    #[doc = "Bit 10 - Clear TX"]
    #[inline(always)]
    pub fn cleartx(&mut self) -> CLEARTX_W<10> {
        CLEARTX_W::new(self)
    }
    #[doc = "Bit 11 - Clear RX"]
    #[inline(always)]
    pub fn clearrx(&mut self) -> CLEARRX_W<11> {
        CLEARRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
