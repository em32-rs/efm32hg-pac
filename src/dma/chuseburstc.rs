#[doc = "Register `CHUSEBURSTC` writer"]
pub struct W(crate::W<CHUSEBURSTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHUSEBURSTC_SPEC>;
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
impl From<crate::W<CHUSEBURSTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHUSEBURSTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0USEBURSTC` writer - Channel 0 Useburst Clear"]
pub type CH0USEBURSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, O>;
#[doc = "Field `CH1USEBURSTC` writer - Channel 1 Useburst Clear"]
pub type CH1USEBURSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, O>;
#[doc = "Field `CH2USEBURSTC` writer - Channel 2 Useburst Clear"]
pub type CH2USEBURSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, O>;
#[doc = "Field `CH3USEBURSTC` writer - Channel 3 Useburst Clear"]
pub type CH3USEBURSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, O>;
#[doc = "Field `CH4USEBURSTC` writer - Channel 4 Useburst Clear"]
pub type CH4USEBURSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, O>;
#[doc = "Field `CH5USEBURSTC` writer - Channel 5 Useburst Clear"]
pub type CH5USEBURSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Useburst Clear"]
    #[inline(always)]
    pub fn ch0useburstc(&mut self) -> CH0USEBURSTC_W<0> {
        CH0USEBURSTC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Clear"]
    #[inline(always)]
    pub fn ch1useburstc(&mut self) -> CH1USEBURSTC_W<1> {
        CH1USEBURSTC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Clear"]
    #[inline(always)]
    pub fn ch2useburstc(&mut self) -> CH2USEBURSTC_W<2> {
        CH2USEBURSTC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Clear"]
    #[inline(always)]
    pub fn ch3useburstc(&mut self) -> CH3USEBURSTC_W<3> {
        CH3USEBURSTC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Clear"]
    #[inline(always)]
    pub fn ch4useburstc(&mut self) -> CH4USEBURSTC_W<4> {
        CH4USEBURSTC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Clear"]
    #[inline(always)]
    pub fn ch5useburstc(&mut self) -> CH5USEBURSTC_W<5> {
        CH5USEBURSTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Useburst Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chuseburstc](index.html) module"]
pub struct CHUSEBURSTC_SPEC;
impl crate::RegisterSpec for CHUSEBURSTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chuseburstc::W](W) writer structure"]
impl crate::Writable for CHUSEBURSTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHUSEBURSTC to value 0"]
impl crate::Resettable for CHUSEBURSTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
