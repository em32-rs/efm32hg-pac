#[doc = "Register `SIGFRAME` reader"]
pub struct R(crate::R<SIGFRAME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGFRAME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGFRAME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGFRAME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGFRAME` writer"]
pub struct W(crate::W<SIGFRAME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGFRAME_SPEC>;
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
impl From<crate::W<SIGFRAME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGFRAME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGFRAME` reader - Signal Frame"]
pub struct SIGFRAME_R(crate::FieldReader<u16, u16>);
impl SIGFRAME_R {
    pub(crate) fn new(bits: u16) -> Self {
        SIGFRAME_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIGFRAME_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGFRAME` writer - Signal Frame"]
pub struct SIGFRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGFRAME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Signal Frame"]
    #[inline(always)]
    pub fn sigframe(&self) -> SIGFRAME_R {
        SIGFRAME_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Signal Frame"]
    #[inline(always)]
    pub fn sigframe(&mut self) -> SIGFRAME_W {
        SIGFRAME_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Signal Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigframe](index.html) module"]
pub struct SIGFRAME_SPEC;
impl crate::RegisterSpec for SIGFRAME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigframe::R](R) reader structure"]
impl crate::Readable for SIGFRAME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigframe::W](W) writer structure"]
impl crate::Writable for SIGFRAME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIGFRAME to value 0"]
impl crate::Resettable for SIGFRAME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
