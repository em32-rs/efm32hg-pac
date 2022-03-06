#[doc = "Register `EXTIRISE` reader"]
pub struct R(crate::R<EXTIRISE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIRISE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIRISE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIRISE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIRISE` writer"]
pub struct W(crate::W<EXTIRISE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIRISE_SPEC>;
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
impl From<crate::W<EXTIRISE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIRISE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTIRISE` reader - External Interrupt n Rising Edge Trigger Enable"]
pub struct EXTIRISE_R(crate::FieldReader<u16, u16>);
impl EXTIRISE_R {
    pub(crate) fn new(bits: u16) -> Self {
        EXTIRISE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTIRISE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIRISE` writer - External Interrupt n Rising Edge Trigger Enable"]
pub struct EXTIRISE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIRISE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - External Interrupt n Rising Edge Trigger Enable"]
    #[inline(always)]
    pub fn extirise(&self) -> EXTIRISE_R {
        EXTIRISE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt n Rising Edge Trigger Enable"]
    #[inline(always)]
    pub fn extirise(&mut self) -> EXTIRISE_W {
        EXTIRISE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Rising Edge Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extirise](index.html) module"]
pub struct EXTIRISE_SPEC;
impl crate::RegisterSpec for EXTIRISE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extirise::R](R) reader structure"]
impl crate::Readable for EXTIRISE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extirise::W](W) writer structure"]
impl crate::Writable for EXTIRISE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTIRISE to value 0"]
impl crate::Resettable for EXTIRISE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
