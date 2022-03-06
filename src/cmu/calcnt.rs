#[doc = "Register `CALCNT` reader"]
pub struct R(crate::R<CALCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALCNT` writer"]
pub struct W(crate::W<CALCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALCNT_SPEC>;
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
impl From<crate::W<CALCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALCNT` reader - Calibration Counter"]
pub struct CALCNT_R(crate::FieldReader<u32, u32>);
impl CALCNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CALCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALCNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALCNT` writer - Calibration Counter"]
pub struct CALCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | (value as u32 & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - Calibration Counter"]
    #[inline(always)]
    pub fn calcnt(&self) -> CALCNT_R {
        CALCNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Calibration Counter"]
    #[inline(always)]
    pub fn calcnt(&mut self) -> CALCNT_W {
        CALCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calcnt](index.html) module"]
pub struct CALCNT_SPEC;
impl crate::RegisterSpec for CALCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calcnt::R](R) reader structure"]
impl crate::Readable for CALCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calcnt::W](W) writer structure"]
impl crate::Writable for CALCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALCNT to value 0"]
impl crate::Resettable for CALCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
