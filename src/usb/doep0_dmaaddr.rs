#[doc = "Register `DOEP0_DMAADDR` reader"]
pub struct R(crate::R<DOEP0_DMAADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP0_DMAADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP0_DMAADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP0_DMAADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP0_DMAADDR` writer"]
pub struct W(crate::W<DOEP0_DMAADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP0_DMAADDR_SPEC>;
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
impl From<crate::W<DOEP0_DMAADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP0_DMAADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR` reader - DMA Address"]
pub type DMAADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAADDR` writer - DMA Address"]
pub type DMAADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DOEP0_DMAADDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<0> {
        DMAADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device OUT Endpoint x+1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0_dmaaddr](index.html) module"]
pub struct DOEP0_DMAADDR_SPEC;
impl crate::RegisterSpec for DOEP0_DMAADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep0_dmaaddr::R](R) reader structure"]
impl crate::Readable for DOEP0_DMAADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep0_dmaaddr::W](W) writer structure"]
impl crate::Writable for DOEP0_DMAADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEP0_DMAADDR to value 0"]
impl crate::Resettable for DOEP0_DMAADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
