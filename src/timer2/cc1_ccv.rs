#[doc = "Register `CC1_CCV` reader"]
pub struct R(crate::R<CC1_CCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC1_CCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC1_CCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC1_CCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC1_CCV` writer"]
pub struct W(crate::W<CC1_CCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC1_CCV_SPEC>;
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
impl From<crate::W<CC1_CCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC1_CCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCV` reader - CC Channel Value"]
pub struct CCV_R(crate::FieldReader<u16, u16>);
impl CCV_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCV_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCV` writer - CC Channel Value"]
pub struct CCV_W<'a> {
    w: &'a mut W,
}
impl<'a> CCV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&self) -> CCV_R {
        CCV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value"]
    #[inline(always)]
    pub fn ccv(&mut self) -> CCV_W {
        CCV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Channel Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc1_ccv](index.html) module"]
pub struct CC1_CCV_SPEC;
impl crate::RegisterSpec for CC1_CCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc1_ccv::R](R) reader structure"]
impl crate::Readable for CC1_CCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc1_ccv::W](W) writer structure"]
impl crate::Writable for CC1_CCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC1_CCV to value 0"]
impl crate::Resettable for CC1_CCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
