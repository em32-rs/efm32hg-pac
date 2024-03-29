#[doc = "Register `CHENS` writer"]
pub struct W(crate::W<CHENS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHENS_SPEC>;
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
impl From<crate::W<CHENS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHENS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0ENS` writer - Channel 0 Enable Set"]
pub type CH0ENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, O>;
#[doc = "Field `CH1ENS` writer - Channel 1 Enable Set"]
pub type CH1ENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, O>;
#[doc = "Field `CH2ENS` writer - Channel 2 Enable Set"]
pub type CH2ENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, O>;
#[doc = "Field `CH3ENS` writer - Channel 3 Enable Set"]
pub type CH3ENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, O>;
#[doc = "Field `CH4ENS` writer - Channel 4 Enable Set"]
pub type CH4ENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, O>;
#[doc = "Field `CH5ENS` writer - Channel 5 Enable Set"]
pub type CH5ENS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHENS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Enable Set"]
    #[inline(always)]
    pub fn ch0ens(&mut self) -> CH0ENS_W<0> {
        CH0ENS_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Enable Set"]
    #[inline(always)]
    pub fn ch1ens(&mut self) -> CH1ENS_W<1> {
        CH1ENS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Enable Set"]
    #[inline(always)]
    pub fn ch2ens(&mut self) -> CH2ENS_W<2> {
        CH2ENS_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Enable Set"]
    #[inline(always)]
    pub fn ch3ens(&mut self) -> CH3ENS_W<3> {
        CH3ENS_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Enable Set"]
    #[inline(always)]
    pub fn ch4ens(&mut self) -> CH4ENS_W<4> {
        CH4ENS_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Enable Set"]
    #[inline(always)]
    pub fn ch5ens(&mut self) -> CH5ENS_W<5> {
        CH5ENS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chens](index.html) module"]
pub struct CHENS_SPEC;
impl crate::RegisterSpec for CHENS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chens::W](W) writer structure"]
impl crate::Writable for CHENS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHENS to value 0"]
impl crate::Resettable for CHENS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
