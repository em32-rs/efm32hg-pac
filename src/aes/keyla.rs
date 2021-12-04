#[doc = "Register `KEYLA` reader"]
pub struct R(crate::R<KEYLA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYLA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYLA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYLA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYLA` writer"]
pub struct W(crate::W<KEYLA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYLA_SPEC>;
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
impl From<crate::W<KEYLA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYLA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYLA` reader - Key Low Access A"]
pub struct KEYLA_R(crate::FieldReader<u32, u32>);
impl KEYLA_R {
    pub(crate) fn new(bits: u32) -> Self {
        KEYLA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEYLA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEYLA` writer - Key Low Access A"]
pub struct KEYLA_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYLA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Key Low Access A"]
    #[inline(always)]
    pub fn keyla(&self) -> KEYLA_R {
        KEYLA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Low Access A"]
    #[inline(always)]
    pub fn keyla(&mut self) -> KEYLA_W {
        KEYLA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keyla](index.html) module"]
pub struct KEYLA_SPEC;
impl crate::RegisterSpec for KEYLA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keyla::R](R) reader structure"]
impl crate::Readable for KEYLA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keyla::W](W) writer structure"]
impl crate::Writable for KEYLA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYLA to value 0"]
impl crate::Resettable for KEYLA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
