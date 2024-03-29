#[doc = "Register `CHPRIC` writer"]
pub struct W(crate::W<CHPRIC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPRIC_SPEC>;
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
impl From<crate::W<CHPRIC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPRIC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0PRIC` writer - Channel 0 High Priority Clear"]
pub type CH0PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH1PRIC` writer - Channel 1 High Priority Clear"]
pub type CH1PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH2PRIC` writer - Channel 2 High Priority Clear"]
pub type CH2PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH3PRIC` writer - Channel 3 High Priority Clear"]
pub type CH3PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH4PRIC` writer - Channel 4 High Priority Clear"]
pub type CH4PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
#[doc = "Field `CH5PRIC` writer - Channel 5 High Priority Clear"]
pub type CH5PRIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHPRIC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 High Priority Clear"]
    #[inline(always)]
    pub fn ch0pric(&mut self) -> CH0PRIC_W<0> {
        CH0PRIC_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 High Priority Clear"]
    #[inline(always)]
    pub fn ch1pric(&mut self) -> CH1PRIC_W<1> {
        CH1PRIC_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 High Priority Clear"]
    #[inline(always)]
    pub fn ch2pric(&mut self) -> CH2PRIC_W<2> {
        CH2PRIC_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 High Priority Clear"]
    #[inline(always)]
    pub fn ch3pric(&mut self) -> CH3PRIC_W<3> {
        CH3PRIC_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 High Priority Clear"]
    #[inline(always)]
    pub fn ch4pric(&mut self) -> CH4PRIC_W<4> {
        CH4PRIC_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 High Priority Clear"]
    #[inline(always)]
    pub fn ch5pric(&mut self) -> CH5PRIC_W<5> {
        CH5PRIC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Priority Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpric](index.html) module"]
pub struct CHPRIC_SPEC;
impl crate::RegisterSpec for CHPRIC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chpric::W](W) writer structure"]
impl crate::Writable for CHPRIC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHPRIC to value 0"]
impl crate::Resettable for CHPRIC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
